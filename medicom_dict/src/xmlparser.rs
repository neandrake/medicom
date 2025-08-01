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

//! Parses the XML format of Part 6 of the DICOM Standard.
//!
//! The XML document is fairly large so this uses quick-xml as a SAX-style XML parser, resulting in
//! efficient processing of the XML document but with added complexity. To use properly the
//! structure of the expected document needs to be tracked as quick-xml emits events from the
//! document.

#![allow(clippy::too_many_lines, clippy::match_same_arms)]

use std::io::BufRead;

use quick_xml::events::{BytesRef, BytesText, Event};
use quick_xml::name::{LocalName, QName};
use quick_xml::Error as XmlError;
use quick_xml::Reader;

pub type XmlDicomDefinitionResult = Result<XmlDicomDefinition, XmlError>;

/// The different types of objects parsed out of the xml document.
#[derive(Eq, PartialEq, Debug)]
pub enum XmlDicomDefinition {
    DicomElement(XmlDicomElement),
    FileMetaElement(XmlDicomElement),
    DirStructureElement(XmlDicomElement),
    Uid(XmlDicomUid),
    TransferSyntax(XmlDicomUid),
    CommandElement(XmlDicomElement),
}

/// The contents of a DICOM Element defined in the xml document.
#[derive(Debug, PartialEq, Eq, PartialOrd, Clone)]
pub struct XmlDicomElement {
    pub tag: u32,
    pub name: String,
    pub keyword: String,
    pub vr: String,
    pub vm: String,
    pub obs: Option<String>,
}

/// The contents of a UID defined in the xml document.
#[derive(Debug, PartialEq, Eq, PartialOrd, Clone)]
pub struct XmlDicomUid {
    pub value: String,
    pub name: String,
    pub keyword: Option<String>,
    pub uid_type: Option<String>,
    pub part: Option<String>,
}

/// The different table structures that appear in the xml document. This is used to track the state
/// of where the parser is currently located.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum XmlDicomDefinitionTable {
    DicomElements,
    FileMetaElements,
    DirStructureElements,
    Uids,
    CommandElements,
    Unknown,
}

/// The cells within the DICOM Element table. This is used to track the state of where the parser
/// is currently located after traversing into `XmlDicomDefinitionTable::DicomElements`,
/// `FileMetaElements`, or `DirStructureElements`.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum XmlDicomElementCell {
    RowStart,
    Tag,
    Name,
    Keyword,
    VR,
    VM,
    Obs,
}

/// The structure of the DICOM UID table. This is used to track the state of where the parser is
/// currently located after traversing into `XmlDicomDefinitionTable::Uids`.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum XmlDicomUidCell {
    RowStart,
    Value,
    Name,
    Keyword,
    Type,
    Part,
}

/// These are additional transient traversal states used when traversing through the different
/// tables and cells.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum XmlDicomReadingState {
    OutsideTable,
    InTable,
    InTableBody,
    InDicomElementCell(XmlDicomElementCell),
    InDicomUidCell(XmlDicomUidCell),
}

pub struct XmlDicomDefinitionIterator<R: BufRead> {
    parser: Reader<R>,
    state: XmlDicomReadingState,

    table: XmlDicomDefinitionTable,

    element_tag: Option<u32>,
    element_name: Option<String>,
    element_keyword: Option<String>,
    element_vr: Option<String>,
    element_vm: Option<String>,
    element_obs: Option<String>,

    uid_value: Option<String>,
    uid_name: Option<String>,
    uid_keyword: Option<String>,
    uid_type: Option<String>,
    uid_part: Option<String>,
}

fn decode_ref_bytes(data: &BytesRef<'_>) -> String {
    if data.is_char_ref() {
        data.resolve_char_ref()
            .unwrap_or_else(|err| {
                panic!("Error parsing DICOM Entry reference: {data:?}\n\t{err:?}")
            })
            .unwrap_or_else(|| panic!("Error parsing DICOM Entry reference: {data:?}"))
            .to_string()
    } else {
        let data = data
            .decode()
            .unwrap_or_else(|err| {
                panic!("Error parsing DICOM Entry reference: {data:?}\n\t{err:?}")
            })
            .trim()
            .replace('\u{200b}', "");
        match data.as_str() {
            "amp" => "&".to_string(),
            "lt" => "<".to_string(),
            "gt" => ">".to_string(),
            other => panic!("Error parsing DICOM Entry reference: {other}"),
        }
    }
}

