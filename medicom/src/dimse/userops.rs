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

use std::io::{Cursor, Read, Write};

use crate::{
    core::{
        charset::CSRef,
        dcmelement::DicomElement,
        dcmobject::DicomRoot,
        defn::{
            constants::tags::FILE_META_GROUP_END, dcmdict::DicomDictionary, tag::Tag, ts::TSRef,
        },
        read::{ParseError, Parser, ParserBuilder, ParserState},
        values::RawValue,
    },
    dict::{
        stdlookup::STANDARD_DICOM_DICTIONARY,
        tags::{QueryRetrieveLevel, SOPClassUID, SOPInstanceUID, SpecificCharacterSet},
        transfer_syntaxes::ImplicitVRLittleEndian,
        uids::{
            PatientRootQueryRetrieveInformationModelFIND,
            PatientRootQueryRetrieveInformationModelGET,
            PatientRootQueryRetrieveInformationModelMOVE,
            StudyRootQueryRetrieveInformationModelFIND, StudyRootQueryRetrieveInformationModelGET,
            StudyRootQueryRetrieveInformationModelMOVE, VerificationSOPClass,
        },
    },
    dimse::{
        assoc::{CommonAssoc, DimseMsg, QueryLevel},
        commands::{messages::CommandMessage, CommandPriority},
        error::{AssocError, DimseError},
        pdus::pduiter::PresDataIter,
        PDIResult,
    },
};

pub enum AssocUserOp {
    Echo(EchoUserOp),
    Find(FindUserOp),
    Get(GetUserOp),
    Move(MoveUserOp),
    Store(StoreUserOp),
}

/// A C-ECHO operation to be managed by an SCU.
pub struct EchoUserOp {
    msg_id: u16,
    is_complete: bool,
}

impl EchoUserOp {
    #[must_use]
    pub fn new(msg_id: u16) -> Self {
        Self {
            msg_id,
            is_complete: false,
        }
    }

    #[must_use]
    pub fn msg_id(&self) -> u16 {
        self.msg_id
    }

    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.is_complete
    }

    /// Create a C-ECHO-RQ request.
    ///
    /// # Errors
    /// - If no presentation context is present for the association, indicating this operation is
    ///   not supported.
    pub fn create_req(&self, assoc: &CommonAssoc) -> Result<CommandMessage, AssocError> {
        let (pres_ctx, _ts) = assoc.get_rq_pres_ctx_and_ts_by_ab(&VerificationSOPClass)?;
        let ctx_id = pres_ctx.ctx_id();
        Ok(CommandMessage::c_echo_req(
            ctx_id,
            self.msg_id,
            VerificationSOPClass.uid(),
        ))
    }

    /// Process a C-ECHO-RP response, marking this operation as completed.
    ///
    /// # Errors
    /// - If the status of the response is not successful.
    pub fn process_rsp<R: Read, W: Write>(
        &mut self,
        _reader: R,
        _writer: W,
        msg: &CommandMessage,
    ) -> Result<(), AssocError> {
        self.is_complete = true;
        if !msg.status().is_success() {
            return Err(AssocError::ab_failure(DimseError::UnexpectedCommandStatus(
                msg.status().clone(),
            )));
        }
        Ok(())
    }
}

/// A C-FIND operation to be managed by an SCU.
pub struct FindUserOp {
    msg_id: u16,
    max_pdu_rcv_size: usize,
    ts: TSRef,
    is_complete: bool,
}

impl FindUserOp {
    #[must_use]
    pub fn new(msg_id: u16, max_pdu_rcv_size: usize) -> Self {
        Self {
            msg_id,
            max_pdu_rcv_size,
            ts: &ImplicitVRLittleEndian,
            is_complete: false,
        }
    }

