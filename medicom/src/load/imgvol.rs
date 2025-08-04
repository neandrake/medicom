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

use std::cmp::Ordering;

use crate::{
    core::{dcmobject::DicomRoot, values::RawValue},
    dict::tags,
    load::pixeldata::{
        pdinfo::PixelDataSliceInfo, pdwinlevel::WindowLevel, pixel_i16::PixelDataSliceI16,
        pixel_i32::PixelDataSliceI32, pixel_u16::PixelDataSliceU16, pixel_u32::PixelDataSliceU32,
        pixel_u8::PixelDataSliceU8, BitsAlloc, PhotoInterp, PixelDataError,
    },
};

const EPSILON_F64: f64 = 0.01_f64;
const EPSILON_F32: f32 = 0.01_f32;

#[derive(Debug)]
pub struct VolDims {
    /// The x-coordinate in DICOM space of the volume's origin (top-left of first slice in z-axis).
    origin_x: f32,
    /// The y-coordinate in DICOM space of the volume's origin (top-left of first slice in z-axis).
    origin_y: f32,
    /// The z-coordinate in DICOM space of the volume's origin (top-left of first slice in z-axis).
    origin_z: f32,

    /// The number of voxels across, for each axis, in (x, y, z).
    counts: (usize, usize, usize),

    /// The distance in mm between voxels, for each axis, in (x, y, z).
    vox_dims: (f32, f32, f32),
}

impl VolDims {
    #[must_use]
    pub fn new(
        origin: (f32, f32, f32),
        counts: (usize, usize, usize),
        vox_dims: (f32, f32, f32),
    ) -> Self {
        Self {
            origin_x: origin.0,
            origin_y: origin.1,
            origin_z: origin.2,
            counts,
            vox_dims,
        }
    }

    /// Checks that a dimension value is valid. A dimension value should be a positive value
    /// greater than zero.
    #[must_use]
    pub fn is_valid_dim(dim: f32) -> bool {
        !dim.is_nan() && dim > 0f32
    }

    /// Get the origin, in (x, y, z)
    #[must_use]
    pub fn origin(&self) -> (f32, f32, f32) {
        (self.origin_x, self.origin_y, self.origin_z)
    }

    #[must_use]
    pub fn counts(&self) -> (usize, usize, usize) {
        self.counts
    }

    #[must_use]
    pub fn vox_dims(&self) -> (f32, f32, f32) {
        self.vox_dims
    }

    pub fn inc_z_count(&mut self) {
        self.counts.2 += 1;
    }

    /// Set the origin, in (x, y, z).
    pub fn set_origin(&mut self, origin: (f32, f32, f32)) {
        self.origin_x = origin.0;
        self.origin_y = origin.1;
        self.origin_z = origin.2;
    }

    /// Compares one `VolDims` with another checking exact dimension matching except for the
    /// `counts.2` (z) and origin, which are values that are not determinable from an individual SOP
    /// instance.
    #[must_use]
    pub fn matches(&self, other: &VolDims) -> bool {
        self.counts.0 == other.counts.0
            && self.counts.1 == other.counts.1
            && (self.vox_dims.0 - other.vox_dims.0).abs() < EPSILON_F32
            && (self.vox_dims.1 - other.vox_dims.1).abs() < EPSILON_F32
            && (self.vox_dims.2 - other.vox_dims.2).abs() < EPSILON_F32
    }

    /// Converts indices for a pixel in the loaded volume into DICOM coordinate space.
    #[must_use]
    pub fn coordinate(&self, x: usize, y: usize, z: usize) -> (f32, f32, f32) {
        let mut coordinate = self.origin();
        coordinate.0 += f32::from(x as u16) * self.vox_dims.0;
        coordinate.1 += f32::from(y as u16) * self.vox_dims.1;
        coordinate.2 += f32::from(z as u16) * self.vox_dims.2;
        coordinate
    }
}

impl Default for VolDims {
    fn default() -> Self {
        Self {
            origin_x: 0_f32,
            origin_y: 0_f32,
            origin_z: 0_f32,
            counts: (0, 0, 0),
            vox_dims: (0f32, 0f32, 0f32),
        }
    }
}

