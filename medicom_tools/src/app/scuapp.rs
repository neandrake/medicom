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
    collections::HashSet,
    fs::File,
    io::{stdout, BufReader, BufWriter, Write},
    net::TcpStream,
};

use medicom::{
    core::{
        dcmobject::DicomRoot,
        defn::{
            constants::ts::{ExplicitVRLittleEndian, ImplicitVRLittleEndian},
            dcmdict::DicomDictionary,
            tag::{Tag, TagNode},
            vr::LT,
        },
        inspect::FormattedElement,
        read::{valdecode::StringAndVr, ParserBuilder},
        values::RawValue,
    },
    dict::{
        stdlookup::STANDARD_DICOM_DICTIONARY,
        tags::AffectedSOPClassUID,
        uids::{
            CTImageStorage, DeformableSpatialRegistrationStorage, MRImageStorage,
            ModalityWorklistInformationModelFIND, NuclearMedicineImageStorage,
            PatientRootQueryRetrieveInformationModelFIND,
            PatientRootQueryRetrieveInformationModelGET,
            PatientRootQueryRetrieveInformationModelMOVE, PositronEmissionTomographyImageStorage,
            RTDoseStorage, RTImageStorage, RTPlanStorage, RTStructureSetStorage, RawDataStorage,
            SecondaryCaptureImageStorage, SpatialRegistrationStorage,
            StudyRootQueryRetrieveInformationModelFIND, StudyRootQueryRetrieveInformationModelGET,
            StudyRootQueryRetrieveInformationModelMOVE, VerificationSOPClass,
        },
    },
    dimse::{
        assoc::{
            scu::{UserAssoc, UserAssocBuilder},
            CommonAssoc, DimseMsg, QueryLevel,
        },
        commands::{messages::CommandMessage, CommandStatus, CommandType, SubOpProgress},
        error::{AssocError, DimseError},
        userops::AssocUserOp,
    },
};

use crate::{
    app::{handle_assoc_result, rename_file_to_sop},
    args::{SvcUserArgs, SvcUserCommand},
    CommandApplication,
};

pub struct SvcUserApp {
    args: SvcUserArgs,
}

impl SvcUserApp {
    pub fn new(args: SvcUserArgs) -> SvcUserApp {
        SvcUserApp { args }
    }

    fn print_progress(cmd: &CommandMessage, title: &str) {
        let progress = SubOpProgress::from(cmd);
        println!("{title}: {}/{}", progress.completed(), progress.total());
    }

    fn print_cfind_result(
        result: Option<DicomRoot>,
        cmd: &CommandMessage,
    ) -> Result<(), AssocError> {
        let mut stdout = stdout().lock();
        match result {
            Some(dcm) => {
                stdout
                    .write_all("### Result\n".as_bytes())
                    .map_err(|e| AssocError::ab_failure(DimseError::from(e)))?;
                let elems = dcm
                    .flatten()
                    .iter()
                    .map(|o| FormattedElement::new(o))
                    .collect::<Vec<FormattedElement>>();
                for elem in elems {
                    stdout
                        .write_all(format!("{elem}\n").as_ref())
                        .map_err(|e| AssocError::ab_failure(DimseError::from(e)))?;
                }
                stdout
                    .write_all("\n".to_owned().as_ref())
                    .map_err(|e| AssocError::ab_failure(DimseError::from(e)))?;
            }
            None => {
                stdout
                    .write_all(format!("### End Results: {:?}\n", cmd.status()).as_ref())
                    .map_err(|e| AssocError::ab_failure(DimseError::from(e)))?;
            }
        }
        Ok(())
    }

