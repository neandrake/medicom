extern crate encoding;
extern crate walkdir;

use dcmpipe_lib::core::dcmobject::DicomRoot;
use dcmpipe_lib::core::dcmparser::{
    Parser, ParserBuilder, DICOM_PREFIX, DICOM_PREFIX_LENGTH, FILE_PREAMBLE_LENGTH,
};
use dcmpipe_lib::core::dcmparser_util::parse_into_object;
use std::fs::File;
use std::io::{Error, Read};
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};
use dcmpipe_dict::dict::stdlookup::STANDARD_DICOM_DICTIONARY;

#[cfg(test)]
mod charsets;
#[cfg(test)]
mod mock;
#[cfg(test)]
mod parsing;

/// Parses the given file into a `DicomObject`
pub fn parse_file(path: &str, with_std: bool) -> Result<DicomRoot, Error> {
    let file: File = File::open(path)?;
    let mut parser: ParserBuilder<File> = ParserBuilder::new(file);
    if with_std {
        parser = parser.dictionary(&STANDARD_DICOM_DICTIONARY);
    }
    let mut parser: Parser<File> = parser.build();
    let dcmroot: DicomRoot = parse_into_object(&mut parser)?;
    Ok(dcmroot)
}

/// Parses through all dicom files in the `fixtures` folder. The `use_std_dict` argument specifies
/// whether the standard dicom dictionary should be reigstered with the parser.
pub fn parse_all_dicom_files(with_std: bool) -> Result<(), Error> {
    for mut pair in get_all_dicom_file_parsers(with_std)? {
        while let Some(element) = pair.1.next() {
            if let Err(e) = element {
                eprintln!(
                    "Error parsing DICOM:\n\t{}\n\t{}",
                    pair.0.to_str().expect("Should get path"),
                    e
                );
            }
        }
    }
    Ok(())
}

/// Creates parsers for every dicom file in the `fixutres` folder. The `use_std_dict` argument
/// specifies whether the standard dicom dictionary should be registered with the parser.
/// See the `readme.md` in this project for information on obtaining test fixtures.
pub fn get_all_dicom_file_parsers(with_std: bool) -> Result<Vec<(PathBuf, Parser<'static, File>)>, Error> {
    let fixtures_path: &Path = Path::new("./fixtures");
    assert!(
        fixtures_path.is_dir(),
        "The fixtures are missing and need downloaded"
    );

    let dirwalker: WalkDir = WalkDir::new(fixtures_path);
    let mut parsers: Vec<(PathBuf, Parser<File>)> = Vec::new();
    for entry_res in dirwalker.into_iter() {
        let entry: DirEntry = entry_res?;
        let path: &Path = entry.path();

        let filename = path
            .file_name()
            .expect("Should be able to get filename")
            .to_str()
            .expect("Should be able to stringify filename");

        // Only attempt to parse .DCM files or files without any extension
        if (!filename.ends_with(".dcm") && filename.contains('.')) || filename.eq("README") {
            continue;
        }

        let file: File = File::open(path)?;
        if file.metadata()?.is_file() {
            let mut parser: ParserBuilder<File> = ParserBuilder::new(file);
            if with_std {
                parser = parser.dictionary(&STANDARD_DICOM_DICTIONARY);
            }

            let parser: Parser<File> = parser.build();
            parsers.push((path.to_path_buf(), parser));
        }
    }

    Ok(parsers)
}

/// Checks that the first 132 bytes are 128 0's followed by 'DICM'.
/// DICOM files do not need to abide by this format to be valid, but it's standard.
pub fn is_standard_dcm_file<DatasetType>(parser: &Parser<DatasetType>) -> bool
where
    DatasetType: Read,
{
    match parser.get_file_preamble() {
        Some(file_preamble) => {
            for b in file_preamble.iter().take(FILE_PREAMBLE_LENGTH) {
                if *b != 0 {
                    eprintln!("BAD PREAMBLE??");
                    return false;
                }
            }
        }
        None => {}
    }

    match parser.get_dicom_prefix() {
        Some(prefix) => {
            for i in 0..DICOM_PREFIX_LENGTH {
                if prefix[i] != DICOM_PREFIX[i] {
                    return false;
                }
            }
        }
        None => {}
    }
    true
}
