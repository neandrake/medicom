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

use std::{
    borrow::Borrow,
    io::{Cursor, Read},
    iter::{once, Peekable},
    mem::swap,
};

use crate::{
    core::{
        charset::CSRef,
        dcmelement::DicomElement,
        dcmobject::DicomRoot,
        defn::{constants::ts::ImplicitVRLittleEndian, ts::TSRef},
        read::{ParserBuilder, ParserState},
        write::{
            builder::WriterBuilder,
            writer::{Writer, WriterState},
        },
    },
    dict::stdlookup::STANDARD_DICOM_DICTIONARY,
    dimse::{
        assoc::DimseMsg,
        commands::messages::CommandMessage,
        pdus::{
            mainpdus::{PresentationDataItem, PresentationDataValue},
            msg_header, Pdu,
        },
        DimseError, PDIResult,
    },
};

pub fn read_next_pdu<R: Read>(
    reader: R,
    max_pdu_rcv_size: usize,
) -> Option<Result<PduIterItem, DimseError>> {
    PduIter::new(reader, max_pdu_rcv_size).next()
}

#[derive(Debug)]
pub enum PduIterItem {
    Pdu(Pdu),
    CmdMessage(CommandMessage),
    Dataset(PresentationDataValue),
}

/// Parsing PDU's from a stream was originally implemented as an iterator however it turns out that
/// most activity over DIMSE protocol is organized as request/response activity. The result is that
/// the user of this API will rarely need to read a stream of PDUs. To simplify, the
/// `read_next_pdu()` function above was created to encapsulate this iterator implementation.
pub struct PduIter<R: Read> {
    stream: R,
    max_pdu_size: usize,
}

impl<R: Read> PduIter<R> {
    pub fn new(stream: R, max_pdu_size: usize) -> Self {
        Self {
            stream,
            max_pdu_size,
        }
    }
}

impl<R: Read> Iterator for PduIter<R> {
    type Item = Result<PduIterItem, DimseError>;

    fn next(&mut self) -> Option<Self::Item> {
        let pdu = Pdu::read(&mut self.stream);

        let pres_data_item = match pdu {
            Ok(Pdu::PresentationDataItemPartial(pres_data_item)) => pres_data_item,
            Ok(other) => return Some(Ok(PduIterItem::Pdu(other))),
            Err(e) => return Some(Err(e)),
        };

        let pdu_size = usize::try_from(pres_data_item.length()).unwrap_or_default();
        if self.max_pdu_size > 0 && pdu_size > self.max_pdu_size {
            return Some(Err(DimseError::MaxPduSizeExceeded(pdu_size)));
        }

        let pres_data_val = PresentationDataValue::read(&mut self.stream);
        if let Err(e) = pres_data_val {
            return Some(Err(e));
        }
        let Ok(pres_data_val) = pres_data_val else {
            return None;
        };

        if pres_data_val.is_command() {
            let ctx_id = pres_data_val.ctx_id();
            // Continue readind PDV segments until the full command message can be read into
            // memory. Unless the `MaxLengthItem` is set abnormally small then the command should
            // rarely be split into multiple PDVs.
            let mut is_done = pres_data_val.is_last_fragment();

            let mut buffer = Vec::<u8>::with_capacity(
                usize::try_from(pres_data_item.length()).unwrap_or_default(),
            );
            buffer.append(&mut pres_data_val.into_data());

            while !is_done {
                // TODO: Check for PDU first which might be `PresentationDataItem`?
                //       The standard is terribly unclear on whether the max length applies to the
                //       PDI or the PDV! Given that PDV is the structure that indicates it being
                //       the last fragment or not, it seems that max length applies to PDV.
                let pres_data_val = PresentationDataValue::read(&mut self.stream);
                if let Err(e) = pres_data_val {
                    return Some(Err(e));
                }
                let Ok(pres_data_val) = pres_data_val else {
                    return None;
                };
                is_done = pres_data_val.is_last_fragment();
                buffer.append(&mut pres_data_val.into_data());
            }

            // Command P-DATA always uses IVRLE. Use the standard dicom dictinoary for
            // parsing, otherwise anything pulling values from the command dataset will
            // have to explicitly specify the VR to parse values as since they would
            // otherwise all be `UN`.
            let mut buffer = Cursor::new(buffer);
            let mut parser = ParserBuilder::default()
                .state(ParserState::ReadElement)
                .dataset_ts(&ImplicitVRLittleEndian)
                .build(&mut buffer, &STANDARD_DICOM_DICTIONARY);
            match DicomRoot::parse(&mut parser) {
                Ok(Some(cmd_root)) => {
                    return Some(Ok(PduIterItem::CmdMessage(CommandMessage::new(
                        ctx_id, cmd_root,
                    ))));
                }
                Ok(None) => return None,
                Err(source) => return Some(Err(DimseError::ParseError(source))),
            }
        }

        // If it's a DICOM fragment then send it up as-is, allowing the user of this API to
        // manually stitch or forward the data without fully loading it into memory.
        Some(Ok(PduIterItem::Dataset(pres_data_val)))
    }
}

