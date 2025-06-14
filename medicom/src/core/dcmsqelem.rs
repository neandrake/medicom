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

use std::fmt::{Debug, Display, Formatter};

use crate::core::{
    charset::CSRef,
    defn::{
        constants::tags::{DOUBLE_PIXEL_DATA, FLOAT_PIXEL_DATA, PIXEL_DATA},
        tag::TagNode,
        vl::ValueLength,
        vr::VRRef,
    },
};

/// Represents the sequence/item position of an element.
/// For elements to track which sequence they are a part of. When an SQ element is parsed the parser
/// adds a new `SequenceElement` to its current path which subsequent elements will clone for
/// themselves. This allows elements to know how they exist within a dicom object.
#[derive(Clone)]
pub struct SequenceElement {
    /// See Part 5 Section 7.5
    /// Items present in an SQ Data Element shall be an ordered set where each Item may be
    /// referenced by its ordinal position. Each Item shall be implicitly assigned an ordinal
    /// position starting with the value 1 for the first Item in the Sequence, and incremented by 1
    /// with each subsequent Item. The last Item in the Sequence shall have an ordinal position
    /// equal to the number of Items in the Sequence.
    ///
    /// This is initialized/incremented whenever an Item tag is parsed. Sequences are not required
    /// to have their contents encoded within items so this cannot be used as an index into a
    /// sequence's total listing of top-level children.
    node: TagNode,

    /// The byte position where the parent sequence ends. This value is set as
    /// `bytes_read + value_length` during parsing. If the sequence has undefined length this is set
    /// to None.
    sq_end_pos: Option<u64>,

    /// The VR of the sequence/item. This will typically be SQ for sequence element and UN for Item
    /// elements. For private sequence elements this should be UN.
    vr: VRRef,

    /// The Value Length of the sequence/item.
    vl: ValueLength,

    /// See Part 5 Section 7.5.3
    /// If an encapsulated Data Set includes the Specific Character Set Attribute, it shall apply
    /// only to the encapsulated Data Set. If the Attribute Specific Character Set is not explicitly
    /// included in an encapsulated Data Set, then the Specific Character Set value of the
    /// encapsulating Data Set applies.
    cs: CSRef,
}

impl SequenceElement {
    #[must_use]
    pub fn new(
        seq_tag: u32,
        seq_end_pos: Option<u64>,
        vr: VRRef,
        vl: ValueLength,
        cs: CSRef,
    ) -> SequenceElement {
        SequenceElement {
            node: TagNode::new(seq_tag, None),
            sq_end_pos: seq_end_pos,
            vr,
            vl,
            cs,
        }
    }

    #[must_use]
    pub fn node(&self) -> &TagNode {
        &self.node
    }

    #[must_use]
    pub fn sq_tag(&self) -> u32 {
        self.node.tagnum()
    }

    #[must_use]
    pub fn item(&self) -> Option<usize> {
        self.node.item()
    }

    #[must_use]
    pub fn sq_end_pos(&self) -> Option<u64> {
        self.sq_end_pos
    }

    #[must_use]
    pub fn vr(&self) -> VRRef {
        self.vr
    }

    #[must_use]
    pub fn vl(&self) -> ValueLength {
        self.vl
    }

    #[must_use]
    pub fn cs(&self) -> CSRef {
        self.cs
    }

    pub fn set_cs(&mut self, cs: CSRef) {
        self.cs = cs;
    }

    /// Returns if this element is for `PixelData`, `FloatPixelData`, or `DoubleFloatPixelData`.
    #[must_use]
    pub fn is_pixel_data(&self) -> bool {
        self.sq_tag() == PIXEL_DATA
            || self.sq_tag() == FLOAT_PIXEL_DATA
            || self.sq_tag() == DOUBLE_PIXEL_DATA
    }

    pub fn increment_item(&mut self) {
        match self.node.item() {
            None => {
                // Item numbers start at 1.
                self.node.item_mut().replace(1);
            }
            Some(val) => {
                self.node.item_mut().replace(val + 1);
            }
        }
    }

    pub fn decrement_item(&mut self) {
        match self.node.item() {
            None => {}
            Some(val) if val > 1 => {
                self.node.item_mut().replace(val - 1);
            }
            Some(_) => {
                self.node.item_mut().take();
            }
        }
    }
}

impl Display for SequenceElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.node)
    }
}

impl Debug for SequenceElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "node: {:?}, end: {:?}", self.node, self.sq_end_pos)
    }
}
