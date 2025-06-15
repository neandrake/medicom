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
        dcmobject::DicomRoot,
        defn::vr::{self, VRRef},
        read::Parser,
        values::RawValue,
    },
    dict::tags,
    load::{
        imgvol::VolDims,
        pixeldata::{
            pdslice::PixelDataSlice, pdwinlevel::WindowLevel, pixel_i16::PixelDataSliceI16,
            pixel_i32::PixelDataSliceI32, pixel_u16::PixelDataSliceU16,
            pixel_u32::PixelDataSliceU32, pixel_u8::PixelDataSliceU8, BitsAlloc, PhotoInterp,
            PixelDataError,
        },
    },
};

pub const I8_SIZE: usize = size_of::<i8>();
pub const I16_SIZE: usize = size_of::<i16>();
pub const I32_SIZE: usize = size_of::<i32>();
pub const U8_SIZE: usize = size_of::<u8>();
pub const U16_SIZE: usize = size_of::<u16>();
pub const U32_SIZE: usize = size_of::<u32>();

/// Parsed tag values relevant to interpreting Pixel Data, including the raw `PixelData` bytes.
pub struct PixelDataSliceInfo {
    dcmroot: DicomRoot,
    big_endian: bool,
    vr: VRRef,
    slice_thickness: f32,
    spacing_between_slices: f32,
    samples_per_pixel: u16,
    photo_interp: Option<PhotoInterp>,
    planar_config: u16,
    num_frames: i32,
    cols: u16,
    rows: u16,
    pixel_spacing: (f32, f32),
    pixel_pad: Option<u16>,
    bits_alloc: BitsAlloc,
    bits_stored: u16,
    high_bit: u16,
    pixel_rep: u16,
    slope: Option<f64>,
    intercept: Option<f64>,
    unit: String,
    patient_pos: String,
    image_pos: [f64; 3],
    patient_orientation: [f64; 6],
    min_val: f64,
    max_val: f64,
    win_levels: Vec<WindowLevel>,
    pd_bytes: Vec<u8>,
}

impl PixelDataSliceInfo {
    #[must_use]
    pub fn image_pos(&self) -> &[f64; 3] {
        &self.image_pos
    }