/// Creates an iterator over the PDU input stream, for parsing incoming `PresentationDataItem` and
/// combining them into appropriate Command or Dataset. Any PDU which is not a
/// `PresentationDataItem` will be returned as an error.
pub struct CommandIter<R: Read> {
    ts: TSRef,
    reader: PduIter<R>,
}

impl<R: Read> CommandIter<R> {
    pub fn new(reader: R, ts: TSRef, max_pdu_size: usize) -> Self {
        Self {
            ts,
            reader: PduIter::new(reader, max_pdu_size),
        }
    }
}

impl<R: Read> Iterator for CommandIter<R> {
    type Item = Result<(CommandMessage, Option<DicomRoot>), DimseError>;

    fn next(&mut self) -> Option<Self::Item> {
        let cmd = match self.reader.next() {
            Some(Ok(PduIterItem::Dataset(pdv))) => {
                return Some(Err(DimseError::DimseCmdMissing(Box::new(
                    DimseMsg::Dataset(pdv),
                ))));
            }
            Some(Ok(PduIterItem::Pdu(pdu))) => {
                return Some(Err(DimseError::UnexpectedPduType(pdu.pdu_type())));
            }
            Some(Ok(PduIterItem::CmdMessage(cmd))) => cmd,
            Some(Err(e)) => return Some(Err(e)),
            None => return None,
        };

        if !cmd.has_dataset() {
            return Some(Ok((cmd, None)));
        }

        let mut buf: Vec<u8> = Vec::new();
        loop {
            match self.reader.next() {
                Some(Ok(PduIterItem::Dataset(pdv))) => {
                    let is_last = pdv.is_last_fragment();
                    buf.append(pdv.into_data().as_mut());
                    if is_last {
                        break;
                    }
                }
                Some(Ok(PduIterItem::Pdu(pdu))) => {
                    return Some(Err(DimseError::UnexpectedPduType(pdu.pdu_type())))
                }
                Some(Ok(PduIterItem::CmdMessage(cmd))) => {
                    return Some(Err(DimseError::DimseDicomMissing(Box::new(DimseMsg::Cmd(
                        cmd,
                    )))));
                }
                Some(Err(e)) => return Some(Err(e)),
                None => break,
            }
        }

        let mut buf = Cursor::new(buf);
        let mut parser = ParserBuilder::default()
            .state(ParserState::ReadElement)
            .dataset_ts(self.ts)
            .build(&mut buf, &STANDARD_DICOM_DICTIONARY);
        match DicomRoot::parse(&mut parser) {
            Ok(Some(dcm_root)) => Some(Ok((cmd, Some(dcm_root)))),
            Ok(None) => Some(Ok((cmd, None))),
            Err(source) => Some(Err(DimseError::ParseError(source))),
        }
    }
}

/// Wraps an `Iterator<Item = DicomElement>` converting them into `PresentationDataItem`, while
/// obeying the `MaxLengthItem`.
pub struct PresDataIter<B, I>
where
    B: Borrow<DicomElement>,
    I: Iterator<Item = B>,
{
    finished: bool,
    ctx_id: u8,
    max_payload_size: usize,
    is_command: bool,
    elements: Peekable<I>,
    writer: Writer<Vec<u8>>,
    big_element_data: Vec<u8>,
}

