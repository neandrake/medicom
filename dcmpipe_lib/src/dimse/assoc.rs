/*
   Copyright 2024 Christopher Speck

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
    collections::{HashMap, HashSet},
    io::{Cursor, Read, Write},
    str::FromStr,
};

use crate::{
    core::{
        charset::DEFAULT_CHARACTER_SET,
        dcmobject::DicomRoot,
        defn::{dcmdict::DicomDictionary, ts::TSRef, uid::UIDRef},
        read::{ParseError, ParserBuilder, ParserState},
    },
    dict::{stdlookup::STANDARD_DICOM_DICTIONARY, transfer_syntaxes::ImplicitVRLittleEndian},
    dimse::{
        commands::messages::CommandMessage,
        error::{AssocError, DimseError},
        pdus::{
            mainpdus::{
                Abort, AssocACPresentationContext, AssocRJ, PresentationDataValue, ReleaseRP,
            },
            pduiter::{read_next_pdu, PduIterItem, PresDataIter},
            userpdus::{AsyncOperationsWindowItem, MaxLengthItem},
            Pdu, UserPdu,
        },
        Syntax,
    },
};

pub mod scp;
pub mod scu;

#[derive(Debug)]
pub enum DimseMsg {
    Cmd(CommandMessage),
    Dataset(PresentationDataValue),
    ReleaseRQ,
    ReleaseRP,
    Reject(AssocRJ),
    Abort(Abort),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum QueryLevel {
    Patient,
    Study,
    Series,
    Image,
}

impl QueryLevel {
    #[must_use]
    pub fn as_str(&self) -> &str {
        match &self {
            Self::Patient => "PATIENT",
            Self::Study => "STUDY",
            Self::Series => "SERIES",
            Self::Image => "IMAGE",
        }
    }

    #[must_use]
    pub fn include_patient_tags(&self) -> bool {
        true
    }

    #[must_use]
    pub fn include_study_tags(&self) -> bool {
        self != &QueryLevel::Patient
    }
}

impl std::fmt::Display for QueryLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl FromStr for QueryLevel {
    type Err = ParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "PATIENT" => Ok(Self::Patient),
            "STUDY" => Ok(Self::Study),
            "SERIES" => Ok(Self::Series),
            "IMAGE" => Ok(Self::Image),
            other => Err(ParseError::GeneralDecodeError(format!(
                "Invalid Query Level: {other}"
            ))),
        }
    }
}

pub struct CommonAssoc {
    /* Fields configured by this SCU. */
    _id: usize,
    this_ae: String,
    supported_abs: HashSet<UIDRef>,
    supported_ts: HashSet<TSRef>,
    this_user_data: Vec<UserPdu>,

    /* Fields negotiated with other SCU. */
    their_user_data: Vec<UserPdu>,
    negotiated_pres_ctx: HashMap<u8, AssocACPresentationContext>,
}

impl CommonAssoc {
    #[must_use]
    pub fn this_ae(&self) -> &String {
        &self.this_ae
    }

    #[must_use]
    pub fn supported_abs(&self) -> &HashSet<UIDRef> {
        &self.supported_abs
    }

    #[must_use]
    pub fn supported_ts(&self) -> &HashSet<TSRef> {
        &self.supported_ts
    }

    /// Write the given PDU to the given writer.
    ///
    /// # Errors
    /// - I/O errors may occur when writing to the writer or flushing the writer.
    pub fn write_pdu<W: Write>(&self, pdu: &Pdu, mut writer: &mut W) -> Result<(), AssocError> {
        pdu.write(&mut writer).map_err(AssocError::error)?;
        writer
            .flush()
            .map_err(|e| AssocError::error(DimseError::IOError(e)))?;
        Ok(())
    }

    /// Retrieve the accepted presentation context by the given context ID.
    #[must_use]
    pub fn get_pres_ctx(&self, ctx_id: u8) -> Option<&AssocACPresentationContext> {
        self.negotiated_pres_ctx.get(&ctx_id)
    }

