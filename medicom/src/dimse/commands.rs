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

//! Constants for DIMSE, DICOM Message Exchangy

use crate::{
    core::{defn::tag::Tag, values::RawValue},
    dict::tags::{
        NumberofCompletedSuboperations, NumberofFailedSuboperations,
        NumberofRemainingSuboperations, NumberofWarningSuboperations,
    },
    dimse::commands::messages::CommandMessage,
};

pub mod messages;

#[cfg(test)]
mod tests;

/// Values of the `CommandField` (0000,0100) field of messages.
///
/// See Part 7, Appendix E.
#[derive(PartialEq, Eq, Clone, Hash)]
pub enum CommandType {
    CStoreReq,
    CStoreRsp,
    CGetReq,
    CGetRsp,
    CFindReq,
    CFindRsp,
    CMoveReq,
    CMoveRsp,
    CEchoReq,
    CEchoRsp,

    NEventReportReq,
    NEventReportRsp,
    NGetReq,
    NGetRsp,
    NSetReq,
    NSetRsp,
    NActionReq,
    NActionRsp,
    NCreateReq,
    NCreateRsp,
    NDeleteReq,
    NDeleteRsp,

    CCancelReq,

    INVALID(u16),
}

impl std::fmt::Debug for CommandType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandType::CStoreReq => write!(f, "C-STORE-RQ"),
            CommandType::CStoreRsp => write!(f, "C-STORE-RP"),
            CommandType::CGetReq => write!(f, "C-GET-RQ"),
            CommandType::CGetRsp => write!(f, "C-GET-RP"),
            CommandType::CFindReq => write!(f, "C-FIND-RQ"),
            CommandType::CFindRsp => write!(f, "C-FIND-RP"),
            CommandType::CMoveReq => write!(f, "C-MOVE-RQ"),
            CommandType::CMoveRsp => write!(f, "C-MOVE-RP"),
            CommandType::CEchoReq => write!(f, "C-ECHO-RQ"),
            CommandType::CEchoRsp => write!(f, "C-ECHO-RP"),
            CommandType::NEventReportReq => write!(f, "N-EVENT-RQ"),
            CommandType::NEventReportRsp => write!(f, "N-EVENT-RP"),
            CommandType::NGetReq => write!(f, "N-GET-RQ"),
            CommandType::NGetRsp => write!(f, "N-GET-RP"),
            CommandType::NSetReq => write!(f, "N-SET-RQ"),
            CommandType::NSetRsp => write!(f, "N-SET-RP"),
            CommandType::NActionReq => write!(f, "N-ACTION-RQ"),
            CommandType::NActionRsp => write!(f, "N-ACTION-RP"),
            CommandType::NCreateReq => write!(f, "N-CREATE-RQ"),
            CommandType::NCreateRsp => write!(f, "N-CREATE-RP"),
            CommandType::NDeleteReq => write!(f, "N-DELETE-RQ"),
            CommandType::NDeleteRsp => write!(f, "N-DELETE-RP"),
            CommandType::CCancelReq => write!(f, "N-CANCEL-RQ"),
            CommandType::INVALID(c) => write!(f, "INVALID: {c:04x}"),
        }
    }
}

impl From<&CommandType> for u16 {
    fn from(value: &CommandType) -> Self {
        match value {
            CommandType::CStoreReq => 0x0001,
            CommandType::CStoreRsp => 0x8001,
            CommandType::CGetReq => 0x0010,
            CommandType::CGetRsp => 0x8010,
            CommandType::CFindReq => 0x0020,
            CommandType::CFindRsp => 0x8020,
            CommandType::CMoveReq => 0x0021,
            CommandType::CMoveRsp => 0x8021,
            CommandType::CEchoReq => 0x0030,
            CommandType::CEchoRsp => 0x8030,

            CommandType::NEventReportReq => 0x0100,
            CommandType::NEventReportRsp => 0x8100,
            CommandType::NGetReq => 0x0110,
            CommandType::NGetRsp => 0x8110,
            CommandType::NSetReq => 0x0120,
            CommandType::NSetRsp => 0x8120,
            CommandType::NActionReq => 0x0130,
            CommandType::NActionRsp => 0x8130,
            CommandType::NCreateReq => 0x0140,
            CommandType::NCreateRsp => 0x8140,
            CommandType::NDeleteReq => 0x0150,
            CommandType::NDeleteRsp => 0x8150,

            CommandType::CCancelReq => 0x0FFF,

            CommandType::INVALID(c) => *c,
        }
    }
}

