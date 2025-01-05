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

use std::io::Read;

use crate::{
    core::{
        dcmelement::DicomElement,
        defn::vr::{self, VRRef},
        pixeldata::{
            pdslice::PixelDataSlice, pdwinlevel::WindowLevel, pixel_i16::PixelDataSliceI16,
            pixel_i32::PixelDataSliceI32, pixel_u16::PixelDataSliceU16,
            pixel_u32::PixelDataSliceU32, pixel_u8::PixelDataSliceU8, BitsAlloc, PhotoInterp,
            PixelDataError,
        },
        read::Parser,
        values::RawValue,
    },
    dict::tags,
};

pub const I8_SIZE: usize = size_of::<i8>();
pub const I16_SIZE: usize = size_of::<i16>();
pub const I32_SIZE: usize = size_of::<i32>();
pub const U8_SIZE: usize = size_of::<u8>();
pub const U16_SIZE: usize = size_of::<u16>();
pub const U32_SIZE: usize = size_of::<u32>();

/// Parsed tag values relevant to interpreting Pixel Data, including the raw `PixelData` bytes.
pub struct PixelDataSliceInfo {
    big_endian: bool,
    vr: VRRef,
    samples_per_pixel: u16,
    photo_interp: Option<PhotoInterp>,
    planar_config: u16,
    num_frames: i32,
    cols: u16,
    rows: u16,
    pixel_pad: Option<u16>,
    bits_alloc: BitsAlloc,
    bits_stored: u16,
    high_bit: u16,
    pixel_rep: u16,
    slope: Option<f64>,
    intercept: Option<f64>,
    unit: String,
    win_levels: Vec<WindowLevel>,
    pd_bytes: Vec<u8>,
}

impl Default for PixelDataSliceInfo {
    fn default() -> Self {
        Self {
            big_endian: false,
            vr: &vr::OB,
            samples_per_pixel: 0,
            photo_interp: None,
            planar_config: 0,
            num_frames: 1,
            cols: 0,
            rows: 0,
            pixel_pad: None,
            bits_alloc: BitsAlloc::Unsupported(0),
            bits_stored: 0,
            high_bit: 0,
            pixel_rep: 0,
            slope: None,
            intercept: None,
            unit: String::new(),
            win_levels: Vec::with_capacity(0),
            pd_bytes: Vec::with_capacity(0),
        }
    }
}

impl std::fmt::Debug for PixelDataSliceInfo {
    // Default Debug implementation but don't print all bytes, just the length.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PixelDataSliceInfo")
            .field("big_endian", &self.big_endian)
            .field("vr", &self.vr)
            .field("samples_per_pixel", &self.samples_per_pixel)
            .field(
                "photo_interp",
                &self
                    .photo_interp
                    .as_ref()
                    .unwrap_or(&PhotoInterp::Unsupported("None".to_string())),
            )
            .field("planar_config", &self.planar_config)
            .field("num_frames", &self.num_frames)
            .field("cols", &self.cols)
            .field("rows", &self.rows)
            .field(
                "pixel_pad",
                &self
                    .pixel_pad
                    .map(|v| v.to_string())
                    .unwrap_or("None".to_string()),
            )
            .field("bits_alloc", &self.bits_alloc)
            .field("bits_stored", &self.bits_stored)
            .field("high_bit", &self.high_bit)
            .field("pixel_rep", &self.pixel_rep)
            .field(
                "slope",
                &self
                    .slope
                    .map(|v| v.to_string())
                    .unwrap_or("None".to_string()),
            )
            .field(
                "intercept",
                &self
                    .intercept
                    .map(|v| v.to_string())
                    .unwrap_or("None".to_string()),
            )
            .field("unit", &self.unit)
            .field("win_levels", &self.win_levels)
            .field("pd_bytes", &self.pd_bytes.len())
            .finish()
    }
}

impl PixelDataSliceInfo {
    #[must_use]
    pub fn big_endian(&self) -> bool {
        self.big_endian
    }

    #[must_use]
    pub fn vr(&self) -> VRRef {
        self.vr
    }

    #[must_use]
    pub fn samples_per_pixel(&self) -> u16 {
        self.samples_per_pixel
    }

    #[must_use]
    pub fn photo_interp(&self) -> Option<&PhotoInterp> {
        self.photo_interp.as_ref()
    }

