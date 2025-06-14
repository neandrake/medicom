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

//! Utilities for inspecting or debugging DICOM.

use std::fmt;

use crate::core::{
    dcmelement::DicomElement,
    dcmsqelem::SequenceElement,
    defn::{
        constants::tags::{ITEM, ITEM_DELIMITATION_ITEM, SEQUENCE_DELIMITATION_ITEM},
        vl::ValueLength,
        vr::{INVALID_VR, UN},
    },
    defn::{
        dcmdict::{DicomDictionary, MultiDicomDictionary},
        tag::Tag,
    },
    values::RawValue,
};

#[cfg(feature = "stddicom")]
use crate::dict::stdlookup::STANDARD_DICOM_DICTIONARY;

/// Convenience for coordinating a tag's type display.
pub enum FormattedTagType {
    Known(u32, String),
    PrivateCreator(u32),
    PrivateSequence(u32),
    PrivateGroupLength(u32),
    Private(u32),
    GroupLength(u32),
    Unknown(u32),
}

impl fmt::Display for FormattedTagType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FormattedTagType::Known(_, display) => write!(f, "{display}"),
            FormattedTagType::PrivateCreator(_) => write!(f, "<PrivateCreator>"),
            FormattedTagType::PrivateSequence(_) => write!(f, "<PrivateSequence>"),
            FormattedTagType::PrivateGroupLength(_) => write!(f, "<PrivateGroupLength>"),
            FormattedTagType::Private(_) => write!(f, "<PrivateTag>"),
            FormattedTagType::GroupLength(_) => write!(f, "<GroupLength>"),
            FormattedTagType::Unknown(_) => write!(f, "<UnknownTag>"),
        }
    }
}

/// Convenience for coordinating a tag's display.
pub enum FormattedTagValue {
    Sequence,
    Error(String),
    Uid(String, String),
    Stringified(String),
}

impl fmt::Display for FormattedTagValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FormattedTagValue::Sequence => Ok(()),
            FormattedTagValue::Error(error_str) => write!(f, "<Error {error_str}>"),
            FormattedTagValue::Uid(uid, name) => {
                if name.is_empty() {
                    write!(f, "{uid}")
                } else {
                    write!(f, "{uid} => {name}")
                }
            }
            FormattedTagValue::Stringified(value) => write!(f, "{value}"),
        }
    }
}

/// Convenience for coordinating an element's display with formatting options.
pub struct FormattedElement<'e> {
    /// The element to render.
    elem: &'e DicomElement,
    /// Whether the value can render across a single line or multiple lines.
    multiline: bool,
    /// The maximum number of value items to display.
    max_items: usize,
    /// Whether sequence and item delimiters should be rendered or not.
    hide_delims: bool,
    /// Whether group length elements should be rendered or not.
    hide_groups: bool,
    /// Dictionary to resolve VRs, tag names, UID names, etc.
    dict: MultiDicomDictionary<'e>,
}

impl<'e> FormattedElement<'e> {
    #[must_use]
    pub fn new(elem: &'e DicomElement) -> Self {
        #[cfg(not(feature = "stddicom"))]
        let dict = MultiDicomDictionary::new(Vec::with_capacity(0));
        #[cfg(feature = "stddicom")]
        let dict = MultiDicomDictionary::new(vec![&STANDARD_DICOM_DICTIONARY]);

        Self {
            elem,
            multiline: false,
            max_items: 16,
            hide_delims: false,
            hide_groups: false,
            dict,
        }
    }

    #[must_use]
    pub fn with_multiline(mut self, multiline: bool) -> Self {
        self.multiline = multiline;
        self
    }

    #[must_use]
    pub fn with_max_items(mut self, max_items: usize) -> Self {
        self.max_items = max_items;
        self
    }

    #[must_use]
    pub fn with_hide_delims(mut self, hide_delims: bool) -> Self {
        self.hide_delims = hide_delims;
        self
    }

    #[must_use]
    pub fn with_hide_groups(mut self, hide_groups: bool) -> Self {
        self.hide_groups = hide_groups;
        self
    }

    #[must_use]
    pub fn with_dict(mut self, dict: MultiDicomDictionary<'e>) -> Self {
        self.dict = dict;
        self
    }

