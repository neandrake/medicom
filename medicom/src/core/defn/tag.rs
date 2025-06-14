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

//! DICOM Data Element (Tag)

use std::{
    fmt::{Debug, Display, Formatter},
    hash::{Hash, Hasher},
};

use crate::core::{
    dcmsqelem::SequenceElement,
    defn::{
        constants::tags::{ITEM, ITEM_DELIMITATION_ITEM, SEQUENCE_DELIMITATION_ITEM},
        dcmdict::DicomDictionary,
        vm::VMRef,
        vr::VRRef,
    },
    read::ParseError,
};

pub type TagRef = &'static Tag;

/// DICOM Data Element (Tag) Definition
#[derive(Debug, Eq)]
pub struct Tag {
    /// Identifier or name which can be used with a `DicomDictionary`.
    ident: &'static str,

    /// The tag number.
    num: u32,

    /// The default value representation which should be used to read this tag when parsing
    /// `ImplicitVR` transfer syntaxes. Some tags may support multiple possible implicit VRs,
    /// however this is not currently supported.
    implicit_vr: Option<VRRef>,

    /// The value multiplicity indicating how many values might be encoded in the element.
    vm: VMRef,

    /// A longer name or description of the tag.
    desc: &'static str,
}

impl Tag {
    /// Create a new tag.
    #[must_use]
    pub const fn new(
        ident: &'static str,
        tag: u32,
        implicit_vr: Option<VRRef>,
        vm: VMRef,
        desc: &'static str,
    ) -> Self {
        Self {
            ident,
            num: tag,
            implicit_vr,
            vm,
            desc,
        }
    }

    /// Get the tag's identifier.
    #[must_use]
    pub fn ident(&self) -> &'static str {
        self.ident
    }

    /// Get the tag's number.
    #[must_use]
    pub fn num(&self) -> u32 {
        self.num
    }

    /// Get the tag's implicit value representation, if it has one.
    #[must_use]
    pub fn implicit_vr(&self) -> Option<VRRef> {
        self.implicit_vr
    }

    /// Get the tag's value multiplicity.
    #[must_use]
    pub fn vm(&self) -> VMRef {
        self.vm
    }

    /// Get the description of the tag.
    #[must_use]
    pub fn desc(&self) -> &'static str {
        self.desc
    }

    #[must_use]
    pub fn as_node(&self) -> TagNode {
        TagNode::from(self)
    }

    #[must_use]
    pub fn as_item_node(&self, index: usize) -> TagNode {
        TagNode::from((self, index))
    }

    /// Detects if the given tag is a private creator, which is defined to be an odd-numbered group
    /// number with an element number between 0x0010-0x00FF.
    pub fn is_private_creator<T>(tag: T) -> bool
    where
        u32: From<T>,
    {
        let tag: u32 = u32::from(tag);
        if Tag::is_private::<u32>(tag) {
            let tag_elem: u32 = tag & 0x0000_FFFF;
            (0x0010..=0x00FF).contains(&tag_elem)
        } else {
            false
        }
    }

    /// Detects if the given tag is a private group length. These tags are deprecated according to
    /// the dicom standard.
    pub fn is_private_group_length<T>(tag: T) -> bool
    where
        u32: From<T>,
    {
        let tag: u32 = u32::from(tag);
        if Tag::is_private::<u32>(tag) {
            Tag::is_group_length::<u32>(tag)
        } else {
            false
        }
    }

    /// Detects if the given tag is a group length tag, defined to have an element value of 0.
    pub fn is_group_length<T>(tag: T) -> bool
    where
        u32: From<T>,
    {
        let tag: u32 = u32::from(tag);
        tag.trailing_zeros() >= 16
    }

    /// Detects if the given tag is a private tag. This is only a basic/rudimentary check and is
    /// not based on previously registered private creators.
    pub fn is_private<T>(tag: T) -> bool
    where
        u32: From<T>,
    {
        let tag_group: u32 = u32::from(tag) >> 16;
        // See Part 5, Section 7.1:
        // Private Data Elements have an odd Group Number that is not (0001,eeee), (0003,eeee),
        // (0005,eeee), (0007,eeee), or (FFFF,eeee).
        tag_group > 0x0008 && tag_group != 0xFFFF && tag_group % 2 == 1
    }

    /// Renders the tag number as `(GGGG,EEEE)`.
    pub fn format_tag_to_display<T>(tag: T) -> String
    where
        u32: From<T>,
    {
        let tag: u32 = u32::from(tag);
        let tag_group: u32 = tag >> 16;
        let tag_elem: u32 = tag & 0x0000_FFFF;
        format!("({tag_group:04X},{tag_elem:04X})")
    }

    /// Renders the tag number as `GGGGEEEE`.
    pub fn format_tag_to_path_display<T>(tag: T) -> String
    where
        u32: From<T>,
    {
        let tag: u32 = u32::from(tag);
        let tag_group: u32 = tag >> 16;
        let tag_elem: u32 = tag & 0x0000_FFFF;
        format!("({tag_group:04X},{tag_elem:04X})")
    }
}

