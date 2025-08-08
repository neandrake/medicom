/*
   Copyright 2024-2025 Christopher Speck

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

use crate::load::{pixeldata::{
    pdinfo::{PixelDataSliceInfo, I32_SIZE, U32_SIZE},
    pdwinlevel::WindowLevel,
    PhotoInterp, LoadError,
}, EPSILON_F32};

pub struct PixelDataSliceI32 {
    info: PixelDataSliceInfo,
    buffer: Vec<i32>,

    stride: usize,
    interp_as_rgb: bool,
}

impl std::fmt::Debug for PixelDataSliceI32 {
    // Default Debug implementation but don't print all bytes, just the length.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PixelDataBufferI32")
            .field("info", &self.info)
            .field("buffer.len", &self.buffer.len())
            .field("stride", &self.stride)
            .field("interp_as_rgb", &self.interp_as_rgb)
            .finish()
    }
}

impl PixelDataSliceI32 {
    /// Create `PixelDataSliceI32` from 32-bit monochrome slice data.
    ///
    /// # Errors
    /// - Any errors interpreting little/big -endian bytes as 32bit numbers.
    pub fn from_mono_32bit(mut pdinfo: PixelDataSliceInfo) -> Result<Self, LoadError> {
        let num_frames = usize::try_from(pdinfo.num_frames()).unwrap_or(1);
        let samples = usize::from(pdinfo.samples_per_pixel());
        let len = usize::from(pdinfo.cols()) * usize::from(pdinfo.rows()) * num_frames;
        let pixel_pad = pdinfo.pixel_pad().map(Into::<i32>::into);

        let mut buffer: Vec<i32> = Vec::with_capacity(len * samples);
        let mut in_pos: usize = 0;
        let mut min: i32 = i32::MAX;
        let mut max: i32 = i32::MIN;
        let bytes = pdinfo.take_bytes();
        for _i in 0..len {
            for _j in 0..samples {
                let val = if pdinfo.big_endian() {
                    if pdinfo.is_signed() {
                        let val = i32::from_be_bytes(bytes[in_pos..in_pos + I32_SIZE].try_into()?);
                        in_pos += I32_SIZE;
                        val
                    } else {
                        let val = u32::from_be_bytes(bytes[in_pos..in_pos + U32_SIZE].try_into()?)
                            .min(i32::MAX as u32) as i32;
                        in_pos += U32_SIZE;
                        val
                    }
                } else if pdinfo.is_signed() {
                    let val = i32::from_le_bytes(bytes[in_pos..in_pos + I32_SIZE].try_into()?);
                    in_pos += I32_SIZE;
                    val
                } else {
                    let val = u32::from_le_bytes(bytes[in_pos..in_pos + U32_SIZE].try_into()?)
                        .min(i32::MAX as u32) as i32;
                    in_pos += U32_SIZE;
                    val
                };

                buffer.push(val);
                if pixel_pad.is_none_or(|pad_val| val != pad_val) {
                    min = min.min(val);
                    max = max.max(val);
                }
            }
        }

        let min = min as f32;
        let max = max as f32;
        pdinfo.set_min_val(min);
        pdinfo.set_max_val(max);

        let minmax_width = max - min;
        let minmax_center = min + minmax_width / 2_f32;
        let mut already_has_minmax = false;
        for winlevel in pdinfo.win_levels_mut() {
            winlevel.set_out_min(i32::MIN as f32);
            winlevel.set_out_max(i32::MAX as f32);

            let same_width = (winlevel.width() - minmax_width).abs() < EPSILON_F32;
            let same_center = (winlevel.center() - minmax_center).abs() < EPSILON_F32;
            if same_width && same_center {
                already_has_minmax = true;
            }
        }
        if !already_has_minmax {
            pdinfo.win_levels_mut().push(WindowLevel::new(
                "Min/Max".to_string(),
                minmax_center,
                minmax_width,
                i32::MIN as f32,
                i32::MAX as f32,
            ));
        }
        Ok(PixelDataSliceI32::new(pdinfo, buffer))
    }

    #[must_use]
    pub fn new(info: PixelDataSliceInfo, buffer: Vec<i32>) -> Self {
        let stride = if info.planar_config() == 0 {
            1
        } else {
            buffer.len() / usize::from(info.samples_per_pixel())
        };
        let interp_as_rgb =
            info.photo_interp().is_some_and(PhotoInterp::is_rgb) && info.samples_per_pixel() == 3;

        Self {
            info,
            buffer,
            stride,
            interp_as_rgb,
        }
    }

    /// Consume this slice and convert into `Vec<i16>`, also returning the `PixelDataSliceInfo`.
    ///
    /// # Errors
    /// - `PixelValueError` if unable to convert the `i32` into `i16`.
    pub fn into_i16(self) -> Result<(PixelDataSliceInfo, Vec<i16>), LoadError> {
        let mut buffer: Vec<i16> = Vec::with_capacity(self.buffer.len());
        for v in &self.buffer {
            buffer.push(
                i16::try_from(*v).map_err(|e| LoadError::PixelValueError { source: e })?,
            );
        }
        Ok((self.info, buffer))
    }

    #[must_use]
    pub fn info(&self) -> &PixelDataSliceInfo {
        &self.info
    }

    #[must_use]
    pub fn buffer(&self) -> &[i32] {
        &self.buffer
    }

    #[must_use]
    pub fn into_buffer(self) -> Vec<i32> {
        self.buffer
    }

    #[must_use]
    pub fn stride(&self) -> usize {
        self.stride
    }

    #[must_use]
    pub fn rescale(&self, val: f32) -> f32 {
        if let Some(slope) = self.info().slope() {
            if let Some(intercept) = self.info().intercept() {
                return val * slope + intercept;
            }
        }
        val
    }

    #[must_use]
    pub fn best_winlevel(&self) -> WindowLevel {
        self.info
            .win_levels()
            // XXX: The window/level computed from the min/max values seems to be better than most
            //      window/levels specified in the dicom, at least prior to applying a color-table.
            .last()
            .map_or_else(
                || {
                    WindowLevel::new(
                        "Default".to_string(),
                        0_f32,
                        i32::MAX as f32,
                        i32::MIN as f32,
                        i32::MAX as f32,
                    )
                },
                |winlevel| {
                    WindowLevel::new(
                        winlevel.name().to_string(),
                        self.rescale(winlevel.center()),
                        self.rescale(winlevel.width()),
                        winlevel.out_min(),
                        winlevel.out_max(),
                    )
                },
            )
    }
}