impl From<u16> for CommandType {
    fn from(value: u16) -> Self {
        match value {
            0x0001 => CommandType::CStoreReq,
            0x8001 => CommandType::CStoreRsp,
            0x0010 => CommandType::CGetReq,
            0x8010 => CommandType::CGetRsp,
            0x0020 => CommandType::CFindReq,
            0x8020 => CommandType::CFindRsp,
            0x0021 => CommandType::CMoveReq,
            0x8021 => CommandType::CMoveRsp,
            0x0030 => CommandType::CEchoReq,
            0x8030 => CommandType::CEchoRsp,

            0x0100 => CommandType::NEventReportReq,
            0x8100 => CommandType::NEventReportRsp,
            0x0110 => CommandType::NGetReq,
            0x8110 => CommandType::NGetRsp,
            0x0120 => CommandType::NSetReq,
            0x8120 => CommandType::NSetRsp,
            0x0130 => CommandType::NActionReq,
            0x8130 => CommandType::NActionRsp,
            0x0140 => CommandType::NCreateReq,
            0x8140 => CommandType::NCreateRsp,
            0x0150 => CommandType::NDeleteReq,
            0x8150 => CommandType::NDeleteRsp,

            0x0FFF => CommandType::CCancelReq,

            c => CommandType::INVALID(c),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    CStoreReq,
    CStoreRsp,
    CGetReq,
    CGetRsp,
    CFindReq,
    CFindRsp,
    CMoveReq,
    CMoveRsp,
    CEchoReq,
    CEchoRsp,

    NEventReportReq,
    NEventReportRsp,
    NGetReq,
    NGetRsp,
    NSetReq,
    NSetRsp,
    NActionReq,
    NActionRsp,
    NCreateReq,
    NCreateRsp,
    NDeleteReq,
    NDeleteRsp,

    CCancelReq,
}

/// Values of the `Priority` (0000,0700) field of messages.
///
/// See Part 7, Appendix E.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CommandPriority {
    Low,
    Medium,
    High,
    INVALID(u16),
}

impl From<&CommandPriority> for u16 {
    fn from(value: &CommandPriority) -> Self {
        match value {
            CommandPriority::Low => 0x0002,
            CommandPriority::Medium => 0x0000,
            CommandPriority::High => 0x0001,
            CommandPriority::INVALID(c) => *c,
        }
    }
}

impl From<&CommandPriority> for RawValue<'_> {
    fn from(value: &CommandPriority) -> Self {
        RawValue::of_ushort(u16::from(value))
    }
}

impl From<u16> for CommandPriority {
    fn from(value: u16) -> Self {
        match value {
            0x0002 => CommandPriority::Low,
            0x0000 => CommandPriority::Medium,
            0x0001 => CommandPriority::High,
            c => CommandPriority::INVALID(c),
        }
    }
}

/// Values of the `Status` (0000,0900) field of messages.
///
/// See Part 7, Appendix C.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CommandStatus {
    Success(u16),
    Warning(u16),
    Failure(u16),
    Cancel(u16),
    Pending(u16),
    INVALID(u16),
}

impl CommandStatus {
    /// Convenience for `CommandStatus::Success(0)`.
    #[must_use]
    pub fn success() -> CommandStatus {
        CommandStatus::Success(0)
    }

    /// Whether this status is the success code.
    #[must_use]
    pub fn is_success(&self) -> bool {
        self == &CommandStatus::Success(0)
    }

    /// Convenience for `CommandStatus::Pending(0xFF00)`.
    #[must_use]
    pub fn pending() -> CommandStatus {
        CommandStatus::Pending(0xFF00)
    }

    /// Matches are continuing - Warning that one or more Optional Keys were not supported for
    /// existence and/or matching for this Identifier.
    ///
    /// Convenience for `CommandStatus::Pending(0xFF01)`.
    ///
    /// C-FIND
    #[must_use]
    pub fn pending_missing_match() -> CommandStatus {
        CommandStatus::Pending(0xFF01)
    }