/// `PartialEq` to allow for ordering of tags based on their number.
impl PartialEq for Tag {
    fn eq(&self, other: &Tag) -> bool {
        self.num.eq(&other.num)
    }
}

/// `Hash` to allow for hashing based on the tag number.
impl Hash for Tag {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.num.hash(state);
    }
}

/// Access the numeric tag, to allow functions to take `Into<u32>` rather than plain u32.
impl From<&Tag> for u32 {
    fn from(value: &Tag) -> Self {
        value.num
    }
}

/// A `TagNode` represents a single entry/node within a `TagPath`. All leaf nodes have an
/// associated tag number. Non-leaf nodes are sequence tags, and will also contain an `item`
/// specifying which 1-based item child node is being referenced in the path.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TagNode {
    tagnum: u32,
    item: Option<usize>,
}

impl TagNode {
    /// Create a new tag node.
    #[must_use]
    pub fn new<T>(tag: T, item: Option<usize>) -> TagNode
    where
        u32: From<T>,
    {
        TagNode {
            tagnum: u32::from(tag),
            item,
        }
    }

    /// Get the tag number for this node.
    #[must_use]
    pub fn tagnum(&self) -> u32 {
        self.tagnum
    }

    /// Get the 1-based item number this node references if this is a non-leaf node.
    #[must_use]
    pub fn item(&self) -> Option<usize> {
        self.item
    }

    /// Get the modifiable 1-based item number this node references if this is a non-leaf node.
    pub fn item_mut(&mut self) -> &mut Option<usize> {
        &mut self.item
    }

    /// Parses a `TagNode` from the given string. The tag can be resolved by name if a dictionary
    /// is supplied, or by standard hexformat (parens are optional). For a `TagNode` which is in a
    /// sequence path, an index can be supplied which must be at the end and contained within
    /// square brackets. Supplying a dictionary is optional however must be supplied in order to
    /// resolve tags by name. Tag names are case-insensitive.
    ///
    /// The acceptable formats are:
    /// ```text
    /// "PatientID" => (0x0010_0020, None)
    /// "(0010,0020)" => (0x0010_0020, None)
    /// "0010,0020" => (0x0010_0020, None)
    /// "0010_0020" => (0x0010_0020, None)
    /// "00100020" => (0x0010_0020, None)
    /// "100020" => (0x0010_0020, None)
    /// "ReferencedFrameOfReferenceSequence[1]" => (0x3006_0010, Some(1))
    /// "(3006,0010)[1]" => (0x3006_0010, Some(1))
    /// ```
    ///
    /// # Errors
    /// `ParseError::InvalidTagPath` if the string is not a valid tag node representation.
    pub fn parse(value: &str, dict: Option<&dyn DicomDictionary>) -> Result<Self, ParseError> {
        let mut index = None;
        let mut tag_id = value.trim();
        if let Some((name_part, index_part)) = tag_id.rsplit_once('[') {
            if let Some((index_part, _bracket)) = index_part.rsplit_once(']') {
                if let Ok(parsed) = str::parse::<usize>(index_part) {
                    index = Some(parsed);
                }
            }
            // Reassign the tag_id to remove the indexing component.
            tag_id = name_part;
        }

        // Look up by name.
        let lookup = dict.and_then(|d| d.get_tag_by_name(tag_id));
        if let Some(tag) = lookup {
            return Ok(TagNode::new(tag.num, index));
        }

        // Remove optional surrounding parens and optional group/elem splitter.
        let tag_id = tag_id.replace(['(', ')', ',', '_'], "");
        let full_tag: u32 =
            u32::from_str_radix(&tag_id, 16).map_err(|e| ParseError::InvalidTagPath {
                string_path: tag_id.clone(),
                details: e.to_string(),
            })?;
        Ok(TagNode::new(full_tag, index))
    }
}

impl Debug for TagNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.item {
            None => write!(f, "{}", Tag::format_tag_to_display(self.tagnum)),
            Some(item) => write!(f, "{}[{}]", Tag::format_tag_to_display(self.tagnum), item),
        }
    }
}

impl From<u32> for TagNode {
    fn from(tag: u32) -> Self {
        TagNode {
            tagnum: tag,
            item: None,
        }
    }
}

impl From<&Tag> for TagNode {
    fn from(tag: &Tag) -> Self {
        TagNode {
            tagnum: u32::from(tag),
            item: None,
        }
    }
}