    #[must_use]
    pub fn elem(&self) -> &'e DicomElement {
        self.elem
    }

    #[must_use]
    pub fn multiline(&self) -> bool {
        self.multiline
    }

    #[must_use]
    pub fn max_items(&self) -> usize {
        self.max_items
    }

    #[must_use]
    pub fn hide_delims(&self) -> bool {
        self.hide_delims
    }

    #[must_use]
    pub fn hide_groups(&self) -> bool {
        self.hide_groups
    }

    #[must_use]
    pub fn dict(&self) -> &MultiDicomDictionary<'e> {
        &self.dict
    }

    #[must_use]
    pub fn should_omit(&self) -> bool {
        (self.hide_delims
            && (self.elem.tag() == ITEM_DELIMITATION_ITEM
                || self.elem.tag() == SEQUENCE_DELIMITATION_ITEM))
            || (self.hide_groups && Tag::is_group_length(self.elem.tag()))
    }

    /// Formats `vec` converting each element to a String based on the given `func`.
    /// Returns true if the input `vec` had more items than rendered, based on `MAX_ITEMS_DISPLAYED`.
    fn format_vec_to_strings<T, F: Fn(&T) -> String>(
        &self,
        vec: &[T],
        func: F,
    ) -> (bool, Vec<String>) {
        let vec_len: usize = vec.len();
        let formatted: Vec<String> = vec
            .iter()
            .take(self.max_items)
            .map(func)
            .collect::<Vec<String>>();
        (formatted.len() < vec_len, formatted)
    }

    #[must_use]
    pub fn get_tag_type(&self) -> FormattedTagType {
        if Tag::is_private_creator(self.elem.tag()) {
            FormattedTagType::PrivateCreator(self.elem.tag())
        } else if Tag::is_private(self.elem.tag()) && self.elem.is_sq_like() {
            FormattedTagType::PrivateSequence(self.elem.tag())
        } else if Tag::is_private_group_length(self.elem.tag()) {
            FormattedTagType::PrivateGroupLength(self.elem.tag())
        } else if Tag::is_private(self.elem.tag()) {
            FormattedTagType::Private(self.elem.tag())
        } else if Tag::is_group_length(self.elem.tag()) {
            FormattedTagType::GroupLength(self.elem.tag())
        } else {
            if let Some(tag) = self.dict.get_tag_by_number(self.elem.tag()) {
                return FormattedTagType::Known(tag.num(), tag.ident().to_string());
            }

            FormattedTagType::Unknown(self.elem.tag())
        }
    }

    #[must_use]
    pub fn get_tag_value(&self) -> FormattedTagValue {
        if self.elem.is_sq_like() {
            return FormattedTagValue::Sequence;
        }

        let mut sep = if self.multiline { " " } else { "\\" };

        let mut vr = self.elem.vr();
        if !self.elem.ts().explicit_vr() || vr == &UN {
            vr = self
                .dict
                .get_tag_by_number(self.elem.tag())
                .and_then(Tag::implicit_vr)
                .unwrap_or(vr);
        }

        let elem_value = match self.elem.parse_value_as(vr) {
            Err(e) => return FormattedTagValue::Error(e.to_string()),
            Ok(val) => val,
        };

        let (add_ellipses, mut str_vals) = match elem_value {
            RawValue::Attributes(attrs) => {
                self.format_vec_to_strings(&attrs, |attr| Tag::format_tag_to_display(attr.0))
            }
            RawValue::Uid(uid_str) => {
                let uid_lookup = self.dict.get_uid_by_uid(&uid_str);
                match uid_lookup {
                    Some(found_uid) => {
                        let uid_name =
                            if let Some((name, _detail)) = found_uid.name().split_once(':') {
                                name
                            } else {
                                found_uid.name()
                            };
                        return FormattedTagValue::Uid(uid_str.to_string(), uid_name.to_string());
                    }
                    None => return FormattedTagValue::Uid(uid_str.to_string(), String::new()),
                }
            }
            RawValue::Strings(strings) => {
                if self.multiline {
                    sep = "\n";
                }
                self.format_vec_to_strings(&strings, |val: &String| {
                    if self.multiline {
                        val.to_owned()
                    } else {
                        val.replace("\r\n", " / ").replace('\n', " / ")
                    }
                })
            }
            RawValue::Shorts(shorts) => {
                self.format_vec_to_strings(&shorts, |val: &i16| format!("{val}"))
            }
            RawValue::UShorts(ushorts) => {
                self.format_vec_to_strings(&ushorts, |val: &u16| format!("{val}"))
            }
            RawValue::Ints(ints) => self.format_vec_to_strings(&ints, |val: &i32| format!("{val}")),
            RawValue::UInts(uints) => {
                self.format_vec_to_strings(&uints, |val: &u32| format!("{val}"))
            }
            RawValue::Longs(longs) => {
                self.format_vec_to_strings(&longs, |val: &i64| format!("{val}"))
            }
            RawValue::ULongs(ulongs) => {
                self.format_vec_to_strings(&ulongs, |val: &u64| format!("{val}"))
            }
            RawValue::Floats(floats) => {
                self.format_vec_to_strings(&floats, |val: &f32| format!("{val:.2}"))
            }
            RawValue::Doubles(doubles) => {
                self.format_vec_to_strings(&doubles, |val: &f64| format!("{val:.2}"))
            }
            RawValue::Bytes(bytes) => {
                self.format_vec_to_strings(&bytes, |val: &u8| format!("{val:02x}"))
            }
            RawValue::Words(words) => {
                self.format_vec_to_strings(&words, |val: &u16| format!("{val:04x}"))
            }
            RawValue::DWords(dwords) => {
                self.format_vec_to_strings(&dwords, |val: &u32| format!("{val:06x}"))
            }
            RawValue::QWords(qwords) => {
                self.format_vec_to_strings(&qwords, |val: &u64| format!("{val:08x}"))
            }

            RawValue::BytesView(bytes) => {
                self.format_vec_to_strings(bytes, |val: &u8| format!("{val:02x}"))
            }
        };

        if add_ellipses {
            str_vals.push("..".to_string());
        }

        let num_vals: usize = str_vals.len();
        if num_vals == 1 {
            return FormattedTagValue::Stringified(str_vals.remove(0));
        }

        FormattedTagValue::Stringified(str_vals.join(sep).to_string())
    }
}

