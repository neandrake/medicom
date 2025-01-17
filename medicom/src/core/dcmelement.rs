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

//! DICOM Element Definition

use std::{fmt, iter::once};

use crate::core::{
    charset::CSRef,
    dcmsqelem::SequenceElement,
    defn::{
        constants::{
            tags::{
                DOUBLE_PIXEL_DATA, FLOAT_PIXEL_DATA, ITEM, ITEM_DELIMITATION_ITEM, PIXEL_DATA,
                SEQUENCE_DELIMITATION_ITEM,
            },
            ts::ExplicitVRLittleEndian,
        },
        is_non_standard_sq, is_sq_delim,
        tag::{Tag, TagNode, TagPath},
        ts::TSRef,
        vl::ValueLength,
        vr::{VRRef, INVALID_VR, SQ},
    },
    read::parser::ParseResult,
    values::{ElementWithVr, RawValue},
    write::valencode::ElemAndRawValue,
};

/// Represents a DICOM Element including its Tag, VR, and Value
/// Provides methods for parsing the element value as different native types
#[derive(Clone)]
pub struct DicomElement {
    tag: u32,
    vr: VRRef,
    vl: ValueLength,

    data: Vec<u8>,
    sq_path: Vec<SequenceElement>,

    ts: TSRef,
    cs: CSRef,
}

impl fmt::Debug for DicomElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: VR[{:?}], VL[{:?}], TS[{}]",
            Tag::format_tag_to_display(self.tag()),
            self.vr(),
            self.vl(),
            self.ts().uid().ident()
        )
    }
}

impl DicomElement {
    /// Creates a new element with all fields specified.
    pub fn new<T>(
        tag: T,
        vr: VRRef,
        vl: ValueLength,
        ts: TSRef,
        cs: CSRef,
        data: Vec<u8>,
        sq_path: Vec<SequenceElement>,
    ) -> Self
    where
        u32: From<T>,
    {
        let cs: CSRef = vr.get_proper_cs(cs);
        Self {
            tag: u32::from(tag),
            vr,
            vl,
            data,
            sq_path,
            ts,
            cs,
        }
    }

    /// Creates a new empty element with the given tag, VR, and transfer syntax. This element will
    /// be initialized with the default character set.
    pub fn new_empty<T>(tag: T, vr: VRRef, ts: TSRef) -> Self
    where
        u32: From<T>,
    {
        let cs: CSRef = vr.get_proper_cs(CSRef::default());
        Self {
            tag: u32::from(tag),
            vr,
            vl: ValueLength::Explicit(0),
            data: Vec::with_capacity(0),
            sq_path: Vec::with_capacity(0),
            ts,
            cs,
        }
    }

    /// Creates a new sentinel element which is not a valid `DicomElement`. This can be
    /// distinguished from a valid `DicomElement` by having a tag value of zero, VR of `INVALID`,
    /// and transfer syntax of `ExplicitVRLittleEndian`. Note that all three of these elements
    /// should be checked as a tag of zero is valid for DIMSE, "Command Group Length". Use the
    /// `is_sentinel()` function to check for this specific case.
    #[must_use]
    pub fn new_sentinel() -> Self {
        DicomElement {
            tag: 0,
            vr: &INVALID_VR,
            vl: ValueLength::Explicit(0),
            data: Vec::with_capacity(0),
            sq_path: Vec::with_capacity(0),
            ts: &ExplicitVRLittleEndian,
            cs: CSRef::default(),
        }
    }

    #[must_use]
    pub fn tag(&self) -> u32 {
        self.tag
    }

    #[must_use]
    pub fn vl(&self) -> ValueLength {
        self.vl
    }

    #[must_use]
    pub fn vr(&self) -> VRRef {
        self.vr
    }

    #[must_use]
    pub fn ts(&self) -> TSRef {
        self.ts
    }

    #[must_use]
    pub fn cs(&self) -> CSRef {
        self.cs
    }

