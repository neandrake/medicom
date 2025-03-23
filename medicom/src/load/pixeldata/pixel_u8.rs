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
    pdinfo::PixelDataSliceInfo, pdwinlevel::WindowLevel, PhotoInterp, PixelDataError,
};

#[derive(Debug)]
pub struct PixelU8 {
    pub x: usize,
    pub y: usize,
    pub z: usize,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

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
    pub fn new(info: PixelDataSliceInfo, buffer: Vec<u8>) -> Self {
        let stride = if info.planar_config() == 0 {
            1
        } else {
            buffer.len() / info.samples_per_pixel() as usize
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
    pub fn rescale(&self, val: f64) -> f64 {
        if let Some(slope) = self.info().slope() {
            if let Some(intercept) = self.info().intercept() {
                return val * slope + intercept;
            }
        }
        val
    }

    /// Gets the pixel at the given x,y coordinate.
    ///
    /// # Errors
    /// - If the x,y coordinate is invalid, either by being outside the image dimensions, or if the
    ///   Planar Configuration and Samples per Pixel are set up such that beginning of RGB values
    ///   must occur at specific indices.
    #[allow(clippy::many_single_char_names)]
    pub fn get_pixel(
        &self,
        x: usize,
        y: usize,
        z: usize,
        winlevel: &WindowLevel,
    ) -> Result<PixelU8, PixelDataError> {
        let cols = usize::from(self.info().cols());
        let rows = usize::from(self.info().rows());
        let samples = usize::from(self.info().samples_per_pixel());
        let stride = self.stride();

        let src_byte_index = x + y * cols + z * (rows * cols);
        let src_byte_index = src_byte_index * samples;
        if src_byte_index >= self.buffer().len()
            || (self.interp_as_rgb && src_byte_index + stride * 2 >= self.buffer().len())
        {
            return Err(PixelDataError::InvalidPixelSource(src_byte_index));
        }

        let (r, g, b) = if self.interp_as_rgb {
            let red = self.buffer()[src_byte_index];
            let green = self.buffer()[src_byte_index + stride];
            let blue = self.buffer()[src_byte_index + stride * 2];
            (red, green, blue)
        } else {
            let applied_val = self
                .buffer()
                .get(src_byte_index)
                .copied()
                .map(f64::from)
                .map(|v| self.rescale(v))
                .map(|v| winlevel.apply(v) as u8)
                .or(self.info().pixel_pad().map(|v| v as u8))
                .unwrap_or_default();
            let val = if self
                .info()
                .photo_interp()
                .is_some_and(|pi| *pi == PhotoInterp::Monochrome1)
            {
                !applied_val
            } else {
                applied_val
            };
            (val, val, val)
        };

        Ok(PixelU8 { x, y, z, r, g, b })
    }

    #[must_use]
    pub fn pixel_iter(&self) -> SlicePixelU8Iter {
        let winlevel = self
            .info()
            .win_levels()
            // XXX: The window/level computed from the min/max values seems to be better than most
            //      window/levels specified in the dicom, at least prior to applying a color-table.
            .last()
            .map_or_else(
                || {
                    WindowLevel::new(
                        "Default".to_string(),
                        f64::from(u8::MAX) / 2_f64,
                        f64::from(u8::MAX) / 2_f64,
                        f64::from(u8::MIN),
                        f64::from(u8::MAX),
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
            );

        self.pixel_iter_with_win(winlevel)
    }

    #[must_use]
    pub fn pixel_iter_with_win(&self, winlevel: WindowLevel) -> SlicePixelU8Iter {
        SlicePixelU8Iter {
            slice: self,
            winlevel,
            src_byte_index: 0,
        }
    }
}

pub struct SlicePixelU8Iter<'buf> {
    slice: &'buf PixelDataSliceU8,
    winlevel: WindowLevel,
    src_byte_index: usize,
}

impl Iterator for SlicePixelU8Iter<'_> {
    type Item = PixelU8;

    fn next(&mut self) -> Option<Self::Item> {
        let cols = usize::from(self.slice.info().cols());
        let rows = usize::from(self.slice.info().rows());
        let x = self.src_byte_index % cols;
        let y = (self.src_byte_index / cols) % rows;
        let z = self.src_byte_index / (cols * rows);
        let pixel = self.slice.get_pixel(x, y, z, &self.winlevel);
        self.src_byte_index += 1;
        pixel.ok()
    }
}
