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

use crate::{
    core::{
        charset::CSRef,
        dcmobject::DicomRoot,
        defn::{constants::ts::ImplicitVRLittleEndian, tag::Tag, vr::US},
        values::RawValue,
    },
    dict::tags::{
        AffectedSOPClassUID, AffectedSOPInstanceUID, CommandDataSetType, CommandField,
        CommandGroupLength, MessageID, MessageIDBeingRespondedTo, MoveDestination,
        MoveOriginatorApplicationEntityTitle, MoveOriginatorMessageID, Priority, Status,
    },
    dimse::{
        commands::{CommandPriority, CommandStatus, CommandType, SubOpProgress},
        DimseError,
    },
};

/// Sentinel value of `CommandDataSetType` (0000,0800) to indicate that there is no Data Set
/// present in the message. Any other value in `CommandDataSetType` indicates a Data Set is present
/// in the message.
///
/// See Part 7, Appendix E.
pub const COMMAND_DATASET_TYPE_NONE: u16 = 0x0101;

/// Sentinel value of `CommandDataSetType` (0000,0800) to indicate that there is some Data Set
/// present in the message. This value is arbitrary, as long as it's not
/// `COMMAND_DATASET_TYPE_NONE`, and avoids using all zeros to make this more intentional and
/// obvious.
const COMMAND_DATASET_TYPE_SOME: u16 = 0x1111;

#[derive(Debug, Clone)]
pub struct CommandMessage {
    ctx_id: u8,
    msg_id: u16,
    cmd_type: CommandType,
    priority: CommandPriority,
    has_dataset: bool,
    status: CommandStatus,
    message: DicomRoot,
}

impl CommandMessage {
    /// Creates a new `CommandMessage` from the given `DicomRoot`.
    ///
    /// # Notes
    /// The given `DicomRoot` is expected to have the following elements populated:
    /// - `MessageID`
    /// - `CommandField`
    /// - `Priority`
    /// - `CommandDataSetType`
    #[must_use]
    pub fn new(ctx_id: u8, message: DicomRoot) -> Self {
        let msg_id = message
            .get_value_by_tag(&MessageID)
            .or_else(|| message.get_value_by_tag(&MessageIDBeingRespondedTo))
            .and_then(|v| v.ushort())
            .unwrap_or_default();
        let cmd_type = CommandType::from(
            message
                .get_value_by_tag(&CommandField)
                .and_then(|v| v.ushort())
                .unwrap_or_default(),
        );
        let priority = CommandPriority::from(
            message
                .get_value_by_tag(&Priority)
                .and_then(|v| v.ushort())
                .unwrap_or_default(),
        );
        let has_dataset = message
            .get_value_by_tag(&CommandDataSetType)
            .and_then(|v| v.ushort())
            .unwrap_or_default()
            != COMMAND_DATASET_TYPE_NONE;
        let status = CommandStatus::from(
            message
                .get_value_by_tag(&Status)
                .and_then(|v| v.ushort())
                .unwrap_or_default(),
        );

        Self {
            ctx_id,
            msg_id,
            cmd_type,
            priority,
            has_dataset,
            status,
            message,
        }
    }

    #[must_use]
    pub fn ctx_id(&self) -> u8 {
        self.ctx_id
    }

    #[must_use]
    pub fn msg_id(&self) -> u16 {
        self.msg_id
    }

    #[must_use]
    pub fn cmd_type(&self) -> &CommandType {
        &self.cmd_type
    }

    #[must_use]
    pub fn priority(&self) -> &CommandPriority {
        &self.priority
    }

    #[must_use]
    pub fn message(&self) -> &DicomRoot {
        &self.message
    }

    #[must_use]
    pub fn has_dataset(&self) -> bool {
        self.has_dataset
    }

    #[must_use]
    pub fn status(&self) -> &CommandStatus {
        &self.status
    }