    #[must_use]
    pub fn planar_config(&self) -> u16 {
        self.planar_config
    }

    #[must_use]
    pub fn num_frames(&self) -> i32 {
        self.num_frames
    }

    #[must_use]
    pub fn cols(&self) -> u16 {
        self.cols
    }

    #[must_use]
    pub fn rows(&self) -> u16 {
        self.rows
    }

    #[must_use]
    pub fn pixel_pad(&self) -> Option<u16> {
        self.pixel_pad
    }

    #[must_use]
    pub fn bits_alloc(&self) -> &BitsAlloc {
        &self.bits_alloc
    }

    #[must_use]
    pub fn bits_stored(&self) -> u16 {
        self.bits_stored
    }

    #[must_use]
    pub fn high_bit(&self) -> u16 {
        self.high_bit
    }

    #[must_use]
    pub fn pixel_rep(&self) -> u16 {
        self.pixel_rep
    }

    #[must_use]
    pub fn slope(&self) -> Option<f64> {
        self.slope
    }

    #[must_use]
    pub fn intercept(&self) -> Option<f64> {
        self.intercept
    }

    #[must_use]
    pub fn unit(&self) -> &str {
        &self.unit
    }

    #[must_use]
    pub fn win_levels(&self) -> &[WindowLevel] {
        &self.win_levels
    }

    #[must_use]
    pub fn win_levels_mut(&mut self) -> &mut Vec<WindowLevel> {
        &mut self.win_levels
    }