impl fmt::Display for FormattedElement<'_> {
    /// Renders an element on a single line, includes indentation based on depth in sequences. If the
    /// standard dicom dictionary feature is enabled then the tag name will attempt to be resolved
    /// using it.
    ///
    /// ```text
    /// (gggg,eeee) VR TagName [VL] | TagValue
    /// ```
    /// or
    /// ```text
    /// (gggg,eeee) VR TagName [0] <empty>
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Group Length tags are deprecated, see note on Part 5 Section 7.2
        if self.hide_groups && self.elem.tag().trailing_zeros() >= 16 {
            return Ok(());
        }

        // These are delimiter items that are not very useful to see
        if self.hide_delims
            && (self.elem.tag() == ITEM_DELIMITATION_ITEM
                || self.elem.tag() == SEQUENCE_DELIMITATION_ITEM)
        {
            return Ok(());
        }

        // Some (malformed?) datasets have a bunch of zeroes between self.elem..
        if self.elem.tag() == 0
            && ((self.elem.ts().explicit_vr() && self.elem.vr() == &INVALID_VR)
                || (!self.elem.ts().explicit_vr() && self.elem.vr() == &UN))
            && self.elem.vl() == ValueLength::Explicit(0)
        {
            return Ok(());
        }

        let tag_num: String = Tag::format_tag_to_display(self.elem.tag());
        let tag_name: FormattedTagType = self.get_tag_type();

        let mut vr = self.elem.vr();
        if vr == &UN {
            vr = self
                .dict
                .get_tag_by_number(self.elem.tag())
                .and_then(Tag::implicit_vr)
                .unwrap_or(vr);
        }
        let vr: &str = vr.ident;

        let vl: String = match self.elem.vl() {
            ValueLength::Explicit(len) => {
                if len % 2 != 0 {
                    format!("[*{:?}]", self.elem.vl())
                } else {
                    format!("[{:?}]", self.elem.vl())
                }
            }
            ValueLength::UndefinedLength => "[u/l]".to_string(),
        };

        // Sequence path will nest tags under ITEM self.elem.. Double the indentation level for the
        // number of nested sequences (non-ITEM), and each ITEM self.elem.should be nested one level.
        // If the current self.elem.is a delimiter then reduce the associated indentation level.
        let seq_path: &Vec<SequenceElement> = self.elem.sq_path();
        let non_item_parents = seq_path
            .iter()
            .filter(|sq_el| sq_el.sq_tag() != ITEM)
            .count();
        let item_parents = seq_path
            .iter()
            .filter(|sq_el| sq_el.sq_tag() == ITEM)
            .count();
        let mut indent_width = non_item_parents * 2 + item_parents;
        if self.elem.tag() == ITEM_DELIMITATION_ITEM {
            indent_width -= 1;
        } else if self.elem.tag() == SEQUENCE_DELIMITATION_ITEM {
            indent_width -= 2;
        }

        if self.elem.tag() == ITEM {
            let item_desc: String = if let Some(last_seq_elem) = seq_path.last() {
                let item_no = last_seq_elem.item().map_or_else(
                    || "#[NO ITEM NUMBER]".to_string(),
                    |item_no: usize| format!("#{item_no}"),
                );
                format!("{item_no} {vr} {vl}")
            } else {
                String::new()
            };
            return write!(
                f,
                "{indentation:indent_width$}{tag_name} {item_desc}",
                indentation = "",
                indent_width = indent_width,
                tag_name = tag_name,
                item_desc = item_desc,
            );
        }

        let tag_value: FormattedTagValue = self.get_tag_value();
        let mut tag_value: String = tag_value.to_string();
        if !tag_value.is_empty() {
            if self.elem.is_empty() {
                tag_value = format!(" {tag_value}");
            } else {
                tag_value = format!(" | {tag_value}");
            }
        }

        write!(
            f,
            "{indentation:indent_width$}{tag_num} {vr} {tag_name} {vl}{tag_value}",
            indentation = "",
            indent_width = indent_width,
            tag_num = tag_num,
            vr = vr,
            tag_name = tag_name,
            vl = vl,
            tag_value = tag_value,
        )
    }
}
