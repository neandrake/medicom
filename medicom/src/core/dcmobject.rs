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

//! DICOM Object, an in-memory tree-like structure representing a DICOM dataset.

use std::{
    collections::{btree_map, BTreeMap},
    fmt,
    io::{stdout, Error, Read, Write},
};

use crate::core::{
    charset::CSRef,
    dcmelement::DicomElement,
    defn::{
        constants::tags,
        tag::{Tag, TagNode, TagPath},
        ts::TSRef,
        vl::ValueLength,
        vr::{VRRef, UN},
    },
    inspect::FormattedElement,
    read::{ParseError, Parser},
    values::RawValue,
};

/// A root node of a DICOM dataset. This is the root object returned after parsing a dataset. It
/// does not contain a `DicomElement` itself but will have either children or items.
#[derive(Clone)]
pub struct DicomRoot {
    ts: TSRef,
    cs: CSRef,

    /// This is an object to be parent of all the root-level elements, but does not itself
    /// represent an element.
    sentinel: DicomObject,
}

impl DicomRoot {
    #[must_use]
    pub fn new(
        ts: TSRef,
        cs: CSRef,
        child_nodes: BTreeMap<u32, DicomObject>,
        items: Vec<DicomObject>,
    ) -> DicomRoot {
        let sentinel_elem = DicomElement::new_sentinel();
        let sentinel = DicomObject::new_with_children(sentinel_elem, child_nodes, items);
        DicomRoot { ts, cs, sentinel }
    }

    #[must_use]
    pub fn new_empty(ts: TSRef, cs: CSRef) -> DicomRoot {
        let sentinel_elem = DicomElement::new_sentinel();
        let sentinel =
            DicomObject::new_with_children(sentinel_elem, BTreeMap::new(), Vec::with_capacity(0));
        Self { ts, cs, sentinel }
    }

    /// Returns a delegate object that holds all the root-level elements. Be cautious not to use
    /// the `DicomObject::as_element()` on the returned value as it does not represent an actual
    /// element within the dataset and does not hold valid data.
    #[must_use]
    pub fn as_obj(&self) -> &DicomObject {
        &self.sentinel
    }

    /// Get the transfer syntax used to encode the dataset.
    #[must_use]
    pub fn ts(&self) -> TSRef {
        self.ts
    }

    /// Get the character set used to encode string values.
    #[must_use]
    pub fn cs(&self) -> CSRef {
        self.cs
    }

    /// Get the SOP Instance UID.
    ///
    /// # Errors
    /// - `ParseError::InvalidTagPath` if this dicom object does not contain the SOP Instance UID
    ///   element.
    pub fn sop_instance_id(&self) -> Result<String, ParseError> {
        self.get_value_by_tag(tags::SOP_INSTANCE_UID)
            .and_then(|v| v.string().map(ToOwned::to_owned))
            .ok_or_else(|| ParseError::InvalidTagPath {
                string_path: "SOPInstanceUID".to_owned(),
                details: "SOPInstanceUID is missing".to_owned(),
            })
    }

    /// Get the Series Instance UID.
    ///
    /// # Errors
    /// - `ParseError::InvalidTagPath` if this dicom object does not contain the Series Instance
    ///   UID element.
    pub fn series_instance_id(&self) -> Result<String, ParseError> {
        self.get_value_by_tag(tags::SERIES_INSTANCE_UID)
            .and_then(|v| v.string().map(ToOwned::to_owned))
            .ok_or_else(|| ParseError::InvalidTagPath {
                string_path: "SeriesInstanceUID".to_owned(),
                details: "SeriesInstanceUID is missing".to_owned(),
            })
    }

    /// The number of child nodes in this `DicomRoot`.
    #[must_use]
    pub fn get_child_count(&self) -> usize {
        self.sentinel.child_count()
    }

    /// Gets a child node by tag number.
    #[must_use]
    pub fn get_child_by_tag<T>(&self, tag: T) -> Option<&DicomObject>
    where
        u32: From<T>,
    {
        self.sentinel.get_child_by_tag(tag)
    }