impl std::fmt::Display for VolDims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}x{}x{}, {}mm by {}mm by {}mm, at {:.2},{:.2},{:.2})",
            self.counts.0,
            self.counts.1,
            self.counts.2,
            self.vox_dims.0,
            self.vox_dims.1,
            self.vox_dims.2,
            self.origin_x,
            self.origin_y,
            self.origin_z,
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum VolAxis {
    X,
    Y,
    Z,
}

impl std::fmt::Display for VolAxis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VolAxis::X => write!(f, "X"),
            VolAxis::Y => write!(f, "Y"),
            VolAxis::Z => write!(f, "Z"),
        }
    }
}

#[derive(Debug)]
pub struct VolPixel {
    pub coord: (usize, usize, usize),
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

/// Slices loaded into memory. Pixel values are `i16`.
pub struct ImageVolume {
    slices: Vec<Vec<i16>>,
    infos: Vec<PixelDataSliceInfo>,

    patient_name: String,
    patient_id: String,
    series_uid: String,
    series_desc: String,

    dims: VolDims,
    stride: usize,
    is_rgb: bool,
    pixel_pad: Option<i16>,
    slope: f64,
    intercept: f64,
    samples_per_pixel: usize,
    photo_interp: PhotoInterp,
    min_val: i16,
    max_val: i16,
}

impl Default for ImageVolume {
    fn default() -> Self {
        Self {
            slices: Vec::new(),
            infos: Vec::new(),

            patient_name: String::new(),
            patient_id: String::new(),
            series_uid: String::new(),
            series_desc: String::new(),

            dims: VolDims::default(),
            stride: 0usize,
            is_rgb: false,
            pixel_pad: None,
            slope: 1f64,
            intercept: 0f64,
            samples_per_pixel: 0usize,
            photo_interp: PhotoInterp::Unsupported("Unspecified".to_owned()),
            min_val: i16::MAX,
            max_val: i16::MIN,
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
    pub fn patient_name(&self) -> &String {
        &self.patient_name
    }

    #[must_use]
    pub fn patient_id(&self) -> &String {
        &self.patient_id
    }

    #[must_use]
    pub fn series_uid(&self) -> &String {
        &self.series_uid
    }

    #[must_use]
    pub fn series_desc(&self) -> &String {
        &self.series_desc
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
    pub fn pixel_pad(&self) -> Option<i16> {
        self.pixel_pad
    }

    #[must_use]
    pub fn photo_interp(&self) -> &PhotoInterp {
        &self.photo_interp
    }

    #[must_use]
    pub fn slope(&self) -> f64 {
        self.slope
    }

    #[must_use]
    pub fn intercept(&self) -> f64 {
        self.intercept
    }

    #[must_use]
    pub fn min_val(&self) -> i16 {
        self.min_val
    }

    #[must_use]
    pub fn max_val(&self) -> i16 {
        self.max_val
    }

    #[must_use]
    pub fn rescale(&self, val: f64) -> f64 {
        val * self.slope + self.intercept
    }

    #[must_use]
    pub fn byte_size(&self) -> usize {
        self.slices().iter().flatten().count() * std::mem::size_of::<i16>()
    }

    /// Returns the dimensions ordered by (width, height, depth) oriented to the given axis.
    #[must_use]
    pub fn axis_dims(&self, axis: &VolAxis) -> (usize, usize, usize) {
        match axis {
            VolAxis::X => {
                let width = self.dims.counts.1;
                let height = self.dims.counts.2;
                let depth = self.dims.counts.0;
                (width, height, depth)
            }
            VolAxis::Y => {
                let width = self.dims.counts.0;
                let height = self.dims.counts.2;
                let depth = self.dims.counts.1;
                (width, height, depth)
            }
            VolAxis::Z => {
                let width = self.dims.counts.0;
                let height = self.dims.counts.1;
                let depth = self.dims.counts.2;
                (width, height, depth)
            }
        }
    }

    /// Creates a `WindowLevel` using the minimum and maximum values occuring in this volume to
    /// compute the center and width. The out range is `f64::MIN` to `f64::MAX`.
    #[must_use]
    pub fn minmax_winlevel(&self) -> WindowLevel {
        let min = self.min_val();
        let max = self.max_val();
        let width = max - min;
        let center = min + width / 2;
        WindowLevel::new(
            String::new(),
            self.rescale(f64::from(center)),
            self.rescale(f64::from(width)),
            f64::MIN,
            f64::MAX,
        )
    }

    /// Loads a slice into this volume.
    ///
    /// # Errors
    /// - `ParseError` any errors parsing the dataset.
    /// - `PixelValueError` if the pixel values fail to parse into `i16`.
    /// - `InconsistentSliceFormat` if the slice is not in the same format as other slices already
    ///   loaded in to this volume.
    pub fn load_slice(&mut self, dcmroot: DicomRoot) -> Result<(), PixelDataError> {
        let sop_uid = dcmroot.sop_instance_id()?;
        let series_uid = dcmroot.series_instance_id()?;

        if let Some(RawValue::Strings(vals)) = dcmroot.get_value_by_tag(&tags::PatientsName) {
            if let Some(patient_name) = vals.first() {
                self.patient_name = patient_name.to_owned();
            }
        }
        if let Some(RawValue::Strings(vals)) = dcmroot.get_value_by_tag(&tags::PatientID) {
            if let Some(patient_id) = vals.first() {
                self.patient_id = patient_id.to_owned();
            }
        }

        let series_desc = dcmroot
            .get_value_by_tag(&tags::SeriesDescription)
            .and_then(|rv| rv.string().cloned())
            .unwrap_or_default();

        let pdinfo = PixelDataSliceInfo::process(dcmroot)?;

        let dims = pdinfo.vol_dims();
        let stride = pdinfo.stride();
        let is_rgb = pdinfo.is_rgb();
        let pixel_pad = pdinfo.pixel_pad().map(|v| v as i16);
        let slope = pdinfo.slope().unwrap_or(1f64);
        let intercept = pdinfo.intercept().unwrap_or(0f64);
        let samples_per_pixel = usize::from(pdinfo.samples_per_pixel());

        if self.infos.is_empty() {
            self.series_uid = series_uid;
            self.series_desc = series_desc;
            self.dims = dims;
            self.stride = stride;
            self.is_rgb = is_rgb;
            self.pixel_pad = pixel_pad;
            self.slope = slope;
            self.intercept = intercept;
            self.samples_per_pixel = samples_per_pixel;
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
            if !self.dims.matches(&dims) {
                return Err(PixelDataError::InconsistentSliceFormat(
                    sop_uid,
                    format!("Dimensions mismatch, this: {dims}, other: {}", self.dims),
                ));
            } else {
                // If volume dims match appropriately, increase the number of loaded slices.
                self.dims.inc_z_count();
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
            if pixel_pad != self.pixel_pad {
                return Err(PixelDataError::InconsistentSliceFormat(
                    sop_uid,
                    format!(
                        "Pixel Padding mismatch, this: {pixel_pad:?}, other: {:?}",
                        self.pixel_pad
                    ),
                ));
            }
            if (slope - self.slope).abs() > EPSILON_F64 {
                return Err(PixelDataError::InconsistentSliceFormat(
                    sop_uid,
                    format!("Slope mismatch: {slope}, other: {}", self.slope),
                ));
            }
            if (intercept - self.intercept).abs() > EPSILON_F64 {
                return Err(PixelDataError::InconsistentSliceFormat(
                    sop_uid,
                    format!("Intercept mismatch: {intercept}, other: {}", self.intercept),
                ));
            }
            if samples_per_pixel != self.samples_per_pixel {
                return Err(PixelDataError::InconsistentSliceFormat(
                    sop_uid,
                    format!(
                        "Samples per Pixel mismatch: {samples_per_pixel}, other: {}",
                        self.samples_per_pixel
                    ),
                ));
            }
        }

        let loaded = Self::load_pixel_data(pdinfo)?;
        self.min_val = self.min_val.min(loaded.0.min_val() as i16);
        self.max_val = self.max_val.max(loaded.0.max_val() as i16);

        let seek = &loaded.0;
        match self.infos.binary_search_by(|i| Self::cmp_by_zpos(seek, i)) {
            Err(loc) => {
                self.infos.insert(loc, loaded.0);
                self.slices.insert(loc, loaded.1);
                // Update the origin of the volume to be the first slice's, after sorted insertion.
                if let Some(first_info) = self.infos.first() {
                    self.dims.set_origin(first_info.vol_dims().origin());
                }
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

    /// Loads the PixelData for the given slice. The pixel values will be trunacted to `i16`.
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

    /// Gets the pixel at the given coordinate (x, y, z).
    ///
    /// # Parameters
    /// `coord`: The coordinate whose pixel value to retrieve. This coordinate must be in the
    ///          native plane orientation, `VolAxis::Z`.
    ///
    /// # Errors
    /// - If the x,y,z coordinate is invalid, either by being outside the image dimensions, or if
    ///   the Planar Configuration and Samples per Pixel are set up such that beginning of RGB
    ///   values must occur at specific indices.
    pub fn get_pixel(&self, coord: (usize, usize, usize)) -> Result<VolPixel, PixelDataError> {
        let Some(buffer) = self.slices().get(coord.2) else {
            return Err(PixelDataError::InvalidDims(format!(
                "Invalid z-pos: {}",
                coord.2
            )));
        };

        let cols = self.dims.counts.0;
        let pixel_count = coord.0 + coord.1 * cols;
        let pixel_count = pixel_count * self.samples_per_pixel;
        if pixel_count >= buffer.len()
            || (self.is_rgb && pixel_count + self.stride * 2 >= buffer.len())
        {
            return Err(PixelDataError::InvalidPixelSource(pixel_count));
        }

        let (r, g, b) = if self.is_rgb {
            let red = buffer[pixel_count];
            let green = buffer[pixel_count + self.stride];
            let blue = buffer[pixel_count + self.stride * 2];
            (f64::from(red), f64::from(green), f64::from(blue))
        } else {
            let applied_val = buffer
                .get(pixel_count)
                .copied()
                .map(f64::from)
                .or_else(|| self.pixel_pad().map(f64::from))
                .map(|v| self.rescale(v))
                .unwrap_or_default();
            let val = applied_val;
            (val, val, val)
        };

        Ok(VolPixel { coord, r, g, b })
    }

    #[must_use]
    pub fn slice_iter(&self, axis: &VolAxis, axis_index: usize) -> ImageVolumeAxisSliceIter {
        ImageVolumeAxisSliceIter {
            vol: self,
            axis: axis.clone(),
            axis_index,
            pixel_count: 0,
        }
    }
}

/// Iterates through a slice within a volume, returning pixels in the order of a standard image
/// layout, starting in the top-left incrementing horizontally and then vertically.
pub struct ImageVolumeAxisSliceIter<'buf> {
    /// The image volume to create a slice for.
    vol: &'buf ImageVolume,
    /// The axis to orient the volume for producing a plane of pixels.
    axis: VolAxis,
    /// The index into the volume indicating the slice to produce, oriented by the axis.
    axis_index: usize,
    /// Internal state of which pixel to return next.
    pixel_count: usize,
}

impl<'buf> ImageVolumeAxisSliceIter<'buf> {
    /// Compute the relative row and column from pixel_count, which used with axis_index to produce
    /// the x,y,z coordinate within the volume whose pixel to retrieve.
    fn compute_coord(&self, index: usize) -> Option<(usize, usize, usize)> {
        match self.axis {
            VolAxis::X => {
                let cols = self.vol.dims.counts.1;
                let rows = self.vol.dims.counts.2;
                if index >= rows * cols {
                    return None;
                }
                let y = index % cols;
                let z = (index / cols) % rows;
                Some((self.axis_index, y, z))
            }
            VolAxis::Y => {
                let cols = self.vol.dims.counts.0;
                let rows = self.vol.dims.counts.2;
                if index >= rows * cols {
                    return None;
                }
                let x = index % cols;
                let z = (index / cols) % rows;
                Some((x, self.axis_index, z))
            }
            VolAxis::Z => {
                let cols = self.vol.dims.counts.0;
                let rows = self.vol.dims.counts.1;
                if index >= rows * cols {
                    return None;
                }
                let x = index % cols;
                let y = (index / cols) % rows;
                Some((x, y, self.axis_index))
            }
        }
    }
}

impl Iterator for ImageVolumeAxisSliceIter<'_> {
    type Item = VolPixel;

    fn next(&mut self) -> Option<Self::Item> {
        let coord = self.compute_coord(self.pixel_count)?;
        self.pixel_count += 1;
        self.vol.get_pixel(coord).ok()
    }
}