    #[must_use]
    pub fn take_bytes(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.pd_bytes)
    }

    /// Whether the byte values in Pixel Data are signed or unsigned values.
    #[must_use]
    pub fn is_signed(&self) -> bool {
        self.pixel_rep != 0
    }

    /// After all relevant elements have been parsed, this will validate the result of this
    /// structure.
    ///
    /// # Errors
    /// - This function returns errors in the validation of values parsed from DICOM elements via
    ///   `PixelDataInfo::process_dcm_parser`.
    pub fn validate(&mut self) -> Result<(), PixelDataError> {
        if self.pd_bytes.is_empty() {
            return Err(PixelDataError::MissingPixelData);
        }

        if self.cols == 0 || self.rows == 0 {
            return Err(PixelDataError::InvalidSize(self.cols, self.rows));
        }

        if self.vr != &vr::OB && self.vr != &vr::OW {
            return Err(PixelDataError::InvalidVR(self.vr));
        };

        if let BitsAlloc::Unsupported(val) = self.bits_alloc {
            return Err(PixelDataError::InvalidBitsAlloc(val));
        }

        // BitsStored will generally be the same value as BitsAllocated.
        if self.bits_stored > self.bits_alloc.val() || self.bits_stored == 0 {
            self.bits_stored = self.bits_alloc.val();
        }
        // HighBit will generally be (BitsStored - 1).
        if self.high_bit > self.bits_alloc.val() - 1 || self.high_bit < self.bits_stored - 1 {
            self.high_bit = self.bits_stored - 1;
        }

        if let Some(pi) = &self.photo_interp {
            if (pi.is_rgb() && self.samples_per_pixel != 3)
                || (pi.is_monochrome() && self.samples_per_pixel != 1)
            {
                // RGB must use 3 Samples Per Pixel.
                // MONOCHROME1/2 must use 1 Sample Per Pixel.
                return Err(PixelDataError::InvalidPhotoInterpSamples(
                    pi.clone(),
                    self.samples_per_pixel,
                ));
            }
        }

        Ok(())
    }

    /// Loads the pixel data raw bytes into values in a `PixelDataBuffer`.
    ///
    /// # Notes
    /// The type of buffer returned depends on the photometric interpretation and bits allocated.
    /// - `PhotometricInterpretation` == RGB always returns unsigned variant.
    ///   - `BitsAlloc` = 8, `PixelDataBuffer::U8`.
    ///   - `BitsAlloc` = 16, `PixelDataBuffer::U16`.
    ///   - `BitsAlloc` = 32, `PixelDataBuffer::U32`.
    /// - `PhotometricInterpretation` == MONOCHROME1/MONOCHROME2 always returns a signed variant.
    ///   While values may be encoded as unsigned, applying the rescale slope/intercept may result
    ///   in negative values.
    ///   - `BitsAlloc` = 8, `PixelDataBuffer::I16` (values are cast from i8/u8 to i16).
    ///   - `BitsAlloc` = 16, `PixelDataBuffer::I16` (u16 values are cast to i16).
    ///   - `BitsAlloc` = 32, `PixelDataBuffer::I32` (u32 values are cast to i32).
    ///
    /// # Errors
    /// - If the value of `BitsAlloc` is unsupported.
    /// - Reading byte/word values from the `PixelData` bytes.
    pub fn load_pixel_data(mut self) -> Result<PixelDataSlice, PixelDataError> {
        self.validate()?;
        let is_rgb = self.photo_interp().is_some_and(PhotoInterp::is_rgb);
        match (self.bits_alloc, is_rgb) {
            (BitsAlloc::Unsupported(val), _) => Err(PixelDataError::InvalidBitsAlloc(val)),
            (BitsAlloc::Eight, true) => {
                Ok(PixelDataSlice::U8(PixelDataSliceU8::from_rgb_8bit(self)))
            }
            (BitsAlloc::Eight, false) => {
                Ok(PixelDataSlice::I16(PixelDataSliceI16::from_mono_8bit(self)))
            }
            (BitsAlloc::Sixteen, true) => {
                PixelDataSliceU16::from_rgb_16bit(self).map(PixelDataSlice::U16)
            }
            (BitsAlloc::Sixteen, false) => {
                PixelDataSliceI16::from_mono_16bit(self).map(PixelDataSlice::I16)
            }
            (BitsAlloc::ThirtyTwo, true) => {
                PixelDataSliceU32::from_rgb_32bit(self).map(PixelDataSlice::U32)
            }
            (BitsAlloc::ThirtyTwo, false) => {
                PixelDataSliceI32::from_mono_32bit(self).map(PixelDataSlice::I32)
            }
        }
    }

    /// Processes a DICOM SOP via a `Parser` into a `PixelDataInfo`.
    ///
    /// # Errors
    /// - I/O errors parsing values out of DICOM elements.
    pub fn process_dcm_parser<R: Read>(
        parser: Parser<'_, R>,
    ) -> Result<PixelDataSliceInfo, PixelDataError> {
        let mut pixdata_info: PixelDataSliceInfo = PixelDataSliceInfo {
            big_endian: parser.ts().big_endian(),
            ..Default::default()
        };
        for elem in parser {
            let mut elem = elem?;
            Self::process_element(&mut pixdata_info, &elem)?;
            if elem.is_pixel_data() || elem.is_within_pixel_data() {
                Self::process_pixdata_element(&mut pixdata_info, &mut elem);
            }
        }
        Ok(pixdata_info)
    }

    /// Process relevant DICOM elements into the `PixelDataInfo` structure.
    ///
    /// # Errors
    /// - I/O errors parsing values out of DICOM elements.
    fn process_element(
        pixdata_info: &mut PixelDataSliceInfo,
        elem: &DicomElement,
    ) -> Result<(), PixelDataError> {
        // The order of the tag checks here are the order they will appear in a DICOM protocol.
        if elem.tag() == tags::SamplesperPixel.tag() {
            if let Some(val) = elem.parse_value()?.ushort() {
                pixdata_info.samples_per_pixel = val;
            }
        } else if elem.tag() == tags::PhotometricInterpretation.tag() {
            if let Some(val) = elem.parse_value()?.string() {
                pixdata_info.photo_interp = Some(Into::<PhotoInterp>::into(val.as_str()));
            }
        } else if elem.tag() == tags::PlanarConfiguration.tag() {
            if let Some(val) = elem.parse_value()?.ushort() {
                pixdata_info.planar_config = val;
            }
        } else if elem.tag() == tags::NumberofFrames.tag() {
            if let Some(val) = elem.parse_value()?.int() {
                pixdata_info.num_frames = val;
            }
        } else if elem.tag() == tags::Rows.tag() {
            if let Some(val) = elem.parse_value()?.ushort() {
                pixdata_info.rows = val;
            }
        } else if elem.tag() == tags::Columns.tag() {
            if let Some(val) = elem.parse_value()?.ushort() {
                pixdata_info.cols = val;
            }
        } else if elem.tag() == tags::BitsAllocated.tag() {
            if let Some(val) = elem.parse_value()?.ushort() {
                pixdata_info.bits_alloc = BitsAlloc::from_val(val);
            }
        } else if elem.tag() == tags::BitsStored.tag() {
            if let Some(val) = elem.parse_value()?.ushort() {
                pixdata_info.bits_stored = val;
            }
        } else if elem.tag() == tags::HighBit.tag() {
            if let Some(val) = elem.parse_value()?.ushort() {
                pixdata_info.high_bit = val;
            }
        } else if elem.tag() == tags::PixelRepresentation.tag() {
            if let Some(val) = elem.parse_value()?.ushort() {
                pixdata_info.pixel_rep = val;
            }
        } else if elem.tag() == tags::PixelPaddingValue.tag() {
            if let Some(val) = elem.parse_value()?.ushort() {
                pixdata_info.pixel_pad = Some(val);
            }
        } else if elem.tag() == tags::WindowCenter.tag() {
            if let RawValue::Doubles(vals) = elem.parse_value()? {
                for (i, val) in vals.into_iter().enumerate() {
                    if let Some(winlevel) = pixdata_info.win_levels.get_mut(i) {
                        winlevel.set_center(val);
                    } else {
                        pixdata_info.win_levels.push(WindowLevel::new(
                            format!("winlevel_{i}"),
                            val,
                            0.0f64,
                            f64::MIN,
                            f64::MAX,
                        ));
                    }
                }
            }
        } else if elem.tag() == tags::WindowWidth.tag() {
            if let RawValue::Doubles(vals) = elem.parse_value()? {
                for (i, val) in vals.into_iter().enumerate() {
                    if let Some(winlevel) = pixdata_info.win_levels.get_mut(i) {
                        winlevel.set_width(val);
                    } else {
                        pixdata_info.win_levels.push(WindowLevel::new(
                            format!("winlevel_{i}"),
                            0.0f64,
                            val,
                            f64::MIN,
                            f64::MAX,
                        ));
                    }
                }
            }
        } else if elem.tag() == tags::RescaleIntercept.tag() {
            if let Some(val) = elem.parse_value()?.double() {
                pixdata_info.intercept = Some(val);
            }
        } else if elem.tag() == tags::RescaleSlope.tag() {
            if let Some(val) = elem.parse_value()?.double() {
                pixdata_info.slope = Some(val);
            }
        } else if elem.tag() == tags::RescaleType.tag() || elem.tag() == tags::Units.tag() {
            if let Some(val) = elem.parse_value()?.string() {
                // Only use Units if RescaleType wasn't present. RescaleType occurs prior to Units.
                if pixdata_info.unit.is_empty() {
                    val.clone_into(&mut pixdata_info.unit);
                }
            }
        } else if elem.tag() == tags::WindowCenter_and_WidthExplanation.tag() {
            if let RawValue::Strings(vals) = elem.parse_value()? {
                for (i, val) in vals.into_iter().enumerate() {
                    if let Some(winlevel) = pixdata_info.win_levels.get_mut(i) {
                        winlevel.set_name(val);
                    } else {
                        pixdata_info.win_levels.push(WindowLevel::new(
                            val,
                            0.0f64,
                            0.0f64,
                            f64::MIN,
                            f64::MAX,
                        ));
                    }
                }
            }
        }

        Ok(())
    }

    /// Process the relevant `PixelData` element/fragments by copying the data/bytes into the
    /// `PixelDataInfo::pd_bytes` field, replacing the element's data/bytes with an empty vec.
    fn process_pixdata_element(pixdata_info: &mut PixelDataSliceInfo, elem: &mut DicomElement) {
        if pixdata_info.pd_bytes.is_empty() {
            // The common case of a single-frame dataset, or the first frame of a multi-frame
            // datset, swapping results in more efficient memory usage since the bytes do not need
            // to be individually copied/moved.
            std::mem::swap(&mut pixdata_info.pd_bytes, elem.mut_data());
        } else {
            // Otherwise the additional fragments have to be appended. Shrink the element's data
            // buffer so it's not hanging on to an empty vec with a large capacity.
            pixdata_info.pd_bytes.append(elem.mut_data());
            elem.mut_data().shrink_to(0);
        }
    }
}