    /// Gets the value for the given tag, as a u16.
    ///
    /// # Errors
    /// If the tag is not present, then `DimseError::ElementMissingFromRequest` is returned.
    pub fn get_ushort<T>(&self, tag: T) -> Result<u16, DimseError>
    where
        u32: From<T>,
        T: Clone,
    {
        self.message
            .get_value_as_by_tag(tag.clone(), &US)
            .and_then(|v| v.ushort())
            .ok_or_else(|| DimseError::DimseElementMissing(Tag::format_tag_to_display(tag)))
    }

    /// Gets the value for the given tag, as a String.
    ///
    /// # Errors
    /// If the tag is not present, then `DimseError::ElementMissingFromRequest` is returned.
    pub fn get_string<T>(&self, tag: T) -> Result<String, DimseError>
    where
        u32: From<T>,
        T: Clone,
    {
        self.message
            .get_value_by_tag(tag.clone())
            .and_then(|v| v.string().cloned())
            .ok_or_else(|| DimseError::DimseElementMissing(Tag::format_tag_to_display(tag)))
    }

    /// Create a `CommandMessage` from a list of tag/value pairs.
    ///
    /// This handles the `CommandGroupLength` element, computing the total number of bytes for the
    /// given list of tag/value pairs.
    fn create(ctx_id: u8, elements: Vec<(&Tag, RawValue)>) -> Self {
        let mut message = DicomRoot::new_empty(&ImplicitVRLittleEndian, CSRef::default());
        for elem_pair in elements {
            message.add_child_with_val(elem_pair.0, elem_pair.1);
        }

        // Calculate the number of bytes of all the elements added above. This takes advantage of
        // the fact that CommandGroupLength will be the first element in the dataset but is not yet
        // added to the `DicomRoot`.
        let cmd_grp_len_val: u32 = u32::try_from(message.byte_size()).unwrap_or_default();
        message.add_child_with_val(&CommandGroupLength, RawValue::of_uint(cmd_grp_len_val));

        Self::new(ctx_id, message)
    }

    /// Create a C-ECHO request.
    #[must_use]
    pub fn c_echo_req(ctx_id: u8, msg_id: u16, sop_class_uid: &str) -> Self {
        CommandMessage::create(
            ctx_id,
            vec![
                (&AffectedSOPClassUID, RawValue::of_uid(sop_class_uid)),
                (
                    &CommandField,
                    RawValue::of_ushort(u16::from(&CommandType::CEchoReq)),
                ),
                (&MessageID, RawValue::of_ushort(msg_id)),
                (
                    &CommandDataSetType,
                    RawValue::of_ushort(COMMAND_DATASET_TYPE_NONE),
                ),
            ],
        )
    }

    /// Create a C-ECHO response.
    #[must_use]
    pub fn c_echo_rsp(
        ctx_id: u8,
        msg_id: u16,
        aff_sop_class_uid: &str,
        status: &CommandStatus,
    ) -> Self {
        CommandMessage::create(
            ctx_id,
            vec![
                (&AffectedSOPClassUID, RawValue::of_uid(aff_sop_class_uid)),
                (
                    &CommandField,
                    RawValue::of_ushort(u16::from(&CommandType::CEchoRsp)),
                ),
                (&MessageIDBeingRespondedTo, RawValue::of_ushort(msg_id)),
                (
                    &CommandDataSetType,
                    RawValue::of_ushort(COMMAND_DATASET_TYPE_NONE),
                ),
                (&Status, RawValue::from(status)),
            ],
        )
    }

    /// Create a C-ECHO response from the given C-ECHO request.
    ///
    /// # Errors
    /// This pulls values from the given `req`, however if those values are not present in the
    /// request this will be propagated as a `DimseError::ElementMissingFromRequest`.
    pub fn c_echo_rsp_from_req(
        req: &CommandMessage,
        status: &CommandStatus,
    ) -> Result<Self, DimseError> {
        let msg_id = req.get_ushort(&MessageID)?;
        let aff_sop_class_uid = req.get_string(&AffectedSOPClassUID)?;
        Ok(Self::c_echo_rsp(
            req.ctx_id(),
            msg_id,
            &aff_sop_class_uid,
            status,
        ))
    }

