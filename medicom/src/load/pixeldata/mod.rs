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

use thiserror::Error;

use crate::core::{defn::vr::VRRef, read::ParseError};

pub mod pdinfo;
pub mod pdslice;
pub mod pdwinlevel;
pub mod pixel_i16;
pub mod pixel_i32;
pub mod pixel_i8;
pub mod pixel_u16;
pub mod pixel_u32;
pub mod pixel_u8;

#[derive(Error, Debug)]
pub enum PixelDataError {
    #[error("No Pixel Data bytes found")]
    MissingPixelData,

    #[error("Invalid dimensions: {0}x{1}")]
    InvalidSize(u16, u16),

    #[error("Invalid VR: {0:?}")]
    InvalidVR(VRRef),

    #[error("Invalid Bits Allocated: {0}")]
    InvalidBitsAlloc(u16),

    #[error("Invalid Photometric Interpretation and Samples per Pixel combo: {0:?}, {1}")]
    InvalidPhotoInterpSamples(PhotoInterp, u16),

    #[error("Invalid source location to interpret pixel data: {0}")]
    InvalidPixelSource(usize),

    #[error("Error parsing DICOM")]
    ParseError {
        #[from]
        source: ParseError,
    },

    #[error("Error interpreting bytes")]
    BytesError {
        #[from]
        source: std::array::TryFromSliceError,
    },
}

/// Supported values of Photometric Interpretation.
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum PhotoInterp {
    Unsupported(String),
    Rgb,
    Monochrome1,
    Monochrome2,
}

impl PhotoInterp {
    /// Whether this `PhotoInterp` is `RGB`.
    #[must_use]
    pub fn is_rgb(&self) -> bool {
        *self == PhotoInterp::Rgb
    }

    /// Whether this `PhotoInterp` is one of the supported monochrome values.
    #[must_use]
    pub fn is_monochrome(&self) -> bool {
        *self == PhotoInterp::Monochrome1 || *self == PhotoInterp::Monochrome2
    }
}

impl From<&str> for PhotoInterp {
    /// Parse Photometric Interpretation from its DICOM element value.
    #[must_use]
    fn from(value: &str) -> Self {
        if value == "RGB" {
            Self::Rgb
        } else if value == "MONOCHROME1" {
            Self::Monochrome1
        } else if value == "MONOCHROME2" {
            Self::Monochrome2
        } else {
            Self::Unsupported(value.to_owned())
        }
    }
}

/// Supported values of Bits Allocated.
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum BitsAlloc {
    Unsupported(u16),
    Eight,
    Sixteen,
    ThirtyTwo,
}

impl std::fmt::Display for BitsAlloc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::fmt::Debug for BitsAlloc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unsupported(other) => write!(f, "BitsAlloc(Unsupported:{other})"),
            Self::Eight => write!(f, "BitsAlloc(8)"),
            Self::Sixteen => write!(f, "BitsAlloc(16)"),
            Self::ThirtyTwo => write!(f, "BitsAlloc(32)"),
        }
    }
}

impl BitsAlloc {
    /// Parse Bits Allocated from its DICOM value.
    #[must_use]
    pub fn from_val(val: u16) -> Self {
        match val {
            8 => BitsAlloc::Eight,
            16 => BitsAlloc::Sixteen,
            32 => BitsAlloc::ThirtyTwo,
            other => BitsAlloc::Unsupported(other),
        }
    }

    #[must_use]
    pub fn val(&self) -> u16 {
        match self {
            Self::Unsupported(val) => *val,
            Self::Eight => 8,
            Self::Sixteen => 16,
            Self::ThirtyTwo => 32,
        }
    }
}