    fn process_cstore_req(
        msg: &CommandMessage,
        assoc: &mut UserAssoc,
        mut reader: &mut BufReader<&TcpStream>,
        mut writer: &mut BufWriter<&TcpStream>,
    ) -> Result<Option<DimseMsg>, AssocError> {
        let cstore_ctx_id = msg.ctx_id();
        let cstore_msg_id = msg.msg_id();
        let cstore_aff_sop = msg.get_string(&AffectedSOPClassUID).unwrap_or_default();

        let filename = "rcv_dcm.tmp";
        let mut output = BufWriter::with_capacity(
            1024 * 1024,
            File::create(filename).map_err(|e| AssocError::ab_failure(DimseError::from(e)))?,
        );
        // TODO: Write FileMeta

        match CommonAssoc::next_msg(
            &mut reader,
            &mut writer,
            assoc.common().get_pdu_max_rcv_size(),
        )? {
            DimseMsg::Dataset(pdv) => {
                output
                    .write_all(pdv.data())
                    .map_err(|e| AssocError::ab_failure(DimseError::from(e)))?;

                if pdv.is_last_fragment() {
                    output
                        .flush()
                        .map_err(|e| AssocError::ab_failure(DimseError::from(e)))?;
                    let file = output.into_inner().map_err(|e| {
                        AssocError::ab_failure(DimseError::ApplicationError(e.into()))
                    })?;
                    file.sync_all()
                        .map_err(|e| AssocError::ab_failure(DimseError::from(e)))?;
                    drop(file);

                    let (_pres_ctx, ts) = assoc.common().get_pres_ctx_and_ts(cstore_ctx_id)?;

                    rename_file_to_sop(filename, ts).map_err(AssocError::ab_failure)?;

                    let cmd = CommandMessage::c_store_rsp(
                        cstore_ctx_id,
                        cstore_msg_id,
                        &cstore_aff_sop,
                        &CommandStatus::success(),
                    );
                    CommonAssoc::write_command(
                        &cmd,
                        writer,
                        assoc.common().get_pdu_max_snd_size(),
                    )?;
                }
            }
            other => {
                return Err(AssocError::error(DimseError::DimseDicomMissing(Box::new(
                    other,
                ))));
            }
        }

        let _ = std::fs::remove_file(filename);

        assoc.release_association(&mut reader, &mut writer)
    }

    fn resolve_cli_query<'e>(
        query: &'e [(String, String)],
    ) -> Result<Vec<(&'e Tag, RawValue<'e>)>, AssocError> {
        let mut query_vals_resolved: Vec<(&'e Tag, RawValue<'e>)> = Vec::with_capacity(query.len());
        for (tag, val) in query {
            let tag = TagNode::parse(tag, Some(&STANDARD_DICOM_DICTIONARY))
                .map_err(|e| AssocError::error(DimseError::from(e)))
                .map(|t| STANDARD_DICOM_DICTIONARY.get_tag_by_number(t.tag()))?
                .ok_or_else(|| {
                    AssocError::error(DimseError::ApplicationError(
                        format!("Unable resolve tag: {tag}").into(),
                    ))
                })?;
            let val = RawValue::try_from(StringAndVr(val, tag.implicit_vr().unwrap_or(&LT)))
                .map_err(|e| AssocError::error(DimseError::from(e)))?;
            query_vals_resolved.push((tag, val.clone()));
        }
        Ok(query_vals_resolved)
    }
}

impl CommandApplication for SvcUserApp {
    fn run(&mut self) -> anyhow::Result<()> {
        let supported_abs = HashSet::from([
            &VerificationSOPClass,
            &PatientRootQueryRetrieveInformationModelFIND,
            &StudyRootQueryRetrieveInformationModelFIND,
            &ModalityWorklistInformationModelFIND,
            &PatientRootQueryRetrieveInformationModelMOVE,
            &StudyRootQueryRetrieveInformationModelMOVE,
            &PatientRootQueryRetrieveInformationModelGET,
            &StudyRootQueryRetrieveInformationModelGET,
            &CTImageStorage,
            &MRImageStorage,
            &RTImageStorage,
            &PositronEmissionTomographyImageStorage,
            &NuclearMedicineImageStorage,
            &SecondaryCaptureImageStorage,
            &RTStructureSetStorage,
            &RTDoseStorage,
            &RTPlanStorage,
            &RawDataStorage,
            &SpatialRegistrationStorage,
            &DeformableSpatialRegistrationStorage,
        ]);
        let supported_ts = HashSet::from([&ImplicitVRLittleEndian, &ExplicitVRLittleEndian]);
        let max_pdu_size = self
            .args
            .max_pdu_size
            .and_then(|s| u32::try_from(s).ok())
            .unwrap_or(0);
        let mut assoc = UserAssocBuilder::default()
            .id(0)
            .my_ae(self.args.my_ae.clone())
            .service_ae(self.args.host_ae.clone())
            .supported_abs(supported_abs)
            .supported_ts(supported_ts)
            .pdu_rcv_max_len(max_pdu_size)
            .build();

        let stream = TcpStream::connect(&self.args.host)?;
        let mut reader = BufReader::new(&stream);
        let mut writer = BufWriter::new(&stream);
        let result = self.start(&mut reader, &mut writer, &mut assoc);
        match result {
            Ok(None) => {}
            Ok(Some(res)) => {
                let output = handle_assoc_result(Ok(res), &mut writer);
                for line in output {
                    match line {
                        Ok(line) => println!("{line}"),
                        Err(line) => eprintln!("{line}"),
                    }
                }
            }
            Err(e) => {
                let _ = e.write(&mut writer);
                eprintln!("[ err xx] {}", e.err());
            }
        }
        Ok(())
    }
}

