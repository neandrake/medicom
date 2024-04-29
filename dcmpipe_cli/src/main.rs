extern crate byteorder;
extern crate dcmpipe_lib;

use byteorder::{BigEndian, ByteOrder, LittleEndian};
use dcmpipe_lib::core::charset::CSRef;
use dcmpipe_lib::core::dcmelement::{DicomElement, DicomSequencePosition};
use dcmpipe_lib::core::dict::dicom_elements as tags;
use dcmpipe_lib::core::dict::lookup::{TAG_BY_VALUE, UID_BY_ID};
use dcmpipe_lib::core::tag::Tag;
use dcmpipe_lib::core::ts::TSRef;
use dcmpipe_lib::core::vr;
use dcmpipe_lib::read::dcmparser::DicomStreamParser;
use dcmpipe_lib::read::tagstop::TagStop;
use std::env;
use std::fs::File;
use std::io::{self, Error, ErrorKind, Write};
use std::path::Path;
use std::process;

static MAX_BYTES_DISPLAY: usize = 16;
static MAX_ITEMS_DISPLAYED: usize = 4;

fn main() {
    let result: Result<(), Error> = appmain();
    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn appmain() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "first and only argument should be a file",
        ));
    }
    let path: &Path = Path::new(&args[1]);

    if !path.is_file() {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!("invalid file: {}", path.display()),
        ));
    }

    let file: File = File::open(path)?;
    let mut dicom_iter: DicomStreamParser<File> =
        DicomStreamParser::new(file, TagStop::EndOfStream);

    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    stdout.write(format!(
        "\n# Dicom-File-Format File: {:#?}\n\n# Dicom-Meta-Information-Header\n# Used TransferSyntax: {}\n",
        path,
        dicom_iter.get_ts().uid.ident).as_ref()
    )?;

    let mut prev_was_file_meta: bool = true;

    while let Some(elem) = dicom_iter.next() {
        let mut elem: DicomElement = elem?;
        if prev_was_file_meta && elem.tag > 0x0002FFFF {
            stdout.write(
                format!(
                    "\n# Dicom-Data-Set\n# Used TransferSyntax: {}\n",
                    dicom_iter.get_ts().uid.ident
                )
                .as_ref(),
            )?;
            prev_was_file_meta = false;
        }

        let printed: Option<String> =
            render_element(&mut elem, dicom_iter.get_ts(), dicom_iter.get_cs())?;

        if let Some(printed) = printed {
            stdout.write(format!("{}\n", printed).as_ref())?;
        }
    }

    Ok(())
}

fn render_element(
    element: &mut DicomElement,
    ts: TSRef,
    cs: CSRef,
) -> Result<Option<String>, Error> {
    if element.tag & 0xFFFF == 0 {
        // Group Length tags are deprecated, see note on Ch 5 Part 7.2
        return Ok(None);
    }

    let tag_num: String = Tag::format_tag_to_display(element.tag);
    let tag_name: &str = if let Some(tag) = TAG_BY_VALUE.get(&element.tag) {
        tag.ident
    } else {
        "<Private Tag>"
    };
    let vr: &str = element.vr.ident;

    let cs: CSRef = element.vr.get_proper_cs(cs);
    let mut tag_value: String = if element.vr == &vr::SQ {
        String::new()
    } else if element.is_empty() {
        "<empty>".to_owned()
    } else if ts.big_endian {
        render_dicom_value::<BigEndian>(element, cs)?
    } else {
        render_dicom_value::<LittleEndian>(element, cs)?
    };

    let seq_path: &Vec<DicomSequencePosition> = element.get_sequence_path();

    let mut indent_width: usize = seq_path.len();
    if indent_width > 0 && element.tag != tags::Item.tag {
        indent_width = indent_width + 1;
    }
    indent_width = indent_width * 2;

    if element.tag == tags::Item.tag {
        let path: String = seq_path
            .iter()
            .map(|seq_elem: &DicomSequencePosition| {
                format!(
                    "{}[{}]",
                    Tag::format_tag_to_path_display(seq_elem.get_seq_tag()),
                    seq_elem.get_item_number().unwrap_or(0xFFFFFFFF)
                )
            })
            .collect::<Vec<String>>()
            .join(".");
        let item_desc: String = if let Some(last_seq_elem) = seq_path.last() {
            format!(
                "#{} - {}",
                last_seq_elem.get_item_number().unwrap_or(0xFFFFFFFF),
                path
            )
        } else {
            String::new()
        };
        return Ok(Some(format!(
            "{indentation:indent_width$}{tag_name} {item_desc}",
            indentation = "",
            indent_width = indent_width,
            tag_name = tag_name,
            item_desc = item_desc
        )));
    }

    if !tag_value.is_empty() {
        if element.is_empty() {
            tag_value = format!(" {}", tag_value);
        } else {
            tag_value = format!(" | {}", tag_value);
        }
    }

    Ok(Some(format!(
        "{indentation:indent_width$}{tag_num} {vr} {tag_name}{tag_value}",
        indentation = "",
        indent_width = indent_width,
        tag_num = tag_num,
        vr = vr,
        tag_name = tag_name,
        tag_value = tag_value
    )))
}