    /// Retrieve the accepted presentation context and its negotiated transfer syntax, by the given
    /// context ID.
    ///
    /// # Errors
    /// - `AssocError` may occur if the negotiated presentation context cannot be resolved, or if a
    /// known transfer syntax for it cannot be resolved.
    pub fn get_pres_ctx_and_ts(
        &self,
        ctx_id: u8,
    ) -> Result<(&AssocACPresentationContext, TSRef), AssocError> {
        let Some(pres_ctx) = self.get_pres_ctx(ctx_id) else {
            return Err(AssocError::ab_failure(DimseError::GeneralError(format!(
                "Negotiated Presentation Context ID not found: {ctx_id}"
            ))));
        };

        let ts = String::try_from(&Syntax(pres_ctx.transfer_syntax().transfer_syntaxes()))
            .ok()
            .and_then(|v| STANDARD_DICOM_DICTIONARY.get_ts_by_uid(&v))
            .ok_or_else(|| {
                AssocError::ab_failure(DimseError::GeneralError(
                    "Failed to resolve transfer syntax".to_string(),
                ))
            })?;

        Ok((pres_ctx, ts))
    }

    /// Gets the `MaxLengthItem` that the other SCU indicated should be the largest
    /// `PresentationDataItem` transferred. A value of zero indicates no limit.
    #[must_use]
    pub fn get_pdu_max_snd_size(&self) -> usize {
        for user_pdu in &self.their_user_data {
            if let UserPdu::MaxLengthItem(mli) = user_pdu {
                return usize::try_from(mli.max_length()).unwrap_or_default();
            }
        }
        0
    }

    /// Gets the `MaxLengthItem` configured by this SCU. If the other/connected SCU is properly
    /// obeying this then no `PresentationDataItem` exceeding this size should be received. A value
    /// of zero indicates no limit.
    #[must_use]
    pub fn get_pdu_max_rcv_size(&self) -> usize {
        for user_pdu in &self.this_user_data {
            if let UserPdu::MaxLengthItem(mli) = user_pdu {
                return usize::try_from(mli.max_length()).unwrap_or_default();
            }
        }
        0
    }

    /// Gets the `AsyncOperationsWindowItem` that the other SCU indicated is acceptable for
    /// transmitting PDUs for different operations within the same association.
    #[must_use]
    pub fn get_their_async_config(&self) -> Option<&AsyncOperationsWindowItem> {
        self.their_user_data.iter().find_map(|i| {
            if let UserPdu::AsyncOperationsWindowItem(async_item) = i {
                Some(async_item)
            } else {
                None
            }
        })
    }

    /// Gets the `AsyncOperationsWindowItem` that this SCU indicated is acceptable for receiving
    /// PDUs for different operations within the same association.
    #[must_use]
    pub fn get_this_async_config(&self) -> Option<&AsyncOperationsWindowItem> {
        self.this_user_data.iter().find_map(|i| {
            if let UserPdu::AsyncOperationsWindowItem(async_item) = i {
                Some(async_item)
            } else {
                None
            }
        })
    }

    /// Parse the next message, either a Command, DICOM Dataset, release/abort, or unexpected PDU.
    ///
    /// # Errors
    /// - I/O errors may occcur attempting to read PDU from the reader, or write an appropriate
    /// disconnect response to the writer.
    pub fn next_msg<R: Read, W: Write>(
        &self,
        reader: &mut R,
        writer: &mut W,
    ) -> Result<DimseMsg, AssocError> {
        match read_next_pdu(reader, self.get_pdu_max_rcv_size()) {
            Some(Ok(PduIterItem::Pdu(pdu))) => self.handle_disconnect(pdu, writer),
            Some(Ok(PduIterItem::CmdMessage(cmd))) => Ok(DimseMsg::Cmd(cmd)),
            Some(Ok(PduIterItem::Dataset(dataset))) => Ok(DimseMsg::Dataset(dataset)),
            Some(Err(err)) => Err(AssocError::ab_failure(err)),
            None => Err(AssocError::ab_failure(DimseError::GeneralError(
                "No DIMSE message received".to_owned(),
            ))),
        }
    }

