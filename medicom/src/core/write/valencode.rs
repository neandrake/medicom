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

//! This module contains implementations for encoding values for a DICOM element's value field
//! bytes, based on the element's value representation and transfer syntax.

use std::{iter::once, mem::size_of};

use crate::core::{
    dcmelement::DicomElement,
    defn::vr::CS_SEPARATOR_BYTE,
    read::{ParseError, ParseResult},
    values::{Attribute, RawValue},
};

/// Encodes a `RawValue` into the binary data for the given element, based on the element's
/// currently set Value Representation, Character Set, and Transfer Syntax.
pub struct ElemAndRawValue<'e>(pub &'e DicomElement, pub RawValue<'e>);
impl<'e> TryFrom<ElemAndRawValue<'e>> for Vec<u8> {
    type Error = ParseError;

    fn try_from(value: ElemAndRawValue<'e>) -> Result<Self, Self::Error> {
        let elem = value.0;
        let value = value.1;

        let mut bytes: Vec<u8> = match value {
            RawValue::Attributes(attrs) => ElemAndAttributes(elem, attrs).into(),
            RawValue::Uid(uid) => ElemAndStrings(elem, vec![uid]).try_into()?,
            RawValue::Strings(strings) => ElemAndStrings(elem, strings).try_into()?,
            RawValue::Shorts(shorts) => ElemAndShorts(elem, shorts).into(),
            RawValue::UShorts(ushorts) => ElemAndUnsignedShorts(elem, ushorts).into(),
            RawValue::Ints(ints) => ElemAndIntegers(elem, ints).into(),
            RawValue::UInts(uints) => ElemAndUnsignedIntegers(elem, uints).into(),
            RawValue::Longs(longs) => ElemAndLongs(elem, longs).into(),
            RawValue::ULongs(ulongs) => ElemAndUnsignedLongs(elem, ulongs).into(),
            RawValue::Floats(floats) => ElemAndFloats(elem, floats).into(),
            RawValue::Doubles(doubles) => ElemAndDoubles(elem, doubles).into(),
            RawValue::Bytes(bytes) => bytes,
            RawValue::Words(words) => ElemAndWords(elem, words).into(),
            RawValue::DWords(dwords) => ElemAndDoubleWords(elem, dwords).into(),
            RawValue::QWords(qwords) => ElemAndQuadWords(elem, qwords).into(),

            RawValue::BytesView(bytes) => bytes.clone(),
        };

        // All fields are required to be of even length, with padding added as necessary. Note
        // that the standard refers to values of "character string" however binary values are
        // expected to always result in even number of bytes.
        //
        // Part 5, Ch 6.4:
        // Each string Value in a multiple valued character string may be of even or odd length,
        // but the length of the entire Value Field (including "\" delimiters) shall be of even
        // length. If padding is required to make the Value Field of even length, a single padding
        // character shall be applied to the end of the Value Field (to the last Value), in which
        // case the length of the last Value may exceed the Length of Value by 1.
        if bytes.len() % 2 != 0 {
            bytes.push(elem.vr().padding);
        }

        Ok(bytes)
    }
}

struct ElemAndAttributes<'e>(&'e DicomElement, Vec<Attribute>);
impl<'e> From<ElemAndAttributes<'e>> for Vec<u8> {
    fn from(value: ElemAndAttributes<'e>) -> Self {
        const U32_SIZE: usize = size_of::<u32>();

        let elem = value.0;
        let attrs = value.1;

        let num_attrs = attrs.len();
        let mut bytes: Vec<u8> = vec![0u8; U32_SIZE * num_attrs];
        for (i, attr) in attrs.iter().enumerate() {
            let Attribute(attr) = attr;
            let group_number: u16 = u16::try_from((attr >> 16) & 0xFFFF).unwrap_or(u16::MAX);
            let elem_number: u16 = u16::try_from(attr & 0xFFFF).unwrap_or(u16::MAX);
            let idx = i * U32_SIZE;
            if elem.ts().big_endian() {
                bytes[idx..(idx + 2)].copy_from_slice(&group_number.to_be_bytes());
                bytes[(idx + 2)..(idx + 4)].copy_from_slice(&elem_number.to_be_bytes());
            } else {
                bytes[idx..(idx + 2)].copy_from_slice(&group_number.to_le_bytes());
                bytes[(idx + 2)..(idx + 4)].copy_from_slice(&elem_number.to_le_bytes());
            }
        }
        bytes
    }
}