    /// Gets a mutable child node by tag number.
    pub fn get_child_by_tag_mut<T>(&mut self, tag: T) -> Option<&mut DicomObject>
    where
        u32: From<T>,
    {
        self.sentinel.get_child_by_tag_mut(tag)
    }

    /// Returns an iterator for the child nodes in this `DicomRoot`.
    pub fn iter_child_nodes(&self) -> btree_map::Iter<'_, u32, DicomObject> {
        self.sentinel.iter_child_nodes()
    }

    /// Returns an iterator for the mutable child nodes in this `DicomRoot`.
    pub fn iter_child_nodes_mut(&mut self) -> btree_map::IterMut<'_, u32, DicomObject> {
        self.sentinel.iter_child_nodes_mut()
    }

    /// The number of item nodes in this `DicomRoot`.
    #[must_use]
    pub fn item_count(&self) -> usize {
        self.sentinel.item_count()
    }

    /// Get an item node, by 1-based index.
    #[must_use]
    pub fn get_item_by_index(&self, index: usize) -> Option<&DicomObject> {
        self.sentinel.get_item_by_index(index)
    }

    /// Get a mutable item node by 1-based index.
    pub fn get_item_by_index_mut(&mut self, index: usize) -> Option<&mut DicomObject> {
        self.sentinel.get_item_by_index_mut(index)
    }

    /// Returns an iterator for the item nodes in this `DicomRoot`.
    pub fn iter_items(&self) -> std::slice::Iter<DicomObject> {
        self.sentinel.iter_items()
    }

    /// Returns an iterator for the mutable item nodes in this `DicomRoot`.
    pub fn iter_items_mut(&mut self) -> std::slice::IterMut<DicomObject> {
        self.sentinel.iter_items_mut()
    }

    /// Get a child node with the given `TagNode`.
    #[must_use]
    pub fn get_child_by_tagnode<T>(&self, tag_node: T) -> Option<&DicomObject>
    where
        TagNode: From<T>,
    {
        self.sentinel.get_child_by_tagnode(&TagNode::from(tag_node))
    }

    /// Get a mutable child node with the given `TagNode`.
    pub fn get_child_by_tagnode_mut(&mut self, tag_node: &TagNode) -> Option<&mut DicomObject> {
        self.sentinel.get_child_by_tagnode_mut(tag_node)
    }

    /// Get a child node with the given `TagPath`.
    #[must_use]
    pub fn get_child_by_tagpath(&self, tagpath: &TagPath) -> Option<&DicomObject> {
        self.sentinel.get_child_by_tagpath(tagpath)
    }

    /// Get a mutable child node with the given `TagPath`.
    pub fn get_child_by_tagpath_mut(&mut self, tagpath: &TagPath) -> Option<&mut DicomObject> {
        self.sentinel.get_child_by_tagpath_mut(tagpath)
    }

    /// Get a child element's value by element tag.
    #[must_use]
    pub fn get_value_by_tag<T>(&self, tag: T) -> Option<RawValue>
    where
        u32: From<T>,
    {
        self.sentinel.get_value_by_tag(tag)
    }

    /// Get a child element's value by element tag, parsed as the given VR.
    #[must_use]
    pub fn get_value_as_by_tag<T>(&self, tag: T, vr: VRRef) -> Option<RawValue>
    where
        u32: From<T>,
    {
        self.sentinel.get_value_as_by_tag(tag, vr)
    }

    /// Get a descendant element's value by tagpath.
    #[must_use]
    pub fn get_value_by_tagpath(&self, tagpath: &TagPath) -> Option<RawValue> {
        self.sentinel.get_value_by_tagpath(tagpath)
    }

    /// Get a descendant element's value by tagpath, parsed as the given VR.
    #[must_use]
    pub fn get_value_as_by_tagpath(&self, tagpath: &TagPath, vr: VRRef) -> Option<RawValue> {
        self.sentinel.get_value_as_by_tagpath(tagpath, vr)
    }

    /// Gets the total number of bytes that will be needed to encode this `DicomRoot` and its
    /// child/index nodes into a dataset.
    #[must_use]
    pub fn byte_size(&self) -> usize {
        self.sentinel.byte_size()
    }

    /// Flattens this object into an ordered list of elements as they would appear in a dataset.
    #[must_use]
    pub fn flatten(&self) -> Vec<&DicomElement> {
        self.sentinel.flatten()
    }

    /// Creates a new `DicomElement` from the given `Tag`, using this `DicomRoot`'s transfer syntax
    /// and the tag's implicit VR if present, or `UN` if not.
    #[must_use]
    pub fn create_element(&self, tag: &Tag) -> DicomElement {
        DicomElement::new_empty(tag.num(), tag.implicit_vr().unwrap_or(&UN), self.ts)
    }

    /// Add the given element as a child to this root object. Returns a reference to the newly
    /// created `DicomObject` for the given element.
    pub fn add_element(&mut self, elem: DicomElement) -> &mut DicomObject {
        self.sentinel.add_element(elem)
    }

    /// Creates a new `DicomElement` using `self::create_element()` and adds it to this `DicomRoot`
    /// using `self::add_element()`.
    pub fn add_child(&mut self, tag: &Tag) -> &mut DicomObject {
        self.add_element(self.create_element(tag))
    }

    /// Creates a new `DicomElement` using `self::create_element()` with the given value, and adds
    /// it to this `DicomRoot` using `self::add_element()`.
    pub fn add_child_with_val(&mut self, tag: &Tag, val: RawValue) -> &mut DicomObject {
        let mut elem = self.create_element(tag);
        let _ = elem.encode_val(val);
        self.add_element(elem)
    }

    /// Prints out all dicom elements to standard out.
    ///
    /// # Errors
    /// Any failures writing to stdout.
    pub fn dbg_dump(&self) -> Result<(), Error> {
        let elems = self
            .flatten()
            .iter()
            .map(|o| FormattedElement::new(o))
            .collect::<Vec<FormattedElement>>();
        let mut stdout = stdout().lock();
        for elem in elems {
            stdout.write_all(format!("{elem}\n").as_ref())?;
        }
        Ok(())
    }

    /// Parses elements to build a `DicomObject` to represent the parsed dataset as an in-memory tree.
    /// Returns `None` if the parser's first element fails to parse properly, assumed to be a non-DICOM
    /// dataset. Any errors after a successful first element being parsed are returned as `Result::Err`.
    ///
    /// # Errors
    /// The parser over the dataset stream may fail parsing DICOM.
    pub fn parse<R: Read>(parser: &mut Parser<R>) -> Result<Option<DicomRoot>, ParseError> {
        let mut child_nodes: BTreeMap<u32, DicomObject> = BTreeMap::new();
        let mut items: Vec<DicomObject> = Vec::new();

        let parse_result: Option<Result<DicomElement, ParseError>> =
            DicomRoot::parse_recurse(parser, &mut child_nodes, &mut items, true);

        if !parser.behavior().allow_partial_object() {
            if let Some(Err(e)) = parse_result {
                return Err(e);
            }
        }

        // If no child nodes and no items were parsed then this isn't valid dicom.
        if child_nodes.is_empty() && items.is_empty() {
            return Ok(None);
        }

        // Copy the parser state only after having parsed elements, to get appropriate transfer syntax
        // and specific character set.
        let root: DicomRoot = DicomRoot::new(parser.ts(), parser.cs(), child_nodes, items);
        Ok(Some(root))
    }

    /// Iterates through the parser populating values into the given `child_nodes` map. Elements which
    /// are sequence-like (contain sub-elements) will be recursed into so child elements are added to
    /// their node. The sequence path length is used to determine when parsing an element whether it
    /// escapes the current level a of recursion, and how far back up it should go (the end of a
    /// sequence can be the end of multiple sequences).
    ///
    /// `parser` The parser elements are being read from
    /// `child_nodes` The map of child nodes which elements should be parsed into
    /// `items` The list of nodes which item elements should be parsed into
    /// `is_root_level` Whether the root level is being parsed, or within child nodes
    fn parse_recurse<R: Read>(
        parser: &mut Parser<'_, R>,
        child_nodes: &mut BTreeMap<u32, DicomObject>,
        items: &mut Vec<DicomObject>,
        is_root_level: bool,
    ) -> Option<Result<DicomElement, ParseError>> {
        let mut prev_seq_path_len: usize = 0;
        let mut next_element: Option<Result<DicomElement, ParseError>> = parser.next();

        // If the first element at the root level is an error then this is probably not valid dicom.
        if is_root_level {
            if let Some(Err(_)) = next_element {
                return None;
            }
        }

        while let Some(Ok(element)) = next_element {
            let tag: u32 = element.tag();
            let cur_seq_path_len: usize = element.sq_path().len() + 1;

            if prev_seq_path_len == 0 {
                prev_seq_path_len = cur_seq_path_len;
            }

            if cur_seq_path_len < prev_seq_path_len {
                // If the next element has a shorter path than the previous one it should not be added
                // to the given node but returned so it can be added to a parent node.
                return Some(Ok(element));
            }

            let mut possible_next_elem: Option<Result<DicomElement, ParseError>> = None;
            // Checking sequence or item tag should match dcmparser.read_dicom_element() which
            // does not read a value for those elements but lets the parser read its value as
            // separate elements which we're considering child elements.
            let dcmobj: DicomObject = if element.is_sq_like()
                || (tag == tags::ITEM && element.vl() != ValueLength::Explicit(0))
            {
                let mut child_nodes: BTreeMap<u32, DicomObject> = BTreeMap::new();
                let mut items: Vec<DicomObject> = Vec::new();
                possible_next_elem =
                    DicomRoot::parse_recurse(parser, &mut child_nodes, &mut items, false);
                DicomObject::new_with_children(element, child_nodes, items)
            } else {
                DicomObject::new(element)
            };
            if tag == tags::ITEM {
                items.push(dcmobj);
            } else {
                child_nodes.insert(tag, dcmobj);
            }

            prev_seq_path_len = cur_seq_path_len;

            // If an element was returned from the recursive call that means the recursive call iterated
            // into an element which is not a child of the node we passed into the recursive call and
            // it should instead be added elsewhere up the tree.
            if let Some(Ok(next_elem)) = possible_next_elem {
                let next_seq_path_len: usize = next_elem.sq_path().len() + 1;
                match next_seq_path_len {
                    val if val < cur_seq_path_len => {
                        // If that element is still shorter than the current then it needs passed up further.
                        return Some(Ok(next_elem));
                    }
                    val if val == cur_seq_path_len => {
                        // If it matches the same level as the current node then it should be "inserted"
                        // into the element iteration so it will be checked for being sequence-like and
                        // then added into the current node.
                        next_element = Some(Ok(next_elem));
                        continue;
                    }
                    _ => {}
                }
            } else if let Some(Err(_)) = possible_next_elem {
                // Errors need propagated all the way back up.
                return possible_next_elem;
            }

            // Parse the next element from the dataset.
            next_element = parser.next();
        }

        // Return the last value from the parser which will either be None or Error.
        next_element
    }
}