    /// Continuously reads DICOM `PresentationDataValue` PDUs from the reader and writes the bytes
    /// to the given `out_writer`, stopping after processing the last fragment.
    ///
    /// # Parameters
    /// `reader` - The reader the `PresentationDataValue` PDUs will be read from.
    /// `writer` - The protocol's corresponding writer for sending A-ABORT if encountering errors.
    /// `out_writer` - The destination to write the `PresentationDataValue` data bytes to.
    ///
    /// # Errors
    /// - I/O errors may occur with the reader/writer.
    pub fn read_dataset<R: Read, W: Write, OW: Write>(
        &self,
        mut reader: &mut R,
        mut writer: &mut W,
        out_writer: &mut OW,
    ) -> Result<(), AssocError> {
        let mut all_read = false;
        while !all_read {
            let dcm_msg = self.next_msg(&mut reader, &mut writer)?;
            let DimseMsg::Dataset(pdv) = dcm_msg else {
                return Err(AssocError::ab_failure(DimseError::GeneralError(
                    "Expected DICOM dataset".to_string(),
                )));
            };

            all_read = pdv.is_last_fragment();
            out_writer
                .write_all(pdv.data())
                .map_err(|e| AssocError::ab_failure(DimseError::IOError(e)))?;
        }

        Ok(())
    }

    /// Reads a dataset into an in-memory buffer and parses it to a `DicomRoot`. Be cautious using
    /// this, as it is only meant for datasets that are expected to be rather small, duch as C-FIND
    /// queries.
    ///
    /// # Errors
    /// - I/O errors may occur with reader/writer.
    /// - `DimseError` may occur in parsing DICOM/DIMSE.
    pub fn read_dataset_in_mem<R: Read, W: Write>(
        &self,
        mut reader: &mut R,
        writer: &mut W,
        ts: TSRef,
    ) -> Result<DicomRoot, AssocError> {
        // Read bytes into vec, then wrap in cursor. Wrap in cursor after reading otherwise the
        // cursor's position will be the end of the vec (alternatively could set_position(0)).
        let mut dcm_bytes: Vec<u8> = Vec::new();
        self.read_dataset(&mut reader, writer, &mut dcm_bytes)?;
        let dcm_bytes = Cursor::new(dcm_bytes);
        let mut parser = ParserBuilder::default()
            .state(ParserState::ReadElement)
            .dataset_ts(ts)
            .build(dcm_bytes, &STANDARD_DICOM_DICTIONARY);
        DicomRoot::parse(&mut parser)
            .map_err(|e| AssocError::ab_failure(DimseError::ParseError(e)))?
            .ok_or_else(|| {
                AssocError::ab_failure(DimseError::GeneralError(
                    "Parsing dicom query did not result in any query".to_owned(),
                ))
            })
    }

    /// Writes the given command, chunking into `PresentationDataItem`'s based on the SCU's
    /// indicated `MaxLengthItem`.
    ///
    /// # Errors
    /// - I/O errors may occur when writing to the stream.
    /// - Parsing/encoding errors may occur when serializing the given command to
    /// `PresentationDataItem`s.
    pub fn write_command<W: Write>(
        &mut self,
        cmd: &CommandMessage,
        mut writer: &mut W,
    ) -> Result<(), AssocError> {
        let elements = cmd.message().flatten();
        let pdi_iter = PresDataIter::new(
            cmd.ctx_id(),
            self.get_pdu_max_snd_size(),
            true,
            elements.iter().copied(),
            &ImplicitVRLittleEndian,
            DEFAULT_CHARACTER_SET,
        );
        for pdi in pdi_iter {
            match pdi {
                Ok(pdi) => self.write_pdu(&Pdu::PresentationDataItem(pdi), &mut writer)?,
                Err(e) => return Err(AssocError::ab_failure(e)),
            }
        }
        Ok(())
    }