    /// Create a C-FIND request.
    #[must_use]
    pub fn c_find_req(ctx_id: u8, msg_id: u16, sop_class_uid: &str) -> Self {
        CommandMessage::create(
            ctx_id,
            vec![
                (&AffectedSOPClassUID, RawValue::of_uid(sop_class_uid)),
                (
                    &CommandField,
                    RawValue::of_ushort(u16::from(&CommandType::CFindReq)),
                ),
                (&MessageID, RawValue::of_ushort(msg_id)),
                (&Priority, RawValue::from(&CommandPriority::Medium)),
                (
                    &CommandDataSetType,
                    RawValue::of_ushort(COMMAND_DATASET_TYPE_SOME),
                ),
            ],
        )
    }

    /// Create a C-FIND response.
    #[must_use]
    pub fn c_find_rsp(
        ctx_id: u8,
        msg_id: u16,
        aff_sop_class_uid: &str,
        status: &CommandStatus,
    ) -> Self {
        // A C-FIND response will include the result as a following dataset only if the status is
        // marked as pending.
        let dataset_type = if status.is_pending() {
            COMMAND_DATASET_TYPE_SOME
        } else {
            COMMAND_DATASET_TYPE_NONE
        };
        CommandMessage::create(
            ctx_id,
            vec![
                (&AffectedSOPClassUID, RawValue::of_uid(aff_sop_class_uid)),
                (
                    &CommandField,
                    RawValue::of_ushort(u16::from(&CommandType::CFindRsp)),
                ),
                (&MessageIDBeingRespondedTo, RawValue::of_ushort(msg_id)),
                (&CommandDataSetType, RawValue::of_ushort(dataset_type)),
                (&Status, RawValue::from(status)),
            ],
        )
    }

    /// Create a C-FIND response from the given C-FIND request.
    ///
    /// # Errors
    /// This pulls values from the given `req`, however if those values are not present in the
    /// request this will be propagated as a `DimseError::ElementMissingFromRequest`.
    pub fn c_find_rsp_from_req(
        req: &CommandMessage,
        status: &CommandStatus,
    ) -> Result<Self, DimseError> {
        let msg_id = req.get_ushort(&MessageID)?;
        let aff_sop_class_uid = req.get_string(&AffectedSOPClassUID)?;
        Ok(Self::c_find_rsp(
            req.ctx_id(),
            msg_id,
            &aff_sop_class_uid,
            status,
        ))
    }

    /// Create a C-STORE request.
    #[must_use]
    pub fn c_store_req(
        ctx_id: u8,
        msg_id: u16,
        priority: &CommandPriority,
        aff_sop_class_uid: &str,
        aff_sop_inst_uid: &str,
        origin_ae: &str,
        origin_msg_id: u16,
    ) -> Self {
        CommandMessage::create(
            ctx_id,
            vec![
                (&AffectedSOPClassUID, RawValue::of_uid(aff_sop_class_uid)),
                (
                    &CommandField,
                    RawValue::of_ushort(u16::from(&CommandType::CStoreReq)),
                ),
                (&MessageID, RawValue::of_ushort(msg_id)),
                (&Priority, RawValue::from(priority)),
                (&AffectedSOPInstanceUID, RawValue::of_uid(aff_sop_inst_uid)),
                (
                    &MoveOriginatorApplicationEntityTitle,
                    RawValue::of_string(origin_ae),
                ),
                (&MoveOriginatorMessageID, RawValue::of_ushort(origin_msg_id)),
                (
                    &CommandDataSetType,
                    RawValue::of_ushort(COMMAND_DATASET_TYPE_SOME),
                ),
            ],
        )
    }

