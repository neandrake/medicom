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

use std::{fmt::Display, io::Write};

use crate::{
    core::{charset::CSError, defn::uid::UIDRef, read::ParseError, write::error::WriteError},
    dimse::{
        assoc::{CloseMsg, DimseMsg},
        commands::{CommandStatus, CommandType},
        pdus::{
            mainpdus::{Abort, AssocRJ},
            PduType,
        },
    },
};

/// Errors related to the DIMSE protocol.
#[derive(Debug, thiserror::Error)]
pub enum DimseError {
    /// The connection was closed.
    #[error("connection closed: {0:?}")]
    ConnectionClosed(CloseMsg),

    /// The stream closed unexpectedly.
    #[error("unexpected end of byte stream")]
    UnexpectedEOF,

    /// Wrapper around other errors, intended to be used by users of this library.
    #[error("application error: {0}")]
    ApplicationError(Box<dyn std::error::Error>),

    /// A PDU type was encountered that is unknown, likely non-standard or a corruption in the
    /// stream.
    #[error("invalid pdu type: {0:04X}")]
    InvalidPduType(u8),

    /// Error while parsing a DICOM element in a DIMSE request/response.
    #[error("error parsing value from request: {0}")]
    ParseError(#[from] ParseError),

    /// Character encoding errors while parsing DICOM element values in a DIMSE request/response.
    #[error("error decoding string: {0}")]
    CharsetError(#[from] CSError),

    /// Errors when writing DICOM elements in a response request/response stream.
    #[error("error encoding DICOM: {0}")]
    WriteError(#[from] WriteError),

    /// Wrapper around `std::io::Error`.
    #[error("i/o error reading from dataset: {0}")]
    IOError(#[from] std::io::Error),

    #[error("malformed ae title: {0}")]
    MalformedAeTitle(String),

    #[error("called ae title is not this ae title: {0}")]
    InvalidCalledAeTitle(String),

    #[error("calling ae title is not in accept-list: {0}")]
    InvalidCallingAeTitle(String),

    /// A Pdu was encountered which was not anticipated.
    #[error("unexpected pdu type {0:?}")]
    UnexpectedPduType(PduType),

    #[error("expected PDU from association but did not receive one, connection may have closed")]
    DimsePDUMissing,

    #[error("invalid app context: {0}")]
    InvalidAppContext(String),

    #[error("expected command from association but got {0:?}")]
    DimseCmdMissing(Box<DimseMsg>),

    #[error("expected dicom from association but got: {0:?}")]
    DimseDicomMissing(Box<DimseMsg>),

    #[error("expected close command from association but got: {0:?}")]
    DimseCloseMissing(Box<DimseMsg>),

    /// DIMSE Command Messages are expected to have a minimal set of elements.
    #[error("element missing from request: {0}")]
    DimseElementMissing(String),

    #[error("dimse query parsing failed")]
    QueryParseError,

    #[error("unsupported abstract syntax {}", uid.name())]
    UnsupportedAbstractSyntax { uid: UIDRef },

    #[error("maximum pdu size exceeded, PDU is {0} bytes")]
    MaxPduSizeExceeded(usize),

    #[error("Association negotiation failed: {0}")]
    AssocNegotiationFailure(String),

    #[error("PDU received with unknown context ID: {0}")]
    UnknownContext(u8),

    #[error("unknown transfer syntax: {0}")]
    UnknownTransferSyntax(String),

    #[error("unknown abstract syntax: {0}")]
    UnknownAbstractSyntax(String),

    #[error("unknown message id: {0}")]
    UnknownMessageID(u16),

    #[error("unespected command type: {0:?}")]
    UnexpectedCommandType(CommandType),

    #[error("unexpected response status: {0:?}")]
    UnexpectedCommandStatus(CommandStatus),
}

#[derive(Debug)]
pub enum AssocRsp {
    RJ(AssocRJ),
    AB(Abort),
}

impl AssocRsp {
    #[must_use]
    pub fn pdu_type(&self) -> PduType {
        match self {
            AssocRsp::RJ(rj) => rj.pdu_type(),
            AssocRsp::AB(ab) => ab.pdu_type(),
        }
    }
}

/// Errors related to the association, which can aid in managing sending back appropriate rejection
/// or abort PDUs if needed.
#[derive(Debug)]
pub struct AssocError {
    rsp: Option<AssocRsp>,
    err: DimseError,
}

impl Display for AssocError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.err)
    }
}

impl AssocError {
    #[must_use]
    pub fn rsp(&self) -> &Option<AssocRsp> {
        &self.rsp
    }

    #[must_use]
    pub fn err(&self) -> &DimseError {
        &self.err
    }