impl std::fmt::Debug for DicomRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<DicomRoot {} {}> {:?}",
            self.ts().uid().ident(),
            self.cs.name(),
            &self.sentinel
        )
    }
}

/// A DICOM object which represents a DICOM Element and may have child elements
#[derive(Clone)]
pub struct DicomObject {
    /// The element of the current node. For sequence elements this will be the sequence element
    /// itself.
    element: DicomElement,
    /// Child nodes which may be elements of sub-sequences.
    child_nodes: BTreeMap<u32, DicomObject>,
    /// Item nodes don't go into the map since there can be multiple of them.
    items: Vec<DicomObject>,
}

impl DicomObject {
    #[must_use]
    pub fn new(element: DicomElement) -> DicomObject {
        DicomObject {
            element,
            child_nodes: BTreeMap::new(),
            items: Vec::new(),
        }
    }

    #[must_use]
    pub fn new_with_children(
        element: DicomElement,
        child_nodes: BTreeMap<u32, DicomObject>,
        items: Vec<DicomObject>,
    ) -> DicomObject {
        DicomObject {
            element,
            child_nodes,
            items,
        }
    }

    /// A reference to the wrapped `DicomElement` of this `DicomObject`.
    #[must_use]
    pub fn element(&self) -> &DicomElement {
        &self.element
    }

