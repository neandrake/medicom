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
    fs::File,
    io::{BufReader, Read, Write},
};

use medicom::{
    core::read::ParserBuilder,
    dict::stdlookup::STANDARD_DICOM_DICTIONARY,
    dimse::{
        assoc::CommonAssoc,
        commands::messages::CommandMessage,
        error::{AssocError, DimseError},
        svcops::GetSvcOp,
        userops::AssocUserOp,
    },
};

use crate::app::scpapp::{fail, prog, AssociationDevice, Stat, StatusMsgBuilder};

impl<R: Read, W: Write> AssociationDevice<R, W> {
    pub(crate) fn handle_c_get_req(
        &mut self,
        op: &mut GetSvcOp,
        cmd: &CommandMessage,
    ) -> Result<(), AssocError> {
        let dcm_query =
            op.process_req(cmd, self.assoc.common(), &mut self.reader, &mut self.writer)?;
        let statter = StatusMsgBuilder::new(
            false,
            op.ctx_id(),
            op.msg_id(),
            op.aff_sop_class().to_owned(),
        );

        let query_results = self.query_database(&dcm_query)?;
        let path_map = Self::resolve_to_paths(query_results.group_map);

        let sop_count = path_map.values().map(Vec::len).sum::<usize>();
        let sop_count = u16::try_from(sop_count).unwrap_or_default();

        let mut successful: u16 = 0;
        let mut remaining: u16 = sop_count;
        let mut store_msg_id = op.msg_id() + 1;
        for (_key, paths) in path_map {
            for path in paths {
                let file = match File::open(&path)
                    .map_err(|e| AssocError::ab_failure(DimseError::IOError(e)))
                {
                    Ok(file) => file,
                    Err(err) => {
                        // Abort the C-STORE association with the destination AE.
                        let _ = err.write(&mut self.writer);

                        // For now, if one fails then do not attempt the rest.
                        CommonAssoc::write_command(
                            &statter.msg(&Stat::fail(), &prog(0, successful, remaining, 0)),
                            &mut self.writer,
                            self.assoc.common().get_pdu_max_snd_size(),
                        )?;
                        return Err(fail(&format!("Failed resolving {}", path.display())));
                    }
                };

                let input = BufReader::with_capacity(1024 * 1024, file);
                let parser = ParserBuilder::default().build(input, &STANDARD_DICOM_DICTIONARY);
                self.assoc.common_mut().c_store_req(
                    &mut self.reader,
                    &mut self.writer,
                    parser,
                    store_msg_id,
                    op.this_ae(),
                    op.msg_id(),
                )?;
                // TODO: This handling of response needs extracted for async handling.
                let store_rsp = CommonAssoc::next_msg(
                    &mut self.reader,
                    &mut self.writer,
                    self.assoc.common().get_pdu_max_rcv_size(),
                );
                let store_rsp = self.interpret_cstore_rsp(
                    store_rsp,
                    &statter,
                    &prog(0, successful, remaining, 0),
                )?;
                if let Some(AssocUserOp::Store(store_op)) =
                    self.assoc.common_mut().get_user_op(store_msg_id)
                {
                    store_op.process_rsp(&store_rsp)?;
                    if store_op.is_complete() {
                        self.assoc.common_mut().remove_user_op(store_msg_id);
                    }
                }

                store_msg_id += 1;
                successful += 1;
                remaining -= 1;

                op.write_response(
                    &mut self.writer,
                    self.assoc.common().get_pdu_max_snd_size(),
                    &Stat::pending(),
                    &prog(remaining, successful, 0, 0),
                )?;
            }
        }

        op.write_response(
            &mut self.writer,
            self.assoc.common().get_pdu_max_snd_size(),
            &Stat::success(),
            &prog(0, successful, 0, 0),
        )?;

        Ok(())
    }
}