struct ElemAndStrings<'e>(&'e DicomElement, Vec<String>);
impl<'e> TryFrom<ElemAndStrings<'e>> for Vec<u8> {
    type Error = ParseError;

    fn try_from(value: ElemAndStrings<'e>) -> Result<Self, Self::Error> {
        type MaybeBytes = Vec<ParseResult<Vec<u8>>>;

        let elem = value.0;
        let strings = value.1;

        let (values, errs): (MaybeBytes, MaybeBytes) = strings
            .iter()
            .map(|s| {
                elem.cs()
                    .encode(s)
                    // Add the separator after each encoded value. Below the last separator
                    // will be popped off.
                    .map(|mut v| {
                        v.push(CS_SEPARATOR_BYTE);
                        v
                    })
                    .map_err(|e| ParseError::CharsetError { source: e })
            })
            .partition(ParseResult::is_ok);

        if let Some(Err(e)) = errs.into_iter().next_back() {
            return Err(e);
        }

        // Flatten the bytes for all strings.
        let mut bytes: Vec<u8> = values
            .into_iter()
            .flat_map(ParseResult::unwrap)
            .collect::<Vec<u8>>();
        // Remove last separator.
        bytes.pop();

        if bytes.len() % 2 != 0 {
            bytes.push(elem.vr().padding);
        }

        Ok(bytes)
    }
}

struct ElemAndShorts<'e>(&'e DicomElement, Vec<i16>);
impl<'e> From<ElemAndShorts<'e>> for Vec<u8> {
    fn from(value: ElemAndShorts<'e>) -> Self {
        let elem = value.0;
        let shorts = value.1;

        if elem.vr().is_character_string {
            // This should only be the case for a VR of IS.
            let mut encoded = shorts
                .into_iter()
                // In theory this should use the default character set, but this
                // relies on i16::to_string only using ascii which falls under that.
                .map(|short: i16| short.to_string().into_bytes())
                .flat_map(|v| v.into_iter().chain(once(CS_SEPARATOR_BYTE)))
                .collect::<Vec<u8>>();
            encoded.pop();
            encoded
        } else {
            // This should only be the case for a VR of SS
            shorts
                .into_iter()
                .flat_map(|short: i16| {
                    if elem.ts().big_endian() {
                        short.to_be_bytes()
                    } else {
                        short.to_le_bytes()
                    }
                })
                .collect::<Vec<u8>>()
        }
    }
}

struct ElemAndUnsignedShorts<'e>(&'e DicomElement, Vec<u16>);
impl<'e> From<ElemAndUnsignedShorts<'e>> for Vec<u8> {
    fn from(value: ElemAndUnsignedShorts<'e>) -> Self {
        let elem = value.0;
        let ushorts = value.1;

        if elem.vr().is_character_string {
            // This should only be the case for a VR of IS.
            let mut encoded = ushorts
                .into_iter()
                // In theory this should use the default character set, but this
                // relies on u16::to_string only using ascii which falls under that.
                .map(|ushort: u16| ushort.to_string().into_bytes())
                .flat_map(|v| v.into_iter().chain(once(CS_SEPARATOR_BYTE)))
                .collect::<Vec<u8>>();
            encoded.pop();
            encoded
        } else {
            // This should only be the case for a VR of US
            ushorts
                .into_iter()
                .flat_map(|ushort: u16| {
                    if elem.ts().big_endian() {
                        ushort.to_be_bytes()
                    } else {
                        ushort.to_le_bytes()
                    }
                })
                .collect::<Vec<u8>>()
        }
    }
}

struct ElemAndIntegers<'e>(&'e DicomElement, Vec<i32>);
impl<'e> From<ElemAndIntegers<'e>> for Vec<u8> {
    fn from(value: ElemAndIntegers<'e>) -> Self {
        let elem = value.0;
        let ints = value.1;

        if elem.vr().is_character_string {
            // This should only be the case for a VR of IS.
            let mut encoded = ints
                .into_iter()
                // In theory this should use the default character set, but this
                // relies on i32::to_string only using ascii which falls under that.
                .map(|int: i32| int.to_string().into_bytes())
                .flat_map(|v| v.into_iter().chain(once(CS_SEPARATOR_BYTE)))
                .collect::<Vec<u8>>();
            encoded.pop();
            encoded
        } else {
            // This should only be the case for a VR of SL.
            ints.into_iter()
                .flat_map(|int: i32| {
                    if elem.ts().big_endian() {
                        int.to_be_bytes()
                    } else {
                        int.to_le_bytes()
                    }
                })
                .collect::<Vec<u8>>()
        }
    }
}