    #[allow(clippy::too_many_lines)] // No great way to shrink this down.
    #[must_use]
    pub(crate) fn process(dcmroot: DicomRoot) -> Self {
        let big_endian = dcmroot.ts().big_endian();
        let mut pdinfo = Self {
            dcmroot,
            big_endian,
            vr: &vr::OB,
            slice_thickness: 0f32,
            spacing_between_slices: 0f32,
            samples_per_pixel: 0,
            photo_interp: None,
            planar_config: 0,
            num_frames: 1,
            cols: 0,
            rows: 0,
            pixel_spacing: (0f32, 0f32),
            pixel_pad: None,
            bits_alloc: BitsAlloc::Unsupported(0),
            bits_stored: 0,
            high_bit: 0,
            pixel_rep: 0,
            slope: None,
            intercept: None,
            unit: String::new(),
            patient_pos: String::new(),
            image_pos: [0f64; 3],
            patient_orientation: [0f64; 6],
            min_val: 0f64,
            max_val: 0f64,
            win_levels: Vec::with_capacity(0),
            pd_bytes: Vec::with_capacity(0),
        };

        if let Some(val) = pdinfo
            .dcmroot()
            .get_value_by_tag(&tags::SliceThickness)
            .and_then(|v| v.float())
        {
            pdinfo.slice_thickness = val;
        }
        if let Some(val) = pdinfo
            .dcmroot()
            .get_value_by_tag(&tags::SpacingBetweenSlices)
            .and_then(|v| v.float())
        {
            pdinfo.spacing_between_slices = val;
        }
        if let Some(val) = pdinfo
            .dcmroot()
            .get_value_by_tag(&tags::SamplesperPixel)
            .and_then(|v| v.ushort())
        {
            pdinfo.samples_per_pixel = val;
        }
        if let Some(val) = pdinfo
            .dcmroot()
            .get_value_by_tag(&tags::PhotometricInterpretation)
            .and_then(|v| v.string().cloned())
        {
            pdinfo.photo_interp = Some(Into::<PhotoInterp>::into(val.as_str()));
        }
        if let Some(val) = pdinfo
            .dcmroot()
            .get_value_by_tag(&tags::PlanarConfiguration)
            .and_then(|v| v.ushort())
        {
            pdinfo.planar_config = val;
        }
        if let Some(val) = pdinfo
            .dcmroot()
            .get_value_by_tag(&tags::NumberofFrames)
            .and_then(|v| v.int())
        {
            pdinfo.num_frames = val;
        }
        if let Some(val) = pdinfo
            .dcmroot()
            .get_value_by_tag(&tags::Rows)
            .and_then(|v| v.ushort())
        {
            pdinfo.rows = val;
        }
        if let Some(val) = pdinfo
            .dcmroot()
            .get_value_by_tag(&tags::Columns)
            .and_then(|v| v.ushort())
        {
            pdinfo.cols = val;
        }
        if let Some(RawValue::Floats(val)) = pdinfo.dcmroot().get_value_by_tag(&tags::PixelSpacing)
        {
            if val.len() == 2 {
                pdinfo.pixel_spacing = (val[0], val[1]);
            }
        }
        if let Some(val) = pdinfo
            .dcmroot()
            .get_value_by_tag(&tags::BitsAllocated)
            .and_then(|v| v.ushort())
        {
            pdinfo.bits_alloc = BitsAlloc::from_val(val);
        }
        if let Some(val) = pdinfo
            .dcmroot()
            .get_value_by_tag(&tags::BitsStored)
            .and_then(|v| v.ushort())
        {
            pdinfo.bits_stored = val;
        }
        if let Some(val) = pdinfo
            .dcmroot()
            .get_value_by_tag(&tags::HighBit)
            .and_then(|v| v.ushort())
        {
            pdinfo.high_bit = val;
        }
        if let Some(val) = pdinfo
            .dcmroot()
            .get_value_by_tag(&tags::PixelRepresentation)
            .and_then(|v| v.ushort())
        {
            pdinfo.pixel_rep = val;
        }
        pdinfo.pixel_pad = pdinfo
            .dcmroot()
            .get_value_by_tag(&tags::PixelPaddingValue)
            .and_then(|v| v.ushort());
        if let Some(RawValue::Doubles(vals)) =
            pdinfo.dcmroot().get_value_by_tag(&tags::WindowCenter)
        {
            for (i, val) in vals.into_iter().enumerate() {
                if let Some(winlevel) = pdinfo.win_levels.get_mut(i) {
                    winlevel.set_center(val);
                } else {
                    pdinfo.win_levels.push(WindowLevel::new(
                        format!("winlevel_{i}"),
                        val,
                        0.0f64,
                        f64::MIN,
                        f64::MAX,
                    ));
                }
            }
        }
        if let Some(RawValue::Doubles(vals)) = pdinfo.dcmroot().get_value_by_tag(&tags::WindowWidth)
        {
            for (i, val) in vals.into_iter().enumerate() {
                if let Some(winlevel) = pdinfo.win_levels.get_mut(i) {
                    winlevel.set_width(val);
                } else {
                    pdinfo.win_levels.push(WindowLevel::new(
                        format!("winlevel_{i}"),
                        0.0f64,
                        val,
                        f64::MIN,
                        f64::MAX,
                    ));
                }
            }
        }
        pdinfo.intercept = pdinfo
            .dcmroot()
            .get_value_by_tag(&tags::RescaleIntercept)
            .and_then(|v| v.double());
        pdinfo.slope = pdinfo
            .dcmroot()
            .get_value_by_tag(&tags::RescaleSlope)
            .and_then(|v| v.double());
        if let Some(val) = pdinfo
            .dcmroot()
            .get_value_by_tag(&tags::RescaleType)
            .and_then(|v| v.string().cloned())
        {
            pdinfo.unit = val;
        } else if let Some(val) = pdinfo
            .dcmroot()
            .get_value_by_tag(&tags::Units)
            .and_then(|v| v.string().cloned())
        {
            pdinfo.unit = val;
        }

        if let Some(RawValue::Strings(vals)) = pdinfo
            .dcmroot()
            .get_value_by_tag(&tags::WindowCenter_and_WidthExplanation)
        {
            for (i, val) in vals.into_iter().enumerate() {
                if let Some(winlevel) = pdinfo.win_levels.get_mut(i) {
                    winlevel.set_name(val);
                } else {
                    pdinfo.win_levels.push(WindowLevel::new(
                        val,
                        0.0f64,
                        0.0f64,
                        f64::MIN,
                        f64::MAX,
                    ));
                }
            }
        }

        if let Some(val) = pdinfo
            .dcmroot()
            .get_value_by_tag(&tags::PatientPosition)
            .and_then(|v| v.string().cloned())
        {
            pdinfo.patient_pos = val;
        }
        if let Some(RawValue::Doubles(vals)) = pdinfo
            .dcmroot()
            .get_value_by_tag(&tags::ImagePositionPatient)
        {
            if vals.len() <= pdinfo.image_pos.len() {
                pdinfo.image_pos[..vals.len()].copy_from_slice(&vals[..]);
            }
        }
        if let Some(RawValue::Doubles(vals)) =
            pdinfo.dcmroot().get_value_by_tag(&tags::PatientOrientation)
        {
            if vals.len() <= pdinfo.patient_orientation.len() {
                pdinfo.patient_orientation[..vals.len()].copy_from_slice(&vals[..]);
            }
        }

        let mut pd_bytes = Vec::with_capacity(0);
        let mut vr = &vr::OB;
        if let Some(obj) = pdinfo.dcmroot_mut().get_child_by_tag_mut(&tags::PixelData) {
            let elem = obj.element_mut();
            vr = elem.vr();
            if elem.has_fragments() {
                // Otherwise the additional fragments have to be appended. Shrink the element's data
                // buffer so it's not hanging on to an empty vec with a large capacity.
                for ch in obj.iter_items_mut() {
                    pd_bytes.append(ch.element_mut().data_mut());
                    ch.element_mut().data_mut().shrink_to(0);
                }
            } else {
                // The common case of a single-frame dataset, or the first frame of a multi-frame
                // datset, swapping results in more efficient memory usage since the bytes do not need
                // to be individually copied/moved.
                std::mem::swap(&mut pd_bytes, elem.data_mut());
            }
        }
        pdinfo.vr = vr;
        pdinfo.pd_bytes = pd_bytes;

        pdinfo
    }
}