    #[must_use]
    pub fn into_err(self) -> DimseError {
        self.err
    }

    /// Generates a `AssocError`, wrapping the given error in `DimseError`. This error response
    /// will not result in sending an abort or rejection to the other end.
    #[must_use]
    pub fn error<E>(err: E) -> Self
    where
        DimseError: From<E>,
    {
        Self {
            rsp: None,
            err: DimseError::from(err),
        }
    }

    /// Creates an `AssocError` indicating the connection was clsoed and no response to be
    /// returned.
    #[must_use]
    pub fn handled_close(close_msg: CloseMsg) -> Self {
        Self {
            rsp: None,
            err: DimseError::ConnectionClosed(close_msg),
        }
    }

    /// Creates an `AssocError` indicating the connection should be closed.
    #[must_use]
    pub fn unhandled_close(close_msg: CloseMsg) -> Self {
        let rsp = match close_msg {
            CloseMsg::ReleaseRQ | CloseMsg::ReleaseRP => None,
            CloseMsg::Reject(ref rj) => Some(AssocRsp::RJ(rj.clone())),
            CloseMsg::Abort(ref ab) => Some(AssocRsp::AB(ab.clone())),
        };
        Self {
            rsp,
            err: DimseError::ConnectionClosed(close_msg),
        }
    }

    /// Generates a generic `Abort`, for closing the connection without further context or
    /// information.
    #[must_use]
    pub fn ab_failure<E>(err: E) -> Self
    where
        DimseError: From<E>,
    {
        Self {
            rsp: Some(AssocRsp::AB(Abort::new(0u8, 0u8))),
            err: DimseError::from(err),
        }
    }

    /// Generates a `Abort`, indicating that an unexpected PDU was encountered.
    #[must_use]
    pub fn ab_unexpected_pdu<E>(err: E) -> Self
    where
        DimseError: From<E>,
    {
        Self {
            rsp: Some(AssocRsp::AB(Abort::new(2u8, 2u8))),
            err: DimseError::from(err),
        }
    }

    /// Generates a `Abort`, indicating that the PDU is not valid or well-formed.
    #[must_use]
    pub fn ab_invalid_pdu<E>(err: E) -> Self
    where
        DimseError: From<E>,
    {
        Self {
            rsp: Some(AssocRsp::AB(Abort::new(2u8, 6u8))),
            err: DimseError::from(err),
        }
    }

    /// Generates a generic `AssocRJ`, rejecting the association without further context or
    /// information.
    #[must_use]
    pub fn rj_failure<E>(err: E) -> Self
    where
        DimseError: From<E>,
    {
        Self {
            rsp: Some(AssocRsp::RJ(AssocRJ::new(1u8, 1u8, 1u8))),
            err: DimseError::from(err),
        }
    }

    /// Generates an `AssocRJ` indicating the calling AE Title has been rejected.
    #[must_use]
    pub fn rj_calling_aet<E>(err: E) -> Self
    where
        DimseError: From<E>,
    {
        Self {
            rsp: Some(AssocRsp::RJ(AssocRJ::new(1u8, 1u8, 3u8))),
            err: DimseError::from(err),
        }
    }

    /// Generates an `AssocRJ` indicating the called AE Title has been rejected.
    #[must_use]
    pub fn rj_called_aet<E>(err: E) -> Self
    where
        DimseError: From<E>,
    {
        Self {
            rsp: Some(AssocRsp::RJ(AssocRJ::new(1u8, 1u8, 7u8))),
            err: DimseError::from(err),
        }
    }

    /// Generates an `AssocRJ` indicating the Application Context Name isn't supported. This has
    /// limited use, for the initial association negotiation.
    #[must_use]
    pub fn rj_unsupported<E>(err: E) -> Self
    where
        DimseError: From<E>,
    {
        Self {
            rsp: Some(AssocRsp::RJ(AssocRJ::new(1u8, 1u8, 2u8))),
            err: DimseError::from(err),
        }
    }

    /// Writes this error response, if any, to the given writer, consuming this error.
    ///
    /// # Errors
    /// I/O errors may occur writing the PDU to the writer, or flushing the writer.
    pub fn write<W: Write>(&self, mut writer: W) -> Result<(), DimseError> {
        match &self.rsp {
            Some(AssocRsp::RJ(rj)) => {
                rj.write(&mut writer)?;
                writer.flush().map_err(DimseError::IOError)?;
            }
            Some(AssocRsp::AB(ab)) => {
                ab.write(&mut writer)?;
                writer.flush().map_err(DimseError::IOError)?;
            }
            _ => {}
        }
        Ok(())
    }
}
