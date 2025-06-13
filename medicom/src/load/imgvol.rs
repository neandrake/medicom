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
    dict::stdlookup::STANDARD_DICOM_DICTIONARY,
    load::pixeldata::{pdinfo::PixelDataSliceInfo, PixelDataError},
};

/// Slices loaded into memory.
pub struct ImageVolume {
    slices: Vec<Vec<i16>>,
    infos: Vec<PixelDataSliceInfo>,

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

        let pdinfo = PixelDataSliceInfo::process(dcmroot);
        self.min_val = self.min_val.min(pdinfo.min_val());
        self.max_val = self.max_val.max(pdinfo.max_val());

        self.infos.push(pdinfo);

        Ok(())
    }
}

impl Default for ImageVolume {
    fn default() -> Self {
        Self {
            slices: Vec::new(),
            infos: Vec::new(),
            stride: 0usize,
            is_rgb: false,
            min_val: f64::MAX,
            max_val: f64::MIN,
        }
    }
}