struct ElemAndUnsignedIntegers<'e>(&'e DicomElement, Vec<u32>);
impl<'e> From<ElemAndUnsignedIntegers<'e>> for Vec<u8> {
    fn from(value: ElemAndUnsignedIntegers<'e>) -> Self {
        let elem = value.0;
        let uints = value.1;

        if elem.vr().is_character_string {
            // XXX: This shouldn't happen. Unsigned integers should only ever be encoded
            // as binary.
            let mut encoded = uints
                .into_iter()
                // In theory this should use the default character set, but this
                // relies on i16::to_string only using ascii which falls under that.
                .map(|uint: u32| uint.to_string().into_bytes())
                .flat_map(|v| v.into_iter().chain(once(CS_SEPARATOR_BYTE)))
                .collect::<Vec<u8>>();
            encoded.pop();
            encoded
        } else {
            // This should only be the case for a VR of UL.
            uints
                .into_iter()
                .flat_map(|uint: u32| {
                    if elem.ts().big_endian() {
                        uint.to_be_bytes()
                    } else {
                        uint.to_le_bytes()
                    }
                })
                .collect::<Vec<u8>>()
        }
    }
}

struct ElemAndLongs<'e>(&'e DicomElement, Vec<i64>);
impl<'e> From<ElemAndLongs<'e>> for Vec<u8> {
    fn from(value: ElemAndLongs<'e>) -> Self {
        let elem = value.0;
        let longs = value.1;

        if elem.vr().is_character_string {
            // This should only be the case for a VR of IS.
            let mut encoded = longs
                .into_iter()
                // In theory this should use the default character set, but this
                // relies on i32::to_string only using ascii which falls under that.
                .map(|long: i64| long.to_string().into_bytes())
                .flat_map(|v| v.into_iter().chain(once(CS_SEPARATOR_BYTE)))
                .collect::<Vec<u8>>();
            encoded.pop();
            encoded
        } else {
            // This should only be the case for a VR of SL.
            longs
                .into_iter()
                .flat_map(|long: i64| {
                    if elem.ts().big_endian() {
                        long.to_be_bytes()
                    } else {
                        long.to_le_bytes()
                    }
                })
                .collect::<Vec<u8>>()
        }
    }
}

struct ElemAndUnsignedLongs<'e>(&'e DicomElement, Vec<u64>);
impl<'e> From<ElemAndUnsignedLongs<'e>> for Vec<u8> {
    fn from(value: ElemAndUnsignedLongs<'e>) -> Self {
        let elem = value.0;
        let ulongs = value.1;

        if elem.vr().is_character_string {
            // XXX: This shouldn't happen. Unsigned integers should only ever be encoded
            // as binary.
            let mut encoded = ulongs
                .into_iter()
                // In theory this should use the default character set, but this
                // relies on i16::to_string only using ascii which falls under that.
                .map(|ulong: u64| ulong.to_string().into_bytes())
                .flat_map(|v| v.into_iter().chain(once(CS_SEPARATOR_BYTE)))
                .collect::<Vec<u8>>();
            encoded.pop();
            encoded
        } else {
            // This should only be the case for a VR of UL.
            ulongs
                .into_iter()
                .flat_map(|ulong: u64| {
                    if elem.ts().big_endian() {
                        ulong.to_be_bytes()
                    } else {
                        ulong.to_le_bytes()
                    }
                })
                .collect::<Vec<u8>>()
        }
    }
}

struct ElemAndFloats<'e>(&'e DicomElement, Vec<f32>);
impl<'e> From<ElemAndFloats<'e>> for Vec<u8> {
    fn from(value: ElemAndFloats<'e>) -> Self {
        let elem = value.0;
        let floats = value.1;

        if elem.vr().is_character_string {
            // This should only be the case for a VR of DS.
            let mut encoded = floats
                .into_iter()
                .filter(|float: &f32| float.is_finite())
                // In theory this should use the default character set, but this
                // relies on f32::to_string only using ascii which falls under that.
                .map(|float: f32| {
                    // Force at least one digit of precision.
                    if float.fract() == 0.0 {
                        format!("{float:.1}").into_bytes()
                    } else {
                        float.to_string().into_bytes()
                    }
                })
                .flat_map(|v| v.into_iter().chain(once(CS_SEPARATOR_BYTE)))
                .collect::<Vec<u8>>();
            encoded.pop();
            encoded
        } else {
            // This should only be the case for a VR of FL.
            floats
                .into_iter()
                .filter(|float: &f32| float.is_finite())
                .flat_map(|float: f32| {
                    if elem.ts().big_endian() {
                        float.to_be_bytes()
                    } else {
                        float.to_le_bytes()
                    }
                })
                .collect::<Vec<u8>>()
        }
    }
}