    #[must_use]
    pub fn msg_id(&self) -> u16 {
        self.msg_id
    }

    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.is_complete
    }

    /// Create a C-FIND-RQ request.
    ///
    /// # Errors
    /// - If the query level specified is not supported by the association.
    pub fn create_req(
        &mut self,
        assoc: &CommonAssoc,
        ql: QueryLevel,
        query: Vec<(&Tag, RawValue)>,
    ) -> Result<(CommandMessage, DicomRoot), AssocError> {
        let sop_class_uid = match ql {
            QueryLevel::Patient => &PatientRootQueryRetrieveInformationModelFIND,
            _ => &StudyRootQueryRetrieveInformationModelFIND,
        };
        let (pres_ctx, ts) = assoc.get_rq_pres_ctx_and_ts_by_ab(sop_class_uid)?;
        self.ts = ts;

        let ctx_id = pres_ctx.ctx_id();
        let cmd = CommandMessage::c_find_req(ctx_id, self.msg_id, sop_class_uid.uid());

        let mut dcm_query = DicomRoot::new_empty(ts, CSRef::default());
        dcm_query.add_child_with_val(&QueryRetrieveLevel, RawValue::of_string(ql.as_str()));
        for (tag, val) in query {
            dcm_query.add_child_with_val(tag, val);
        }
        Ok((cmd, dcm_query))
    }

    /// Process a C-FIND-RP response, marking the operation as completed if the status is not
    /// pending, or no dataset is present for the command. Returns the parsed DICOM object
    /// representing a C-FIND result.
    ///
    /// # Errors
    /// - I/O errors reading from the stream or parsing the DICOM result.
    pub fn process_rsp<R: Read, W: Write>(
        &mut self,
        mut reader: R,
        mut writer: W,
        msg: &CommandMessage,
    ) -> Result<Option<DicomRoot>, AssocError> {
        if !msg.has_dataset() || !msg.status().is_pending() {
            self.is_complete = true;
            return Ok(None);
        }

        // Continually loop reading messages appending the result to a buffer, as the result may
        // exceed this association's max PDU size.
        let mut buf = Vec::<u8>::with_capacity(self.max_pdu_rcv_size);
        loop {
            let next_msg = CommonAssoc::next_msg(&mut reader, &mut writer, self.max_pdu_rcv_size);
            let dimse_msg = next_msg?;
            match dimse_msg {
                DimseMsg::Dataset(pdv) => {
                    let is_last = pdv.is_last_fragment();
                    buf.append(pdv.into_data().as_mut());
                    if is_last {
                        break;
                    }
                }
                DimseMsg::Cmd(cmd) => {
                    return Err(AssocError::ab_failure(DimseError::DimseDicomMissing(
                        Box::new(DimseMsg::Cmd(cmd)),
                    )));
                }
                DimseMsg::CloseMsg(close_msg) => return Err(AssocError::handled_close(close_msg)),
            }
        }

        if buf.is_empty() {
            return Ok(None);
        }

        let mut buf = Cursor::new(buf);
        let mut parser = ParserBuilder::default()
            .state(ParserState::ReadElement)
            .dataset_ts(self.ts)
            .build(&mut buf, &STANDARD_DICOM_DICTIONARY);
        match DicomRoot::parse(&mut parser) {
            Ok(Some(dcm_root)) => Ok(Some(dcm_root)),
            Ok(None) => Err(AssocError::ab_failure(DimseError::ParseError(
                ParseError::GeneralDecodeError("Failed parsing dicom dataset".to_owned()),
            ))),
            Err(err) => Err(AssocError::ab_failure(DimseError::from(err))),
        }
    }
}

/// A C-STORE operation to be managed by an SCU.
pub struct StoreUserOp {
    msg_id: u16,
    max_pdu_snd_size: usize,
    is_complete: bool,
}

impl StoreUserOp {
    #[must_use]
    pub fn new(msg_id: u16, max_pdu_snd_size: usize) -> Self {
        Self {
            msg_id,
            max_pdu_snd_size,
            is_complete: false,
        }
    }