    /// Create a C-STORE response.
    #[must_use]
    pub fn c_store_rsp(
        ctx_id: u8,
        msg_id: u16,
        aff_sop_class_uid: &str,
        status: &CommandStatus,
    ) -> Self {
        CommandMessage::create(
            ctx_id,
            vec![
                (&AffectedSOPClassUID, RawValue::of_uid(aff_sop_class_uid)),
                (
                    &CommandField,
                    RawValue::of_ushort(u16::from(&CommandType::CStoreRsp)),
                ),
                (&MessageIDBeingRespondedTo, RawValue::of_ushort(msg_id)),
                (
                    &CommandDataSetType,
                    RawValue::of_ushort(COMMAND_DATASET_TYPE_NONE),
                ),
                (&Status, RawValue::from(status)),
            ],
        )
    }

    #[must_use]
    pub fn c_move_req(ctx_id: u8, msg_id: u16, aff_sop_class_uid: &str, dest_ae: &str) -> Self {
        CommandMessage::create(
            ctx_id,
            vec![
                (&AffectedSOPClassUID, RawValue::of_uid(aff_sop_class_uid)),
                (
                    &CommandField,
                    RawValue::of_ushort(u16::from(&CommandType::CMoveReq)),
                ),
                (&MessageID, RawValue::of_ushort(msg_id)),
                (&Priority, RawValue::from(&CommandPriority::Medium)),
                (
                    &CommandDataSetType,
                    RawValue::of_ushort(COMMAND_DATASET_TYPE_SOME),
                ),
                (&MoveDestination, RawValue::of_string(dest_ae)),
            ],
        )
    }

    /// Create a C-STORE response.
    #[must_use]
    pub fn c_move_rsp(
        ctx_id: u8,
        msg_id: u16,
        aff_sop_class_uid: &str,
        status: &CommandStatus,
        progress: &SubOpProgress,
    ) -> Self {
        let mut elements = vec![
            (&AffectedSOPClassUID, RawValue::of_uid(aff_sop_class_uid)),
            (
                &CommandField,
                RawValue::of_ushort(u16::from(&CommandType::CMoveRsp)),
            ),
            (&MessageIDBeingRespondedTo, RawValue::of_ushort(msg_id)),
            (
                &CommandDataSetType,
                RawValue::of_ushort(COMMAND_DATASET_TYPE_NONE),
            ),
            (&Status, RawValue::from(status)),
        ];
        elements.append(&mut progress.as_elements(status));
        CommandMessage::create(ctx_id, elements)
    }

    /// Creates a C-GET request.
    #[must_use]
    pub fn c_get_req(ctx_id: u8, msg_id: u16, aff_sop_class_uid: &str) -> Self {
        CommandMessage::create(
            ctx_id,
            vec![
                (&AffectedSOPClassUID, RawValue::of_uid(aff_sop_class_uid)),
                (
                    &CommandField,
                    RawValue::of_ushort(u16::from(&CommandType::CGetReq)),
                ),
                (&MessageID, RawValue::of_ushort(msg_id)),
                (&Priority, RawValue::from(&CommandPriority::Medium)),
                (
                    &CommandDataSetType,
                    RawValue::of_ushort(COMMAND_DATASET_TYPE_SOME),
                ),
            ],
        )
    }

    /// Creates a C-GET response.
    #[must_use]
    pub fn c_get_rsp(
        ctx_id: u8,
        msg_id: u16,
        aff_sop_class_uid: &str,
        status: &CommandStatus,
        progress: &SubOpProgress,
    ) -> Self {
        let mut elements = vec![
            (&AffectedSOPClassUID, RawValue::of_uid(aff_sop_class_uid)),
            (
                &CommandField,
                RawValue::of_ushort(u16::from(&CommandType::CGetRsp)),
            ),
            (&MessageIDBeingRespondedTo, RawValue::of_ushort(msg_id)),
            (
                &CommandDataSetType,
                RawValue::of_ushort(COMMAND_DATASET_TYPE_NONE),
            ),
            (&Status, RawValue::from(status)),
        ];
        elements.append(&mut progress.as_elements(status));
        CommandMessage::create(ctx_id, elements)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        core::{dcmobject::DicomObject, defn::tag::Tag, values::RawValue},
        dict::{
            tags::{
                AffectedSOPClassUID, CommandDataSetType, CommandField, CommandGroupLength,
                MessageID, MessageIDBeingRespondedTo, Status,
            },
            uids::{CTImageStorage, MRImageStorage},
        },
        dimse::commands::{
            messages::{CommandMessage, COMMAND_DATASET_TYPE_NONE},
            CommandStatus, CommandType,
        },
    };