    #[must_use]
    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }

    #[must_use]
    pub fn data_mut(&mut self) -> &mut Vec<u8> {
        &mut self.data
    }

    #[must_use]
    pub fn sq_path(&self) -> &Vec<SequenceElement> {
        &self.sq_path
    }

    /// Returns the number of bytes this element will consist of when encoded into a dataset.
    /// Refer to Part 5, Chapter 7.1
    #[must_use]
    pub fn byte_size(&self) -> usize {
        let mut byte_len = 0usize;

        // tag
        byte_len += 4;

        // vr
        byte_len += if self.ts.explicit_vr() {
            if self.vr.has_explicit_2byte_pad {
                4
            } else {
                2
            }
        } else {
            0
        };

        // vl
        byte_len += if self.ts.explicit_vr() {
            if self.vr.has_explicit_2byte_pad {
                4
            } else {
                2
            }
        } else {
            4
        };

        // value
        byte_len += self.data.len();

        byte_len
    }

    /// Returns if this element is a `SQ` or if it should be parsed as though it were a sequence.
    #[must_use]
    pub fn is_sq_like(&self) -> bool {
        self.vr == &SQ || self.has_fragments()
    }

    /// Returns if this element's value is encoded in fragments.
    #[must_use]
    pub fn has_fragments(&self) -> bool {
        is_non_standard_sq(self.tag, self.vr, self.vl)
    }

    /// Returns if this element is a sequence delimiter element.
    #[must_use]
    pub fn is_sq_delim(&self) -> bool {
        is_sq_delim(self.tag)
    }

    /// Returns if this element is for `PixelData`, `FloatPixelData`, or `DoubleFloatPixelData`.
    #[must_use]
    pub fn is_pixel_data(&self) -> bool {
        self.tag == PIXEL_DATA || self.tag == FLOAT_PIXEL_DATA || self.tag == DOUBLE_PIXEL_DATA
    }

    /// Returns if this element is within a `PixelData` element (via `is_pixel_data()`).
    #[must_use]
    pub fn is_within_pixel_data(&self) -> bool {
        !self.is_pixel_data() && self.sq_path.iter().any(SequenceElement::is_pixel_data)
    }

    /// Returns whether the the size of the value field for this element is zero.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Checks if this `DicomElement` is a sentinel value, identified by having the following
    /// properties:
    ///
    /// - A root element (empty `sq_path`)
    /// - Tag value of zero
    /// - VR of `&vr::INVALID`
    /// - Transfer Syntax of `ExplicitVRLittleEndian`
    #[must_use]
    pub fn is_sentinel(&self) -> bool {
        self.sq_path.is_empty()
            && self.tag == 0
            && self.vr == &INVALID_VR
            && self.ts == &ExplicitVRLittleEndian
    }

    /// Creates a `TagPath` for the current element.
    #[must_use]
    pub fn create_tagpath(&self) -> TagPath {
        if self.is_sentinel() {
            TagPath::empty()
        } else {
            self.sq_path
                .iter()
                .filter(|sq| sq.sq_tag() != ITEM)
                .map(|sq| sq.node().clone())
                .chain(once(self.tag.into()))
                .collect::<Vec<TagNode>>()
                .into()
        }
    }

    /// Parses this element's data into native/raw value type.
    ///
    /// # Errors
    /// Decoding of the element's value may fail.
    pub fn parse_value(&self) -> ParseResult<RawValue> {
        RawValue::try_from(self)
    }

    /// Parses this element's data into native/raw value type, using a specific value
    /// representation.
    ///
    /// # Errors
    /// Decoding of the element's value based on the given VR may fail.
    pub fn parse_value_as(&self, vr: VRRef) -> ParseResult<RawValue> {
        RawValue::try_from(&ElementWithVr(self, vr))
    }

    /// Encodes a `RawValue` into the binary data for this element.
    ///
    /// This will overwrite any existing value in this element's `self.data`.
    ///
    /// # Errors
    /// Encoding string values may fail.
    pub fn encode_val(&mut self, value: RawValue) -> ParseResult<()> {
        self.encode_val_with_vl(value, None)
    }

    /// Encodes a `RawValue` into the binary data for this element.
    ///
    /// This will overwrite any existing value in this element's `self.data`.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to be encoded according to `self.vr`.
    /// * `vl` - The value length to use. If `None` then the value length will be computed and
    ///   `ValueLength::Explicit` will be assigned to `self.vl`. If `Some` then it will only be used
    ///   if `self.is_seq_like()` would return true, otherwise the value length will be computed and
    ///   `ValueLength::Explicit` will be assigned to `self.vl`. Unconditionally, `self.vl` will be
    ///   assigned `ValueLength::Explicit(0)` if this element is `Item`, `ItemDelimitationItem`, or
    ///   `SequenceDelimitationItem`.
    ///
    /// # Errors
    /// Encoding string values may fail.
    pub fn encode_val_with_vl(
        &mut self,
        value: RawValue,
        vl: Option<ValueLength>,
    ) -> ParseResult<()> {
        self.data = ElemAndRawValue(self, value).try_into()?;

        let calc_vl = u32::try_from(self.data.len())
            .map(ValueLength::Explicit)
            .unwrap_or(ValueLength::UndefinedLength);

        self.vl = if self.tag == ITEM_DELIMITATION_ITEM || self.tag == SEQUENCE_DELIMITATION_ITEM {
            ValueLength::Explicit(0)
        } else if let Some(vl) = vl {
            // If the element is sequence-like then we have to trust the given VL because this
            // element's value won't contain any actual data.
            if self.is_sq_like() || self.tag == ITEM {
                vl
            } else {
                calc_vl
            }
        } else {
            calc_vl
        };

        Ok(())
    }
}