    #[must_use]
    pub fn msg_id(&self) -> u16 {
        self.msg_id
    }

    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.is_complete
    }

    /// Create a C-STORE-RQ request. Returns the command message to send, followed by an iterator
    /// that will produce PDIs of appropriate size from the DICOM SOP payload.
    ///
    /// # Errors
    /// - Any required elements are missing: `SpecificCharacterSet`, `SOPClassUID`, or
    ///   `SOPInstanceUID`.
    /// - I/O errors reading from the parser.
    pub fn create_req<'p, PR: Read + 'p>(
        &self,
        assoc: &CommonAssoc,
        parser: Parser<'p, PR>,
        store_msg_id: u16,
        origin_ae: &str,
        orig_msg_id: u16,
    ) -> Result<(CommandMessage, Box<dyn Iterator<Item = PDIResult> + 'p>), AssocError> {
        let mut parser = parser
            .filter_map(Result::ok)
            // Do not transfer any beginning FileMeta elements.
            .skip_while(|e| e.tag() <= FILE_META_GROUP_END);

        // Pull out all necessary early tags for processing the remaining DICOM elements of this
        // SOP.
        let mut spec_char_set: Option<String> = None;
        let mut sop_class_uid: Option<String> = None;
        let mut sop_inst_uid: Option<String> = None;
        let mut header_elems: Vec<DicomElement> = Vec::new();
        for elem in parser.by_ref() {
            let tag = elem.tag();
            if tag == SpecificCharacterSet.tag() {
                spec_char_set = elem
                    .parse_value()
                    .map_err(|e| AssocError::ab_failure(DimseError::ParseError(e)))?
                    .string()
                    .cloned();
            } else if tag == SOPClassUID.tag() {
                sop_class_uid = elem
                    .parse_value()
                    .map_err(|e| AssocError::ab_failure(DimseError::ParseError(e)))?
                    .string()
                    .cloned();
            } else if tag == SOPInstanceUID.tag() {
                sop_inst_uid = elem
                    .parse_value()
                    .map_err(|e| AssocError::ab_failure(DimseError::ParseError(e)))?
                    .string()
                    .cloned();
            }
            header_elems.push(elem);
            if tag >= SOPInstanceUID.tag() {
                break;
            }
        }

        // Chain the header elements parsed above with the parser to reconstitute the DICOM SOP.
        let stitched_elems = header_elems.into_iter().chain(parser);

        let spec_char_set = spec_char_set
            .and_then(|s| CSRef::lookup_charset(&s))
            .unwrap_or_default();
        let sop_class_uid = sop_class_uid
            .and_then(|s| STANDARD_DICOM_DICTIONARY.get_uid_by_uid(&s))
            .ok_or_else(|| {
                AssocError::ab_failure(DimseError::DimseElementMissing(
                    SOPClassUID.ident().to_owned(),
                ))
            })?;
        let sop_inst_uid = sop_inst_uid.ok_or_else(|| {
            AssocError::ab_failure(DimseError::DimseElementMissing(
                SOPInstanceUID.ident().to_owned(),
            ))
        })?;

        let (pres_ctx, ts) = assoc.get_rq_pres_ctx_and_ts_by_ab(sop_class_uid)?;
        let ctx_id = pres_ctx.ctx_id();
        let priority = CommandPriority::Medium;
        let cmd = CommandMessage::c_store_req(
            ctx_id,
            store_msg_id,
            &priority,
            sop_class_uid.uid(),
            &sop_inst_uid,
            origin_ae,
            orig_msg_id,
        );

        let pdi_iter = Box::new(PresDataIter::new(
            ctx_id,
            self.max_pdu_snd_size,
            false,
            stitched_elems,
            ts,
            spec_char_set,
        ));

        Ok((cmd, pdi_iter))
    }

    /// Process a C-STORE-RP response, marking this operation as completed if the status is not
    /// pending.
    ///
    /// # Errors
    /// - None, this returns a `Result` for consistency with other operations.
    pub fn process_rsp(&mut self, msg: &CommandMessage) -> Result<(), AssocError> {
        self.is_complete |= !msg.status().is_pending();
        Ok(())
    }
}

/// A C-GET operation to be managed by an SCU.
pub struct GetUserOp {
    msg_id: u16,
    is_complete: bool,
}

impl GetUserOp {
    #[must_use]
    pub fn new(msg_id: u16) -> Self {
        Self {
            msg_id,
            is_complete: false,
        }
    }

