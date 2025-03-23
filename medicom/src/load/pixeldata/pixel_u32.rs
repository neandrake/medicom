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
    pdinfo::{PixelDataSliceInfo, I32_SIZE, U32_SIZE},
    pdslice::PixelDataSlice,
    pdwinlevel::WindowLevel,
    PhotoInterp, PixelDataError,
};

#[derive(Debug)]
pub struct PixelU32 {
    pub x: usize,
    pub y: usize,
    pub z: usize,
    pub r: u32,
    pub g: u32,
    pub b: u32,
}

pub struct PixelDataSliceU32 {
    info: PixelDataSliceInfo,
    buffer: Vec<u32>,

    stride: usize,
    interp_as_rgb: bool,
}

impl std::fmt::Debug for PixelDataSliceU32 {
    // Default Debug implementation but don't print all bytes, just the length.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PixelDataSliceU32")
            .field("info", &self.info)
            .field("buffer.len", &self.buffer.len())
            .field("stride", &self.stride)
            .field("interp_as_rgb", &self.interp_as_rgb)
            .finish()
    }
}

impl PixelDataSliceU32 {
    /// Interpret as 32bit RGB.
    ///
    /// # Errors
    /// - I/O errors reading the data.
    pub fn from_rgb_32bit(mut pdinfo: PixelDataSliceInfo) -> Result<Self, PixelDataError> {
        let num_frames = usize::try_from(pdinfo.num_frames()).unwrap_or(1);
        let samples = usize::from(pdinfo.samples_per_pixel());
        let len = usize::from(pdinfo.cols()) * usize::from(pdinfo.rows()) * num_frames;

        let mut in_pos: usize = 0;
        let mut buffer: Vec<u32> = Vec::with_capacity(len * samples);
        let bytes = pdinfo.take_bytes();
        for _i in 0..len {
            for _j in 0..samples {
                let val = if pdinfo.big_endian() {
                    if pdinfo.is_signed() {
                        // There shouldn't be signed data with RGB photometric interpretation.
                        let val = PixelDataSlice::shift_i32(i32::from_be_bytes(
                            bytes[in_pos..in_pos + I32_SIZE].try_into()?,
                        ));
                        in_pos += I32_SIZE;
                        val
                    } else {
                        let val = u32::from_be_bytes(bytes[in_pos..in_pos + U32_SIZE].try_into()?);
                        in_pos += U32_SIZE;
                        val
                    }
                } else if pdinfo.is_signed() {
                    // There shouldn't be signed data with RGB photometric interpretation.
                    let val = PixelDataSlice::shift_i32(i32::from_le_bytes(
                        bytes[in_pos..in_pos + I32_SIZE].try_into()?,
                    ));
                    in_pos += I32_SIZE;
                    val
                } else {
                    let val = u32::from_le_bytes(bytes[in_pos..in_pos + U32_SIZE].try_into()?);
                    in_pos += U32_SIZE;
                    val
                };
                buffer.push(val);
            }
        }
        Ok(PixelDataSliceU32::new(pdinfo, buffer))
    }

    #[must_use]
    pub fn new(info: PixelDataSliceInfo, buffer: Vec<u32>) -> Self {
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
    pub fn buffer(&self) -> &[u32] {
        &self.buffer
    }

    #[must_use]
    pub fn into_buffer(self) -> Vec<u32> {
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
    ) -> Result<PixelU32, PixelDataError> {
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
                .map(|v| winlevel.apply(v) as u32)
                .or(self.info().pixel_pad().map(|v| v as u32))
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

        Ok(PixelU32 { x, y, z, r, g, b })
    }

    #[must_use]
    pub fn pixel_iter(&self) -> SlicePixelU32Iter {
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
                        f64::from(u32::MAX) / 2_f64,
                        f64::from(u32::MAX) / 2_f64,
                        f64::from(i32::MIN),
                        f64::from(i32::MAX),
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
    pub fn pixel_iter_with_win(&self, winlevel: WindowLevel) -> SlicePixelU32Iter {
        SlicePixelU32Iter {
            slice: self,
            winlevel,
            src_byte_index: 0,
        }
    }
}

pub struct SlicePixelU32Iter<'buf> {
    slice: &'buf PixelDataSliceU32,
    winlevel: WindowLevel,
    src_byte_index: usize,
}

impl Iterator for SlicePixelU32Iter<'_> {
    type Item = PixelU32;

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