impl From<&SequenceElement> for TagNode {
    fn from(element: &SequenceElement) -> Self {
        TagNode {
            tagnum: element.sq_tag(),
            item: element.item(),
        }
    }
}

impl<T> From<(T, usize)> for TagNode
where
    u32: From<T>,
{
    fn from(tuple: (T, usize)) -> Self {
        TagNode {
            tagnum: u32::from(tuple.0),
            item: Some(tuple.1),
        }
    }
}

/// A `TagPath` is an ordered collection of `TagNode`s. The path specifies a unique traversal into
/// a DICOM dataset referencing a single DICOM tag/element. Example:
///
/// ```text
/// ReferencedFrameOfReferenceSequence[1]
///   RTReferencedStudySequence[1]
///     RTReferencedSeriesSequence[1]
///       ContourImageSequence[11]
///         ReferencedSOPInstanceUID
/// ```
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct TagPath {
    nodes: Vec<TagNode>,
}

impl TagPath {
    /// Creates a tag path with no nodes.
    #[must_use]
    pub fn empty() -> TagPath {
        TagPath {
            nodes: Vec::with_capacity(0),
        }
    }

    /// The nodes that make up this `TagPath`.
    #[must_use]
    pub fn nodes(&self) -> &Vec<TagNode> {
        &self.nodes
    }

    /// Mutable access to the nodes that make up this `TagPath`.
    pub fn nodes_mut(&mut self) -> &mut Vec<TagNode> {
        &mut self.nodes
    }

    /// Return whether there are any nodes in this tag path.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Formats the tag path as readable text, optionally using the tag's display name where
    /// possible, otherwise tags will be displayed as `(gggg,eeee)`.
    #[must_use]
    pub fn format_tagpath_to_display(
        tagpath: &TagPath,
        dict: Option<&dyn DicomDictionary>,
    ) -> String {
        tagpath
            .nodes
            .iter()
            // Filter out tags related to items & delimiters as they are markers which are already
            // contextually conveyed by the item number indicators.
            .filter(|node| {
                node.tagnum != ITEM
                    && node.tagnum != ITEM_DELIMITATION_ITEM
                    && node.tagnum != SEQUENCE_DELIMITATION_ITEM
            })
            .map(|node| {
                let tag: String = dict
                    .and_then(|d| d.get_tag_by_number(node.tagnum))
                    .map_or_else(
                        || Tag::format_tag_to_display(node.tagnum),
                        |t| t.ident.to_string(),
                    );
                match node.item {
                    None => tag,
                    Some(item_num) => format!("{tag}[{item_num}]"),
                }
            })
            .collect::<Vec<String>>()
            .join(".")
    }

    /// Parses `TagNodes` from the given string and converts to a `TagPath`. The `TagNode`s must be
    /// separated by the period character `.`. The format of `TagNode` is described in
    /// `TagNode::from_str()`.
    ///
    /// Example:
    /// ```text
    /// "ReferencedFrameOfReferenceSequence
    ///   .RTReferencedStudySequence
    ///   .RTReferencedSeriesSequence
    ///   .ContourImageSequence[11]
    ///   .ReferencedSOPInstanceUID"
    /// ```
    /// will parse as:
    /// ```text
    /// [(0x3006_0010, Some(1)),
    ///  (0x3006_0012, Some(1)),
    ///  (0x3006_0014, Some(1)),
    ///  (0x3006_0016, Some(11)),
    ///  (0x0004_1504, None)]
    /// ```
    ///
    /// # Errors
    /// `ParseError::InvalidTagPath` if the string does not represent a valid tag path.
    pub fn parse(value: &str, dict: Option<&dyn DicomDictionary>) -> Result<TagPath, ParseError> {
        let tags = value.split('.').collect::<Vec<&str>>();
        let mut nodes: Vec<TagNode> = Vec::with_capacity(tags.len());
        for (i, tag) in tags.iter().enumerate() {
            let mut node = TagNode::parse(tag, dict)?;
            // Assume item #1 for all but the last TagNode if no item is supplied.
            if i < tags.len() - 1 {
                node.item_mut().get_or_insert(1);
            }
            nodes.push(node);
        }
        Ok(TagPath::from(nodes))
    }
}

impl Display for TagPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &TagPath::format_tagpath_to_display(self, None))
    }
}

impl<T> From<T> for TagPath
where
    TagNode: From<T>,
{
    fn from(value: T) -> Self {
        TagPath {
            nodes: vec![TagNode::from(value)],
        }
    }
}

