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

use std::io::{Read, Write};

use medicom::dimse::{
    assoc::CommonAssoc,
    commands::{messages::CommandMessage, CommandStatus},
    error::AssocError,
    svcops::StoreSvcOp,
};

use crate::app::scpapp::AssociationDevice;

impl<R: Read, W: Write> AssociationDevice<R, W> {
    pub(crate) fn handle_c_store_req(
        &mut self,
        op: &mut StoreSvcOp,
        cmd: &CommandMessage,
    ) -> Result<(), AssocError> {
        op.process_req(cmd)?;

        // TODO: Tuck this away somewhere. Add appropriate FileMeta elements.
        let mut empty = std::io::empty();
        CommonAssoc::read_dataset(
            &mut self.reader,
            &mut self.writer,
            self.assoc.common().get_pdu_max_rcv_size(),
            &mut empty,
        )?;

        op.write_response(
            &mut self.writer,
            self.assoc.common().get_pdu_max_snd_size(),
            &CommandStatus::success(),
        )
    }
}