fn parse_text_bytes(data: &BytesText<'_>) -> String {
    data.decode()
        .unwrap_or_else(|err| panic!("Error parsing DICOM Entry text: {data:?}\n\t{err:?}"))
        .trim()
        .replace('\u{200b}', "")
}

fn parse_text_bytes_as_u32(data: &BytesText<'_>) -> Option<u32> {
    u32::from_str_radix(&parse_text_bytes(data).replace(['(', ')', ','], ""), 16).ok()
}

impl<R: BufRead> XmlDicomDefinitionIterator<R> {
    pub fn new(xml: R) -> XmlDicomDefinitionIterator<R> {
        let mut reader = Reader::from_reader(xml);
        reader.config_mut().expand_empty_elements = true;
        reader.config_mut().trim_text(true);
        XmlDicomDefinitionIterator {
            parser: reader,
            state: XmlDicomReadingState::OutsideTable,

            table: XmlDicomDefinitionTable::Unknown,

            element_tag: None,
            element_name: None,
            element_keyword: None,
            element_vr: None,
            element_vm: None,
            element_obs: None,

            uid_value: None,
            uid_name: None,
            uid_keyword: None,
            uid_type: None,
            uid_part: None,
        }
    }

    /// Determine if all required cell fields for a Dicom Element have been read.
    fn is_next_element_fully_read(&self) -> bool {
        // Observation may not have content.
        self.element_tag.is_some()
            && self.element_name.is_some()
            && self.element_keyword.is_some()
            && self.element_vr.is_some()
            && self.element_vm.is_some()
    }

    /// Determine if all required cell fields for a UID have been read.
    fn is_next_uid_fully_read(&self) -> bool {
        // The type, part, and keyword fields may not have content.
        self.uid_value.is_some() && self.uid_name.is_some()
    }

    /// Clear out all cell data that has been parsed.
    fn clear_next(&mut self) {
        self.element_tag = None;
        self.element_name = None;
        self.element_keyword = None;
        self.element_vr = None;
        self.element_vm = None;
        self.element_obs = None;

        self.uid_value = None;
        self.uid_name = None;
        self.uid_keyword = None;
        self.uid_type = None;
        self.uid_part = None;
    }
}

impl<R: BufRead> Iterator for XmlDicomDefinitionIterator<R> {
    type Item = XmlDicomDefinitionResult;

