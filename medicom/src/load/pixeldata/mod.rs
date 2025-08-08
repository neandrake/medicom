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

use pdinfo::PixelDataSliceInfo;
use pixel_i16::PixelDataSliceI16;
use pixel_i32::PixelDataSliceI32;
use pixel_i8::PixelDataSliceI8;
use pixel_u16::PixelDataSliceU16;
use pixel_u32::PixelDataSliceU32;
use pixel_u8::PixelDataSliceU8;
use thiserror::Error;

use crate::core::{defn::vr::VRRef, read::ParseError};

pub mod pdinfo;
pub mod pixel_i16;
pub mod pixel_i32;
pub mod pixel_i8;
pub mod pixel_u16;
pub mod pixel_u32;
pub mod pixel_u8;
pub mod winlevel;

#[derive(Error, Debug)]
pub enum LoadError {
    #[error("Not DICOM")]
    NotDICOM,

    #[error("No Pixel Data bytes found")]
    MissingPixelData,

    #[error("Invalid size: {0}x{1}")]
    InvalidSize(u16, u16),

    #[error("Invalid dimensions: {0}")]
    InvalidDims(String),

    #[error("Invalid VR: {0:?}")]
    InvalidVR(VRRef),

    #[error("Invalid Bits Allocated: {0}")]
    InvalidBitsAlloc(u16),

    #[error("Invalid Photometric Interpretation and Samples per Pixel combo: {0:?}, {1}")]
    InvalidPhotoInterpSamples(PhotoInterp, u16),

    #[error("Invalid source location to interpret pixel data: {0}")]
    InvalidPixelSource(usize),

    #[error("Slice format does not match others in volume. SOP: {0}, error: {1}")]
    InconsistentSliceFormat(String, String),

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

    #[error("Value not within expected range")]
    PixelValueError {
        #[from]
        source: std::num::TryFromIntError,
    },

    #[error("{0}")]
    LockError(String),
}

impl From<std::io::Error> for LoadError {
    fn from(value: std::io::Error) -> Self {
        LoadError::from(ParseError::from(value))
    }
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

/// Container for the raw pixel values parsed from the DICOM binary data.
#[derive(Debug)]
pub enum PixelDataSlice {
    I8(PixelDataSliceI8),
    U8(PixelDataSliceU8),
    I16(PixelDataSliceI16),
    U16(PixelDataSliceU16),
    I32(PixelDataSliceI32),
    U32(PixelDataSliceU32),
}

impl PixelDataSlice {
    #[must_use]
    pub fn info(&self) -> &PixelDataSliceInfo {
        match self {
            PixelDataSlice::I8(pds) => pds.info(),
            PixelDataSlice::U8(pds) => pds.info(),
            PixelDataSlice::I16(pds) => pds.info(),
            PixelDataSlice::U16(pds) => pds.info(),
            PixelDataSlice::I32(pds) => pds.info(),
            PixelDataSlice::U32(pds) => pds.info(),
        }
    }

    /// Shift an `i8` value into `u8` space, so `i8::MIN` -> `u8::MIN`.
    #[must_use]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn shift_i8(val: i8) -> u8 {
        (i16::from(val).saturating_add(1) + i16::from(i8::MAX)) as u8
    }

    /// Shift an `i16` value into `u16` space, so `i16::MIN` -> `u16::MIN`.
    #[must_use]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn shift_i16(val: i16) -> u16 {
        (i32::from(val).saturating_add(1) + i32::from(i16::MAX)) as u16
    }

    /// Shift an `i32` value into `u32` space, so `i32::MIN` -> `u32::MIN`.
    #[must_use]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn shift_i32(val: i32) -> u32 {
        (i64::from(val).saturating_add(1) + i64::from(i32::MAX)) as u32
    }
}

#[cfg(test)]
mod tests {
    use crate::load::pixeldata::PixelDataSlice;

    #[test]
    pub fn test_shift_i8() {
        assert_eq!(0u8, PixelDataSlice::shift_i8(i8::MIN));
        assert_eq!(1u8, PixelDataSlice::shift_i8(i8::MIN + 1));
        assert_eq!(127u8, PixelDataSlice::shift_i8(-1));
        assert_eq!(128u8, PixelDataSlice::shift_i8(0));
        assert_eq!(129u8, PixelDataSlice::shift_i8(1));
        assert_eq!(254u8, PixelDataSlice::shift_i8(i8::MAX - 1));
        assert_eq!(255u8, PixelDataSlice::shift_i8(i8::MAX));
    }

    #[test]
    pub fn test_shift_i16() {
        assert_eq!(0u16, PixelDataSlice::shift_i16(i16::MIN));
        assert_eq!(1u16, PixelDataSlice::shift_i16(i16::MIN + 1));
        assert_eq!(32767u16, PixelDataSlice::shift_i16(-1));
        assert_eq!(32768u16, PixelDataSlice::shift_i16(0));
        assert_eq!(32769u16, PixelDataSlice::shift_i16(1));
        assert_eq!(65534u16, PixelDataSlice::shift_i16(i16::MAX - 1));
        assert_eq!(65535u16, PixelDataSlice::shift_i16(i16::MAX));
    }

    #[test]
    pub fn test_shift_i32() {
        assert_eq!(0u32, PixelDataSlice::shift_i32(i32::MIN));
        assert_eq!(1u32, PixelDataSlice::shift_i32(i32::MIN + 1));
        assert_eq!(2147483647u32, PixelDataSlice::shift_i32(-1));
        assert_eq!(2147483648u32, PixelDataSlice::shift_i32(0));
        assert_eq!(2147483649u32, PixelDataSlice::shift_i32(1));
        assert_eq!(4294967294u32, PixelDataSlice::shift_i32(i32::MAX - 1));
        assert_eq!(4294967295u32, PixelDataSlice::shift_i32(i32::MAX));
    }
}
