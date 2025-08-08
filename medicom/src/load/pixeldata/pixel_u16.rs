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
    pdinfo::{PixelDataSliceInfo, I16_SIZE, U16_SIZE},
    winlevel::WindowLevel,
    LoadError, PhotoInterp, PixelDataSlice,
};

pub struct PixelDataSliceU16 {
    info: PixelDataSliceInfo,
    buffer: Vec<u16>,

    stride: usize,
    interp_as_rgb: bool,
}

impl std::fmt::Debug for PixelDataSliceU16 {
    // Default Debug implementation but don't print all bytes, just the length.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PixelDataSliceU16")
            .field("info", &self.info)
            .field("buffer.len", &self.buffer.len())
            .field("stride", &self.stride)
            .field("interp_as_rgb", &self.interp_as_rgb)
            .finish()
    }
}

impl PixelDataSliceU16 {
    /// Interpret as 16-bit RGB.
    ///
    /// # Errors
    /// - I/O errors reading the data.
    pub fn from_rgb_16bit(mut pdinfo: PixelDataSliceInfo) -> Result<Self, LoadError> {
        let num_frames = usize::try_from(pdinfo.num_frames()).unwrap_or(1);
        let samples = usize::from(pdinfo.samples_per_pixel());
        let len = usize::from(pdinfo.cols()) * usize::from(pdinfo.rows()) * num_frames;
        let pixel_pad = pdinfo
            .pixel_pad()
            .and_then(|pad_val| TryInto::<u16>::try_into(pad_val).ok());

        let mut buffer: Vec<u16> = Vec::with_capacity(len * samples);
        let mut in_pos: usize = 0;
        let mut min: u16 = u16::MAX;
        let mut max: u16 = u16::MIN;
        let bytes = pdinfo.take_bytes();
        for _i in 0..len {
            for _j in 0..samples {
                let val = if pdinfo.big_endian() {
                    if pdinfo.is_signed() {
                        // There should't be signed values with RGB photometric interpretation.
                        let val = PixelDataSlice::shift_i16(i16::from_be_bytes(
                            bytes[in_pos..in_pos + I16_SIZE].try_into()?,
                        ));
                        in_pos += I16_SIZE;
                        val
                    } else {
                        let val = u16::from_be_bytes(bytes[in_pos..in_pos + U16_SIZE].try_into()?);
                        in_pos += U16_SIZE;
                        val
                    }
                } else if pdinfo.is_signed() {
                    // There should't be signed values with RGB photometric interpretation.
                    let val = PixelDataSlice::shift_i16(i16::from_le_bytes(
                        bytes[in_pos..in_pos + I16_SIZE].try_into()?,
                    ));
                    in_pos += I16_SIZE;
                    val
                } else {
                    let val = u16::from_le_bytes(bytes[in_pos..in_pos + U16_SIZE].try_into()?);
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

        Ok(Self::new(pdinfo, buffer))
    }

    #[must_use]
    pub fn new(info: PixelDataSliceInfo, buffer: Vec<u16>) -> Self {
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
    /// - `PixelValueError` if unable to convert the `u16` into `i16`.
    pub fn into_i16(self) -> Result<(PixelDataSliceInfo, Vec<i16>), LoadError> {
        let mut buffer: Vec<i16> = Vec::with_capacity(self.buffer.len());
        for v in &self.buffer {
            buffer.push(i16::try_from(*v).map_err(|e| LoadError::PixelValueError { source: e })?);
        }
        Ok((self.info, buffer))
    }

    #[must_use]
    pub fn info(&self) -> &PixelDataSliceInfo {
        &self.info
    }

    #[must_use]
    pub fn buffer(&self) -> &[u16] {
        &self.buffer
    }

    #[must_use]
    pub fn into_buffer(self) -> Vec<u16> {
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
                        u16::MAX as f32 / 2_f32,
                        u16::MAX as f32 / 2_f32,
                        u16::MIN as f32,
                        u16::MAX as f32,
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
