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

#![allow(clippy::too_many_lines)]

use std::fs::File;
use std::io::{BufReader, BufWriter, Error, Write};
use std::path::{Path, PathBuf};

use phf_codegen::Map;

use crate::xmlparser::{
    XmlDicomDefinition, XmlDicomDefinitionIterator, XmlDicomElement, XmlDicomUid,
};

static LOOKUP_PREAMBLE: &str = "/*
   Copyright 2024-2025 Christopher Speck

   Licensed under the Apache License, Version 2.0 (the \"License\");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an \"AS IS\" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

//! This is an auto-generated file. Do not make modifications here.
//!
//! This contains perfect-hashed maps for looking up DICOM Elements, Transfer Syntaxes, and DICOM
//! UIDs based on their \"identity\" or their associated value. The \"identity\" is computed based
//! on the name, with spaces removed and all letters lower-cased, to allow for case-insensitive
//! look-ups. The associated value is the common value for the item, such as tag number for DICOM
//! Elements, and the stringified UID for Transfer Syntaxes and DICOM UIDs.

#![allow(clippy::unreadable_literal, clippy::doc_markdown)]

use crate::core::defn::{tag::TagRef, ts::TSRef, uid::UIDRef};
use crate::dict::{tags, transfer_syntaxes as ts, uids};

";

static DICOM_ELEMENT_PREAMBLE: &str = "/*
   Copyright 2024-2025 Christopher Speck

   Licensed under the Apache License, Version 2.0 (the \"License\");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an \"AS IS\" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

//! This is an auto-generated file. Do not make modifications here.
//!
//! This contains definitions of DICOM Elements.

#![allow(non_upper_case_globals, clippy::doc_markdown)]

use crate::core::defn::{tag::Tag, vm::VM, vr};

";

static TRANSFER_SYNTAX_PREAMBLE: &str = "/*
   Copyright 2024-2025 Christopher Speck

   Licensed under the Apache License, Version 2.0 (the \"License\");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an \"AS IS\" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

//! This is an auto-generated file. Do not make modifications here.
//!
//! This contains definitions of Transfer Syntaxes.

#![allow(non_upper_case_globals, clippy::doc_markdown)]

use crate::core::defn::ts::TransferSyntax;
use crate::dict::uids;

";

static UID_PREAMBLE: &str = "/*
   Copyright 2024-2025 Christopher Speck

   Licensed under the Apache License, Version 2.0 (the \"License\");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an \"AS IS\" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

//! This is an auto-generated file. Do not make modifications here.
//!
//! This contains definitions of DICOM UIDs.

#![allow(non_upper_case_globals, clippy::doc_markdown)]

use crate::core::defn::uid::UID;

";

macro_rules! uid_definition {
    ($($args:tt)*) => {
        format!("/// {}
///
/// - **UID:** {}
/// - **UID Type:** {}
pub static {}: UID = UID::new(
    \"{}\",
    \"{}\",
    \"{}\",
);

", $($args)*)
    };
}

macro_rules! transfer_syntax_definition {
    ($($args:tt)*) => {
        format!(
"/// {}
///
/// - **UID:** {}
pub static {}: TransferSyntax = TransferSyntax::new(
    {},
    {},
    {},
    {},
    {},
);

", $($args)*)
    };
}

macro_rules! dicom_element_definition {
    ($($args:tt)*) => {
        format!(
"/// {}
///
/// - **Tag:** {}
/// - **VR:** {}
/// - **VM:** {}
pub static {}: Tag = Tag::new(
    \"{}\",
    0x{:04X}_{:04X},
    {},
    {},
    \"{}\",
);

", $($args)*)
    };
}

/// Parse the given input XML files based on the DICOM standard structure and output the resulting
/// definition files to the given folder.
///
/// # Errors
/// I/O errors encountered reading/writing files.
pub fn process_xml_files(files: Vec<File>, folder: &Path) -> Result<(), Error> {
    type PossibleDef = Result<XmlDicomDefinition, quick_xml::Error>;

    let mut xml_definitions: Vec<XmlDicomDefinition> = Vec::new();
    for file in files {
        let bufread: BufReader<File> = BufReader::new(file);
        let (file_definitions, errors): (Vec<PossibleDef>, Vec<PossibleDef>) =
            XmlDicomDefinitionIterator::new(bufread).partition(Result::is_ok);

        if !errors.is_empty() {
            let error = errors
                .into_iter()
                .filter_map(Result::err)
                .map(std::io::Error::other)
                .next();
            if let Some(error) = error {
                return Err(error);
            }
        }
        let mut file_definitions = file_definitions
            .into_iter()
            .filter_map(Result::ok)
            .collect();
        xml_definitions.append(&mut file_definitions);
    }

    process_entries(xml_definitions, folder)?;

    Ok(())
}

fn process_entries(xml_definitions: Vec<XmlDicomDefinition>, folder: &Path) -> Result<(), Error> {
    let mut tag_ident_lookup_phf: phf_codegen::Map<String> = phf_codegen::Map::new();
    let mut tag_tag_lookup_phf: phf_codegen::Map<u32> = phf_codegen::Map::new();

    let mut uid_ident_lookup_phf: phf_codegen::Map<String> = phf_codegen::Map::new();
    let mut uid_id_lookup_phf: phf_codegen::Map<String> = phf_codegen::Map::new();

    let mut ts_ident_lookup_phf: phf_codegen::Map<String> = phf_codegen::Map::new();
    let mut ts_id_lookup_phf: phf_codegen::Map<String> = phf_codegen::Map::new();

    let mut xml_dicom_elements: Vec<XmlDicomElement> = Vec::new();
    let mut xml_uids: Vec<XmlDicomUid> = Vec::new();
    let mut xml_ts: Vec<XmlDicomUid> = Vec::new();

    for defn in xml_definitions {
        match defn {
            XmlDicomDefinition::DicomElement(e)
            | XmlDicomDefinition::FileMetaElement(e)
            | XmlDicomDefinition::DirStructureElement(e)
            | XmlDicomDefinition::CommandElement(e) => xml_dicom_elements.push(e),
            XmlDicomDefinition::Uid(uid) => xml_uids.push(uid),
            XmlDicomDefinition::TransferSyntax(uid) => {
                xml_uids.push(uid.clone());
                xml_ts.push(uid);
            }
        }
    }

    xml_dicom_elements.sort_by(|a, b| a.tag.cmp(&b.tag));
    let mut dicom_elements = xml_dicom_elements
        .iter()
        .filter_map(|e| {
            process_element(
                e,
                "tags::",
                &mut tag_ident_lookup_phf,
                &mut tag_tag_lookup_phf,
            )
        })
        .collect::<String>();

    let mut uids = xml_uids
        .iter()
        .filter_map(|uid| process_uid(uid, &mut uid_ident_lookup_phf, &mut uid_id_lookup_phf))
        .collect::<String>();

    let mut transfer_syntax_uids = xml_ts
        .iter()
        .filter_map(|ts| {
            process_transfer_syntax(ts, &mut ts_ident_lookup_phf, &mut ts_id_lookup_phf)
        })
        .collect::<String>();

    // Remove trailing newlines
    if dicom_elements.len() > 2 {
        dicom_elements.remove(dicom_elements.len() - 2);
    }
    if uids.len() > 2 {
        uids.remove(uids.len() - 2);
    }
    if transfer_syntax_uids.len() > 2 {
        transfer_syntax_uids.remove(transfer_syntax_uids.len() - 2);
    }

    std::fs::create_dir_all(folder)?;

    let path_buf: PathBuf = folder.to_path_buf();
    let mut dicom_elements_file = path_buf.clone();
    dicom_elements_file.push("tags.rs");
    let mut uids_file = path_buf.clone();
    uids_file.push("uids.rs");
    let mut transfer_syntaxes_file = path_buf;
    transfer_syntaxes_file.push("transfer_syntaxes.rs");

    save_codefile(
        dicom_elements_file.as_path(),
        DICOM_ELEMENT_PREAMBLE.to_owned(),
        &dicom_elements,
    )?;
    save_codefile(uids_file.as_path(), UID_PREAMBLE.to_owned(), &uids)?;
    save_codefile(
        transfer_syntaxes_file.as_path(),
        TRANSFER_SYNTAX_PREAMBLE.to_owned(),
        &transfer_syntax_uids,
    )?;

    let mut lookup_file_path: PathBuf = folder.to_path_buf();
    lookup_file_path.push("lookup.rs");

    let mut lookup_file: BufWriter<File> =
        BufWriter::new(File::create(lookup_file_path.as_path()).unwrap());
    write!(&mut lookup_file, "{LOOKUP_PREAMBLE}")?;

    let write_stringkey_map =
        |mut lookup_file: &mut BufWriter<File>, decl: &str, map: Map<String>| {
            write!(&mut lookup_file, "{decl}")?;
            let map_display = map.build();
            write!(&mut lookup_file, "{map_display}")?;
            write!(&mut lookup_file, ";\n\n")
        };

    let write_uintkey_map = |mut lookup_file: &mut BufWriter<File>, decl: &str, map: Map<u32>| {
        write!(&mut lookup_file, "{decl}")?;
        let map_display = map.build();
        write!(&mut lookup_file, "{map_display}")?;
        write!(&mut lookup_file, ";\n\n")
    };

    write_stringkey_map(
        &mut lookup_file,
        "pub static TAG_BY_IDENT: phf::Map<&'static str, TagRef> = ",
        tag_ident_lookup_phf,
    )?;
    write_uintkey_map(
        &mut lookup_file,
        "pub static TAG_BY_VALUE: phf::Map<u32, TagRef> = ",
        tag_tag_lookup_phf,
    )?;
    write_stringkey_map(
        &mut lookup_file,
        "pub static TS_BY_IDENT: phf::Map<&'static str, TSRef> = ",
        ts_ident_lookup_phf,
    )?;
    write_stringkey_map(
        &mut lookup_file,
        "pub static TS_BY_UID: phf::Map<&'static str, TSRef> = ",
        ts_id_lookup_phf,
    )?;
    write_stringkey_map(
        &mut lookup_file,
        "pub static UID_BY_IDENT: phf::Map<&'static str, UIDRef> = ",
        uid_ident_lookup_phf,
    )?;
    write_stringkey_map(
        &mut lookup_file,
        "pub static UID_BY_UID: phf::Map<&'static str, UIDRef> = ",
        uid_id_lookup_phf,
    )?;

    Ok(())
}

/// Processes a UID entry into code definition
fn process_uid(
    uid: &XmlDicomUid,
    ident_lookup: &mut phf_codegen::Map<String>,
    id_lookup: &mut phf_codegen::Map<String>,
) -> Option<String> {
    let var_name: String = sanitize_var_name(&uid.name);
    if var_name.is_empty() {
        return None;
    }

    let type_clone = uid.uid_type.clone();
    let comment_uid_type = type_clone.unwrap_or_default();

    let code: String = uid_definition!(
        uid.name,
        uid.value,
        comment_uid_type, // comment placeholders
        var_name,
        var_name,
        uid.value,
        uid.name
    ); // field placeholders

    let var_name_key: String = var_name.to_lowercase();
    ident_lookup.entry(var_name_key, &format!("&uids::{var_name}"));

    let id_val_key: String = uid.value.clone();
    id_lookup.entry(id_val_key, &format!("&uids::{var_name}"));

    Some(code)
}

/// Processes a `TransferSyntax` UID into code definition
fn process_transfer_syntax(
    uid: &XmlDicomUid,
    ident_lookup: &mut phf_codegen::Map<String>,
    id_lookup: &mut phf_codegen::Map<String>,
) -> Option<String> {
    let name_text: String = sanitize_text(&uid.name);
    let uid_text: String = sanitize_text(&uid.value);

    let var_name: String = sanitize_var_name(&uid.name);
    if var_name.is_empty() {
        return None;
    }

    let var_uid: String = format!("&uids::{var_name}");
    let explicit_vr_val: String = if var_name.contains("Implicit") {
        "false"
    } else {
        "true"
    }
    .to_owned();

    // Part 5 section 7.3
    // All unretired Transfer Syntaxes in DICOM require the use of Little Endian Byte Ordering.
    let big_endian_val: String = if var_name.contains("BigEndian") {
        "true"
    } else {
        "false"
    }
    .to_owned();

    let deflated_val: String =
        if var_name.contains("Deflate") && !var_name.contains("Uncompressed") {
            "true"
        } else {
            "false"
        }
        .to_owned();

    let encapsulated_val: String = if var_name.contains("Encapsulated")
        || var_name.contains("JPEG")
        || var_name.contains("RLE")
        || var_name.contains("MPEG")
        || var_name.contains("HEVC")
    {
        "true"
    } else {
        "false"
    }
    .to_owned();

    let code: String = transfer_syntax_definition!(
        // comment placeholders
        name_text,
        uid_text,
        var_name,
        // field placeholders
        var_uid,
        explicit_vr_val,
        big_endian_val,
        deflated_val,
        encapsulated_val
    );
    let var_name_key: String = var_name.to_lowercase();
    ident_lookup.entry(var_name_key, &format!("&ts::{var_name}"));
    let id_val_lookup: String = uid.value.clone();
    id_lookup.entry(id_val_lookup, &format!("&ts::{var_name}"));

    Some(code)
}

/// Processes a dicom element into code definition
fn process_element(
    element: &XmlDicomElement,
    dict: &str,
    ident_lookup: &mut phf_codegen::Map<String>,
    tag_lookup: &mut phf_codegen::Map<u32>,
) -> Option<String> {
    let var_name: String = sanitize_var_name(&element.name);
    if var_name.is_empty() {
        return None;
    }

    let tag_group: u32 = (element.tag >> 16) & 0x0000_FFFF;
    let tag_element: u32 = element.tag & 0x0000_FFFF;

    let tag_display: String = format!("({tag_group:04X},{tag_element:04X})");

    // VR will either be listed as a valid VR, a list of possible VR a la "US or SS or DS", or have
    // the text "See Note ..."
    let vr: &str = element.vr.split_whitespace().next().unwrap();
    let vr_value: String = if vr == "See" {
        "None".to_owned()
    } else {
        format!("Some(&vr::{vr})")
    };

    let vm: String = if &element.vm == "1-n or 1" {
        "&VM::OneOrMore".to_owned()
    } else if let Ok(vm_val) = element.vm.parse::<u32>() {
        format!("&VM::Distinct({vm_val})")
    } else {
        let parts: Vec<&str> = element.vm.split('-').collect::<Vec<&str>>();
        let start: u32 = parts[0]
            .parse::<u32>()
            .unwrap_or_else(|_| panic!("Missing start to VM: {}", element.vm));
        let end: &str = parts[1];

        if end == "n" {
            format!("&VM::AtLeast({start})")
        } else if let Ok(end_val) = end.parse::<u32>() {
            format!("&VM::AtMost({end_val})")
        } else {
            format!("&VM::MultipleOf({start})")
        }
    };

    let code: String = dicom_element_definition!(
        element.name,
        tag_display,
        vr,
        element.vm, // comment placeholders
        var_name,
        var_name,
        tag_group,
        tag_element,
        vr_value,
        vm,
        element.name
    ); // field placeholders

    let var_name_key: String = var_name.to_lowercase();
    ident_lookup.entry(var_name_key, &format!("&{dict}{var_name}"));
    tag_lookup.entry(element.tag, &format!("&{dict}{var_name}"));

    Some(code)
}

fn save_codefile(filename: &Path, preamble: String, code: &str) -> Result<(), Error> {
    let mut out_rs_file: File = File::create(filename)?;
    out_rs_file.write_all(&preamble.into_bytes())?;
    out_rs_file.write_all(code.as_bytes())?;
    out_rs_file.flush()?;
    out_rs_file.sync_all()?;
    Ok(())
}

/// Cleans up the definition name to be a valid rust identifier.
/// Some things straight up don't have a name aside from "(Retired)" o.O
/// Such as: 1.2.840.10008.5.1.4.1.1.40 and (0018,0061)
/// So check whether the returned string is empty
fn sanitize_var_name(var_name: &str) -> String {
    let is_retired: bool = var_name.contains("Retired");
    // let is_trial: bool = var_name.contains("Trial");

    let mut sanitized: String = var_name
        .replace("(Retired)", "")
        .replace("(Trial)", "_Trial")
        // see comment on sanitize_text()
        .replace('\u{200b}', "")
        .replace([' ', '\'', '-', ',', '(', ')', '.', '/', '[', ']'], "")
        .replace('\"', "_")
        .replace('&', "_and_")
        .replace('µ', "micro")
        .split(':')
        .next()
        .unwrap()
        .to_owned();

    if sanitized.is_empty() {
        return sanitized;
    }

    // I think in general having "Retired" in the tag name is undesired
    // (see Explicit VR Big Endian (Retired)..)
    // However if we just remove "(Retired)" then it results in a
    // few duplicate definitions so we'll add back "_Retired"
    if is_retired
        && (&sanitized == "UltrasoundMultiframeImageStorage"
            || &sanitized == "UltrasoundImageStorage"
            || &sanitized == "NuclearMedicineImageStorage"
            || &sanitized == "DopplerSampleVolumeXPosition"
            || &sanitized == "DopplerSampleVolumeYPosition"
            || &sanitized == "TMLinePositionX0"
            || &sanitized == "TMLinePositionY0"
            || &sanitized == "TMLinePositionX1"
            || &sanitized == "TMLinePositionY1"
            || &sanitized == "ParallelReductionFactorInplane"
            || &sanitized == "LossyImageCompression"
            || &sanitized == "PlacerOrderNumberImagingServiceRequest"
            || &sanitized == "FillerOrderNumberImagingServiceRequest"
            || &sanitized == "ImageRotation"
            || &sanitized == "ReferencedImageBoxSequence")
    {
        sanitized = format!("{sanitized}_Retired");
    }

    // if is_trial
    //     && (sanitized == "SpecimenDescriptionSequence"
    //     || sanitized == "RequestedProcedureDescription"
    //     || sanitized == "AlgorithmDescription"
    //     || sanitized == "LanguageCodeSequence"
    //     || sanitized == "DisplayedAreaTopLeftHandCorner"
    //     || sanitized == "DisplayedAreaBottomRightHandCorner"
    //     || sanitized == "BeamOrderIndex"
    //     || sanitized == "DoubleExposureMeterset"
    //     || sanitized == "DoubleExposureFieldDelta") {
    //     sanitized = format!("{}_Trial", sanitized);
    // }

    if let Some(first_char) = sanitized.chars().next() {
        if !first_char.is_ascii() || !first_char.is_alphabetic() {
            return format!("Tag_{sanitized}");
        }
        if !first_char.is_uppercase() {
            return format!("{}{}", first_char.to_uppercase(), &sanitized[1..]);
        }
    }
    sanitized
}

/// Sanitizes text from the XML definition file. The definition file has a
/// number of zero-width space unicode characters strewn throughout.
fn sanitize_text(text: &str) -> String {
    text.replace('\u{200b}', "")
}
