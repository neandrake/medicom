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

use crate::load::pixeldata::{
    pdinfo::{PixelDataSliceInfo, I16_SIZE, I8_SIZE, U16_SIZE},
    pdwinlevel::WindowLevel,
    PhotoInterp, PixelDataError,
};

pub struct PixelDataSliceI16 {
    info: PixelDataSliceInfo,
    buffer: Vec<i16>,

    stride: usize,
    interp_as_rgb: bool,
}

impl std::fmt::Debug for PixelDataSliceI16 {
    // Default Debug implementation but don't print all bytes, just the length.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PixelDataSliceI16")
            .field("info", &self.info)
            .field("buffer.len", &self.buffer.len())
            .field("stride", &self.stride)
            .field("interp_as_rgb", &self.interp_as_rgb)
            .finish()
    }
}

impl PixelDataSliceI16 {
    #[must_use]
    pub fn from_mono_8bit(mut pdinfo: PixelDataSliceInfo) -> Self {
        let num_frames = usize::try_from(pdinfo.num_frames()).unwrap_or(1);
        let samples = usize::from(pdinfo.samples_per_pixel());
        let len = usize::from(pdinfo.cols()) * usize::from(pdinfo.rows()) * num_frames;
        let pixel_pad = pdinfo
            .pixel_pad()
            .and_then(|pad_val| TryInto::<i16>::try_into(pad_val).ok());

        let mut buffer: Vec<i16> = Vec::with_capacity(len * samples);
        let mut in_pos: usize = 0;
        let mut min: i16 = i16::MAX;
        let mut max: i16 = i16::MIN;
        let bytes = pdinfo.take_bytes();
        for _i in 0..len {
            for _j in 0..samples {
                let val = i16::from(bytes[in_pos]);
                in_pos += I8_SIZE;

                buffer.push(val);
                if pixel_pad.is_none_or(|pad_val| val != pad_val) {
                    min = min.min(val);
                    max = max.max(val);
                }
            }
        }

        pdinfo.set_min_val(min.into());
        pdinfo.set_max_val(max.into());

        let minmax_width = f64::from(max) - f64::from(min);
        let minmax_center = f64::from(min) + minmax_width / 2_f64;
        let mut already_has_minmax = false;
        for winlevel in pdinfo.win_levels_mut() {
            winlevel.set_out_min(f64::from(i16::MIN));
            winlevel.set_out_max(f64::from(i16::MAX));

            let same_width = (winlevel.width() - minmax_width).abs() < 0.01;
            let same_center = (winlevel.center() - minmax_center).abs() < 0.01;
            if same_width && same_center {
                already_has_minmax = true;
            }
        }
        if !already_has_minmax {
            pdinfo.win_levels_mut().push(WindowLevel::new(
                "Min/Max".to_string(),
                minmax_center,
                minmax_width,
                f64::from(i16::MIN),
                f64::from(i16::MAX),
            ));
        }
        Self::new(pdinfo, buffer)
    }

