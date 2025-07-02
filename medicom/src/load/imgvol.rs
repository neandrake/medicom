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

//! Loaded DICOM image volume datasets.

use std::{
    cmp::Ordering,
    io::{BufReader, Read},
};

use crate::{
    core::{dcmobject::DicomRoot, read::ParserBuilder},
    dict::stdlookup::STANDARD_DICOM_DICTIONARY,
    load::pixeldata::{
        pdinfo::PixelDataSliceInfo, pixel_i16::PixelDataSliceI16, pixel_i32::PixelDataSliceI32,
        pixel_u16::PixelDataSliceU16, pixel_u32::PixelDataSliceU32, pixel_u8::PixelDataSliceU8,
        BitsAlloc, PixelDataError,
    },
};

#[derive(Debug)]
pub struct VolDims {
    /// The number of voxels across the y-axis.
    rows: u16,
    /// The number of voxels across the x-axis.
    cols: u16,
    /// The distance in mm from the center of one voxel to another, across columns.
    x_mm: f32,
    /// The distance in mm from the center of one voxel to another, across rows.
    y_mm: f32,
    /// The distance in mm from the center of one voxel to another, across slices.
    z_mm: f32,
}

impl VolDims {
    #[must_use]
    pub fn new(rows: u16, cols: u16, x_mm: f32, y_mm: f32, z_mm: f32) -> Self {
        Self {
            rows,
            cols,
            x_mm,
            y_mm,
            z_mm,
        }
    }

    /// Checks that a dimension value is valid. A dimension value should be a positive value
    /// greater than zero.
    #[must_use]
    pub fn is_valid_dim(dim: f32) -> bool {
        !dim.is_nan() && dim > 0f32
    }

    #[must_use]
    pub fn rows(&self) -> u16 {
        self.rows
    }

    #[must_use]
    pub fn cols(&self) -> u16 {
        self.cols
    }

    #[must_use]
    pub fn x_mm(&self) -> f32 {
        self.x_mm
    }

    #[must_use]
    pub fn y_mm(&self) -> f32 {
        self.y_mm
    }

    #[must_use]
    pub fn z_mm(&self) -> f32 {
        self.z_mm
    }
}

impl Default for VolDims {
    fn default() -> Self {
        Self {
            rows: 0,
            cols: 0,
            x_mm: 0f32,
            y_mm: 0f32,
            z_mm: 0f32,
        }
    }
}

impl std::fmt::Display for VolDims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}x{}, {}mm by {}mm by {}mm)",
            self.cols, self.rows, self.x_mm, self.y_mm, self.z_mm
        )
    }
}

impl PartialEq for VolDims {
    fn eq(&self, other: &Self) -> bool {
        self.rows == other.rows
            && self.cols == other.cols
            && self.x_mm == other.x_mm
            && self.y_mm == other.y_mm
            && self.z_mm == other.z_mm
    }
}

impl Eq for VolDims {}

/// Slices loaded into memory.
pub struct ImageVolume {
    slices: Vec<Vec<i16>>,
    infos: Vec<PixelDataSliceInfo>,

    series_uid: String,
    dims: VolDims,
    stride: usize,
    is_rgb: bool,
    min_val: f64,
    max_val: f64,
}

impl Default for ImageVolume {
    fn default() -> Self {
        Self {
            slices: Vec::new(),
            infos: Vec::new(),

            series_uid: String::new(),
            dims: VolDims::default(),
            stride: 0usize,
            is_rgb: false,
            min_val: f64::MAX,
            max_val: f64::MIN,
        }
    }
}

impl ImageVolume {
    #[must_use]
    pub fn slices(&self) -> &Vec<Vec<i16>> {
        &self.slices
    }

    #[must_use]
    pub fn infos(&self) -> &Vec<PixelDataSliceInfo> {
        &self.infos
    }

    #[must_use]
    pub fn dims(&self) -> &VolDims {
        &self.dims
    }

    #[must_use]
    pub fn stride(&self) -> usize {
        self.stride
    }

    #[must_use]
    pub fn is_rgb(&self) -> bool {
        self.is_rgb
    }

    #[must_use]
    pub fn series_uid(&self) -> &String {
        &self.series_uid
    }