impl<B, I> PresDataIter<B, I>
where
    B: Borrow<DicomElement>,
    I: Iterator<Item = B>,
{
    /// Create a new iterator to wrap the given iterator over elements.
    ///
    /// # Params
    /// `ctx_id` - The `ctx_id` that each resulting `PresentationDataItem` will be associated with.
    /// `max_pdu_size` - The maximum size of each `PresentationDataItem`, in bytes. This should be
    ///                  either zero to indicate no maximum, or a value greater than the sum of the
    ///                  PDI and PDV header sizes.
    /// `is_command` - Whether this is for command messages or for DICOM datasets.
    /// `elements` - The iterator over elements to convert.
    /// `ts` - The transfer syntax that the elements should be written with.
    /// `cs` - The character set for DICOM encoding.
    #[must_use]
    pub fn new(
        ctx_id: u8,
        max_pdu_size: usize,
        is_command: bool,
        elements: I,
        ts: TSRef,
        cs: CSRef,
    ) -> Self {
        let header_size =
            PresentationDataItem::header_byte_size() + PresentationDataValue::header_byte_size();
        let max_pdu_size = if max_pdu_size == 0 {
            usize::MAX - header_size
        } else {
            max_pdu_size - header_size
        };
        let max_payload_size = max_pdu_size
            - PresentationDataItem::header_byte_size()
            - PresentationDataValue::header_byte_size();
        Self {
            finished: false,
            ctx_id,
            max_payload_size,
            is_command,
            elements: elements.peekable(),
            writer: Self::new_writer(ts, cs, max_payload_size),
            big_element_data: Vec::new(),
        }
    }

    #[must_use]
    fn create_pdi(&self, payload: Vec<u8>, is_last: bool) -> PresentationDataItem {
        let msg_header = msg_header(self.is_command, is_last);
        PresentationDataItem::new(vec![PresentationDataValue::new(
            self.ctx_id,
            msg_header,
            payload,
        )])
    }

    #[must_use]
    fn swap_writer(&mut self) -> Vec<u8> {
        let mut tmp = Self::new_writer(self.writer.ts(), self.writer.cs(), self.max_payload_size);
        swap(&mut tmp, &mut self.writer);
        tmp.into_dataset()
    }

    /// Create new writer
    #[must_use]
    fn new_writer(ts: TSRef, cs: CSRef, max_payload_size: usize) -> Writer<Vec<u8>> {
        WriterBuilder::default()
            .ts(ts)
            .cs(cs)
            .state(WriterState::WriteElement)
            .build(Vec::with_capacity(max_payload_size))
    }
}

impl<B, I> Iterator for PresDataIter<B, I>
where
    B: Borrow<DicomElement>,
    I: Iterator<Item = B>,
{
    type Item = PDIResult;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        loop {
            // If a previous iteration placed bytes into this field then the element is too large
            // to fit into a single PresentationDataItem.
            if !self.big_element_data.is_empty() {
                let payload_size = self.max_payload_size.min(self.big_element_data.len());
                let payload = self
                    .big_element_data
                    .drain(0..payload_size)
                    .collect::<Vec<u8>>();
                // If there is no more big element data and no more elements then this is the last
                // chunk.
                let is_last = self.big_element_data.is_empty() && self.elements.peek().is_none();
                if is_last {
                    self.finished = true;
                }
                return Some(Ok(self.create_pdi(payload, is_last)));
            }

            let element = self.elements.next();

            let Some(element) = element else {
                self.finished = true;
                if self.writer.bytes_written() == 0 {
                    return None;
                }

                let buffer = self.swap_writer();
                return Some(Ok(self.create_pdi(buffer, true)));
            };

            let element = element.borrow();
            let elem_size = element.byte_size();
            let bytes_written = usize::try_from(self.writer.bytes_written()).unwrap_or_default();

            // If adding this element to the writer will cause it to exceed the max PDU size then
            // swap out the writer with a new one to write the element to, and return the old one
            // in a new PresentationDataItem.
            if bytes_written + elem_size > self.max_payload_size {
                // Grab any existing elements/bytes that need sent out before moving on to the
                // element that will cause the current writer to overflow.
                let buffer = self.swap_writer();

                // Check if this element would cause a brand new writer to also overflow.
                let big_elem = elem_size > self.max_payload_size;

                // If the writer hasn't written anything and the element's size is still too large
                // then we're in a big element like PixelData. Put the bytes into the leftover
                // field and let the beginning of the loop handle this.
                if big_elem {
                    let write_result = self
                        .writer
                        .write_elements(once(element))
                        .map_err(DimseError::from);
                    if let Err(e) = write_result {
                        self.finished = true;
                        return Some(Err(e));
                    }
                    self.big_element_data = self.swap_writer();

                    if !buffer.is_empty() {
                        // This isn't the last PDI since big_element_data was just prepped for next
                        // iteration.
                        return Some(Ok(self.create_pdi(buffer, false)));
                    }
                    continue;
                }

                // The element will not cause a new writer to overflow.
                let write_result = self
                    .writer
                    .write_elements(once(element))
                    .map_err(DimseError::from);

                // If an error occurred return it immediately, even though there's technically a
                // full PresentationDataItem that can be returned.
                if let Err(e) = write_result {
                    self.finished = true;
                    return Some(Err(e));
                }

                let is_last = self.elements.peek().is_none();
                if is_last {
                    self.finished = true;
                }
                // Return the previous writer buffer.
                return Some(Ok(self.create_pdi(buffer, is_last)));
            }

            // Writing this element should not exceed the max PDU size, loop to the next one.
            let write_result = self
                .writer
                .write_elements(once(element))
                .map_err(DimseError::from);
            if let Err(e) = write_result {
                self.finished = true;
                return Some(Err(e));
            }
        }
    }
}
