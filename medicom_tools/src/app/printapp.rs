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

//! The print command renders the contents of a DICOM dataset to stdout, in a format similar to the
//! dcmdump tool.

use std::{
    io::{self, Write},
    path::{Path, PathBuf},
};

use anyhow::Result;

use medicom::core::{
    dcmelement::DicomElement, defn::constants::tags::FILE_META_GROUP_END, inspect::FormattedElement,
};

use crate::{
    app::{parse_file, CommandApplication},
    args::PrintArgs,
};

pub struct PrintApp {
    args: PrintArgs,
}

impl PrintApp {
    pub fn new(args: PrintArgs) -> PrintApp {
        PrintApp { args }
    }
}

impl CommandApplication for PrintApp {
    fn run(&mut self) -> Result<()> {
        let path_buf: PathBuf = self.args.file.clone();
        let path: &Path = path_buf.as_path();
        let mut parser = parse_file(path, true)?;

        let mut stdout = io::stdout().lock();
        stdout.write_all(format!(
            "\n# Dicom-File-Format File: {:#}\n\n# Dicom-Meta-Information-Header\n# Used TransferSyntax: {}\n",
            path.display(),
            parser.ts().uid().ident()).as_ref()
        )?;

        let mut prev_was_file_meta: bool = true;

        while let Some(elem) = parser.next() {
            let elem: DicomElement = elem?;

            if prev_was_file_meta && elem.tag() > FILE_META_GROUP_END {
                stdout.write_all(
                    format!(
                        "\n# Dicom-Data-Set\n# Used TransferSyntax: {}\n",
                        parser.ts().uid().ident()
                    )
                    .as_ref(),
                )?;
                prev_was_file_meta = false;
            }

            let printed: String = FormattedElement::new(&elem).to_string();
            stdout.write_all(format!("{printed}\n").as_ref())?;
        }

        Ok(())
    }
}