    /// Whether this status is either of the "pending" codes.
    #[must_use]
    pub fn is_pending(&self) -> bool {
        self == &CommandStatus::Pending(0xFF00) || self == &CommandStatus::Pending(0xFF01)
    }

    /// Convenience for `CommandStatus::Cancel(0xFE00)`.
    #[must_use]
    pub fn cancel() -> CommandStatus {
        CommandStatus::Cancel(0xFE00)
    }

    /// Whether this status is the "cancel" code.
    #[must_use]
    pub fn is_canceled(&self) -> bool {
        self == &CommandStatus::Cancel(0xFE00)
    }

    /// Sub-operations Complete - One or more Failures (or Warnings).
    ///
    /// Convenience for `CommandStatus::Warning(0xB000)`.
    ///
    /// C-MOVE, C-GET
    #[must_use]
    pub fn warn_one_or_more_fails() -> CommandStatus {
        CommandStatus::Warning(0xB000)
    }

    /// Whether this status is any of the "warning" codes.
    #[must_use]
    pub fn is_warning(&self) -> bool {
        if let CommandStatus::Warning(code) = self {
            *code == 0x0001 || *code == 0x0107 || *code == 0x0116 || *code & 0xF000 == 0xB000
        } else {
            false
        }
    }

    /// Refused: Out of resources.
    ///
    /// Convenience for `CommandStatus::Failure(0xA700)`.
    ///
    /// C-FIND
    #[must_use]
    pub fn fail_rsrc() -> CommandStatus {
        CommandStatus::Failure(0xA700)
    }

    /// Refused: Out of resources - Unable to calculate number of matches.
    ///
    /// Convenience for `CommandStatus::Failure(0xA701)`.
    ///
    /// C-MOVE
    #[must_use]
    pub fn fail_rsrc_calcmatch() -> CommandStatus {
        CommandStatus::Failure(0xA701)
    }

    /// Refused: Out of resources - Unable to perform sub-operations.
    ///
    /// Convenience for `CommandStatus::Failure(0xA702)`.
    ///
    /// C-MOVE
    #[must_use]
    pub fn fail_subops() -> CommandStatus {
        CommandStatus::Failure(0xA702)
    }

    /// Refused: Move Destination unknown.
    ///
    /// Convenience for `CommandStatus::Failure(0xA801)`.
    ///
    /// C-MOVE
    #[must_use]
    pub fn fail_unknown_dest() -> CommandStatus {
        CommandStatus::Failure(0xA801)
    }

    /// Error: Data Set does not match SOP Class.
    ///
    /// Convenience for `CommandStatus::Failure(0xA900)`.
    ///
    /// C-MOVE, C-FIND
    #[must_use]
    pub fn fail_sop_mismatch() -> CommandStatus {
        CommandStatus::Failure(0xA900)
    }

    /// Duplicate invocation: Indicates that the Message ID (0000,0110) specified is allocated to
    /// another notification or operation.
    ///
    /// Convenience for `CommandStatus::Failure(0x0210)`.
    ///
    /// C-GET,
    #[must_use]
    pub fn fail_dup_invoke() -> CommandStatus {
        CommandStatus::Failure(0x0210)
    }

    /// Mistyped argument: Indicates that one of the parameters supplied has not been agreed for
    /// use on the Association between the DIMSE Service Users.
    ///
    /// Convenience for `CommandStatus::Failure(0x0212)`.
    ///
    /// C-GET
    #[must_use]
    pub fn fail_mistyped_arg() -> CommandStatus {
        CommandStatus::Failure(0x0212)
    }

    /// Unrecognized operation: Indicates that the operation is not one of those agreed between the
    /// DIMSE Service Users.
    ///
    /// Convenience for `CommandStatus::Failure(0x0211)`.
    ///
    /// C-GET
    #[must_use]
    pub fn fail_unrecog_op() -> CommandStatus {
        CommandStatus::Failure(0x0211)
    }

    /// Refused: Not authorized: Indicates that the peer DIMSE Service User was not authorized to
    /// invoke the C-FIND operation.
    ///
    /// Convenience for `CommandStatus::Failure(0x0124)`.
    ///
    /// C-GET
    #[must_use]
    pub fn fail_not_auth() -> CommandStatus {
        CommandStatus::Failure(0x0124)
    }