    /// A mutable reference to the wrapped `DicomElement` of this `DicomObject`.
    pub fn element_mut(&mut self) -> &mut DicomElement {
        &mut self.element
    }

    /// Get the SOP Instance UID.
    ///
    /// # Errors
    /// - `ParseError::InvalidTagPath` if this dicom object does not contain the SOP Instance UID
    ///   element.
    pub fn sop_instance_id(&self) -> Result<String, ParseError> {
        self.get_value_by_tag(tags::SOP_INSTANCE_UID)
            .and_then(|v| v.string().map(ToOwned::to_owned))
            .ok_or_else(|| ParseError::InvalidTagPath {
                string_path: "SOPInstanceUID".to_owned(),
                details: "SOPInstanceUID is missing".to_owned(),
            })
    }

    /// Get the Series Instance UID.
    ///
    /// # Errors
    /// - `ParseError::InvalidTagPath` if this dicom object does not contain the Series Instance
    ///   UID element.
    pub fn series_instance_id(&self) -> Result<String, ParseError> {
        self.get_value_by_tag(tags::SERIES_INSTANCE_UID)
            .and_then(|v| v.string().map(ToOwned::to_owned))
            .ok_or_else(|| ParseError::InvalidTagPath {
                string_path: "SeriesInstanceUID".to_owned(),
                details: "SeriesInstanceUID is missing".to_owned(),
            })
    }