/// Formats the value of this element as a string based on the VR
fn render_dicom_value<Endian: ByteOrder>(
    elem: &mut DicomElement,
    cs: CSRef,
) -> Result<String, Error> {
    let mut ellipses: bool = false;
    let mut sep: &str = ", ";
    let mut str_vals: Vec<String> = Vec::new();
    if elem.vr == &vr::AT {
        str_vals.push(Tag::format_tag_to_display(
            elem.parse_attribute::<Endian>()?,
        ));
    } else if elem.vr == &vr::FL || elem.vr == &vr::OF {
        sep = " / ";
        let vec: Vec<f32> = elem.parse_f32s::<Endian>()?;
        let vec_len: usize = vec.len();
        vec.into_iter()
            .take(MAX_ITEMS_DISPLAYED)
            .map(|val: f32| format!("{:.2}", val))
            .for_each(|val: String| str_vals.push(val));
        ellipses = vec_len > str_vals.len();
    } else if elem.vr == &vr::FD || elem.vr == &vr::OD {
        sep = " / ";
        let vec: Vec<f64> = elem.parse_f64s::<Endian>()?;
        let vec_len: usize = vec.len();
        vec.into_iter()
            .take(MAX_ITEMS_DISPLAYED)
            .map(|val: f64| format!("{:.2}", val))
            .for_each(|val: String| str_vals.push(val));
        ellipses = vec_len > str_vals.len();
    } else if elem.vr == &vr::SS || elem.vr == &vr::OW {
        sep = " / ";
        let vec: Vec<i16> = elem.parse_i16s::<Endian>()?;
        let vec_len: usize = vec.len();
        vec.into_iter()
            .take(MAX_ITEMS_DISPLAYED)
            .map(|val: i16| format!("{}", val))
            .for_each(|val: String| str_vals.push(val));
        ellipses = vec_len > str_vals.len();
    } else if elem.vr == &vr::SL || elem.vr == &vr::OL {
        sep = " / ";
        let vec: Vec<i32> = elem.parse_i32s::<Endian>()?;
        let vec_len: usize = vec.len();
        vec.into_iter()
            .take(MAX_ITEMS_DISPLAYED)
            .map(|val: i32| format!("{}", val))
            .for_each(|val: String| str_vals.push(val));
        ellipses = vec_len > str_vals.len();
    } else if elem.vr == &vr::UI {
        let str_val: String = elem.parse_string(cs)?;
        if let Some(uid) = UID_BY_ID.get(str_val.as_str()) {
            str_vals.push(format!("{} ({})", str_val, uid.name));
        } else {
            str_vals.push(str_val);
        }
    } else if elem.vr == &vr::UL {
        str_vals.push(format!("{}", elem.parse_u32::<Endian>()?));
    } else if elem.vr == &vr::US {
        str_vals.push(format!("{}", elem.parse_u16::<Endian>()?));
    } else if elem.vr.is_character_string {
        let vec: Vec<String> = elem.parse_strings(cs)?;
        let vec_len: usize = vec.len();
        vec.iter()
            .take(MAX_ITEMS_DISPLAYED)
            .map(|val: &String| format!("\"{}\"", val))
            .for_each(|val: String| str_vals.push(val));
        ellipses = vec_len > str_vals.len();
    } else {
        let vec: &Vec<u8> = elem.get_data().get_ref();
        vec.iter()
            .take(MAX_BYTES_DISPLAY)
            .map(|val: &u8| format!("{}", val))
            .for_each(|val: String| str_vals.push(val));
        ellipses = vec.len() > str_vals.len();
    }

    if ellipses {
        str_vals.push("..".to_string());
    }

    let num_vals: usize = str_vals.len();
    if num_vals == 1 {
        return Ok(str_vals.remove(0));
    }

    Ok(format!(
        "[{}]",
        str_vals.into_iter().collect::<Vec<String>>().join(sep)
    ))
}
