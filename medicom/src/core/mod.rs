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

// A number of the structures defined in this module will include the module name in the structure
// name. This is intentional as many of these structs will be used in common code and have a chance
// to collide, e.g. `ParseError` and `WriteError` are likely to be used in the same code, and the
// distinguishing structure names aid readability and ergonomics.
#![allow(clippy::module_name_repetitions)]

pub mod charset;
pub mod dcmelement;
pub mod dcmobject;
pub mod dcmsqelem;
pub mod defn;
pub mod inspect;
pub mod read;
pub mod values;
pub mod write;

//pub use values::Attribute;
//pub use values::RawValue;

/// The DICOM Prefix that should follow File Preambles.
pub static DICOM_PREFIX: &[u8; DICOM_PREFIX_LENGTH] = b"DICM";

/// The length of a DICOM Prefix.
pub const DICOM_PREFIX_LENGTH: usize = 4;

/// The length of a File Preamble.
pub const FILE_PREAMBLE_LENGTH: usize = 128;