impl<T> From<Vec<T>> for TagPath
where
    TagNode: From<T>,
{
    fn from(value: Vec<T>) -> Self {
        let len = value.len();
        let mut nodes: Vec<TagNode> = Vec::with_capacity(len);
        for (i, t) in value.into_iter().enumerate() {
            let tag_node = TagNode::from(t);
            // For convenience, when making a tag path from a list of things that can be converted
            // into TagNode, assume that many of those things got the default into implementation
            // which defaults with an index of `None`, however unless it's the last node it should
            // not have an index of `None`. Assume `Some(1)` is the appropriate item index.
            let item = if i < len - 1 {
                tag_node.item().or(Some(1))
            } else {
                tag_node.item()
            };
            nodes.push(TagNode {
                tagnum: tag_node.tagnum(),
                item,
            });
        }
        TagPath { nodes }
    }
}

impl From<&[u32]> for TagPath {
    fn from(tags: &[u32]) -> Self {
        let len = tags.len();
        let mut nodes: Vec<TagNode> = Vec::with_capacity(len);
        for (i, tag) in tags.iter().enumerate() {
            let item = if i == len - 1 { None } else { Some(1) };
            nodes.push(TagNode::new(*tag, item));
        }
        TagPath { nodes }
    }
}

impl From<&[&Tag]> for TagPath {
    fn from(tags: &[&Tag]) -> Self {
        let len = tags.len();
        let mut nodes: Vec<TagNode> = Vec::with_capacity(len);
        for (i, tag) in tags.iter().enumerate() {
            let item = if i == len - 1 { None } else { Some(1) };
            nodes.push(TagNode::new(tag.num, item));
        }
        TagPath { nodes }
    }
}

impl From<&[&SequenceElement]> for TagPath {
    fn from(elements: &[&SequenceElement]) -> Self {
        elements
            .iter()
            .map(|sq_el| TagNode::from(*sq_el))
            .collect::<Vec<TagNode>>()
            .into()
    }
}

impl From<&[SequenceElement]> for TagPath {
    fn from(elements: &[SequenceElement]) -> Self {
        elements
            .iter()
            .map(TagNode::from)
            .collect::<Vec<TagNode>>()
            .into()
    }
}

#[cfg(test)]
#[cfg(feature = "stddicom")]
mod tests {
    use crate::{
        core::defn::tag::{TagNode, TagPath},
        dict::{
            stdlookup::STANDARD_DICOM_DICTIONARY,
            tags::{
                ContourImageSequence, PatientID, RTReferencedSeriesSequence,
                RTReferencedStudySequence, ReferencedFrameofReferenceSequence,
                ReferencedSOPInstanceUID,
            },
        },
    };

    #[test]
    fn test_parse_tagnode() {
        let pid = TagNode::new(&PatientID, None);
        let rfr = TagNode::new(&ReferencedFrameofReferenceSequence, Some(5));

        let val = TagNode::parse("00100020", None).expect("parse");
        assert_eq!(pid, val);

        let val = TagNode::parse("100020", None).expect("parse");
        assert_eq!(pid, val);

        let val = TagNode::parse("0010,0020", None).expect("parse");
        assert_eq!(pid, val);

        let val = TagNode::parse("(0010,0020)", None).expect("parse");
        assert_eq!(pid, val);

        let val = TagNode::parse("(0010_0020)", None).expect("parse");
        assert_eq!(pid, val);

        let val = TagNode::parse("PatientID", None).ok();
        assert_eq!(None, val);

        let val = TagNode::parse("PatientID", Some(&STANDARD_DICOM_DICTIONARY)).expect("parse");
        assert_eq!(pid, val);

        let val = TagNode::parse("PatientId", Some(&STANDARD_DICOM_DICTIONARY)).expect("parse");
        assert_eq!(pid, val);

        let val = TagNode::parse("3006,0010[5]", None).expect("parse");
        assert_eq!(rfr, val);

        let val = TagNode::parse(
            "ReferencedFrameOfReferenceSequence[5]",
            Some(&STANDARD_DICOM_DICTIONARY),
        )
        .expect("parse");
        assert_eq!(rfr, val);
    }

    #[test]
    fn test_parse_tagpath() {
        let tagpath = TagPath::from(vec![
            TagNode::from(&ReferencedFrameofReferenceSequence),
            TagNode::from(&RTReferencedStudySequence),
            TagNode::from(&RTReferencedSeriesSequence),
            TagNode::from((&ContourImageSequence, 11)),
            TagNode::from(&ReferencedSOPInstanceUID),
        ]);

        let parsed = TagPath::parse(
            "ReferencedFrameOfReferenceSequence
            .RTReferencedStudySequence
            .RTReferencedSeriesSequence
            .ContourImageSequence[11]
            .ReferencedSOPInstanceUID",
            Some(&STANDARD_DICOM_DICTIONARY),
        )
        .expect("parse");

        assert_eq!(tagpath, parsed);
    }
}
