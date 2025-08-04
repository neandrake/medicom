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

//! Protocol Data Units for DIMSE. Part 8, Chapter 9.3
//!
//! PDU headers are encoded with Big Endian. The value fields are sent using the transfer syntax
//! negotiated during establishment of the association.

use std::{
    io::{Read, Write},
    mem::size_of,
};

use crate::{
    core::defn::{ts::TSRef, uid::UIDRef, vr::UI},
    dimse::{
        pdus::{PduType, UserPdu},
        DimseError,
    },
};

/// This is the current DICOM standard defined version for Associations.
/// See Part 8, Section 9.3.2 (ASSOCIATE-RQ) and Section 9.3.3 (ASSOCIATE-AC).
pub(crate) static ASSOCIATION_VERSION: u16 = 0b0000_0000_0000_0001;

/// This is the sole/single application context name for the current version of the DICOM standard.
/// See Part 7, Annex A.2.1
pub static STD_APP_CONTEXT_NAME: &str = "1.2.840.10008.3.1.1.1";

/// Value for `PresentationDataItem`'s `msg_header` to indicate the data is a DICOM dataset, and is
/// not the last fragment.
pub static P_DATA_DCM_DATASET: u8 = 0b00;
/// Value for `PresentationDataItem`'s `msg_header` to indicate the data is a DICOM dataset, and is
/// the last fragment.
pub static P_DATA_DCM_DATASET_LAST: u8 = 0b10;
/// Value for `PresentationDataItem`'s `msg_header` to indicate the data is a command, and is not
/// the last fragment.
pub static P_DATA_CMD: u8 = 0b01;
/// Value for `PresentationDataItem`'s `msg_header` to indicate the data is a command, and is the
/// last fragment.
pub static P_DATA_CMD_LAST: u8 = 0b11;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AssocRQ {
    /// Reserved, should be zero.
    reserved_1: u8,
    length: u32,
    version: u16,
    /// Reserved, should be zeros.
    reserved_2: [u8; 2],
    called_ae: [u8; 16],
    calling_ae: [u8; 16],
    /// Reserved, should be zeros.
    reserved_3: [u8; 32],
    app_ctx: ApplicationContextItem,
    pres_ctxs: Vec<AssocRQPresentationContext>,
    user_info: UserInformationItem,
}

impl AssocRQ {
    /// The type of this PDU, `PduType::AssocRQ`.
    #[must_use]
    pub fn pdu_type(&self) -> PduType {
        PduType::AssocRQ
    }

    /// Create a new `AssocRQ`.
    #[must_use]
    pub fn new(
        called_ae: [u8; 16],
        calling_ae: [u8; 16],
        app_ctx: ApplicationContextItem,
        pres_ctxs: Vec<AssocRQPresentationContext>,
        user_info: UserInformationItem,
    ) -> Self {
        let length: usize = size_of::<u16>() + // version
            size_of::<[u8; 2]>() + // reserved_2
            size_of::<[u8; 16]>() + // called_ae
            size_of::<[u8; 16]>() + // calling_ae
            size_of::<[u8; 32]>() + // reserved_3
            app_ctx.byte_size() +
            pres_ctxs.iter().map(AssocRQPresentationContext::byte_size).sum::<usize>() +
            user_info.byte_size();

        Self {
            reserved_1: 0u8,
            length: length.try_into().unwrap_or_default(),
            version: ASSOCIATION_VERSION,
            reserved_2: [0u8; 2],
            called_ae,
            calling_ae,
            reserved_3: [0u8; 32],
            app_ctx,
            pres_ctxs,
            user_info,
        }
    }

    /// The number of bytes from the first byte of the following field to the last byte of the
    /// variable field (User Information).
    #[must_use]
    pub fn length(&self) -> u32 {
        self.length
    }

    /// Identifies each version of the DICOM Upper Layer protocol supported by the calling
    /// end-system. Currently this is version 1, identified with bit 0 set.
    #[must_use]
    pub fn version(&self) -> u16 {
        self.version
    }

    /// Destination AE, 16 characters encoded with the Basic G0 Set (essentially ASCII), with
    /// leading and trailing spaces being non-significant.
    #[must_use]
    pub fn called_ae(&self) -> &[u8; 16] {
        &self.called_ae
    }

    /// Source AE, 16 characters encoded with the Basic G0 Set (essentially ASCII), with leading
    /// and trailing spaces being non-significant.
    #[must_use]
    pub fn calling_ae(&self) -> &[u8; 16] {
        &self.calling_ae
    }

    /// The third reserved field, 32 bytes. This is exposed as the standard requires this value be
    /// copied in the `AssocAC` response.
    #[must_use]
    pub fn reserved_3(&self) -> &[u8; 32] {
        &self.reserved_3
    }

    /// Application Context.
    #[must_use]
    pub fn app_ctx(&self) -> &ApplicationContextItem {
        &self.app_ctx
    }

    /// Presentation Contexts, at least one.
    #[must_use]
    pub fn pres_ctxs(&self) -> &Vec<AssocRQPresentationContext> {
        &self.pres_ctxs
    }

    /// User Information.
    #[must_use]
    pub fn user_info(&self) -> &UserInformationItem {
        &self.user_info
    }

    /// The total number of bytes that this PDU will require to write to a dataset.
    #[must_use]
    pub fn byte_size(&self) -> usize {
        size_of::<u8>() // pdu_type
            + size_of::<u8>() // reserved_1
            + size_of::<u32>() // length
            + size_of::<u16>() // version
            + self.reserved_2.len()
            + self.called_ae.len()
            + self.calling_ae.len()
            + self.reserved_3.len()
            + self.app_ctx.byte_size()
            + self.pres_ctxs.iter().map(AssocRQPresentationContext::byte_size).sum::<usize>()
            + self.user_info.byte_size()
    }

    /// Write this PDU to the given dataset.
    ///
    /// # Errors
    /// I/O errors writing to the dataset.
    pub fn write<W: Write>(&self, mut dataset: W) -> Result<(), DimseError> {
        let buf: [u8; 2] = [u8::from(&self.pdu_type()), self.reserved_1];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.length.to_be_bytes())?;
        dataset.write_all(&self.version.to_be_bytes())?;
        dataset.write_all(&self.reserved_2)?;
        dataset.write_all(&self.called_ae)?;
        dataset.write_all(&self.calling_ae)?;
        dataset.write_all(&self.reserved_3)?;

        self.app_ctx.write(&mut dataset)?;
        for pres_ctx in &self.pres_ctxs {
            pres_ctx.write(&mut dataset)?;
        }
        self.user_info.write(&mut dataset)?;

