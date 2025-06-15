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

use std::io::{BufReader, Read};

use crate::{
    core::{dcmobject::DicomRoot, read::ParserBuilder},
    dict::{stdlookup::STANDARD_DICOM_DICTIONARY, tags::SOPInstanceUID},
    load::pixeldata::{pdinfo::PixelDataSliceInfo, PixelDataError},
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

    dims: VolDims,
    stride: usize,
    is_rgb: bool,
    min_val: f64,
    max_val: f64,
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

    pub fn load_slice<R: Read>(&mut self, reader: R) -> Result<(), PixelDataError> {
        let dataset = BufReader::with_capacity(1024 * 1024, reader);
        let mut parser = ParserBuilder::default().build(dataset, &STANDARD_DICOM_DICTIONARY);
        let Some(dcmroot) = DicomRoot::parse(&mut parser)? else {
            return Err(PixelDataError::MissingPixelData);
        };

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
        } else {
            let sop = pdinfo
                .dcmroot()
                .get_value_by_tag(&SOPInstanceUID)
                .and_then(|v| v.string().cloned())
                .unwrap_or_else(|| "<NO SOP>".to_owned());
            if dims != self.dims {
                return Err(PixelDataError::InconsistentSliceFormat(
                    sop,
                    format!("Dimensions mismatch, this: {dims}, other: {}", self.dims),
                ));
            }

            if stride != self.stride {
                return Err(PixelDataError::InconsistentSliceFormat(
                    sop,
                    format!("Stride mismatch, this: {stride}, other: {}", self.stride),
                ));
            }
            if is_rgb != self.is_rgb {
                return Err(PixelDataError::InconsistentSliceFormat(
                    sop,
                    format!("RGB mismatch, this: {is_rgb}, other: {}", self.is_rgb),
                ));
            }
        }

        self.infos.push(pdinfo);

        Ok(())
    }
}

impl Default for ImageVolume {
    fn default() -> Self {
        Self {
            slices: Vec::new(),
            infos: Vec::new(),
            min_val: f64::MAX,
            max_val: f64::MIN,
            dims: VolDims::default(),
            stride: 0usize,
            is_rgb: false,
        }
    }
}
