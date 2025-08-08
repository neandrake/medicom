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

use std::{
    io::{BufReader, Read},
    marker::PhantomData,
    sync::RwLock,
};

use imgvol::ImageVolume;
use pixeldata::LoadError;
use workspace::Workspace;

use crate::{
    core::{dcmobject::DicomRoot, read::ParserBuilder},
    dict::stdlookup::STANDARD_DICOM_DICTIONARY,
};

pub mod imgvol;
pub mod pixeldata;
pub mod workspace;

/// General epsilon when comparing f32s which should be valid for most units within DICOM.
pub(crate) const EPSILON_F32: f32 = 0.01_f32;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub struct LoadableKey {
    key: String,
}

impl LoadableKey {
    pub fn new(series_uid: String) -> Self {
        Self { key: series_uid }
    }

    pub fn key(&self) -> &str {
        &self.key
    }
}

impl From<&str> for LoadableKey {
    fn from(value: &str) -> Self {
        LoadableKey::new(value.to_owned())
    }
}

impl From<&String> for LoadableKey {
    fn from(value: &String) -> Self {
        LoadableKey::new(value.to_owned())
    }
}

impl std::fmt::Display for LoadableKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.key)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub struct LoadableChunkKey {
    chunk_key: String,
}

impl LoadableChunkKey {
    pub fn new(chunk_key: String) -> Self {
        Self { chunk_key }
    }

    pub fn chunk_key(&self) -> &String {
        &self.chunk_key
    }
}

impl From<&str> for LoadableChunkKey {
    fn from(value: &str) -> Self {
        LoadableChunkKey::new(value.to_owned())
    }
}

impl From<&String> for LoadableChunkKey {
    fn from(value: &String) -> Self {
        LoadableChunkKey::new(value.to_owned())
    }
}

impl std::fmt::Display for LoadableChunkKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.chunk_key)
    }
}

/// Provides the datasets for loading into a volume.
pub trait SeriesSource<R: Read> {
    /// The `DatasetKey` this `DatasetSource` is for.
    fn loadable_key(&self) -> LoadableKey;
    /// The list of SOPs within this Series.
    fn chunks(&self) -> Result<Vec<LoadableChunkKey>, LoadError>;
    /// Retrieve the dataset stream for a `SopKey`.
    ///
    /// # Errors
    /// - `LoadError` if there's failure establishing the dataset stream.
    fn chunk_stream(&self, chunk_key: &LoadableChunkKey) -> Result<R, LoadError>;
}

/// Provides implementations for loading `SeriesSource` into `Workspace`.
pub struct Loader<R: Read> {
    _phantom: PhantomData<R>,
}

impl<R: Read> Loader<R> {
    // Deriving Default doesn't seem to work properly with the template <R>.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }

    /// Loads this source into a `Workspace`.
    pub fn load_into(
        &self,
        source: &impl SeriesSource<R>,
        workspace: &RwLock<Workspace>,
        progress: Option<&RwLock<SeriesSourceLoadResult>>,
    ) -> Result<(), LoadError> {
        for chunk_key in source.chunks()? {
            let mut workspace = match workspace.write() {
                Err(e) => return Err(LoadError::LockError(format!("{e:?}"))),
                Ok(workspace) => workspace,
            };

            let imgvol = if let Some(imgvol) = workspace.volume_mut(&source.loadable_key()) {
                imgvol
            } else {
                workspace.initialize_vol(source.loadable_key())
            };
            let success = self.load_chunk(source, imgvol, &chunk_key).is_ok();
            if let Some(progress) = progress {
                if let Ok(mut progress) = progress.write() {
                    if success {
                        progress.add_loaded(chunk_key);
                    } else {
                        progress.add_failed(chunk_key);
                    }
                }
            }
        }
        Ok(())
    }

    fn load_chunk(
        &self,
        source: &impl SeriesSource<R>,
        imgvol: &mut ImageVolume,
        chunk_key: &LoadableChunkKey,
    ) -> Result<(), LoadError> {
        let ds = source.chunk_stream(chunk_key)?;
        let dataset = BufReader::with_capacity(1024 * 1024, ds);
        let mut parser = ParserBuilder::default().build(dataset, &STANDARD_DICOM_DICTIONARY);
        let dcmroot = DicomRoot::parse(&mut parser)?.ok_or(LoadError::NotDICOM)?;
        imgvol.load_slice(dcmroot)?;
        Ok(())
    }
}

pub struct SeriesSourceLoadResult {
    total: Vec<LoadableChunkKey>,
    loaded: Vec<LoadableChunkKey>,
    failed: Vec<LoadableChunkKey>,
}

impl SeriesSourceLoadResult {
    pub fn new(total: Vec<LoadableChunkKey>) -> Self {
        Self {
            total,
            loaded: Vec::new(),
            failed: Vec::new(),
        }
    }

    pub fn total(&self) -> &Vec<LoadableChunkKey> {
        &self.total
    }

    pub fn add_loaded(&mut self, loaded: LoadableChunkKey) {
        self.loaded.push(loaded);
    }

    pub fn add_failed(&mut self, failed: LoadableChunkKey) {
        self.failed.push(failed);
    }

    pub fn num_total(&self) -> usize {
        self.total.len()
    }

    pub fn num_loaded(&self) -> usize {
        self.loaded.len()
    }

    pub fn num_failed(&self) -> usize {
        self.failed.len()
    }
}

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