    /// Loads a slice into this volume.
    ///
    /// # Errors
    /// - `ParseError` any errors parsing the dataset.
    /// - `PixelValueError` if the pixel values fail to parse into `i16`.
    /// - `InconsistentSliceFormat` if the slice is not in the same format as other slices already
    ///   loaded in to this volume.
    pub fn load_slice<R: Read>(&mut self, reader: R) -> Result<(), PixelDataError> {
        let dataset = BufReader::with_capacity(1024 * 1024, reader);
        let mut parser = ParserBuilder::default().build(dataset, &STANDARD_DICOM_DICTIONARY);
        let Some(dcmroot) = DicomRoot::parse(&mut parser)? else {
            return Err(PixelDataError::MissingPixelData);
        };

        let sop_uid = dcmroot.sop_instance_id()?;
        let series_uid = dcmroot.series_instance_id()?;

        let mut pdinfo = PixelDataSliceInfo::process(dcmroot);
        pdinfo.validate()?;

        self.min_val = self.min_val.min(pdinfo.min_val());
        self.max_val = self.max_val.max(pdinfo.max_val());

        let dims = pdinfo.vol_dims();
        let stride = pdinfo.stride();
        let is_rgb = pdinfo.is_rgb();

        if self.infos.is_empty() {
            self.dims = dims;
            self.stride = stride;
            self.is_rgb = is_rgb;
            self.series_uid = series_uid;
        } else {
            if series_uid != self.series_uid {
                return Err(PixelDataError::InconsistentSliceFormat(
                    sop_uid,
                    format!(
                        "SeriesInstanceUID mismatch, this: {series_uid}, other: {}",
                        self.series_uid
                    ),
                ));
            }
            if dims != self.dims {
                return Err(PixelDataError::InconsistentSliceFormat(
                    sop_uid,
                    format!("Dimensions mismatch, this: {dims}, other: {}", self.dims),
                ));
            }
            if stride != self.stride {
                return Err(PixelDataError::InconsistentSliceFormat(
                    sop_uid,
                    format!("Stride mismatch, this: {stride}, other: {}", self.stride),
                ));
            }
            if is_rgb != self.is_rgb {
                return Err(PixelDataError::InconsistentSliceFormat(
                    sop_uid,
                    format!("RGB mismatch, this: {is_rgb}, other: {}", self.is_rgb),
                ));
            }
        }

        let loaded = Self::load_pixel_data(pdinfo)?;
        let seek = &loaded.0;
        match self.infos.binary_search_by(|i| Self::cmp_by_zpos(seek, i)) {
            Err(loc) => {
                self.infos.insert(loc, loaded.0);
                self.slices.insert(loc, loaded.1);
            }
            Ok(_existing) => {
                return Err(PixelDataError::InconsistentSliceFormat(
                    loaded.0.sop_instance_id(),
                    "Multiple slices in the same z-pos".to_owned(),
                ))
            }
        }

        Ok(())
    }

    fn cmp_by_zpos(a: &PixelDataSliceInfo, b: &PixelDataSliceInfo) -> Ordering {
        // The X and Y of image position are likely to be the same, unless it's something like
        // a spinal MR acquisition.
        let a_pos = a.image_pos()[2];
        let b_pos = b.image_pos()[2];
        if a_pos < b_pos {
            Ordering::Less
        } else if a_pos > b_pos {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }

    fn load_pixel_data(
        pdinfo: PixelDataSliceInfo,
    ) -> Result<(PixelDataSliceInfo, Vec<i16>), PixelDataError> {
        match (pdinfo.bits_alloc(), pdinfo.is_rgb()) {
            (BitsAlloc::Unsupported(val), _) => Err(PixelDataError::InvalidBitsAlloc(*val)),
            (BitsAlloc::Eight, true) => Ok(PixelDataSliceU8::from_rgb_8bit(pdinfo).into_i16()),
            (BitsAlloc::Eight, false) => {
                Ok(PixelDataSliceI16::from_mono_8bit(pdinfo).into_buffer())
            }
            (BitsAlloc::Sixteen, true) => PixelDataSliceU16::from_rgb_16bit(pdinfo)?.into_i16(),
            (BitsAlloc::Sixteen, false) => {
                Ok(PixelDataSliceI16::from_mono_16bit(pdinfo)?.into_buffer())
            }
            (BitsAlloc::ThirtyTwo, true) => PixelDataSliceU32::from_rgb_32bit(pdinfo)?.into_i16(),
            (BitsAlloc::ThirtyTwo, false) => PixelDataSliceI32::from_mono_32bit(pdinfo)?.into_i16(),
        }
    }
}