    /// The number of child nodes in this `DicomObject`.
    #[must_use]
    pub fn child_count(&self) -> usize {
        self.child_nodes.len()
    }

    /// Gets a child node by tag number.
    #[must_use]
    pub fn get_child_by_tag<T>(&self, tag: T) -> Option<&DicomObject>
    where
        u32: From<T>,
    {
        self.child_nodes.get(&u32::from(tag))
    }

    /// Gets a mutable child node by tag number.
    pub fn get_child_by_tag_mut<T>(&mut self, tag: T) -> Option<&mut DicomObject>
    where
        u32: From<T>,
    {
        self.child_nodes.get_mut(&u32::from(tag))
    }

    /// Returns an iterator for the child nodes in this `DicomObject`.
    pub fn iter_child_nodes(&self) -> btree_map::Iter<'_, u32, DicomObject> {
        self.child_nodes.iter()
    }

    /// Returns an iterator for the mutable child nodes in this `DicomObject`.
    pub fn iter_child_nodes_mut(&mut self) -> btree_map::IterMut<'_, u32, DicomObject> {
        self.child_nodes.iter_mut()
    }

    /// The number of item nodes in this `DicomObject`.
    #[must_use]
    pub fn item_count(&self) -> usize {
        self.items.len()
    }

    /// Get an item node, by 1-based index.
    #[must_use]
    pub fn get_item_by_index(&self, index: usize) -> Option<&DicomObject> {
        self.items.get(index - 1)
    }

    /// Get a mutable item node by 1-based index.
    pub fn get_item_by_index_mut(&mut self, index: usize) -> Option<&mut DicomObject> {
        self.items.get_mut(index - 1)
    }

    /// Returns an iterator for the item nodes in this `DicomObject`.
    pub fn iter_items(&self) -> std::slice::Iter<DicomObject> {
        self.items.iter()
    }

    /// Returns an iterator for the mutable item nodes in this `DicomObject`.
    pub fn iter_items_mut(&mut self) -> std::slice::IterMut<DicomObject> {
        self.items.iter_mut()
    }

    /// Get a child node with the given `TagNode`.
    #[must_use]
    pub fn get_child_by_tagnode(&self, tag_node: &TagNode) -> Option<&DicomObject> {
        self.get_child_by_tag(tag_node.tagnum())
            .and_then(|o| match tag_node.item() {
                None => Some(o),
                Some(item_num) => o.get_item_by_index(item_num),
            })
    }

    /// Get a mutable child node with the given `TagNode`.
    pub fn get_child_by_tagnode_mut(&mut self, tag_node: &TagNode) -> Option<&mut DicomObject> {
        self.get_child_by_tag_mut(tag_node.tagnum())
            .and_then(|o| match tag_node.item() {
                None => Some(o),
                Some(item_num) => o.get_item_by_index_mut(item_num),
            })
    }

    /// Get a child node with the given `TagPath`.
    #[must_use]
    pub fn get_child_by_tagpath(&self, tagpath: &TagPath) -> Option<&DicomObject> {
        let mut target = tagpath
            .nodes()
            .first()
            .map(|n| self.get_child_by_tagnode(n))?;
        for node in tagpath.nodes().iter().skip(1) {
            target = target.and_then(|parent| parent.get_child_by_tagnode(node));
        }
        target
    }

    /// Get a mutable child node with the given `TagPath`.
    pub fn get_child_by_tagpath_mut(&mut self, tagpath: &TagPath) -> Option<&mut DicomObject> {
        let mut target = tagpath
            .nodes()
            .first()
            .map(|n| self.get_child_by_tagnode_mut(n))?;
        for node in tagpath.nodes().iter().skip(1) {
            target = target.and_then(|parent| parent.get_child_by_tagnode_mut(node));
        }
        target
    }

    /// Get a child element's value by element tag.
    #[must_use]
    pub fn get_value_by_tag<T>(&self, tag: T) -> Option<RawValue>
    where
        u32: From<T>,
    {
        self.child_nodes
            .get(&u32::from(tag))
            .and_then(|o| o.element().parse_value().ok())
    }

    /// Get a child element's value by element tag, parsed as the given VR.
    #[must_use]
    pub fn get_value_as_by_tag<T>(&self, tag: T, vr: VRRef) -> Option<RawValue>
    where
        u32: From<T>,
    {
        self.child_nodes
            .get(&u32::from(tag))
            .and_then(|o| o.element().parse_value_as(vr).ok())
    }

    /// Get a descendant element's value by tagpath.
    #[must_use]
    pub fn get_value_by_tagpath(&self, tagpath: &TagPath) -> Option<RawValue> {
        self.get_child_by_tagpath(tagpath)
            .and_then(|o| o.element().parse_value().ok())
    }

    /// Get a descendant element's value by tagpath, parsed as a given VR.
    #[must_use]
    pub fn get_value_as_by_tagpath(&self, tagpath: &TagPath, vr: VRRef) -> Option<RawValue> {
        self.get_child_by_tagpath(tagpath)
            .and_then(|o| o.element().parse_value_as(vr).ok())
    }

    /// Gets the total number of bytes that will be needed to encode this `DicomObject` and its
    /// child/index nodes into a dataset.
    #[must_use]
    pub fn byte_size(&self) -> usize {
        self.flatten().iter().map(|e| e.byte_size()).sum::<usize>()
    }

    /// Flattens this object into an ordered list of elements as they would appear in a dataset.
    // TODO: How to make this return an iterator?
    #[must_use]
    pub fn flatten(&self) -> Vec<&DicomElement> {
        let mut elements: Vec<&DicomElement> =
            Vec::with_capacity(1 + self.item_count() + self.child_count());

        // Add this element, if it's not a phantom/sentinel.
        if !self.element().is_sentinel() {
            elements.push(self.element());
        }

        // List items + contents first, as SQ objects will include both items for its contents as
        // well as the sequence delimiter as a child node.
        for dcmobj in self.iter_items() {
            elements.append(&mut (dcmobj.flatten()));
        }
        for (_tag, dcmobj) in self.iter_child_nodes() {
            elements.append(&mut (dcmobj.flatten()));
        }

        elements
    }

    /// Add the given element as a child to this current object/element.
    /// Returns a reference to the newly created `DicomObject` for the given element.
    pub fn add_element(&mut self, elem: DicomElement) -> &mut DicomObject {
        let tag = elem.tag();
        let obj = DicomObject::new(elem);
        self.child_nodes.entry(tag).or_insert(obj)
    }
}

impl fmt::Debug for DicomObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.items.is_empty() && self.child_nodes.is_empty() {
            write!(f, "{:?}", &self.element)
        } else {
            write!(
                f,
                "{:?}, items:{}, child_nodes:{}",
                &self.element,
                self.items.len(),
                self.child_nodes.len()
            )
        }
    }
}