impl std::fmt::Debug for PixelDataSliceInfo {
    // Default Debug implementation but don't print all bytes, just the length.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PixelDataSliceInfo")
            .field("dcmroot", &self.dcmroot)
            .field("big_endian", &self.big_endian)
            .field("vr", &self.vr)
            .field("slice_thickness", &self.slice_thickness)
            .field("spacing_between_slices", &self.spacing_between_slices)
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
            .field("pixel_spacing", &self.pixel_spacing)
            .field(
                "pixel_pad",
                &self.pixel_pad.map_or("None".to_string(), |v| v.to_string()),
            )
            .field("bits_alloc", &self.bits_alloc)
            .field("bits_stored", &self.bits_stored)
            .field("high_bit", &self.high_bit)
            .field("pixel_rep", &self.pixel_rep)
            .field(
                "slope",
                &self.slope.map_or("None".to_string(), |v| v.to_string()),
            )
            .field(
                "intercept",
                &self.intercept.map_or("None".to_string(), |v| v.to_string()),
            )
            .field("unit", &self.unit)
            .field("patient_pos", &self.patient_pos)
            .field("image_pos", &self.image_pos)
            .field("patient_orientation", &self.patient_orientation)
            .field("min_val", &self.min_val)
            .field("max_val", &self.max_val)
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
    pub fn dcmroot(&self) -> &DicomRoot {
        &self.dcmroot
    }

    #[must_use]
    pub fn dcmroot_mut(&mut self) -> &mut DicomRoot {
        &mut self.dcmroot
    }

    #[must_use]
    pub fn vr(&self) -> VRRef {
        self.vr
    }

    #[must_use]
    pub fn slice_thickness(&self) -> f32 {
        self.slice_thickness
    }

    #[must_use]
    pub fn spacing_between_slices(&self) -> f32 {
        self.spacing_between_slices
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
    pub fn pixel_spacing(&self) -> (f32, f32) {
        self.pixel_spacing
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
    pub fn min_val(&self) -> f64 {
        self.min_val
    }

    #[must_use]
    pub fn max_val(&self) -> f64 {
        self.max_val
    }

    pub fn set_min_val(&mut self, min_val: f64) {
        self.min_val = min_val;
    }

    pub fn set_max_val(&mut self, max_val: f64) {
        self.max_val = max_val;
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
    pub fn stride(&self) -> usize {
        if self.planar_config == 0 {
            1
        } else {
            self.pd_bytes.len() / self.samples_per_pixel as usize
        }
    }

    #[must_use]
    pub fn is_rgb(&self) -> bool {
        self.photo_interp.as_ref().is_some_and(PhotoInterp::is_rgb) && self.samples_per_pixel == 3
    }

    /// Whether the byte values in Pixel Data are signed or unsigned values.
    #[must_use]
    pub fn is_signed(&self) -> bool {
        self.pixel_rep != 0
    }

    #[must_use]
    pub fn vol_dims(&self) -> VolDims {
        let mut z_mm = 0f32;
        if VolDims::is_valid_dim(self.spacing_between_slices) {
            z_mm = self.spacing_between_slices;
        } else if VolDims::is_valid_dim(self.slice_thickness) {
            z_mm = self.slice_thickness;
        }
        VolDims::new(
            self.rows,
            self.cols,
            // PixelSpacing first value is space between rows (y) and second value is space between
            // columns (x).
            self.pixel_spacing.1,
            self.pixel_spacing.0,
            z_mm,
        )
    }

    #[must_use]
    pub fn take_bytes(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.pd_bytes)
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
        }

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

        // One of SliceThickness or SpacingBetweenSlices should be present/valid.
        if !VolDims::is_valid_dim(self.slice_thickness)
            && !VolDims::is_valid_dim(self.spacing_between_slices)
        {
            return Err(PixelDataError::InvalidDims(format!(
                "SliceThickness and SpacingBetweenSlices are both invalid: {}, {}",
                self.slice_thickness, self.spacing_between_slices
            )));
        }

        // Both values from PixelSpacing must be valid.
        if !VolDims::is_valid_dim(self.pixel_spacing.0)
            || !VolDims::is_valid_dim(self.pixel_spacing.1)
        {
            return Err(PixelDataError::InvalidDims(format!(
                "PixelSpacing is invalid: {:?}",
                self.pixel_spacing
            )));
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
        match (self.bits_alloc, self.is_rgb()) {
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
        mut parser: Parser<'_, R>,
    ) -> Result<PixelDataSliceInfo, PixelDataError> {
        let Some(dcmroot) = DicomRoot::parse(&mut parser)? else {
            return Err(PixelDataError::MissingPixelData);
        };
        Ok(PixelDataSliceInfo::process(dcmroot))
    }
}