    fn next(&mut self) -> Option<XmlDicomDefinitionResult> {
        let mut buf: Vec<u8> = Vec::new();
        loop {
            buf.clear();
            let res: Result<Event<'_>, XmlError> = self.parser.read_event_into(&mut buf);
            match res {
                // The start of new elements (e.g. "<table>" instead of "</table>") are used to
                // keep the state machine (`state` and `table` fields) updated to track what the
                // data we can expect to encounter.
                Ok(Event::Start(ref e)) => {
                    let local_name: LocalName = e.local_name();
                    match self.state {
                        // If we're currently in the Off state then that means we've just started
                        // parsing or outside of tables so we should first expect to hit a "table"
                        // element.
                        XmlDicomReadingState::OutsideTable => {
                            if local_name == QName(b"table").into() {
                                // The tables that we're interested in parsing have a "xml:id"
                                // attribute identifying which table this is for.
                                if let Some(xml_id_attr) = e
                                    .attributes()
                                    .find(|attr| {
                                        !attr.is_err()
                                            && attr.as_ref().unwrap().key == QName(b"xml:id")
                                    })
                                    .map(Result::unwrap)
                                {
                                    // Flip state of the table we're entering so we expected to
                                    // encounter the cells for the expected structure.
                                    match xml_id_attr.value.as_ref() {
                                        b"table_6-1" => {
                                            self.table = XmlDicomDefinitionTable::DicomElements;
                                            self.state = XmlDicomReadingState::InTable;
                                        }
                                        b"table_7-1" => {
                                            self.table = XmlDicomDefinitionTable::FileMetaElements;
                                            self.state = XmlDicomReadingState::InTable;
                                        }
                                        b"table_8-1" => {
                                            self.table =
                                                XmlDicomDefinitionTable::DirStructureElements;
                                            self.state = XmlDicomReadingState::InTable;
                                        }
                                        b"table_A-1" => {
                                            self.table = XmlDicomDefinitionTable::Uids;
                                            self.state = XmlDicomReadingState::InTable;
                                        }
                                        // XXX: This table exists in Part 7 document while all
                                        // other tables parsed here are in Part 6 document. This
                                        // parser currently doesn't make any distinguishes between
                                        // the two documents, as coincidentally the table IDs do
                                        // not overlap.
                                        b"table_E.1-1" => {
                                            self.table = XmlDicomDefinitionTable::CommandElements;
                                            self.state = XmlDicomReadingState::InTable;
                                        }
                                        // Unknown table, set the initial state.
                                        _ => {
                                            self.table = XmlDicomDefinitionTable::Unknown;
                                            self.state = XmlDicomReadingState::OutsideTable;
                                        }
                                    }
                                }
                            }
                        }
                        // If a "tbody" is started then update state so the next thing we expect
                        // are "tr" and cell data.
                        XmlDicomReadingState::InTable => {
                            // Flip state to being in a table body once we've passed the "thead"
                            // element.
                            if local_name == QName(b"tbody").into() {
                                self.state = XmlDicomReadingState::InTableBody;
                            }
                        }
                        // If a "tr" has started then switch the state to the next expected cell.
                        XmlDicomReadingState::InTableBody => {
                            if local_name == QName(b"tr").into() {
                                // New row, clear existing values.
                                self.clear_next();
                                match self.table {
                                    XmlDicomDefinitionTable::DicomElements
                                    | XmlDicomDefinitionTable::FileMetaElements
                                    | XmlDicomDefinitionTable::DirStructureElements
                                    | XmlDicomDefinitionTable::CommandElements => {
                                        // The first cell in these tables is the Tag number.
                                        self.state = XmlDicomReadingState::InDicomElementCell(
                                            XmlDicomElementCell::RowStart,
                                        );
                                    }
                                    XmlDicomDefinitionTable::Uids => {
                                        // The first cell in this table is the Value (actual UID).
                                        self.state = XmlDicomReadingState::InDicomUidCell(
                                            XmlDicomUidCell::RowStart,
                                        );
                                    }
                                    // We're in an unknown state, bounce the state machine back out
                                    // to top-level. The rest of this table will be ignored but
                                    // that's probably the only reasonable thing to do.
                                    XmlDicomDefinitionTable::Unknown => {
                                        self.table = XmlDicomDefinitionTable::Unknown;
                                        self.state = XmlDicomReadingState::OutsideTable;
                                    }
                                }
                            }
                        }
                        // We've previously entered into a table structured for DICOM elements and
                        // should expect to find "td" elements.
                        XmlDicomReadingState::InDicomElementCell(element_cell) => {
                            if local_name == QName(b"td").into() {
                                // Set the next expected cell to encounter based on the current.
                                self.state = match element_cell {
                                    XmlDicomElementCell::RowStart => {
                                        XmlDicomReadingState::InDicomElementCell(
                                            XmlDicomElementCell::Tag,
                                        )
                                    }
                                    XmlDicomElementCell::Tag => {
                                        XmlDicomReadingState::InDicomElementCell(
                                            XmlDicomElementCell::Name,
                                        )
                                    }
                                    XmlDicomElementCell::Name => {
                                        XmlDicomReadingState::InDicomElementCell(
                                            XmlDicomElementCell::Keyword,
                                        )
                                    }
                                    XmlDicomElementCell::Keyword => {
                                        XmlDicomReadingState::InDicomElementCell(
                                            XmlDicomElementCell::VR,
                                        )
                                    }
                                    XmlDicomElementCell::VR => {
                                        XmlDicomReadingState::InDicomElementCell(
                                            XmlDicomElementCell::VM,
                                        )
                                    }
                                    XmlDicomElementCell::VM => {
                                        XmlDicomReadingState::InDicomElementCell(
                                            XmlDicomElementCell::Obs,
                                        )
                                    }
                                    XmlDicomElementCell::Obs => {
                                        XmlDicomReadingState::InDicomElementCell(
                                            XmlDicomElementCell::Tag,
                                        )
                                    }
                                };
                            }
                        }
                        // We've previously entered into a table structured for DICOM UIDs.
                        XmlDicomReadingState::InDicomUidCell(uid_cell) => {
                            if local_name == QName(b"td").into() {
                                self.state = match uid_cell {
                                    XmlDicomUidCell::RowStart => {
                                        XmlDicomReadingState::InDicomUidCell(XmlDicomUidCell::Value)
                                    }
                                    XmlDicomUidCell::Value => {
                                        XmlDicomReadingState::InDicomUidCell(XmlDicomUidCell::Name)
                                    }
                                    XmlDicomUidCell::Name => XmlDicomReadingState::InDicomUidCell(
                                        XmlDicomUidCell::Keyword,
                                    ),
                                    XmlDicomUidCell::Keyword => {
                                        XmlDicomReadingState::InDicomUidCell(XmlDicomUidCell::Type)
                                    }
                                    XmlDicomUidCell::Type => {
                                        XmlDicomReadingState::InDicomUidCell(XmlDicomUidCell::Part)
                                    }
                                    XmlDicomUidCell::Part => {
                                        XmlDicomReadingState::InDicomUidCell(XmlDicomUidCell::Value)
                                    }
                                };
                            }
                        }
                    }
                }

                // Events for element-ends (e.g. "</tr>" and not "<tr>") are used to check if
                // enough data was extracted and structured based on the expected format.
                Ok(Event::End(ref e)) => {
                    let local_name: LocalName = e.local_name();
                    match self.state {
                        // Unexpected state.
                        XmlDicomReadingState::OutsideTable => {}

                        _ => {
                            // If a "tr" element ended check to see if all necessary fields for a
                            // DICOM Element or UID were extracted.
                            if local_name == QName(b"tr").into() {
                                if self.is_next_element_fully_read() {
                                    let out = XmlDicomElement {
                                        tag: self.element_tag.take().unwrap(),
                                        name: self.element_name.take().unwrap(),
                                        keyword: self.element_keyword.take().unwrap(),
                                        vr: self.element_vr.take().unwrap(),
                                        vm: self.element_vm.take().unwrap(),
                                        obs: self.element_obs.take(),
                                    };

                                    // Clear out all cell data extracted and put back into a state
                                    // of being in the table body, ready for the next row.
                                    self.clear_next();
                                    self.state = XmlDicomReadingState::InTableBody;

                                    // For the iterator, return the item extracted, typed based on
                                    // the current table being parsed.
                                    match self.table {
                                        XmlDicomDefinitionTable::DicomElements => {
                                            return Some(Ok(XmlDicomDefinition::DicomElement(out)))
                                        }
                                        XmlDicomDefinitionTable::FileMetaElements => {
                                            return Some(Ok(XmlDicomDefinition::FileMetaElement(
                                                out,
                                            )))
                                        }
                                        XmlDicomDefinitionTable::DirStructureElements => {
                                            return Some(Ok(
                                                XmlDicomDefinition::DirStructureElement(out),
                                            ))
                                        }
                                        XmlDicomDefinitionTable::CommandElements => {
                                            return Some(Ok(XmlDicomDefinition::CommandElement(
                                                out,
                                            )))
                                        }
                                        // All fields for dicom element filled but we're in UIDs
                                        // table??
                                        XmlDicomDefinitionTable::Uids => {}
                                        // Unexpected state.
                                        XmlDicomDefinitionTable::Unknown => {}
                                    }
                                } else if self.is_next_uid_fully_read() {
                                    let out = XmlDicomUid {
                                        value: self.uid_value.take().unwrap(),
                                        name: self.uid_name.take().unwrap(),
                                        keyword: self.uid_keyword.take(),
                                        uid_type: self.uid_type.take(),
                                        part: self.uid_part.take(),
                                    };

                                    // Clear out all cell data extracted and put back into a state
                                    // of being in the table body, ready for the next row.
                                    self.clear_next();
                                    self.state = XmlDicomReadingState::InTableBody;

                                    // For the iterator, return the item extracted, typed based on
                                    // the current table being parsed.
                                    #[allow(clippy::single_match)]
                                    match self.table {
                                        // The UID table lumps all UIDs together, but here the
                                        // Transfer Syntaxes are distinguished separately.
                                        XmlDicomDefinitionTable::Uids => {
                                            if out
                                                .uid_type
                                                .as_ref()
                                                .filter(|v| "Transfer Syntax".eq(*v))
                                                .is_some()
                                            {
                                                return Some(Ok(
                                                    XmlDicomDefinition::TransferSyntax(out),
                                                ));
                                            }
                                            return Some(Ok(XmlDicomDefinition::Uid(out)));
                                        }
                                        // All fields for UID filled in but we're in an elements
                                        // table??
                                        XmlDicomDefinitionTable::DicomElements
                                        | XmlDicomDefinitionTable::FileMetaElements
                                        | XmlDicomDefinitionTable::DirStructureElements
                                        | XmlDicomDefinitionTable::CommandElements => {}
                                        // Unexpected state.
                                        XmlDicomDefinitionTable::Unknown => {}
                                    }
                                } else {
                                    // If a row ended and not all fields were filled in then clear all
                                    // the fields to avoid polluting the next row.
                                    self.clear_next();
                                }
                            } else if local_name == QName(b"tbody").into() {
                                // If the "table" element ended bump the state back out into
                                // detecting the next table being parsed.
                                self.state = XmlDicomReadingState::OutsideTable;
                                self.table = XmlDicomDefinitionTable::Unknown;
                                self.clear_next();
                            }
                        }
                    }
                }

                // Occurrences of xml-escaped characters such as & ('&amp;') will occur between Text events.
                Ok(Event::GeneralRef(data)) => match self.state {
                    XmlDicomReadingState::InDicomElementCell(element_cell) => match element_cell {
                        XmlDicomElementCell::Name => {
                            if let Some(ref mut name) = self.element_name {
                                name.push_str(&decode_ref_bytes(&data));
                            }
                        }
                        XmlDicomElementCell::Keyword => {
                            if let Some(ref mut keyword) = self.element_keyword {
                                keyword.push_str(&decode_ref_bytes(&data));
                            }
                        }
                        XmlDicomElementCell::Obs => {
                            if let Some(ref mut obs) = self.element_obs {
                                obs.push_str(&decode_ref_bytes(&data));
                            }
                        }
                        _ => {}
                    },

                    XmlDicomReadingState::InDicomUidCell(uid_cell) => match uid_cell {
                        XmlDicomUidCell::Name => {
                            if let Some(ref mut name) = self.uid_name {
                                name.push_str(&decode_ref_bytes(&data));
                            }
                        }
                        XmlDicomUidCell::Keyword => {
                            if let Some(ref mut keyword) = self.uid_keyword {
                                keyword.push_str(&decode_ref_bytes(&data));
                            }
                        }
                        _ => {}
                    },
                    _ => {}
                },

                // When Text content occurs the data is extracted and set based on the current
                // state machine.
                Ok(Event::Text(data)) => match self.state {
                    XmlDicomReadingState::InDicomElementCell(element_cell) => match element_cell {
                        XmlDicomElementCell::RowStart => {}
                        XmlDicomElementCell::Tag => {
                            if self.element_tag.is_none() {
                                self.element_tag = parse_text_bytes_as_u32(&data);
                            }
                        }
                        XmlDicomElementCell::Name => {
                            if let Some(ref mut name) = self.element_name {
                                name.push_str(&parse_text_bytes(&data));
                            } else {
                                self.element_name = Some(parse_text_bytes(&data));
                            }
                        }
                        XmlDicomElementCell::Keyword => {
                            if let Some(ref mut keyword) = self.element_keyword {
                                keyword.push_str(&parse_text_bytes(&data));
                            } else {
                                self.element_keyword = Some(parse_text_bytes(&data));
                            }
                        }
                        XmlDicomElementCell::VR => {
                            if self.element_vr.is_none() {
                                self.element_vr = Some(parse_text_bytes(&data));
                            }
                        }
                        XmlDicomElementCell::VM => {
                            if self.element_vm.is_none() {
                                self.element_vm = Some(parse_text_bytes(&data));
                            }
                        }
                        XmlDicomElementCell::Obs => {
                            if self.element_obs.is_none() {
                                self.element_obs = Some(parse_text_bytes(&data));
                            }
                        }
                    },
                    XmlDicomReadingState::InDicomUidCell(uid_cell) => match uid_cell {
                        XmlDicomUidCell::RowStart => {}
                        XmlDicomUidCell::Value => {
                            if self.uid_value.is_none() {
                                self.uid_value = Some(parse_text_bytes(&data));
                            }
                        }
                        XmlDicomUidCell::Name => {
                            if let Some(ref mut name) = self.uid_name {
                                name.push_str(&parse_text_bytes(&data));
                            } else {
                                self.uid_name = Some(parse_text_bytes(&data));
                            }
                        }
                        XmlDicomUidCell::Keyword => {
                            if let Some(ref mut keyword) = self.uid_keyword {
                                keyword.push_str(&parse_text_bytes(&data));
                            } else {
                                self.uid_keyword = Some(parse_text_bytes(&data));
                            }
                        }
                        XmlDicomUidCell::Type => {
                            if self.uid_type.is_none() {
                                self.uid_type = Some(parse_text_bytes(&data));
                            }
                        }
                        XmlDicomUidCell::Part => {
                            if self.uid_part.is_none() {
                                self.uid_part = Some(parse_text_bytes(&data));
                            }
                        }
                    },
                    _ => {}
                },
                Ok(Event::Eof { .. }) => {
                    break;
                }
                Ok(_) => {}
                Err(e) => {
                    return Some(Err(e));
                }
            }
        }

        None
    }
}