    /// Failed: Unable to process.
    ///
    /// Convenience for `CommandStatus::Failure(0xC000)`.
    ///
    /// C-FIND, C-MOVE, C-GET
    #[must_use]
    pub fn fail() -> CommandStatus {
        CommandStatus::Failure(0xC000)
    }

    /// Whether this status is any of the "failure" codes.
    #[must_use]
    pub fn is_failed(&self) -> bool {
        if let CommandStatus::Failure(code) = self {
            *code & 0xF000 == 0xA000
                || *code & 0xF000 == 0xC000
                || *code & 0x0F00 == 0x0100
                || *code & 0x0F00 == 0x0200
        } else {
            false
        }
    }
}

impl From<&CommandStatus> for RawValue<'_> {
    fn from(value: &CommandStatus) -> Self {
        match value {
            CommandStatus::Success(c)
            | CommandStatus::Warning(c)
            | CommandStatus::Failure(c)
            | CommandStatus::Cancel(c)
            | CommandStatus::Pending(c)
            | CommandStatus::INVALID(c) => RawValue::of_ushort(*c),
        }
    }
}

impl From<&CommandStatus> for u16 {
    fn from(value: &CommandStatus) -> Self {
        match value {
            CommandStatus::Success(code)
            | CommandStatus::Warning(code)
            | CommandStatus::Failure(code)
            | CommandStatus::Cancel(code)
            | CommandStatus::Pending(code)
            | CommandStatus::INVALID(code) => *code,
        }
    }
}

impl From<u16> for CommandStatus {
    fn from(value: u16) -> Self {
        if value == 0x0000 {
            return CommandStatus::Success(value);
        }

        if value == 0x0001 || value == 0x0107 || value == 0x0116 || value & 0xF000 == 0xB000 {
            return CommandStatus::Warning(value);
        }
        if value & 0xF000 == 0xA000
            || value & 0xF000 == 0xC000
            || value & 0x0F00 == 0x0100
            || value & 0x0F00 == 0x0200
        {
            return CommandStatus::Failure(value);
        }
        if value == 0xFE00 {
            return CommandStatus::Cancel(value);
        }
        if value == 0xFF00 || value == 0xFF01 {
            return CommandStatus::Pending(value);
        }

        CommandStatus::INVALID(value)
    }
}

/// When reporting progress of C-MOVE transfers. The order of progress fields are: (remaining,
/// completed, failed, warning)
pub struct SubOpProgress(pub u16, pub u16, pub u16, pub u16);

impl SubOpProgress {
    #[must_use]
    pub fn remaining(&self) -> u16 {
        self.0
    }

    #[must_use]
    pub fn completed(&self) -> u16 {
        self.1
    }

    #[must_use]
    pub fn failed(&self) -> u16 {
        self.2
    }

    #[must_use]
    pub fn warning(&self) -> u16 {
        self.3
    }

    #[must_use]
    pub fn total(&self) -> u16 {
        self.0 + self.1 + self.2 + self.3
    }

    /// Create the individual elements that will report this progress.
    #[must_use]
    pub fn as_elements(&'_ self, status: &CommandStatus) -> Vec<(&'_ Tag, RawValue<'_>)> {
        // A status of successful, failed, or warning shall NOT contain Number of Remaining.
        let mut elements = Vec::<(&Tag, RawValue)>::with_capacity(4);
        if status.is_pending() || status.is_canceled() {
            elements.push((&NumberofRemainingSuboperations, RawValue::of_ushort(self.0)));
        }
        elements.append(&mut vec![
            (&NumberofCompletedSuboperations, RawValue::of_ushort(self.1)),
            (&NumberofFailedSuboperations, RawValue::of_ushort(self.2)),
            (&NumberofWarningSuboperations, RawValue::of_ushort(self.3)),
        ]);
        elements
    }
}

impl From<&CommandMessage> for SubOpProgress {
    fn from(value: &CommandMessage) -> SubOpProgress {
        let remaining = value
            .get_ushort(&NumberofRemainingSuboperations)
            .unwrap_or_default();
        let completed = value
            .get_ushort(&NumberofCompletedSuboperations)
            .unwrap_or_default();
        let failed = value
            .get_ushort(&NumberofFailedSuboperations)
            .unwrap_or_default();
        let warning = value
            .get_ushort(&NumberofWarningSuboperations)
            .unwrap_or_default();
        SubOpProgress(remaining, completed, failed, warning)
    }
}