struct ElemAndDoubles<'e>(&'e DicomElement, Vec<f64>);
impl<'e> From<ElemAndDoubles<'e>> for Vec<u8> {
    fn from(value: ElemAndDoubles<'e>) -> Self {
        let elem = value.0;
        let doubles = value.1;

        if elem.vr().is_character_string {
            // This should only be the case for a VR of DS.
            let mut encoded = doubles
                .into_iter()
                .filter(|double: &f64| double.is_finite())
                // In theory this should use the default character set, but this
                // relies on f64::to_string only using ascii which falls under that.
                .map(|double: f64| {
                    // Force at least one digit of precision.
                    if double.fract() == 0.0 {
                        format!("{double:.1}").into_bytes()
                    } else {
                        double.to_string().into_bytes()
                    }
                })
                .flat_map(|v| v.into_iter().chain(once(CS_SEPARATOR_BYTE)))
                .collect::<Vec<u8>>();
            encoded.pop();
            encoded
        } else {
            // This should only be the case for a VR of FL.
            doubles
                .into_iter()
                .filter(|double: &f64| double.is_finite())
                .flat_map(|double: f64| {
                    if elem.ts().big_endian() {
                        double.to_be_bytes()
                    } else {
                        double.to_le_bytes()
                    }
                })
                .collect::<Vec<u8>>()
        }
    }
}

struct ElemAndWords<'e>(&'e DicomElement, Vec<u16>);
impl<'e> From<ElemAndWords<'e>> for Vec<u8> {
    fn from(value: ElemAndWords<'e>) -> Self {
        let elem = value.0;
        let words = value.1;

        words
            .into_iter()
            .flat_map(|word| {
                if elem.ts().big_endian() {
                    word.to_be_bytes()
                } else {
                    word.to_le_bytes()
                }
            })
            .collect::<Vec<u8>>()
    }
}

struct ElemAndDoubleWords<'e>(&'e DicomElement, Vec<u32>);
impl<'e> From<ElemAndDoubleWords<'e>> for Vec<u8> {
    fn from(value: ElemAndDoubleWords<'e>) -> Self {
        let elem = value.0;
        let dwords = value.1;

        dwords
            .into_iter()
            .flat_map(|dword| {
                if elem.ts().big_endian() {
                    dword.to_be_bytes()
                } else {
                    dword.to_le_bytes()
                }
            })
            .collect::<Vec<u8>>()
    }
}

struct ElemAndQuadWords<'e>(&'e DicomElement, Vec<u64>);
impl<'e> From<ElemAndQuadWords<'e>> for Vec<u8> {
    fn from(value: ElemAndQuadWords<'e>) -> Self {
        let elem = value.0;
        let qwords = value.1;

        qwords
            .into_iter()
            .flat_map(|qword| {
                if elem.ts().big_endian() {
                    qword.to_be_bytes()
                } else {
                    qword.to_le_bytes()
                }
            })
            .collect::<Vec<u8>>()
    }
}

#[cfg(test)]
#[cfg(feature = "stddicom")]
mod tests {
    use crate::{
        core::{dcmelement::DicomElement, defn::vr::UI, values::RawValue},
        dict::{tags::AffectedSOPClassUID, transfer_syntaxes::ExplicitVRLittleEndian},
    };

    #[test]
    fn test_odd_length_string() {
        let odd_len_uid = String::from("1.2.840.10008.1.1");
        assert!(odd_len_uid.len() % 2 == 1, "uid should be odd length");

        let mut elem = DicomElement::new_empty(&AffectedSOPClassUID, &UI, &ExplicitVRLittleEndian);
        elem.encode_val(crate::core::values::RawValue::of_uid(&odd_len_uid))
            .expect("encode odd-length uid");

        let data = elem.data();
        assert!(data.len() % 2 == 0, "encoded value should be even");
        assert_eq!(
            0,
            data[data.len() - 1],
            "encoded value should be padded with a null byte"
        );

        let parsed = elem.parse_value().expect("parse encoded value");
        let RawValue::Uid(parsed) = parsed else {
            panic!("resulting value was not a UID: {parsed:?}");
        };

        assert!(parsed.len() % 2 == 1, "parsed value should be odd length");
        assert_eq!(
            odd_len_uid, parsed,
            "parsed value should match the original"
        );
    }
}
