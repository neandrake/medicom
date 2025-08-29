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

use anyhow::{anyhow, Result};

use std::{
    fs::File,
    io::{BufReader, Write},
    path::Path,
};

use medicom::{
    core::{
        dcmobject::DicomRoot,
        defn::ts::TSRef,
        read::{stop::ParseStop, Parser, ParserBuilder},
    },
    dict::{stdlookup::STANDARD_DICOM_DICTIONARY, tags::SOPInstanceUID},
    dimse::{
        assoc::{CloseMsg, DimseMsg},
        error::{AssocError, AssocRsp, DimseError},
        pdus::PduType,
    },
};

pub(crate) mod archiveapp;
#[cfg(feature = "image")]
pub(crate) mod extractapp;
#[cfg(feature = "index")]
pub(crate) mod indexapp;
pub(crate) mod inspectapp;
pub(crate) mod printapp;
#[cfg(feature = "index")]
pub(crate) mod scpapp;
pub(crate) mod scuapp;
pub(crate) mod viewapp;

pub(crate) trait CommandApplication {
    fn run(&mut self) -> Result<()>;
}

fn parse_file(path: &Path, allow_partial_object: bool) -> Result<Parser<'_, BufReader<File>>> {
    if !path.is_file() {
        return Err(anyhow!("invalid file: {}", path.display()));
    }

    let file = File::open(path)?;
    let dataset = BufReader::with_capacity(1024 * 1024, file);
    let mut parser = ParserBuilder::default()
        .allow_partial_object(allow_partial_object)
        .build(dataset, &STANDARD_DICOM_DICTIONARY);

    let mut peeker = parser.by_ref().peekable();

    let first = peeker.peek();
    if let Some(Err(_)) = first {
        return Err(anyhow!("file is not dicom: {}", path.display()));
    } else if first.is_none() {
        return Err(anyhow!("file is empty: {}", path.display()));
    }

    Ok(parser)
}

/// Returns a log statement appropriate for the result of an association.
fn handle_assoc_result<W: Write>(
    result: Result<DimseMsg, AssocError>,
    writer: W,
) -> Vec<Result<String, String>> {
    match result {
        Ok(DimseMsg::CloseMsg(CloseMsg::ReleaseRQ)) => {
            vec![Ok(format!("[info <-]: {:?}", PduType::ReleaseRQ))]
        }
        Ok(DimseMsg::CloseMsg(CloseMsg::ReleaseRP)) => {
            vec![Ok(format!("[info <-]: {:?}", PduType::ReleaseRP))]
        }
        Ok(DimseMsg::CloseMsg(CloseMsg::Reject(rj))) => vec![Ok(format!(
            "[warn <-]: {:?}: {}",
            PduType::AssocRJ,
            rj.get_reason_desc()
        ))],
        Ok(DimseMsg::CloseMsg(CloseMsg::Abort(ab))) => vec![Ok(format!(
            "[warn <-]: {:?}: {}",
            PduType::Abort,
            ab.get_reason_desc()
        ))],
        Ok(other) => vec![Err(format!(
            "[ err xx]: Unexpected ending state: {other:?}"
        ))],
        Err(e) => {
            let mut output: Vec<Result<String, String>> = Vec::with_capacity(3);
            output.push(Err(format!("[ err ><]: {e}")));
            match e.rsp() {
                Some(AssocRsp::RJ(rj)) => {
                    output.push(Ok(format!("[info ->]: {:?}", rj.pdu_type())));
                }
                Some(AssocRsp::AB(ab)) => {
                    output.push(Ok(format!("[info ->]: {:?}", ab.pdu_type())));
                }
                None => {}
            }
            if let Err(inner) = e.write(writer) {
                output.push(Err(format!(
                    "[ err xx]: Failure writing error response: {inner}"
                )));
            }
            output
        }
    }
}

/// Reads a DICOM file to parse the SOP Instance UID, then renames the file based on the SOP.
///
/// # Errors
/// - I/O errors may occur reading or renaming the file.
/// - `DimseError` will occur if the file could not be parsed as DICOM or did not contain a SOP
///   Instance UID.
fn rename_file_to_sop(filename: &str, ts: TSRef) -> Result<(), DimseError> {
    let file = BufReader::with_capacity(8 * 1024, File::open(filename).map_err(DimseError::from)?);
    let mut parser = ParserBuilder::default()
        .dataset_ts(ts)
        .stop(ParseStop::after(&SOPInstanceUID))
        .build(file, &STANDARD_DICOM_DICTIONARY);
    let sop_uid = DicomRoot::parse(&mut parser)
        .map_err(DimseError::from)?
        .and_then(|dcm_root| {
            dcm_root
                .get_value_by_tag(&SOPInstanceUID)
                .and_then(|v| v.string().cloned())
        });
    let Some(sop_uid) = sop_uid else {
        return Err(DimseError::DimseElementMissing(filename.to_owned()));
    };
    drop(parser);
    std::fs::rename(filename, format!("{sop_uid}.dcm")).map_err(DimseError::from)?;
    Ok(())
}