    /// Create `PixelDataSliceI16` from 16-bit monochrome slice data.
    ///
    /// # Errors
    /// - Any errors interpreting little/big -endian bytes as 16bit numbers.
    pub fn from_mono_16bit(mut pdinfo: PixelDataSliceInfo) -> Result<Self, PixelDataError> {
        let num_frames = usize::try_from(pdinfo.num_frames()).unwrap_or(1);
        let samples = usize::from(pdinfo.samples_per_pixel());
        let len = usize::from(pdinfo.cols()) * usize::from(pdinfo.rows()) * num_frames;
        let pixel_pad = pdinfo
            .pixel_pad()
            .and_then(|pad_val| TryInto::<i16>::try_into(pad_val).ok());

        let mut buffer: Vec<i16> = Vec::with_capacity(len * samples);
        let mut in_pos: usize = 0;
        let mut min: i16 = i16::MAX;
        let mut max: i16 = i16::MIN;
        let bytes = pdinfo.take_bytes();
        for _i in 0..len {
            for _j in 0..samples {
                let val = if pdinfo.big_endian() {
                    if pdinfo.is_signed() {
                        let val = i16::from_be_bytes(bytes[in_pos..in_pos + I16_SIZE].try_into()?);
                        in_pos += I16_SIZE;
                        val
                    } else {
                        // Wrapping cast won't happen since we take the minimum value between the
                        // u16 number and i16::MAX.
                        #[allow(clippy::cast_possible_wrap)]
                        let val = u16::from_be_bytes(bytes[in_pos..in_pos + U16_SIZE].try_into()?)
                            .min(i16::MAX as u16) as i16;
                        in_pos += U16_SIZE;
                        val
                    }
                } else if pdinfo.is_signed() {
                    let val = i16::from_le_bytes(bytes[in_pos..in_pos + I16_SIZE].try_into()?);
                    in_pos += I16_SIZE;
                    val
                } else {
                    // Wrapping cast won't happen since we take the minimum value between the
                    // u16 number and i16::MAX.
                    #[allow(clippy::cast_possible_wrap)]
                    let val = u16::from_le_bytes(bytes[in_pos..in_pos + U16_SIZE].try_into()?)
                        .min(i16::MAX as u16) as i16;
                    in_pos += U16_SIZE;
                    val
                };

                buffer.push(val);
                if pixel_pad.is_none_or(|pad_val| val != pad_val) {
                    min = min.min(val);
                    max = max.max(val);
                }
            }
        }

        pdinfo.set_min_val(min.into());
        pdinfo.set_max_val(max.into());

        let minmax_width = f64::from(max) - f64::from(min);
        let minmax_center = f64::from(min) + minmax_width / 2_f64;
        let mut already_has_minmax = false;
        for winlevel in pdinfo.win_levels_mut() {
            winlevel.set_out_min(f64::from(i16::MIN));
            winlevel.set_out_max(f64::from(i16::MAX));

            let same_width = (winlevel.width() - minmax_width).abs() < 0.01;
            let same_center = (winlevel.center() - minmax_center).abs() < 0.01;
            if same_width && same_center {
                already_has_minmax = true;
            }
        }
        if !already_has_minmax {
            pdinfo.win_levels_mut().push(WindowLevel::new(
                "Min/Max".to_string(),
                minmax_center,
                minmax_width,
                f64::from(i16::MIN),
                f64::from(i16::MAX),
            ));
        }
        Ok(PixelDataSliceI16::new(pdinfo, buffer))
    }

    #[must_use]
    pub fn new(info: PixelDataSliceInfo, buffer: Vec<i16>) -> Self {
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

    #[must_use]
    pub fn info(&self) -> &PixelDataSliceInfo {
        &self.info
    }

    #[must_use]
    pub fn buffer(&self) -> &[i16] {
        &self.buffer
    }

    #[must_use]
    pub fn into_buffer(mut self) -> (PixelDataSliceInfo, Vec<i16>) {
        let buffer = std::mem::take(&mut self.buffer);
        (self.info, buffer)
    }

    #[must_use]
    pub fn stride(&self) -> usize {
        self.stride
    }

    #[must_use]
    pub fn rescale(&self, val: f64) -> f64 {
        if let Some(slope) = self.info().slope() {
            if let Some(intercept) = self.info().intercept() {
                return val * slope + intercept;
            }
        }
        val
    }

    #[must_use]
    pub fn best_winlevel(&self) -> WindowLevel {
        self.info()
            .win_levels()
            // XXX: The window/level computed from the min/max values seems to be better than most
            //      window/levels specified in the dicom, at least prior to applying a color-table.
            .last()
            .map_or_else(
                || {
                    WindowLevel::new(
                        "Default".to_string(),
                        0_f64,
                        f64::from(i16::MAX),
                        f64::from(i16::MIN),
                        f64::from(i16::MAX),
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