        Ok(())
    }

    /// Read a `AssocAQ` from the given dataset.
    ///
    /// # Errors
    /// I/O errors reading from the dataset.
    pub fn read<R: Read>(mut dataset: R, reserved: u8) -> Result<Self, DimseError> {
        let mut buf: [u8; 4] = [0u8; 4];
        dataset.read_exact(&mut buf)?;
        let length = u32::from_be_bytes(buf);

        let mut buf: [u8; 2] = [0u8; 2];
        dataset.read_exact(&mut buf)?;
        let version = u16::from_be_bytes(buf);

        let mut reserved_2 = [0u8; 2];
        dataset.read_exact(&mut reserved_2)?;

        let mut called_ae: [u8; 16] = [0u8; 16];
        dataset.read_exact(&mut called_ae)?;

        let mut calling_ae: [u8; 16] = [0u8; 16];
        dataset.read_exact(&mut calling_ae)?;

        let mut reserved_3: [u8; 32] = [0u8; 32];
        dataset.read_exact(&mut reserved_3)?;

        let mut buf: [u8; 2] = [0u8; 2];
        dataset.read_exact(&mut buf)?;
        let app_ctx = ApplicationContextItem::read(&mut dataset, buf[1])?;

        let mut pres_ctxs: Vec<AssocRQPresentationContext> = Vec::new();
        dataset.read_exact(&mut buf)?;
        while buf[0] == u8::from(&PduType::AssocRQPresentationContext) {
            let pres_ctx = AssocRQPresentationContext::read(&mut dataset, buf[1])?;
            pres_ctxs.push(pres_ctx);
            dataset.read_exact(&mut buf)?;
        }

        let user_info = UserInformationItem::read(&mut dataset, buf[1])?;

        Ok(Self {
            reserved_1: reserved,
            length,
            version,
            reserved_2,
            called_ae,
            calling_ae,
            reserved_3,
            app_ctx,
            pres_ctxs,
            user_info,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AssocAC {
    /// Reserved, should be zero.
    reserved_1: u8,
    length: u32,
    version: u16,
    /// Reserved, should be zeros.
    reserved_2: [u8; 2],
    /// Reserved, but this should be populated with the Destination AE field from the RQ though
    /// its value should not be checked.
    reserved_3: [u8; 16],
    /// Reserved, but this should be populated with the Source AE field from the RQ though its
    /// value should not be checked.
    reserved_4: [u8; 16],
    /// Reserved, but this should be populated with the corresponding reserved field from the RQ,
    /// which should be all zeros though its value should not be checked.
    reserved_5: [u8; 32],
    app_ctx: ApplicationContextItem,
    pres_ctxs: Vec<AssocACPresentationContext>,
    user_info: UserInformationItem,
}

impl AssocAC {
    /// The type of this PDU, `PduType::AssocAC`.
    #[must_use]
    pub fn pdu_type(&self) -> PduType {
        PduType::AssocAC
    }

    /// Create a new `AssocAC`.
    #[must_use]
    pub fn new(
        reserved_3: [u8; 16],
        reserved_4: [u8; 16],
        reserved_5: [u8; 32],
        app_ctx: ApplicationContextItem,
        pres_ctxs: Vec<AssocACPresentationContext>,
        user_info: UserInformationItem,
    ) -> Self {
        let length: usize = size_of::<u16>() // version
            + size_of::<[u8; 2]>() // reserved_2
            + size_of::<[u8; 16]>() // reserved_3
            + size_of::<[u8; 16]>() // reserved_4
            + size_of::<[u8; 32]>() // reserved_5
            + app_ctx.byte_size()
            + pres_ctxs.iter().map(AssocACPresentationContext::byte_size).sum::<usize>()
            + user_info.byte_size();

        Self {
            reserved_1: 0u8,
            length: length.try_into().unwrap_or_default(),
            version: ASSOCIATION_VERSION,
            reserved_2: [0u8; 2],
            reserved_3,
            reserved_4,
            reserved_5,
            app_ctx,
            pres_ctxs,
            user_info,
        }
    }

    /// The number of bytes from the first byte of the following field to the last byte of the
    /// variable items field (User Information).
    #[must_use]
    pub fn length(&self) -> u32 {
        self.length
    }

    /// Identifies each version of the DICOM Upper Layer protocol supported by the calling
    /// end-system. Currently this is version 1, identified with bit 0 set.
    #[must_use]
    pub fn version(&self) -> u16 {
        self.version
    }

    /// Reserved, but this should be populated with the Destination AE field from the RQ though
    /// its value should not be checked.
    #[must_use]
    pub fn reserved_3(&self) -> &[u8; 16] {
        &self.reserved_3
    }

    /// Reserved, but this should be populated with the Source AE field from the RQ though its
    /// value should not be checked.
    #[must_use]
    pub fn reserved_4(&self) -> &[u8; 16] {
        &self.reserved_4
    }

    /// Application Context.
    #[must_use]
    pub fn app_ctx(&self) -> &ApplicationContextItem {
        &self.app_ctx
    }

    /// Presentation Contexts, at least one.
    #[must_use]
    pub fn pres_ctxs(&self) -> &Vec<AssocACPresentationContext> {
        &self.pres_ctxs
    }

    /// User Information.
    #[must_use]
    pub fn user_info(&self) -> &UserInformationItem {
        &self.user_info
    }

    /// The total number of bytes that this PDU will require to write to a dataset.
    #[must_use]
    pub fn byte_size(&self) -> usize {
        size_of::<u8>() // pdu_type
            + size_of::<u8>() // reserved_1
            + size_of::<u32>() // length
            + size_of::<u16>() // version
            + self.reserved_2.len()
            + self.reserved_3.len()
            + self.reserved_4.len()
            + self.reserved_5.len()
            + self.app_ctx.byte_size()
            + self.pres_ctxs.iter().map(AssocACPresentationContext::byte_size).sum::<usize>()
            + self.user_info.byte_size()
    }

    /// Write this PDU to the given dataset.
    ///
    /// # Errors
    /// I/O errors writing to the dataset.
    pub fn write<W: Write>(&self, mut dataset: W) -> Result<(), DimseError> {
        let buf: [u8; 2] = [u8::from(&self.pdu_type()), self.reserved_1];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.length.to_be_bytes())?;
        dataset.write_all(&self.version.to_be_bytes())?;
        dataset.write_all(&self.reserved_2)?;
        dataset.write_all(&self.reserved_3)?;
        dataset.write_all(&self.reserved_4)?;
        dataset.write_all(&self.reserved_5)?;

        self.app_ctx.write(&mut dataset)?;
        for pres_ctx in &self.pres_ctxs {
            pres_ctx.write(&mut dataset)?;
        }
        self.user_info.write(&mut dataset)?;

        Ok(())
    }

    /// Read a `AssocAC` from the given dataset.
    ///
    /// # Errors
    /// I/O errors reading from the dataset.
    pub fn read<R: Read>(mut dataset: R, reserved: u8) -> Result<Self, DimseError> {
        let mut buf: [u8; 4] = [0u8; 4];
        dataset.read_exact(&mut buf)?;
        let length = u32::from_be_bytes(buf);

        let mut buf: [u8; 2] = [0u8; 2];
        dataset.read_exact(&mut buf)?;
        let version = u16::from_be_bytes(buf);

        let mut reserved_2 = [0u8; 2];
        dataset.read_exact(&mut reserved_2)?;

        let mut reserved_3: [u8; 16] = [0u8; 16];
        dataset.read_exact(&mut reserved_3)?;

        let mut reserved_4: [u8; 16] = [0u8; 16];
        dataset.read_exact(&mut reserved_4)?;

        let mut reserved_5: [u8; 32] = [0u8; 32];
        dataset.read_exact(&mut reserved_5)?;

        let mut buf: [u8; 2] = [0u8; 2];
        dataset.read_exact(&mut buf)?;
        let app_ctx = ApplicationContextItem::read(&mut dataset, buf[1])?;

        let mut pres_ctxs: Vec<AssocACPresentationContext> = Vec::new();
        dataset.read_exact(&mut buf)?;
        while buf[0] == u8::from(&PduType::AssocACPresentationContext) {
            let pres_ctx = AssocACPresentationContext::read(&mut dataset, buf[1])?;
            pres_ctxs.push(pres_ctx);
            dataset.read_exact(&mut buf)?;
        }

        let user_info = UserInformationItem::read(&mut dataset, buf[1])?;

        Ok(Self {
            reserved_1: reserved,
            length,
            version,
            reserved_2,
            reserved_3,
            reserved_4,
            reserved_5,
            app_ctx,
            pres_ctxs,
            user_info,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AssocRJ {
    /// Reserved, should be zero.
    reserved_1: u8,
    length: u32,
    /// Reserved, should be zero.
    reserved_2: u8,
    result: u8,
    source: u8,
    reason: u8,
}

impl AssocRJ {
    /// The type of this PDU, `PduType::AssocRJ`.
    #[must_use]
    pub fn pdu_type(&self) -> PduType {
        PduType::AssocRJ
    }

    /// Create a new `AssocRJ`.
    ///
    /// # Notes
    /// `result`, whether the rejection is transient or permanent.
    ///   - 1: Rejected-permanent.
    ///   - 2: Rejected-transient.
    ///
    /// `source`, source of the rejection.
    ///   - 1: DICOM Upper Layer Service User.
    ///   - 2: DICOM Upper Layer Service Provider (ACSE-related function).
    ///   - 3: DICOM Upper Layer Service Provider (Presentation-related function).
    ///
    /// `reason`, why the association is rejected.
    ///   - If `source` is 1,
    ///     - 1: No reason given.
    ///     - 2: Application Context Name not supported.
    ///     - 3: Calling AE Title not recognized.
    ///     - 4-6: Reserved.
    ///     - 7: Called AE Title not recognized.
    ///     - 8-10: Reserved.
    ///   - If `source` is 2,
    ///     - 1: No reason given.
    ///     - 2: Protocol version not supported.
    ///   - If `source` is 3,
    ///     - 0: Reserved.
    ///     - 1: Temporary congestion.
    ///     - 2: Local limit exceeded.
    ///     - 3-7: Reserved.
    #[must_use]
    pub fn new(result: u8, source: u8, reason: u8) -> Self {
        let length: usize = size_of::<u8>() // reserved_2
            + size_of::<u8>() // result
            + size_of::<u8>() // source
            + size_of::<u8>(); // reason

        Self {
            reserved_1: 0u8,
            length: length.try_into().unwrap_or_default(),
            reserved_2: 0u8,
            result,
            source,
            reason,
        }
    }

    /// The number of bytes from the first byte of the following field to the last byte of the
    /// Reason/Diag. field. This should be a fixed value of 4.
    #[must_use]
    pub fn length(&self) -> u32 {
        self.length
    }

    /// Result.
    ///
    /// # Notes
    /// - 1: Rejected-permanent.
    /// - 2: Rejected-transient.
    #[must_use]
    pub fn result(&self) -> u8 {
        self.result
    }

    /// Source.
    ///
    /// # Notes
    /// - 1: DICOM Upper Layer Service User.
    /// - 2: DICOM Upper Layer Service Provider (ACSE-related function).
    /// - 3: DICOM Upper Layer Service Provider (Presentation-related function).
    #[must_use]
    pub fn source(&self) -> u8 {
        self.source
    }

    /// Reason / Diagnosis for the rejection.
    ///
    /// # Notes
    /// - If `source` is 1,
    ///   - 1: No reason given.
    ///   - 2: Application Context Name not supported.
    ///   - 3: Calling AE Title not recognized.
    ///   - 4-6: Reserved.
    ///   - 7: Called AE Title not recognized.
    ///   - 8-10: Reserved.
    /// - If `source` is 2,
    ///   - 1: No reason given.
    ///   - 2: Protocol version not supported.
    /// - If `source` is 3,
    ///   - 0: Reserved.
    ///   - 1: Temporary congestion.
    ///   - 2: Local limit exceeded.
    ///   - 3-7: Reserved.
    #[must_use]
    pub fn reason(&self) -> u8 {
        self.reason
    }

    /// Returns a description of the reason for A-ASSOCIATION-RJ.
    #[must_use]
    pub fn get_reason_desc(&self) -> &'static str {
        if self.source == 1 {
            match self.reason {
                1 => "No reason given.",
                2 => "Application Context Name not supported.",
                3 => "Calling AE Title not recognized.",
                7 => "Called AE Title not recognized.",
                _ => "Unknown reason.",
            }
        } else if self.source == 2 {
            match self.reason {
                1 => "No reason given.",
                2 => "Protocol version not supported.",
                _ => "Unknown reason.",
            }
        } else if self.source == 3 {
            match self.reason {
                1 => "Temporary congestion.",
                2 => "Local limit exceeded.",
                _ => "Unknown reason.",
            }
        } else {
            "Unknown reason."
        }
    }

    /// The total number of bytes that this PDU will require to write to a dataset.
    #[must_use]
    pub fn byte_size(&self) -> usize {
        size_of::<u8>() // pdu_type
            + size_of::<u8>() // reserved_1
            + size_of::<u32>() // length
            + size_of::<u8>() // reserved_2
            + size_of::<u8>() // result
            + size_of::<u8>() // source
            + size_of::<u8>() // reason
    }

    /// Write this PDU to the given dataset.
    ///
    /// # Errors
    /// I/O errors writing to the dataset.
    pub fn write<W: Write>(&self, mut dataset: W) -> Result<(), DimseError> {
        let buf: [u8; 2] = [u8::from(&self.pdu_type()), self.reserved_1];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.length.to_be_bytes())?;

        let buf: [u8; 4] = [self.reserved_2, self.result, self.source, self.reason];
        dataset.write_all(&buf)?;

        Ok(())
    }

    /// Read a `AssocRJ` from the given dataset.
    ///
    /// # Errors
    /// I/O errors reading from the dataset.
    pub fn read<R: Read>(mut dataset: R, reserved: u8) -> Result<Self, DimseError> {
        let mut buf: [u8; 4] = [0u8; 4];
        dataset.read_exact(&mut buf)?;
        let length = u32::from_be_bytes(buf);

        dataset.read_exact(&mut buf)?;
        let reserved_2 = buf[0];
        let result = buf[1];
        let source = buf[2];
        let reason = buf[3];

        Ok(Self {
            reserved_1: reserved,
            length,
            reserved_2,
            result,
            source,
            reason,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ReleaseRQ {
    /// Reserved, should be zero.
    reserved_1: u8,
    length: u32,
    /// Reserved, should be zeros.
    reserved_2: [u8; 4],
}

impl ReleaseRQ {
    /// The type of this PDU, `PduType::ReleaseRQ`.
    #[must_use]
    pub fn pdu_type(&self) -> PduType {
        PduType::ReleaseRQ
    }

    /// Create a new `ReleaseRQ`.
    #[must_use]
    pub fn new() -> Self {
        let length: usize = size_of::<[u8; 4]>(); // reserved_2

        Self {
            reserved_1: 0u8,
            length: length.try_into().unwrap_or_default(),
            reserved_2: [0u8; 4],
        }
    }

    /// The number of bytes from the first byte of the following field to the last byte of the
    /// reserved field. This should be a fixed value of 4.
    #[must_use]
    pub fn length(&self) -> u32 {
        self.length
    }

    /// The total number of bytes that this PDU will require to write to a dataset.
    #[must_use]
    pub fn byte_size(&self) -> usize {
        size_of::<u8>() // pdu_type
            + size_of::<u8>() // reserved_1
            + size_of::<u32>() // length
            + self.reserved_2.len()
    }

    /// Write this PDU to the given dataset.
    ///
    /// # Errors
    /// I/O errors writing to the dataset.
    pub fn write<W: Write>(&self, mut dataset: W) -> Result<(), DimseError> {
        let buf: [u8; 2] = [u8::from(&self.pdu_type()), self.reserved_1];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.length.to_be_bytes())?;
        dataset.write_all(&self.reserved_2)?;

        Ok(())
    }

    /// Read a `ReleaseRQ` from the given dataset.
    ///
    /// # Errors
    /// I/O errors reading from the dataset.
    pub fn read<R: Read>(mut dataset: R, reserved: u8) -> Result<Self, DimseError> {
        let mut buf: [u8; 4] = [0u8; 4];
        dataset.read_exact(&mut buf)?;
        let length = u32::from_be_bytes(buf);

        dataset.read_exact(&mut buf)?;

        Ok(Self {
            reserved_1: reserved,
            length,
            reserved_2: buf,
        })
    }
}

impl Default for ReleaseRQ {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ReleaseRP {
    /// Reserved, should be zero.
    reserved_1: u8,
    length: u32,
    /// Reserved, should be zeros.
    reserved_2: [u8; 4],
}

impl ReleaseRP {
    /// The type of this PDU, `PduType::ReleaseRP`.
    #[must_use]
    pub fn pdu_type(&self) -> PduType {
        PduType::ReleaseRP
    }

    /// Create a new `ReleaseRP`.
    #[must_use]
    pub fn new() -> Self {
        let length: usize = size_of::<[u8; 4]>(); // reserved_2

        Self {
            reserved_1: 0u8,
            length: length.try_into().unwrap_or_default(),
            reserved_2: [0u8; 4],
        }
    }

    /// The number of bytes from the first byte of the following field to the last byte of the
    /// reserved field. This should be a fixed value of 4.
    #[must_use]
    pub fn length(&self) -> u32 {
        self.length
    }

    /// The total number of bytes that this PDU will require to write to a dataset.
    #[must_use]
    pub fn byte_size(&self) -> usize {
        size_of::<u8>() // pdu_type
            + size_of::<u8>() // reserved_1
            + size_of::<u32>() // length
            + self.reserved_2.len()
    }

    /// Write this PDU to the given dataset.
    ///
    /// # Errors
    /// I/O errors writing to the dataset.
    pub fn write<W: Write>(&self, mut dataset: W) -> Result<(), DimseError> {
        let buf: [u8; 2] = [u8::from(&self.pdu_type()), self.reserved_1];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.length.to_be_bytes())?;
        dataset.write_all(&self.reserved_2)?;

        Ok(())
    }

    /// Read a `ReleaseRP` from the given dataset.
    ///
    /// # Errors
    /// I/O errors reading from the dataset.
    pub fn read<R: Read>(mut dataset: R, reserved: u8) -> Result<Self, DimseError> {
        let mut buf: [u8; 4] = [0u8; 4];
        dataset.read_exact(&mut buf)?;
        let length = u32::from_be_bytes(buf);

        dataset.read_exact(&mut buf)?;

        Ok(Self {
            reserved_1: reserved,
            length,
            reserved_2: buf,
        })
    }
}

impl Default for ReleaseRP {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Abort {
    /// Reserved, should be zero.
    reserved_1: u8,
    length: u32,
    /// Reserved, should be zero.
    reserved_2: u8,
    /// Reserved, should be zero.
    reserved_3: u8,
    source: u8,
    reason: u8,
}

impl Abort {
    /// The type of this PDU, `PduType::Abort`.
    #[must_use]
    pub fn pdu_type(&self) -> PduType {
        PduType::Abort
    }

    /// Create a new `Abort`.
    ///
    /// # Notes
    /// `source`, the source/initiator of the abort.
    ///   - 0: DICOM Upper Layer service-user.
    ///   - 1: Reserved.
    ///   - 2: DICOM Upper Layer service-provider.
    ///
    /// `reason`, the reason for the abort, only if `source` isn't `0`.
    ///   - 0: Reason not specified.
    ///   - 1: Unrecognized PDU.
    ///   - 2: Unexpected PDU.
    ///   - 3: Reserved / Unexpected session-service primitive.
    ///   - 4: Unrecognized PDU parameter.
    ///   - 5: Unexpected PDU parameter.
    ///   - 6: Invalid PDU parameter.
    #[must_use]
    pub fn new(source: u8, reason: u8) -> Self {
        let length: usize = size_of::<u8>() // reserved_2
            + size_of::<u8>() // reserved_3
            + size_of::<u8>() // source
            + size_of::<u8>(); // reason

        Self {
            reserved_1: 0u8,
            length: length.try_into().unwrap_or_default(),
            reserved_2: 0u8,
            reserved_3: 0u8,
            source,
            reason,
        }
    }

    /// The number of bytes from the first byte of the following field to the last byte of the
    /// Reason/Diag. field. This should be a fixed value of 4.
    #[must_use]
    pub fn length(&self) -> u32 {
        self.length
    }

    /// Source of the Abort.
    ///
    /// # Notes
    /// 0: DICOM Upper Layer service-user (initiated abort).
    /// 1: Reserved.
    /// 2: DICOM Upper Layer service-provider (initiated abort).
    #[must_use]
    pub fn source(&self) -> u8 {
        self.source
    }

    /// Reason/Diagnosis for the Abort.
    ///
    /// # Notes
    /// 0: Reason not specified.
    /// 1: Unrecognized PDU.
    /// 2: Unexpected PDU.
    /// 3: Reserved.
    /// 4: Unrecognized PDU parameter.
    /// 5: Unexpected PDU parameter.
    /// 6: Invalid PDU parameter.
    ///
    /// If the source field has a value of 0 (service-user) then this field shall not be
    /// significant and should be set to zero, but unchecked.
    #[must_use]
    pub fn reason(&self) -> u8 {
        self.reason
    }

    /// Returns a description of the reason for the A-ABORT.
    #[must_use]
    pub fn get_reason_desc(&self) -> &'static str {
        match self.reason {
            0 => "Not-specified",
            1 => "Unrecognized PDU",
            2 => "Unexpected PDU",
            3 => "Unexpected session-service primitive",
            4 => "Unrecognized PDU parameter",
            5 => "Unexpected PDU parameter",
            6 => "Invalid PDU parameter value",
            _ => "Unrecognized reason.",
        }
    }

    /// The total number of bytes that this PDU will require to write to a dataset.
    #[must_use]
    pub fn byte_size(&self) -> usize {
        size_of::<u8>() // pdu_type
            + size_of::<u8>() // reserved_1
            + size_of::<u32>() // length
            + size_of::<u8>() // reserved_2
            + size_of::<u8>() // reserved_3
            + size_of::<u8>() // source
            + size_of::<u8>() // reason
    }

    /// Write this PDU to the given dataset.
    ///
    /// # Errors
    /// I/O errors writing to the dataset.
    pub fn write<W: Write>(&self, mut dataset: W) -> Result<(), DimseError> {
        let buf: [u8; 2] = [u8::from(&self.pdu_type()), self.reserved_1];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.length.to_be_bytes())?;

        let buf: [u8; 4] = [self.reserved_2, self.reserved_3, self.source, self.reason];
        dataset.write_all(&buf)?;

        Ok(())
    }

    /// Read a `Abort` from the given dataset.
    ///
    /// # Errors
    /// I/O errors reading from the dataset.
    pub fn read<R: Read>(mut dataset: R, reserved: u8) -> Result<Self, DimseError> {
        let mut buf: [u8; 4] = [0u8; 4];
        dataset.read_exact(&mut buf)?;
        let length = u32::from_be_bytes(buf);

        dataset.read_exact(&mut buf)?;
        let reserved_2 = buf[0];
        let reserved_3 = buf[1];
        let source = buf[2];
        let reason = buf[3];

        Ok(Self {
            reserved_1: reserved,
            length,
            reserved_2,
            reserved_3,
            source,
            reason,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PresentationDataItem {
    /// Reserved, should be zero.
    reserved: u8,
    length: u32,
    pres_data: Vec<PresentationDataValue>,
}

impl PresentationDataItem {
    /// The type of this PDU, `PduType::PresentationDataItem`.
    #[must_use]
    pub fn pdu_type(&self) -> PduType {
        PduType::PresentationDataItem
    }

    /// Create a new `PresentationDataItem`.
    #[must_use]
    pub fn new(pres_data: Vec<PresentationDataValue>) -> Self {
        let length: usize = pres_data
            .iter()
            .map(PresentationDataValue::byte_size)
            .sum::<usize>();

        Self {
            reserved: 0u8,
            length: length.try_into().unwrap_or_default(),
            pres_data,
        }
    }

    /// The number of bytes from the first byte of the following field to the last byte of the
    /// variable field (Presentation Data values).
    #[must_use]
    pub fn length(&self) -> u32 {
        self.length
    }

    /// Presentation Data values.
    #[must_use]
    pub fn pres_data(&self) -> &Vec<PresentationDataValue> {
        &self.pres_data
    }

    /// The total number of bytes that this PDU will require to write to a dataset.
    #[must_use]
    pub fn byte_size(&self) -> usize {
        Self::header_byte_size()
            + self
                .pres_data
                .iter()
                .map(PresentationDataValue::byte_size)
                .sum::<usize>()
    }

    /// The number of bytes that make up the header of `PresentationDataItem`, everything up
    /// through the variable-sized presentation data values field.
    #[must_use]
    pub fn header_byte_size() -> usize {
        size_of::<u8>() // pdu_type
            + size_of::<u8>() // reserved
            + size_of::<u32>() // length
    }

    /// Write this PDU to the given dataset.
    ///
    /// # Errors
    /// I/O errors writing to the dataset.
    pub fn write<W: Write>(&self, mut dataset: W) -> Result<(), DimseError> {
        let buf: [u8; 2] = [u8::from(&self.pdu_type()), self.reserved];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.length.to_be_bytes())?;
        for value in &self.pres_data {
            value.write(&mut dataset)?;
        }

        Ok(())
    }

    /// Read a `PresentationDataItem` from the given dataset.
    ///
    /// # Notes
    /// This will read the entire PDU field into memory, which may be large. Utilize the
    /// `MaxLengthItem` during association negotiation to require smaller chunked data values for
    /// more efficitent memory usage.
    ///
    /// # Errors
    /// I/O errors reading from the dataset.
    pub fn read<R: Read>(mut dataset: R, reserved: u8) -> Result<Self, DimseError> {
        let mut buf: [u8; 4] = [0u8; 4];
        dataset.read_exact(&mut buf)?;
        let length = u32::from_be_bytes(buf);

        let mut pres_data: Vec<PresentationDataValue> = Vec::new();
        let mut pres_data_len_marker = usize::try_from(length).unwrap_or_default();
        while pres_data_len_marker > 0 {
            let value = PresentationDataValue::read(&mut dataset)?;
            // The total bytes read for a single data value is 4 for the length field and then the
            // value of the length field which includes the ctx_id, msg_header, and data fields.
            let total_value_size =
                size_of::<u32>() + usize::try_from(value.length()).unwrap_or_default();
            pres_data_len_marker -= total_value_size;
            pres_data.push(value);
        }

        Ok(Self {
            reserved,
            length,
            pres_data,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PresentationDataValue {
    length: u32,
    ctx_id: u8,
    msg_header: u8,
    data: Vec<u8>,
}

impl PresentationDataValue {
    /// Create a new `PresentationDataValue`.
    ///
    /// # Notes
    /// `msg_header`, indicates the type of message. Use the `P_DATA_` constants.
    /// LSB 0,
    ///   0: The message contains a DICOM Data Set.
    ///   1: The message contains a Command.
    ///
    /// LSB 1,
    ///   0: The message fragment is not the last fragment.
    ///   1: The message fragment is the last fragment.
    ///
    /// The other bits shall be zeros, but unchecked.
    #[must_use]
    pub fn new(ctx_id: u8, msg_header: u8, data: Vec<u8>) -> Self {
        let length: usize = size_of::<u8>() // ctx_id
            + size_of::<u8>() // msg_header
            + data.len();

        Self {
            length: length.try_into().unwrap_or_default(),
            ctx_id,
            msg_header,
            data,
        }
    }

    /// The number of bytes from the first byte of the following field to the last byte of the
    /// presentation data value field.
    #[must_use]
    pub fn length(&self) -> u32 {
        self.length
    }

    /// Context ID, an odd number between 1-255.
    #[must_use]
    pub fn ctx_id(&self) -> u8 {
        self.ctx_id
    }

    /// Message Header, interpreted as bit fields.
    ///
    /// Refer to `is_command()` and `is_last_fragment()` to assist with interpreting this field.
    ///
    /// # Notes
    /// LSB 0,
    ///   0: The message contains a DICOM Data Set.
    ///   1: The message contains a Command.
    ///
    /// LSB 1,
    ///   0: The message fragment is not the last fragment.
    ///   1: The message fragment is the last fragment.
    ///
    /// The other bits shall be zeros, but unchecked.
    #[must_use]
    pub fn msg_header(&self) -> u8 {
        self.msg_header
    }

    /// Presentation data, a fragment. Either a Command or a DICOM Data Set.
    #[must_use]
    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }

    /// Presentation data, a fragment. Either a Command or a DICOM Data Set. Consumes this
    /// `PresentationDataValue`.
    #[must_use]
    pub fn into_data(self) -> Vec<u8> {
        self.data
    }

    /// The total number of bytes that this PDU will require to write to a dataset.
    #[must_use]
    pub fn byte_size(&self) -> usize {
        Self::header_byte_size() + self.data.len()
    }

    /// The number of bytes that make up the header of `PresentationDataValue`, everything up
    /// through the variable-sized data field.
    #[must_use]
    pub fn header_byte_size() -> usize {
        size_of::<u32>() // length
            + size_of::<u8>() // ctx_id
            + size_of::<u8>() // msg_header
    }

    /// Returns true if this value is a command message, false for a dicom dataset.
    #[must_use]
    pub fn is_command(&self) -> bool {
        self.msg_header & 0b01 == 0b01
    }

    /// Returns true if this value is the last fragment in a presentation data item, false if not.
    #[must_use]
    pub fn is_last_fragment(&self) -> bool {
        self.msg_header & 0b10 == 0b10
    }

    /// Write this PDU to the given dataset.
    ///
    /// # Errors
    /// I/O errors writing to the dataset.
    pub fn write<W: Write>(&self, mut dataset: W) -> Result<(), DimseError> {
        dataset.write_all(&self.length.to_be_bytes())?;

        let buf: [u8; 2] = [self.ctx_id, self.msg_header];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.data)?;

        Ok(())
    }

    /// Read a `PresentationDataValue` from the given dataset.
    ///
    /// # Notes
    /// This will read the entire PDU field into memory, which may be large. Utilize the
    /// `MaxLengthItem` during association negotiation to require smaller chunked data values for
    /// more efficitent memory usage.
    ///
    /// # Errors
    /// I/O errors reading from the dataset.
    pub fn read<R: Read>(mut dataset: R) -> Result<Self, DimseError> {
        let mut buf: [u8; 4] = [0u8; 4];
        dataset.read_exact(&mut buf)?;
        let length = u32::from_be_bytes(buf);

        let mut buf: [u8; 2] = [0u8; 2];
        dataset.read_exact(&mut buf)?;
        let ctx_id = buf[0];
        let msg_header = buf[1];

        let data_len = usize::try_from(length).unwrap_or_default() - size_of::<[u8; 2]>();
        let mut data: Vec<u8> = vec![0u8; data_len];
        dataset.read_exact(&mut data)?;

        Ok(Self {
            length,
            ctx_id,
            msg_header,
            data,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PresentationDataItemPartial {
    /// Reserved, should be zero.
    reserved: u8,
    length: u32,
}

impl PresentationDataItemPartial {
    /// The type of this PDU, `PduType::PresentationDataItemPartial`.
    #[must_use]
    pub fn pdu_type(&self) -> PduType {
        PduType::PresentationDataItemPartial
    }

    /// Create a new `PresentationDataItemPartial`.
    #[must_use]
    pub fn new(length: u32) -> Self {
        Self {
            reserved: 0,
            length,
        }
    }

    /// The number of bytes from the first byte of the following field to the last byte of the
    /// variable field (Presentation Data values).
    #[must_use]
    pub fn length(&self) -> u32 {
        self.length
    }

    /// The total number of bytes that this PDU will require to write to a dataset.
    ///
    /// # Notes
    /// Unlike the `read()` and `write()` functions, this value _DOES_ include the length of the
    /// value fields, as it must be known ahead of time when preparing to write this PDU to a
    /// dataset.
    #[must_use]
    pub fn byte_size(&self) -> usize {
        size_of::<u8>() // pdu_type
            + size_of::<u8>() // reserved
            + size_of::<u32>() // length
            + usize::try_from(self.length).unwrap_or_default()
    }

    /// Write this _partial_ PDU to the given dataset.
    ///
    /// # Notes
    /// This does not write the value fields of the `PresentationDataItem` PDU structure. This is
    /// to allow the caller to determine the best way to handle the variable-sized value fields,
    /// which can be large. Including the value fields as part of this structure would require the
    /// data to be fully loaded into memory.
    ///
    /// # Errors
    /// I/O errors writing to the dataset.
    pub fn write<W: Write>(&self, mut dataset: W) -> Result<(), DimseError> {
        let buf: [u8; 2] = [u8::from(&self.pdu_type()), self.reserved];
        dataset.write_all(&buf)?;
        dataset.write_all(&self.length.to_be_bytes())?;
        Ok(())
    }

    /// Read a _partial_ `PresentationDataItem` from the given dataset.
    ///
    /// # Notes
    /// This does not read the value fields from the dataset. This is to allow the caller to
    /// determine the best way to handle the variable-sized value fields, which can be large.
    /// Reading the value fields into a field here would require loading the entire values into
    /// memory.
    ///
    /// # Errors
    /// I/O errors reading from the dataset.
    pub fn read<R: Read>(mut dataset: R, reserved: u8) -> Result<Self, DimseError> {
        let mut buf: [u8; 4] = [0u8; 4];
        dataset.read_exact(&mut buf)?;
        let length = u32::from_be_bytes(buf);

        Ok(Self { reserved, length })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ApplicationContextItem {
    /// Reserved, should be zero.
    reserved: u8,
    length: u16,
    app_context_name: Vec<u8>,
}

impl ApplicationContextItem {
    /// The type of this PDU, `PduType::ApplicationContextItem`.
    #[must_use]
    pub fn pdu_type(&self) -> PduType {
        PduType::ApplicationContextItem
    }

    /// Create a new `ApplicationContextItem`.
    #[must_use]
    pub fn new(mut app_context_name: Vec<u8>) -> Self {
        if app_context_name.len() % 2 != 0 {
            app_context_name.push(UI.padding);
        }

        let length: u16 = app_context_name.len().try_into().unwrap_or_default();

        Self {
            reserved: 0u8,
            length,
            app_context_name,
        }
    }

    /// The number of bytes from the first byte of the following field to the last byte of the
    /// application context field name.
    #[must_use]
    pub fn length(&self) -> u16 {
        self.length
    }

    /// A valid application context name (essentially a UID).
    #[must_use]
    pub fn app_context_name(&self) -> &Vec<u8> {
        &self.app_context_name
    }

    /// The total number of bytes that this PDU will require to write to a dataset.
    #[must_use]
    pub fn byte_size(&self) -> usize {
        size_of::<u8>() // pdu_type
            + size_of::<u8>() // reserved
            + size_of::<u16>() // length
            + self.app_context_name.len()
    }

    /// Write this PDU to the given dataset.
    ///
    /// # Errors
    /// I/O errors writing to the dataset.
    pub fn write<W: Write>(&self, mut dataset: W) -> Result<(), DimseError> {
        let buf: [u8; 2] = [u8::from(&self.pdu_type()), self.reserved];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.length.to_be_bytes())?;
        dataset.write_all(&self.app_context_name)?;

        Ok(())
    }

    /// Read a `ApplicationContextItem` from the given dataset.
    ///
    /// # Errors
    /// I/O errors reading from the dataset.
    pub fn read<R: Read>(mut dataset: R, reserved: u8) -> Result<Self, DimseError> {
        let mut buf: [u8; 2] = [0u8; 2];
        dataset.read_exact(&mut buf)?;
        let length = u16::from_be_bytes(buf);

        let mut app_context_name: Vec<u8> = vec![0u8; length.into()];
        dataset.read_exact(&mut app_context_name)?;

        Ok(Self {
            reserved,
            length,
            app_context_name,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AssocRQPresentationContext {
    /// Reserved, should be zero.
    reserved_1: u8,
    length: u16,
    ctx_id: u8,
    /// Reserved, should be zero.
    reserved_2: u8,
    /// Reserved, should be zero.
    reserved_3: u8,
    /// Reserved, should be zero.
    reserved_4: u8,
    abstract_syntax: AbstractSyntaxItem,
    transfer_syntaxes: Vec<TransferSyntaxItem>,
}

impl AssocRQPresentationContext {
    /// The type of this PDU, `PduType::AssocRQPresentationContext`.
    #[must_use]
    pub fn pdu_type(&self) -> PduType {
        PduType::AssocRQPresentationContext
    }

    /// Create a new `AssocRQPresentationItem`.
    #[must_use]
    pub fn new(
        ctx_id: u8,
        abstract_syntax: AbstractSyntaxItem,
        transfer_syntaxes: Vec<TransferSyntaxItem>,
    ) -> Self {
        let length: usize = size_of::<u8>() // ctx_id
            + size_of::<u8>() // reserved_2
            + size_of::<u8>() // reserved_3
            + size_of::<u8>() // reserved_4
            + abstract_syntax.byte_size()
            + transfer_syntaxes.iter().map(TransferSyntaxItem::byte_size).sum::<usize>();

        Self {
            reserved_1: 0u8,
            length: length.try_into().unwrap_or_default(),
            ctx_id,
            reserved_2: 0u8,
            reserved_3: 0u8,
            reserved_4: 0u8,
            abstract_syntax,
            transfer_syntaxes,
        }
    }

    /// The number of bytes from the first byte of the following field to the last byte of the last
    /// transfer syntax item.
    #[must_use]
    pub fn length(&self) -> u16 {
        self.length
    }

    /// Presentation context ID, an odd number between 1 and 255.
    #[must_use]
    pub fn ctx_id(&self) -> u8 {
        self.ctx_id
    }

    /// The abstract syntax sub item.
    #[must_use]
    pub fn abstract_syntax(&self) -> &AbstractSyntaxItem {
        &self.abstract_syntax
    }

    /// The transfer syntax sub-items.
    #[must_use]
    pub fn transfer_syntaxes(&self) -> &Vec<TransferSyntaxItem> {
        &self.transfer_syntaxes
    }

    /// The total number of bytes that this PDU will require to write to a dataset.
    #[must_use]
    pub fn byte_size(&self) -> usize {
        size_of::<u8>() // pdu_type
            + size_of::<u8>() // reserved_1
            + size_of::<u16>() // length
            + size_of::<u8>() // ctx_id
            + size_of::<u8>() // reserved_2
            + size_of::<u8>() // reserved_3
            + size_of::<u8>() // reserved_4
            + self.abstract_syntax.byte_size()
            + self.transfer_syntaxes.iter().map(TransferSyntaxItem::byte_size).sum::<usize>()
    }

    /// Write this PDU to the given dataset.
    ///
    /// # Errors
    /// I/O errors writing to the dataset.
    pub fn write<W: Write>(&self, mut dataset: W) -> Result<(), DimseError> {
        let mut buf: [u8; 2] = [u8::from(&self.pdu_type()), self.reserved_1];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.length.to_be_bytes())?;

        buf[0] = self.ctx_id;
        buf[1] = self.reserved_2;
        dataset.write_all(&buf)?;

        buf[0] = self.reserved_3;
        buf[1] = self.reserved_4;
        dataset.write_all(&buf)?;

        self.abstract_syntax.write(&mut dataset)?;

        for transfer_syntax in &self.transfer_syntaxes {
            transfer_syntax.write(&mut dataset)?;
        }

        Ok(())
    }

    /// Read a `AssocRQPresentationContext` from the given dataset.
    ///
    /// # Errors
    /// I/O errors reading from the dataset.
    pub fn read<R: Read>(mut dataset: R, reserved: u8) -> Result<Self, DimseError> {
        let mut buf: [u8; 2] = [0u8; 2];
        dataset.read_exact(&mut buf)?;
        let length = u16::from_be_bytes(buf);

        let mut bytes_read = 0usize;
        dataset.read_exact(&mut buf)?;
        let ctx_id = buf[0];
        let reserved_2 = buf[1];
        bytes_read += buf.len();

        dataset.read_exact(&mut buf)?;
        let reserved_3 = buf[0];
        let reserved_4 = buf[1];
        bytes_read += buf.len();

        dataset.read_exact(&mut buf)?;
        let abstract_syntax = AbstractSyntaxItem::read(&mut dataset, buf[1])?;
        // The total bytes read for AbstractSyntaxItem, 2 bytes for the type and reserved, plus an
        // additional 2 for the length field, and the value of the length field being the size of
        // the last variable-length field.
        bytes_read += buf.len();
        bytes_read += usize::from(2 + abstract_syntax.length());

        // The length of total transfer syntax list is value of this PDU's length field less the
        // number of bytes before the transfer syntaxes field. There are no fields after the
        // transfer syntaxes.
        let mut transfer_syntax_len_marker = usize::from(length) - bytes_read;
        let mut transfer_syntaxes: Vec<TransferSyntaxItem> = Vec::new();
        while transfer_syntax_len_marker > 0 {
            dataset.read_exact(&mut buf)?;
            let transfer_syntax = TransferSyntaxItem::read(&mut dataset, buf[1])?;
            // Total bytes read is 2 for the type and reserved, plus another 2 for the length
            // field, and also the value of the length field being the size of the last
            // variable-length field.
            transfer_syntax_len_marker -= usize::from(4 + transfer_syntax.length());
            transfer_syntaxes.push(transfer_syntax);
        }

        Ok(Self {
            reserved_1: reserved,
            length,
            ctx_id,
            reserved_2,
            reserved_3,
            reserved_4,
            abstract_syntax,
            transfer_syntaxes,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AssocACPresentationContext {
    /// Reserved, should be zero.
    reserved_1: u8,
    length: u16,
    ctx_id: u8,
    /// Reserved, should be zero.
    reserved_2: u8,
    result: u8,
    /// Reserved, should be zero.
    reserved_3: u8,
    transfer_syntax: TransferSyntaxItem,
}

impl AssocACPresentationContext {
    /// The type of this PDU, `PduType::AssocACPresentationContext`.
    #[must_use]
    pub fn pdu_type(&self) -> PduType {
        PduType::AssocACPresentationContext
    }

    /// Create a new `AssocACPresentationContext`.
    ///
    /// # Arguments
    /// `results`
    /// - 0: Acceptance.
    /// - 1: User-rejection.
    /// - 2: No reason (provider rejection).
    /// - 3: Abstract Syntax not supported (provider rejection).
    /// - 4: Transfer Syntaxes not supported (provider rejection).
    #[must_use]
    pub fn new(ctx_id: u8, result: u8, transfer_syntax: TransferSyntaxItem) -> Self {
        let length: usize = size_of::<u8>() // ctx_id
            + size_of::<u8>() // reserved_2
            + size_of::<u8>() // result
            + size_of::<u8>() // reserved_3
            + transfer_syntax.byte_size();

        Self {
            reserved_1: 0u8,
            length: length.try_into().unwrap_or_default(),
            ctx_id,
            reserved_2: 0u8,
            result,
            reserved_3: 0u8,
            transfer_syntax,
        }
    }

    /// The number of bytes from the first byte of the following field to the last byte of the
    /// Transfer Syntax item.
    #[must_use]
    pub fn length(&self) -> u16 {
        self.length
    }

    /// Presentation context ID, an odd number between 1 and 255.
    #[must_use]
    pub fn ctx_id(&self) -> u8 {
        self.ctx_id
    }

    /// Result/Reason.
    ///
    /// # Notes
    /// - 0: Acceptance.
    /// - 1: User-rejection.
    /// - 2: No reason (provider rejection).
    /// - 3: Abstract Syntax not supported (provider rejection).
    /// - 4: Transfer Syntaxes not supported (provider rejection).
    #[must_use]
    pub fn result(&self) -> u8 {
        self.result
    }

    /// The selected transfer syntax. When the `result` field has a value other than 0, this field
    /// shall not be significant and should be ignored. The `TransferSyntaxItem` should contain
    /// only a single transfer syntax.
    #[must_use]
    pub fn transfer_syntax(&self) -> &TransferSyntaxItem {
        &self.transfer_syntax
    }

    /// The total number of bytes that this PDU will require to write to a dataset.
    #[must_use]
    pub fn byte_size(&self) -> usize {
        size_of::<u8>() // pdu_type
            + size_of::<u8>() // reserved_1
            + size_of::<u16>() // length
            + size_of::<u8>() // ctx_id
            + size_of::<u8>() // reserved_2
            + size_of::<u8>() // result
            + size_of::<u8>() // reserved_3
            + self.transfer_syntax.byte_size()
    }

    /// Convenience for checking that this presentation context has a result field indicating it
    /// was accepted.
    #[must_use]
    pub fn is_accepted(&self) -> bool {
        self.result == 0
    }

    /// Write this PDU to the given dataset.
    ///
    /// # Errors
    /// I/O errors writing to the dataset.
    pub fn write<W: Write>(&self, mut dataset: W) -> Result<(), DimseError> {
        let mut buf: [u8; 2] = [u8::from(&self.pdu_type()), self.reserved_1];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.length.to_be_bytes())?;

        buf[0] = self.ctx_id;
        buf[1] = self.reserved_2;
        dataset.write_all(&buf)?;

        buf[0] = self.result;
        buf[1] = self.reserved_3;
        dataset.write_all(&buf)?;

        self.transfer_syntax.write(&mut dataset)?;

        Ok(())
    }

    /// Read a `AssocACPresentationContext` from the given dataset.
    ///
    /// # Errors
    /// I/O errors reading from the dataset.
    pub fn read<R: Read>(mut dataset: R, reserved: u8) -> Result<Self, DimseError> {
        let mut buf: [u8; 2] = [0u8; 2];
        dataset.read_exact(&mut buf)?;
        let length = u16::from_be_bytes(buf);

        dataset.read_exact(&mut buf)?;
        let ctx_id = buf[0];
        let reserved_2 = buf[1];

        dataset.read_exact(&mut buf)?;
        let result = buf[0];
        let reserved_3 = buf[1];

        dataset.read_exact(&mut buf)?;
        let transfer_syntax = TransferSyntaxItem::read(dataset, buf[1])?;

        Ok(Self {
            reserved_1: reserved,
            length,
            ctx_id,
            reserved_2,
            result,
            reserved_3,
            transfer_syntax,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AbstractSyntaxItem {
    /// Reserved, should be zero.
    reserved: u8,
    length: u16,
    abstract_syntax: Vec<u8>,
}

impl AbstractSyntaxItem {
    /// The type of this PDU, `PduType::AbstractSyntaxItem`.
    #[must_use]
    pub fn pdu_type(&self) -> PduType {
        PduType::AbstractSyntaxItem
    }

    /// Create a new `AbstractSyntaxItem`.
    #[must_use]
    pub fn new(mut abstract_syntax: Vec<u8>) -> Self {
        if abstract_syntax.len() % 2 != 0 {
            abstract_syntax.push(UI.padding);
        }

        Self {
            reserved: 0u8,
            length: abstract_syntax.len().try_into().unwrap_or_default(),
            abstract_syntax,
        }
    }

    /// The number of bytes from the first byte of the following field to the last byte of the
    /// abstract syntax name field.
    #[must_use]
    pub fn length(&self) -> u16 {
        self.length
    }

    /// The abstract syntax UID related to the proposed presentation context.
    #[must_use]
    pub fn abstract_syntax(&self) -> &Vec<u8> {
        &self.abstract_syntax
    }

    /// The total number of bytes that this PDU will require to write to a dataset.
    #[must_use]
    pub fn byte_size(&self) -> usize {
        size_of::<u8>() // pdu_type
            + size_of::<u8>() // reserved
            + size_of::<u16>() // length
            + self.abstract_syntax.len()
    }

    /// Write this PDU to the given dataset.
    ///
    /// # Errors
    /// I/O errors writing to the dataset.
    pub fn write<W: Write>(&self, mut dataset: W) -> Result<(), DimseError> {
        let buf: [u8; 2] = [u8::from(&self.pdu_type()), self.reserved];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.length.to_be_bytes())?;
        dataset.write_all(&self.abstract_syntax)?;

        Ok(())
    }

    /// Read a `AbstractSyntaxItem` from the given dataset.
    ///
    /// # Errors
    /// I/O errors reading from the dataset.
    pub fn read<R: Read>(mut dataset: R, reserved: u8) -> Result<Self, DimseError> {
        let mut buf: [u8; 2] = [0u8; 2];
        dataset.read_exact(&mut buf)?;
        let length = u16::from_be_bytes(buf);

        let mut abstract_syntax: Vec<u8> = vec![0u8; length.into()];
        dataset.read_exact(&mut abstract_syntax)?;

        Ok(Self {
            reserved,
            length,
            abstract_syntax,
        })
    }
}

impl From<UIDRef> for AbstractSyntaxItem {
    fn from(value: UIDRef) -> Self {
        let mut bytes = Vec::from(value.uid().as_bytes());
        if bytes.len() % 2 != 0 {
            bytes.push(UI.padding);
        }
        Self::new(bytes)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TransferSyntaxItem {
    /// Reserved, should be zero.
    reserved: u8,
    length: u16,
    transfer_syntaxes: Vec<u8>,
}

impl TransferSyntaxItem {
    /// The type of this PDU, `PduType::TransferSyntaxItem`.
    #[must_use]
    pub fn pdu_type(&self) -> PduType {
        PduType::TransferSyntaxItem
    }

    /// Create a new `TransferSyntaxItem`.
    #[must_use]
    pub fn new(mut transfer_syntaxes: Vec<u8>) -> Self {
        if transfer_syntaxes.len() % 2 != 0 {
            transfer_syntaxes.push(UI.padding);
        }

        Self {
            reserved: 0u8,
            length: transfer_syntaxes.len().try_into().unwrap_or_default(),
            transfer_syntaxes,
        }
    }

    /// The number of bytes from the first byte of the following field to the last byte of the
    /// transfer syntax name fields.
    #[must_use]
    pub fn length(&self) -> u16 {
        self.length
    }

    /// The transfer syntax UIDs related to the proposed presentation context.
    #[must_use]
    pub fn transfer_syntaxes(&self) -> &Vec<u8> {
        &self.transfer_syntaxes
    }

    /// The total number of bytes that this PDU will require to write to a dataset.
    #[must_use]
    pub fn byte_size(&self) -> usize {
        size_of::<u8>() // pdu_type
            + size_of::<u8>() // reserved
            + size_of::<u16>() // length
            + self.transfer_syntaxes.len()
    }

    /// Write this PDU to the given dataset.
    ///
    /// # Errors
    /// I/O errors writing to the dataset.
    pub fn write<W: Write>(&self, mut dataset: W) -> Result<(), DimseError> {
        let buf: [u8; 2] = [u8::from(&self.pdu_type()), self.reserved];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.length.to_be_bytes())?;
        dataset.write_all(&self.transfer_syntaxes)?;

        Ok(())
    }

    /// Read a `TransferSyntaxItem` from the given dataset.
    ///
    /// # Errors
    /// I/O errors reading from the dataset.
    pub fn read<R: Read>(mut dataset: R, reserved: u8) -> Result<Self, DimseError> {
        let mut buf: [u8; 2] = [0u8; 2];
        dataset.read_exact(&mut buf)?;
        let length = u16::from_be_bytes(buf);

        let mut transfer_syntaxes: Vec<u8> = vec![0u8; length.into()];
        dataset.read_exact(&mut transfer_syntaxes)?;

        Ok(Self {
            reserved,
            length,
            transfer_syntaxes,
        })
    }
}

impl From<TSRef> for TransferSyntaxItem {
    fn from(value: TSRef) -> Self {
        let mut bytes = Vec::from(value.uid().uid().as_bytes());
        if bytes.len() % 2 != 0 {
            bytes.push(UI.padding);
        }
        Self::new(bytes)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UserInformationItem {
    /// Reserved, should be zero.
    reserved: u8,
    length: u16,
    user_data: Vec<UserPdu>,
}

impl UserInformationItem {
    /// The type of this PDU, `PduType::UserInformationItem`.
    #[must_use]
    pub fn pdu_type(&self) -> PduType {
        PduType::UserInformationItem
    }

    /// Create a new `UserInformationItem`.
    pub fn new(user_data: Vec<UserPdu>) -> Self {
        let length = u16::try_from(user_data.iter().map(UserPdu::byte_size).sum::<usize>())
            .unwrap_or_default();
        Self {
            reserved: 0u8,
            length,
            user_data,
        }
    }

    /// The number of bytes from the first byte of the following field to the last byte of the user
    /// data fields.
    #[must_use]
    pub fn length(&self) -> u16 {
        self.length
    }

    /// User-data sub-items.
    #[must_use]
    pub fn user_data(&self) -> &Vec<UserPdu> {
        &self.user_data
    }

    /// The total number of bytes that this PDU will require to write to a dataset.
    pub fn byte_size(&self) -> usize {
        size_of::<u8>() // pdu_type
            + size_of::<u8>() // reserved
            + size_of::<u16>() // length
            + self.user_data.iter().map(UserPdu::byte_size).sum::<usize>()
    }

    /// Write this PDU to the given dataset.
    ///
    /// # Errors
    /// I/O errors writing to the dataset.
    pub fn write<W: Write>(&self, mut dataset: W) -> Result<(), DimseError> {
        let buf: [u8; 2] = [u8::from(&self.pdu_type()), self.reserved];
        dataset.write_all(&buf)?;

        dataset.write_all(&self.length.to_be_bytes())?;
        for pdu in &self.user_data {
            pdu.write(&mut dataset)?;
        }

        Ok(())
    }

    /// Read a `UserInformationItem` from the given dataset.
    ///
    /// # Errors
    /// I/O errors reading from the dataset.
    pub fn read<R: Read>(mut dataset: R, reserved: u8) -> Result<Self, DimseError> {
        let mut buf: [u8; 2] = [0u8; 2];
        dataset.read_exact(&mut buf)?;
        let length = u16::from_be_bytes(buf);

        let mut user_data: Vec<UserPdu> = Vec::new();
        let mut bytes_left = usize::from(length);
        while bytes_left > 0 {
            let pdu = UserPdu::read(&mut dataset)?;
            bytes_left -= pdu.byte_size();
            user_data.push(pdu);
        }

        Ok(Self {
            reserved,
            length,
            user_data,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::dimse::{
        pdus::{
            mainpdus::{
                Abort, AbstractSyntaxItem, ApplicationContextItem, AssocAC,
                AssocACPresentationContext, AssocRJ, AssocRQ, AssocRQPresentationContext, PduType,
                PresentationDataItem, PresentationDataValue, ReleaseRP, ReleaseRQ,
                TransferSyntaxItem, UserInformationItem, UserPdu,
            },
            userpdus::{MaxLengthItem, UserIdentityItem},
            Pdu,
        },
        AeTitle,
    };

    /// Creates a dummy `AssocRQ` populated
    fn create_assoc_rq() -> AssocRQ {
        let ae_dest: AeTitle = "AE_DEST".as_bytes().try_into().expect("create ae_dest");
        let ae_source: AeTitle = "AE_SOURCE".as_bytes().try_into().expect("create ae_source");
        let app_context_name: Vec<u8> = "My Sever App Context v1.1".into();
        let pres_ctxs: Vec<AssocRQPresentationContext> = vec![
            AssocRQPresentationContext::new(
                4u8,
                AbstractSyntaxItem::new("1.2.3.4".into()),
                vec![
                    TransferSyntaxItem::new("2.3.4.5".into()),
                    TransferSyntaxItem::new("3.4.5.6.7".into()),
                ],
            ),
            AssocRQPresentationContext::new(
                4u8,
                AbstractSyntaxItem::new("2.3.4.5.6".into()),
                vec![
                    TransferSyntaxItem::new("2.3.4.5".into()),
                    TransferSyntaxItem::new("3.4.5.6.7".into()),
                ],
            ),
        ];
        let user_data: Vec<UserPdu> = vec![
            UserPdu::MaxLengthItem(MaxLengthItem::new(101)),
            UserPdu::UserIdentityItem(UserIdentityItem::new(1, 2, vec![3, 4, 5], vec![6, 7, 8, 9])),
        ];
        let user_info = UserInformationItem::new(user_data);

        let app_ctx = ApplicationContextItem::new(app_context_name);
        AssocRQ::new(
            ae_dest.into(),
            ae_source.into(),
            app_ctx,
            pres_ctxs,
            user_info,
        )
    }

    /// Writes a PDU to an in-memory buffer, reads that buffer into a PDU, and asserts the result
    /// is equal to the input.
    fn assert_pdu_roundtrip(pdu: Pdu) {
        let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());

        pdu.write(&mut cursor).expect("unable to write pdu");

        // reset the position so reading from it starts at the beginning
        cursor.set_position(0);

        let roundtrip_pdu = Pdu::read(&mut cursor).expect("unable to read pdu");
        assert_eq!(roundtrip_pdu, pdu);
    }

    #[test]
    fn test_assoc_rq_roundtrip() {
        let pdu = create_assoc_rq();
        assert_pdu_roundtrip(Pdu::AssocRQ(pdu));
    }

    #[test]
    fn test_assoc_ac_roundtrip() {
        let assrq_pdu = create_assoc_rq();

        let pdu = AssocAC::new(
            assrq_pdu.called_ae().to_owned(),
            assrq_pdu.calling_ae().to_owned(),
            assrq_pdu.reserved_3().to_owned(),
            assrq_pdu.app_ctx().to_owned(),
            vec![
                AssocACPresentationContext::new(
                    4u8,
                    0u8,
                    TransferSyntaxItem::new("2.3.4.5".into()),
                ),
                AssocACPresentationContext::new(
                    4u8,
                    0u8,
                    TransferSyntaxItem::new("3.4.5.6.7".into()),
                ),
            ],
            assrq_pdu.user_info().to_owned(),
        );

        assert_pdu_roundtrip(Pdu::AssocAC(pdu));
    }

    #[test]
    fn test_assoc_rj_roundtrip() {
        let pdu = AssocRJ::new(1u8, 1u8, 1u8);
        assert_pdu_roundtrip(Pdu::AssocRJ(pdu));
    }

    #[test]
    fn test_release_rq_roundtrip() {
        let pdu = ReleaseRQ::new();
        assert_pdu_roundtrip(Pdu::ReleaseRQ(pdu));
    }

    #[test]
    fn test_release_rp_roundtrip() {
        let pdu = ReleaseRP::new();
        assert_pdu_roundtrip(Pdu::ReleaseRP(pdu));
    }

    #[test]
    fn test_abort_roundtrip() {
        let pdu = Abort::new(1u8, 1u8);
        assert_pdu_roundtrip(Pdu::Abort(pdu));
    }

    #[test]
    #[ignore] // PresentationDataItem is parsed as PresentationDataItemPartial.
    fn test_pres_data_item_roundtrip() {
        let pres_data_vals = vec![
            PresentationDataValue::new(1u8, 1u8, vec![1, 2, 3, 4]),
            PresentationDataValue::new(2u8, 2u8, vec![2, 3, 4, 5, 6]),
        ];
        let pres_data = PresentationDataItem::new(pres_data_vals);
        assert_pdu_roundtrip(Pdu::PresentationDataItem(pres_data));
    }

    #[test]
    fn test_app_ctx_roundtrip() {
        let pdu = ApplicationContextItem::new(vec![3, 2, 1, 5, 4]);
        assert_pdu_roundtrip(Pdu::ApplicationContextItem(pdu));
    }

    #[test]
    fn test_pdu_type_roundtrip() {
        assert_eq!(PduType::AssocRQ, (u8::from(&PduType::AssocRQ)).into());
        assert_eq!(PduType::AssocAC, (u8::from(&PduType::AssocAC)).into());
        assert_eq!(PduType::AssocRJ, (u8::from(&PduType::AssocRJ)).into());

        assert_eq!(
            PduType::PresentationDataItemPartial,
            (u8::from(&PduType::PresentationDataItemPartial)).into()
        );

        assert_eq!(PduType::ReleaseRQ, (u8::from(&PduType::ReleaseRQ)).into());
        assert_eq!(PduType::ReleaseRP, (u8::from(&PduType::ReleaseRP)).into());
        assert_eq!(PduType::Abort, (u8::from(&PduType::Abort)).into());

        assert_eq!(
            PduType::ApplicationContextItem,
            (u8::from(&PduType::ApplicationContextItem)).into()
        );

        assert_eq!(
            PduType::AssocRQPresentationContext,
            (u8::from(&PduType::AssocRQPresentationContext)).into()
        );
        assert_eq!(
            PduType::AssocACPresentationContext,
            (u8::from(&PduType::AssocACPresentationContext)).into()
        );

        assert_eq!(
            PduType::AbstractSyntaxItem,
            (u8::from(&PduType::AbstractSyntaxItem)).into()
        );
        assert_eq!(
            PduType::TransferSyntaxItem,
            (u8::from(&PduType::TransferSyntaxItem)).into()
        );
        assert_eq!(
            PduType::UserInformationItem,
            (u8::from(&PduType::UserInformationItem)).into()
        );
    }
}