    /// Asserts the `act_pair` contains an element matching the given expected tag and value. The
    /// actual value is taken as a pair for convenience for working with an iterator's `next()`.
    fn assert_eq_elem(
        exp_tag: &Tag,
        exp_val: RawValue,
        act_pair: Option<(&u32, &DicomObject)>,
    ) -> usize {
        let (_act_tag, act_obj) =
            act_pair.unwrap_or_else(|| panic!("Should have element: {}", exp_tag.ident()));
        assert_eq!(exp_tag.num(), act_obj.element().tag());
        let act_val = act_obj
            .element()
            .parse_value()
            .unwrap_or_else(|_| panic!("Should get value for: {}", exp_tag.ident()));
        assert_eq!(exp_val, act_val, "for {}", exp_tag.ident());
        act_obj.byte_size()
    }

    #[test]
    fn test_cecho_rsp() {
        let exp_msg_id = 718;
        let exp_affected_sop = CTImageStorage.uid();
        let exp_status = CommandStatus::Success(0);
        let exp_bytes = 74usize;

        let req = CommandMessage::c_echo_req(1, exp_msg_id, exp_affected_sop);
        let rsp = CommandMessage::c_echo_rsp_from_req(&req, &exp_status).expect("build response");

        let mut elem_iter = rsp.message().iter_child_nodes();

        assert_eq_elem(
            &CommandGroupLength,
            RawValue::of_uint(u32::try_from(exp_bytes).expect("exp_bytes as u32")),
            elem_iter.next(),
        );

        let mut bytes = 0usize;

        bytes += assert_eq_elem(
            &AffectedSOPClassUID,
            RawValue::Uid(exp_affected_sop.to_string()),
            elem_iter.next(),
        );

        bytes += assert_eq_elem(
            &CommandField,
            RawValue::of_ushort(u16::from(&CommandType::CEchoRsp)),
            elem_iter.next(),
        );

        bytes += assert_eq_elem(
            &MessageIDBeingRespondedTo,
            RawValue::of_ushort(exp_msg_id),
            elem_iter.next(),
        );

        bytes += assert_eq_elem(
            &CommandDataSetType,
            RawValue::of_ushort(COMMAND_DATASET_TYPE_NONE),
            elem_iter.next(),
        );

        bytes += assert_eq_elem(&Status, RawValue::from(&exp_status), elem_iter.next());

        assert_eq!(exp_bytes, bytes);
    }

    #[test]
    fn test_cecho_req() {
        let exp_msg_id = 718;
        let exp_affected_sop = MRImageStorage.uid();
        let exp_bytes = 64usize;

        let req = CommandMessage::c_echo_req(1, exp_msg_id, exp_affected_sop);

        let mut elem_iter = req.message().iter_child_nodes();

        assert_eq_elem(
            &CommandGroupLength,
            RawValue::of_uint(u32::try_from(exp_bytes).expect("exp_bytes as u32")),
            elem_iter.next(),
        );

        let mut bytes = 0usize;

        bytes += assert_eq_elem(
            &AffectedSOPClassUID,
            RawValue::Uid(exp_affected_sop.to_string()),
            elem_iter.next(),
        );

        bytes += assert_eq_elem(
            &CommandField,
            RawValue::of_ushort(u16::from(&CommandType::CEchoReq)),
            elem_iter.next(),
        );

        bytes += assert_eq_elem(
            &MessageID,
            RawValue::of_ushort(exp_msg_id),
            elem_iter.next(),
        );

        bytes += assert_eq_elem(
            &CommandDataSetType,
            RawValue::of_ushort(COMMAND_DATASET_TYPE_NONE),
            elem_iter.next(),
        );

        assert_eq!(exp_bytes, bytes);
    }
}