    /// Writes the given dataset, chunking into `PresentationDataItem`s based on the SCU's
    /// indicated `MaxLengthItem`.
    ///
    /// # Errors
    /// - I/O errors may occur when writing to the stream.
    /// - Parsing/encoding errors may occur when serializing the given dataset to
    /// `PresentationDataItem`s.
    pub fn write_dataset<W: Write>(
        &mut self,
        ctx_id: u8,
        dataset: &DicomRoot,
        mut writer: &mut W,
    ) -> Result<(), AssocError> {
        let elements = dataset.flatten();
        let pdi_iter = PresDataIter::new(
            ctx_id,
            self.get_pdu_max_snd_size(),
            false,
            elements.iter().copied(),
            dataset.ts(),
            dataset.cs(),
        );
        for pdi in pdi_iter {
            match pdi {
                Ok(pdi) => self.write_pdu(&Pdu::PresentationDataItem(pdi), &mut writer)?,
                Err(e) => return Err(AssocError::ab_failure(e)),
            }
        }
        Ok(())
    }

    /// Handles a PDU that is not a `PresentationDataItem`, after the association is negotiated. In
    /// this scenario the only valid PDUs are `ReleaseRQ` or `Abort`.
    ///
    /// # Errors
    /// - I/O errors may occur when writing to the stream.
    /// - `DimseError`s may occur if the response is unexpected.
    fn handle_disconnect<W: Write>(
        &self,
        pdu: Pdu,
        writer: &mut W,
    ) -> Result<DimseMsg, AssocError> {
        match pdu {
            Pdu::ReleaseRQ(_rq) => {
                self.write_pdu(&Pdu::ReleaseRP(ReleaseRP::new()), writer)?;
                Ok(DimseMsg::ReleaseRQ)
            }
            Pdu::ReleaseRP(_rp) => Ok(DimseMsg::ReleaseRP),
            Pdu::Abort(ab) => Ok(DimseMsg::Abort(ab)),
            other => {
                self.write_pdu(&Pdu::Abort(Abort::new(2, 2)), writer)?;
                Err(AssocError::error(DimseError::UnexpectedPduType(
                    other.pdu_type(),
                )))
            }
        }
    }
}

#[derive(Default)]
pub struct CommonAssocBuilder {
    id: usize,
    this_ae: String,
    supported_abs: HashSet<UIDRef>,
    supported_ts: HashSet<TSRef>,
    pdu_rcv_max_len: u32,
}

impl CommonAssocBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn id(mut self, id: usize) -> Self {
        self.id = id;
        self
    }

    #[must_use]
    pub fn this_ae(mut self, this_ae: String) -> Self {
        self.this_ae = this_ae;
        self
    }

    #[must_use]
    pub fn supported_abs(mut self, accept_abs: HashSet<UIDRef>) -> Self {
        self.supported_abs = accept_abs;
        self
    }

    #[must_use]
    pub fn supported_ts(mut self, accept_ts: HashSet<TSRef>) -> Self {
        self.supported_ts = accept_ts;
        self
    }

    #[must_use]
    pub fn pdu_rcv_max_len(mut self, pdu_rcv_max_len: u32) -> Self {
        self.pdu_rcv_max_len = pdu_rcv_max_len;
        self
    }

    #[must_use]
    pub fn build(self) -> CommonAssoc {
        let num_abs = self.supported_abs.len();
        // MaxLengthItem + AsyncOperationsWindowItem + RoleSelectionItem * supported_abs.len()
        let num_user_data = 2 + self.supported_abs.len();
        let mut this_user_data = Vec::<UserPdu>::with_capacity(num_user_data);
        this_user_data.push(UserPdu::MaxLengthItem(MaxLengthItem::new(
            self.pdu_rcv_max_len,
        )));

        // Require synchronous transfers, until async is incorporated.
        this_user_data.push(UserPdu::AsyncOperationsWindowItem(
            AsyncOperationsWindowItem::new(1, 1),
        ));

        CommonAssoc {
            _id: self.id,
            this_ae: self.this_ae,
            supported_abs: self.supported_abs,
            supported_ts: self.supported_ts,
            this_user_data,

            their_user_data: Vec::with_capacity(num_user_data),
            negotiated_pres_ctx: HashMap::with_capacity(num_abs),
        }
    }
}