impl SvcUserApp {
    #[allow(clippy::too_many_lines)]
    fn start(
        &self,
        mut reader: &mut BufReader<&TcpStream>,
        mut writer: &mut BufWriter<&TcpStream>,
        assoc: &mut UserAssoc,
    ) -> Result<Option<DimseMsg>, AssocError> {
        let rsp = assoc.request_association(&mut reader, &mut writer)?;
        if let Some(rsp) = rsp {
            return Ok(Some(rsp));
        }

        match &self.args.cmd {
            SvcUserCommand::Echo => {
                let msg_id = assoc.next_msg_id();
                assoc.common_mut().send_cecho_req(&mut writer, msg_id)?;
            }
            SvcUserCommand::Find { query_level, query } => {
                let msg_id = assoc.next_msg_id();
                let query_vals_resolved = Self::resolve_cli_query(query)?;
                assoc.common_mut().send_cfind_req(
                    &mut writer,
                    msg_id,
                    *query_level,
                    query_vals_resolved,
                )?;
            }
            SvcUserCommand::Store { file } => {
                for f in file {
                    let file = BufReader::new(
                        File::open(f).map_err(|e| AssocError::ab_failure(DimseError::from(e)))?,
                    );
                    let this_ae = assoc.common().this_ae().clone();
                    let store_msg_id = assoc.next_msg_id();
                    let parser = ParserBuilder::default().build(file, &STANDARD_DICOM_DICTIONARY);
                    assoc.common_mut().c_store_req(
                        &mut reader,
                        &mut writer,
                        parser,
                        store_msg_id,
                        &this_ae,
                        store_msg_id,
                    )?;
                }
            }
            SvcUserCommand::Move {
                dest_ae,
                query_level,
                query,
            } => {
                let msg_id = assoc.next_msg_id();
                let query_level = query_level.unwrap_or(QueryLevel::Study);
                let query_vals_resolved = Self::resolve_cli_query(query)?;
                assoc.common_mut().c_move_req(
                    &mut writer,
                    msg_id,
                    dest_ae,
                    query_level,
                    query_vals_resolved,
                )?;
            }
            SvcUserCommand::Get { query_level, query } => {
                let msg_id = assoc.next_msg_id();
                let query_level = query_level.unwrap_or(QueryLevel::Study);
                let query_vals_resolved = Self::resolve_cli_query(query)?;
                assoc.common_mut().c_get_req(
                    &mut writer,
                    msg_id,
                    query_level,
                    query_vals_resolved,
                )?;
            }
        }

        loop {
            let cmd = CommonAssoc::next_cmd(
                &mut reader,
                &mut writer,
                assoc.common().get_pdu_max_rcv_size(),
            )?;
            let msg_id = cmd.msg_id();

            // When issuing a C-GET requests the SCP will respond with a C-STORE request.
            if cmd.cmd_type() == &CommandType::CStoreReq {
                // SCP will respond with individual C-STORE requests for each SOP.
                Self::process_cstore_req(&cmd, assoc, reader, writer)?;
                continue;
            } else if cmd.cmd_type() == &CommandType::CGetRsp && cmd.status().is_pending() {
                // SCP may respond with status updates. As long as the status is "pending" this
                // should continue to expect C-STORE requests.
                let progress = SubOpProgress::from(&cmd);
                println!(
                    "C-GET Progress: {}/{}",
                    progress.0,
                    progress.0 + progress.1 + progress.2 + progress.3
                );
                continue;
            }

            let Some(op) = assoc.common_mut().get_user_op(msg_id) else {
                return Err(AssocError::ab_failure(DimseError::UnknownMessageID(msg_id)));
            };

            let is_complete = match op {
                AssocUserOp::Echo(op) => {
                    op.process_rsp(&mut reader, &mut writer, &cmd)?;
                    op.is_complete()
                }
                AssocUserOp::Find(op) => {
                    let result = op.process_rsp(&mut reader, &mut writer, &cmd)?;
                    Self::print_cfind_result(result, &cmd)?;
                    op.is_complete()
                }
                AssocUserOp::Get(op) => {
                    op.process_rsp(&mut reader, &mut writer, &cmd)?;
                    op.is_complete()
                }
                AssocUserOp::Store(op) => {
                    op.process_rsp(&cmd)?;
                    Self::print_progress(&cmd, "C-STORE");
                    op.is_complete()
                }
                AssocUserOp::Move(op) => {
                    op.process_rsp(&mut reader, &mut writer, &cmd)?;
                    Self::print_progress(&cmd, "C-MOVE");
                    op.is_complete()
                }
            };

            if is_complete {
                assoc.common_mut().remove_user_op(msg_id);
                break;
            }
        }

        assoc.release_association(&mut reader, &mut writer)
    }
}
