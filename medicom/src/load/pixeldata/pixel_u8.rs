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

use crate::load::pixeldata::{pdinfo::PixelDataSliceInfo, pdwinlevel::WindowLevel, PhotoInterp};

pub struct PixelDataSliceU8 {
    info: PixelDataSliceInfo,
    buffer: Vec<u8>,

    stride: usize,
    interp_as_rgb: bool,
}

impl std::fmt::Debug for PixelDataSliceU8 {
    // Default Debug implementation but don't print all bytes, just the length.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PixelDataSliceU8")
            .field("info", &self.info)
            .field("buffer.len", &self.buffer.len())
            .field("stride", &self.stride)
            .field("interp_as_rgb", &self.interp_as_rgb)
            .finish()
    }
}

impl PixelDataSliceU8 {
    #[must_use]
    pub fn from_rgb_8bit(mut pdinfo: PixelDataSliceInfo) -> Self {
        let buffer = pdinfo.take_bytes();
        PixelDataSliceU8::new(pdinfo, buffer)
    }

    #[must_use]
    pub fn new(mut info: PixelDataSliceInfo, buffer: Vec<u8>) -> Self {
        let stride = if info.planar_config() == 0 {
            1
        } else {
            buffer.len() / usize::from(info.samples_per_pixel())
        };
        let interp_as_rgb =
            info.photo_interp().is_some_and(PhotoInterp::is_rgb) && info.samples_per_pixel() == 3;

        info.set_min_val(*buffer.iter().min().unwrap_or(&0) as f32);
        info.set_max_val(*buffer.iter().max().unwrap_or(&0) as f32);

        Self {
            info,
            buffer,
            stride,
            interp_as_rgb,
        }
    }

    #[must_use]
    pub fn into_i16(self) -> (PixelDataSliceInfo, Vec<i16>) {
        let mut buffer: Vec<i16> = Vec::with_capacity(self.buffer().len());
        for b in &self.buffer {
            buffer.push(i16::from(*b));
        }
        (self.info, buffer)
    }

    #[must_use]
    pub fn info(&self) -> &PixelDataSliceInfo {
        &self.info
    }

    #[must_use]
    pub fn buffer(&self) -> &[u8] {
        &self.buffer
    }

    #[must_use]
    pub fn into_buffer(self) -> Vec<u8> {
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
                        u8::MAX as f32 / 2_f32,
                        u8::MAX as f32 / 2_f32,
                        u8::MIN as f32,
                        u8::MAX as f32,
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
