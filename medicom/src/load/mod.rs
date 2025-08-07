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

pub mod imgvol;
pub mod pixeldata;

/// General epsilon when comparing f32s which should be valid for most units within DICOM.
pub(crate) const EPSILON_F32: f32 = 0.01_f32;

#[derive(Clone, Copy, Debug, Default)]
pub struct IndexVec {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DicomVec {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Default)]
pub struct VolDims {
    /// The coordinate in DICOM space of the volume's origin (top-left of first slice in z-axis).
    origin: DicomVec,
    /// The number of voxels across each axis.
    counts: IndexVec,
    /// The distance in mm between voxels.
    voxel_dims: DicomVec,
}

impl VolDims {
    #[must_use]
    pub fn new(origin: DicomVec, counts: IndexVec, voxel_dims: DicomVec) -> Self {
        Self {
            origin,
            counts,
            voxel_dims,
        }
    }

    /// Checks that a dimension value is valid. A dimension value should be a positive value
    /// greater than zero.
    #[must_use]
    pub fn is_valid_dim(dim: f32) -> bool {
        !dim.is_nan() && dim > 0f32
    }

    #[must_use]
    pub fn origin(&self) -> DicomVec {
        self.origin
    }

    #[must_use]
    pub fn counts(&self) -> IndexVec {
        self.counts
    }

    #[must_use]
    pub fn voxel_dims(&self) -> DicomVec {
        self.voxel_dims
    }

    pub fn inc_z_count(&mut self) {
        self.counts.z += 1;
    }

    pub fn set_origin(&mut self, origin: DicomVec) {
        self.origin = origin;
    }

    /// Compares one `VolDims` with another checking exact dimension matching except for the
    /// `counts.z` and origin, which are values that are not determinable from an individual SOP
    /// instance.
    #[must_use]
    pub fn matches(&self, other: &VolDims) -> bool {
        self.counts.x == other.counts.x
            && self.counts.y == other.counts.y
            && (self.voxel_dims.x - other.voxel_dims.x).abs() < EPSILON_F32
            && (self.voxel_dims.y - other.voxel_dims.y).abs() < EPSILON_F32
            && (self.voxel_dims.z - other.voxel_dims.z).abs() < EPSILON_F32
    }

    /// Converts indices for a pixel in the loaded volume into DICOM coordinate space.
    #[must_use]
    pub fn coordinate(&self, pos: IndexVec) -> DicomVec {
        let mut coordinate = self.origin();
        coordinate.x += f32::from(pos.x as u16) * self.voxel_dims.x;
        coordinate.y += f32::from(pos.y as u16) * self.voxel_dims.y;
        coordinate.z += f32::from(pos.z as u16) * self.voxel_dims.z;
        coordinate
    }
}

impl std::fmt::Display for VolDims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}x{}x{}, {}mm by {}mm by {}mm, at {:.2},{:.2},{:.2})",
            self.counts.x,
            self.counts.y,
            self.counts.z,
            self.voxel_dims.x,
            self.voxel_dims.y,
            self.voxel_dims.z,
            self.origin.x,
            self.origin.y,
            self.origin.z,
        )
    }
}

/// Axes of an `ImageVolume`. The `Z` axis is the native plane for the dicom dataset.
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

/// A pixel value within an `ImageVolume`.
#[derive(Debug)]
pub struct VolPixel {
    pub coord: IndexVec,
    pub r: f32,
    pub g: f32,
    pub b: f32,
}
