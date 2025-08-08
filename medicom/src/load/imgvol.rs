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
    load::{
        pixeldata::{
            pdinfo::PixelDataSliceInfo, pixel_i16::PixelDataSliceI16, pixel_i32::PixelDataSliceI32,
            pixel_u16::PixelDataSliceU16, pixel_u32::PixelDataSliceU32, pixel_u8::PixelDataSliceU8,
            winlevel::WindowLevel, BitsAlloc, LoadError, PhotoInterp,
        },
        IndexVec, VolAxis, VolDims, VolPixel, EPSILON_F32,
    },
};

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
    slope: f32,
    intercept: f32,
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
            slope: 1_f32,
            intercept: 0_f32,
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
    pub fn slope(&self) -> f32 {
        self.slope
    }

    #[must_use]
    pub fn intercept(&self) -> f32 {
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
    pub fn rescale(&self, val: f32) -> f32 {
        val * self.slope + self.intercept
    }

    #[must_use]
    pub fn byte_size(&self) -> usize {
        self.slices().iter().flatten().count() * std::mem::size_of::<i16>()
    }

    /// Returns the dimensions ordered by (width, height, depth) oriented to the given axis.
    #[must_use]
    pub fn axis_dims(&self, axis: &VolAxis) -> IndexVec {
        let (width, height, depth) = match axis {
            VolAxis::X => {
                let width = self.dims.counts.y;
                let height = self.dims.counts.z;
                let depth = self.dims.counts.x;
                (width, height, depth)
            }
            VolAxis::Y => {
                let width = self.dims.counts.x;
                let height = self.dims.counts.z;
                let depth = self.dims.counts.y;
                (width, height, depth)
            }
            VolAxis::Z => {
                let width = self.dims.counts.x;
                let height = self.dims.counts.y;
                let depth = self.dims.counts.z;
                (width, height, depth)
            }
        };
        IndexVec {
            x: width,
            y: height,
            z: depth,
        }
    }

    /// Creates a `WindowLevel` using the minimum and maximum values occuring in this volume to
    /// compute the center and width. The out range is `f32::MIN` to `f32::MAX`.
    #[must_use]
    pub fn minmax_winlevel(&self) -> WindowLevel {
        let min = self.min_val();
        let max = self.max_val();
        let width = max - min;
        let center = min + width / 2;
        WindowLevel::new(
            String::new(),
            self.rescale(f32::from(center)),
            self.rescale(f32::from(width)),
            f32::MIN,
            f32::MAX,
        )
    }

    /// Loads a slice into this volume.
    ///
    /// # Errors
    /// - `ParseError` any errors parsing the dataset.
    /// - `PixelValueError` if the pixel values fail to parse into `i16`.
    /// - `InconsistentSliceFormat` if the slice is not in the same format as other slices already
    ///   loaded in to this volume.
    #[allow(clippy::too_many_lines)]
    pub fn load_slice(&mut self, dcmroot: DicomRoot) -> Result<(), LoadError> {
        let sop_uid = dcmroot.sop_instance_id()?;
        let series_uid = dcmroot.series_instance_id()?;

        if let Some(RawValue::Strings(vals)) = dcmroot.get_value_by_tag(&tags::PatientsName) {
            if let Some(patient_name) = vals.first() {
                patient_name.clone_into(&mut self.patient_name);
            }
        }
        if let Some(RawValue::Strings(vals)) = dcmroot.get_value_by_tag(&tags::PatientID) {
            if let Some(patient_id) = vals.first() {
                patient_id.clone_into(&mut self.patient_id);
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
        let slope = pdinfo.slope().unwrap_or(1_f32);
        let intercept = pdinfo.intercept().unwrap_or(0_f32);
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
                return Err(LoadError::InconsistentSliceFormat(
                    sop_uid,
                    format!(
                        "SeriesInstanceUID mismatch, this: {series_uid}, other: {}",
                        self.series_uid
                    ),
                ));
            }
            if self.dims.matches(&dims) {
                // If volume dims match appropriately, increase the number of loaded slices.
                self.dims.inc_z_count();
            } else {
                return Err(LoadError::InconsistentSliceFormat(
                    sop_uid,
                    format!("Dimensions mismatch, this: {dims}, other: {}", self.dims),
                ));
            }
            if stride != self.stride {
                return Err(LoadError::InconsistentSliceFormat(
                    sop_uid,
                    format!("Stride mismatch, this: {stride}, other: {}", self.stride),
                ));
            }
            if is_rgb != self.is_rgb {
                return Err(LoadError::InconsistentSliceFormat(
                    sop_uid,
                    format!("RGB mismatch, this: {is_rgb}, other: {}", self.is_rgb),
                ));
            }
            if pixel_pad != self.pixel_pad {
                return Err(LoadError::InconsistentSliceFormat(
                    sop_uid,
                    format!(
                        "Pixel Padding mismatch, this: {pixel_pad:?}, other: {:?}",
                        self.pixel_pad
                    ),
                ));
            }
            if (slope - self.slope).abs() > EPSILON_F32 {
                return Err(LoadError::InconsistentSliceFormat(
                    sop_uid,
                    format!("Slope mismatch: {slope}, other: {}", self.slope),
                ));
            }
            if (intercept - self.intercept).abs() > EPSILON_F32 {
                return Err(LoadError::InconsistentSliceFormat(
                    sop_uid,
                    format!("Intercept mismatch: {intercept}, other: {}", self.intercept),
                ));
            }
            if samples_per_pixel != self.samples_per_pixel {
                return Err(LoadError::InconsistentSliceFormat(
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
                return Err(LoadError::InconsistentSliceFormat(
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

    /// Loads the `PixelData` for the given slice. The pixel values will be trunacted to `i16`.
    fn load_pixel_data(
        pdinfo: PixelDataSliceInfo,
    ) -> Result<(PixelDataSliceInfo, Vec<i16>), LoadError> {
        match (pdinfo.bits_alloc(), pdinfo.is_rgb()) {
            (BitsAlloc::Unsupported(val), _) => Err(LoadError::InvalidBitsAlloc(*val)),
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
    pub fn get_pixel(&self, coord: IndexVec) -> Result<VolPixel, LoadError> {
        let Some(buffer) = self.slices().get(coord.z) else {
            return Err(LoadError::InvalidDims(format!(
                "Invalid z-pos: {}",
                coord.z
            )));
        };

        let cols = self.dims.counts.x;
        let pixel_count = coord.x + coord.y * cols;
        let pixel_count = pixel_count * self.samples_per_pixel;
        if pixel_count >= buffer.len()
            || (self.is_rgb && pixel_count + self.stride * 2 >= buffer.len())
        {
            return Err(LoadError::InvalidPixelSource(pixel_count));
        }

        let (r, g, b) = if self.is_rgb {
            let red = buffer[pixel_count];
            let green = buffer[pixel_count + self.stride];
            let blue = buffer[pixel_count + self.stride * 2];
            (f32::from(red), f32::from(green), f32::from(blue))
        } else {
            let applied_val = buffer
                .get(pixel_count)
                .copied()
                .map(f32::from)
                .or_else(|| self.pixel_pad().map(f32::from))
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

impl ImageVolumeAxisSliceIter<'_> {
    /// Compute the relative row and column from `pixel_count`, which used with `axis_index` to
    /// produce the x,y,z coordinate within the volume whose pixel to retrieve.
    fn compute_coord(&self, index: usize) -> Option<IndexVec> {
        match self.axis {
            VolAxis::X => {
                let cols = self.vol.dims.counts.y;
                let rows = self.vol.dims.counts.z;
                if index >= rows * cols {
                    return None;
                }
                let y = index % cols;
                let z = (index / cols) % rows;
                Some(IndexVec {
                    x: self.axis_index,
                    y,
                    z,
                })
            }
            VolAxis::Y => {
                let cols = self.vol.dims.counts.x;
                let rows = self.vol.dims.counts.z;
                if index >= rows * cols {
                    return None;
                }
                let x = index % cols;
                let z = (index / cols) % rows;
                Some(IndexVec {
                    x,
                    y: self.axis_index,
                    z,
                })
            }
            VolAxis::Z => {
                let cols = self.vol.dims.counts.x;
                let rows = self.vol.dims.counts.y;
                if index >= rows * cols {
                    return None;
                }
                let x = index % cols;
                let y = (index / cols) % rows;
                Some(IndexVec {
                    x,
                    y,
                    z: self.axis_index,
                })
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