    #[must_use]
    pub fn msg_id(&self) -> u16 {
        self.msg_id
    }

    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.is_complete
    }

    /// Createa a C-GET-RQ request. Returns the command and dataset to be sent.
    ///
    /// # Errors
    /// - If the association has no matching support for the query level specified.
    pub fn create_req(
        &mut self,
        assoc: &CommonAssoc,
        ql: QueryLevel,
        query: Vec<(&Tag, RawValue)>,
    ) -> Result<(CommandMessage, DicomRoot), AssocError> {
        let sop_class_uid = match ql {
            QueryLevel::Patient => &PatientRootQueryRetrieveInformationModelGET,
            _ => &StudyRootQueryRetrieveInformationModelGET,
        };
        let (pres_ctx, ts) = assoc.get_rq_pres_ctx_and_ts_by_ab(sop_class_uid)?;

        let ctx_id = pres_ctx.ctx_id();
        let cmd = CommandMessage::c_get_req(ctx_id, self.msg_id, sop_class_uid.uid());

        let mut dcm_query = DicomRoot::new_empty(ts, CSRef::default());
        dcm_query.add_child_with_val(&QueryRetrieveLevel, RawValue::of_string(ql.as_str()));
        for (tag, val) in query {
            dcm_query.add_child_with_val(tag, val);
        }

        Ok((cmd, dcm_query))
    }

    /// Process a C-GET-RP response. If the status is not pending then this operation will be
    /// marked as completed.
    ///
    /// # Errors
    /// - None, this returns a `Result` for consistency with other operations.
    pub fn process_rsp<R: Read, W: Write>(
        &mut self,
        mut _reader: R,
        mut _writer: W,
        msg: &CommandMessage,
    ) -> Result<(), AssocError> {
        self.is_complete |= !msg.status().is_pending();
        Ok(())
    }
}

/// A C-MOVE operation to be managed by an SCU.
pub struct MoveUserOp {
    msg_id: u16,
    is_complete: bool,
}

impl MoveUserOp {
    #[must_use]
    pub fn new(msg_id: u16) -> Self {
        Self {
            msg_id,
            is_complete: false,
        }
    }

    #[must_use]
    pub fn msg_id(&self) -> u16 {
        self.msg_id
    }

    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.is_complete
    }

    /// Create a C-MOVE-RQ request. Returns the command and query to be sent.
    ///
    /// # Errors
    /// - If the association does not support the query level for this move operation.
    pub fn create_req(
        &mut self,
        assoc: &CommonAssoc,
        dest_ae: &str,
        ql: QueryLevel,
        query: Vec<(&Tag, RawValue)>,
    ) -> Result<(CommandMessage, DicomRoot), AssocError> {
        let sop_class_uid = if ql == QueryLevel::Patient {
            &PatientRootQueryRetrieveInformationModelMOVE
        } else {
            &StudyRootQueryRetrieveInformationModelMOVE
        };

        let (pres_ctx, ts) = assoc.get_rq_pres_ctx_and_ts_by_ab(sop_class_uid)?;
        let ctx_id = pres_ctx.ctx_id();
        let cmd = CommandMessage::c_move_req(ctx_id, self.msg_id, sop_class_uid.uid(), dest_ae);

        let mut dcm_root = DicomRoot::new_empty(ts, CSRef::default());
        dcm_root.add_child_with_val(&QueryRetrieveLevel, RawValue::of_string(ql.as_str()));
        for (tag, val) in query {
            dcm_root.add_child_with_val(tag, val);
        }

        Ok((cmd, dcm_root))
    }

    /// Process a C-MOVE-RP response. If the status is not pending then this operation will be
    /// marked as completed.
    ///
    /// # Errors
    /// - None, this returns a `Result` for consistency with other operations.
    pub fn process_rsp<R: Read, W: Write>(
        &mut self,
        mut _reader: R,
        mut _writer: W,
        msg: &CommandMessage,
    ) -> Result<(), AssocError> {
        self.is_complete |= !msg.status().is_pending();
        Ok(())
    }
}
