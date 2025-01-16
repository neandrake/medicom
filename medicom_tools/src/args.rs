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

use std::{error::Error, path::PathBuf};

use clap::{Args, Parser, Subcommand};
use medicom::dimse::assoc::QueryLevel;

#[derive(Parser, Debug)]
/// Explore DICOM
pub struct Arguments {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Parses a single file and prints the DICOM elements to stdout.
    Print(PrintArgs),

    /// Browse a DICOM dataset in a text-based user interface.
    Browse(BrowseArgs),

    /// Extract Pixel Data from a DICOM file into a standard image format.
    #[cfg(feature = "image")]
    Extract(ExtractArgs),

    /// Opens a window to view image data for a DICOM file.
    #[cfg(feature = "image")]
    View(ViewArgs),

    /// Manage a database index of DICOM on disk.
    ///
    /// Recursively scans a folder for DICOM datasets, indexing them into a database.
    #[cfg(feature = "index")]
    Index(IndexArgs),

    /// Archives DICOM datasets from a source folder into a destination folder.
    ///
    /// The source folder is assumed to be unstructured whereas the DICOM datasets will be copied
    /// into the destination folder in a consistent structure:
    ///   - One series per folder
    ///   - Each DICOM file will be named in the format `[SOP_UID].dcm`
    Archive(ArchiveArgs),

    /// Starts an SCP service.
    #[cfg(feature = "index")]
    Scp(SvcProviderArgs),

    /// Issue commands as an SCU.
    Scu(SvcUserArgs),
}

#[derive(Args, Debug)]
pub struct PrintArgs {
    /// The file to process as a DICOM dataset.
    pub file: PathBuf,
}

#[derive(Args, Debug)]
pub struct BrowseArgs {
    /// The file to process as a DICOM dataset.
    pub file: PathBuf,
}

#[cfg(feature = "image")]
#[derive(Args, Debug)]
pub struct ExtractArgs {
    /// The DICOM file to extract image data from.
    pub file: PathBuf,

    /// The output file to save the encoded image.
    pub output: PathBuf,
}

#[cfg(feature = "image")]
#[derive(Args, Debug)]
pub struct ViewArgs {
    /// A folder containing DICOM files to view.
    pub input: PathBuf,
}

#[cfg(feature = "index")]
#[derive(Args, Debug)]
pub struct IndexArgs {
    #[arg(short, long)]
    /// The db URI of the index.
    pub db: String,

    #[clap(subcommand)]
    /// Index sub-command
    pub cmd: IndexCommand,
}

#[cfg(feature = "index")]
#[derive(Parser, Debug)]
pub enum IndexCommand {
    /// Recursively scans a folder for DICOM datasets, indexing them into a database.
    Scan {
        /// The folder to scan for DICOM datasets.
        folder: PathBuf,
    },
    /// Verify records in the database reference valid files on-disk.
    Verify,
}

#[derive(Args, Debug)]
pub struct ArchiveArgs {
    /// The source folder of DICOM datasets to process.
    pub source: PathBuf,

    /// The destination folder to archive datasets into.
    pub destination: PathBuf,
}

#[cfg(feature = "index")]
#[derive(Args, Debug)]
pub struct SvcProviderArgs {
    #[arg(short, long)]
    /// The host/port to bind the service on.
    pub host: String,

    #[arg(short, long)]
    /// The AE Title to run as.
    pub aetitle: String,

    #[arg(short, long)]
    /// The maximum PDU size to receive.
    ///
    /// Size is specified in bytes and should be no more than `u32::MAX`. If not specified then no
    /// maximum is configured.
    pub max_pdu_size: Option<usize>,

    #[arg(short, long)]
    /// The maximum number of concurrent associations.
    pub max_assoc: usize,

    #[arg(short, long)]
    /// The database URL for resolving DIMSE queries.
    ///
    /// If not specified then Query/Retrieve will not return results, and Store will not persist
    /// received series.
    pub db: Option<String>,

    #[arg(short, long, value_parser = parse_key_val)]
    /// Specifies an accepted AE Title for associations. Can be specified multiple times.
    ///
    /// The format for each accepted AE Title is `key=val` where `key` is a valid AE Title such as
    /// `MY_AE`, and `val` is the IP + port such as `127.0.0.1:4001`.
    ///
    /// If no accepted AE Titles are specified then all AE Titles are accepted, but cannot be
    /// connected to, such as for handling C-MOVE requests.
    pub accept_aet: Vec<(String, String)>,
}

#[derive(Args, Debug)]
pub struct SvcUserArgs {
    /// The host/port to connect to.
    #[arg(short, long)]
    pub host: String,

    /// The AE Title to represent this SCU.
    #[arg(short, long)]
    pub my_ae: String,

    /// The target AE Title on the host.
    #[arg(short, long)]
    pub host_ae: String,

    #[arg(short, long)]
    /// The maximum PDU size to receive.
    ///
    /// Size is specified in bytes and should be no more than `u32::MAX`. If not specified then no
    /// maximum is configured.
    pub max_pdu_size: Option<usize>,

    /// The SCU command to issue.
    #[clap(subcommand)]
    pub cmd: SvcUserCommand,
}

#[derive(Parser, Debug)]
pub enum SvcUserCommand {
    /// Issue a C-ECHO command.
    Echo,

    /// Issue a C-FIND command.
    Find {
        /// The "level" of the query, one of PATIENT, STUDY, SERIES, or IMAGE.
        #[arg(short, long)]
        query_level: QueryLevel,

        /// A query parameter for matching, in the form `key=val`, where `key` is a valid tag
        /// signifier. This argument can be supplied multiple times, resulting in a logical AND
        /// effect with all supplied query parameters.
        ///
        /// A valid tag signifier can take the format `GGGGEEEE`, with optional
        /// surrounding parenthesis and optional comma separating the group number from the element
        /// number. A tag signifier can also be a tag name/identifier such as `PatientID` or
        /// `PatientsName`.
        #[arg(short, long, value_parser = parse_key_val)]
        query: Vec<(String, String)>,
    },

    /// Issue a C-STORE command.
    Store {
        /// A file to transfer using C-STORE.
        #[arg(short, long)]
        file: Vec<PathBuf>,
    },

    /// Issue a C-MOVE command.
    ///
    /// Optional arguments for a query may be supplied, which will result in issuing a C-FIND first
    /// and then issuing C-MOVE on the results.
    Move {
        /// The destination AE Title for the C-MOVE command to transfer the to.
        #[arg(short, long)]
        dest_ae: String,

        /// The "level" of the query, one of PATIENT, STUDY, SERIES, or IMAGE.
        ///
        /// If not specified, but the "query" argument is, this will default to STUDY.
        #[arg(short, long)]
        query_level: Option<QueryLevel>,

        /// A query parameter for matching, in the form `key=val`, where `key` is a valid tag
        /// signifier. This argument can be supplied multiple times, resulting in a logical AND
        /// effect with all supplied query parameters.
        ///
        /// A valid tag signifier can take the format `GGGGEEEE`, with optional
        /// surrounding parenthesis and optional comma separating the group number from the element
        /// number. A tag signifier can also be a tag name/identifier such as `PatientID` or
        /// `PatientsName`.
        #[arg(short, long, value_parser = parse_key_val)]
        query: Vec<(String, String)>,
    },

    /// Issue a C-GET command.
    ///
    /// Optional arguments for a query may be supplied, which will result in issuing a C-FIND first
    /// and then issuing C-GET on the results.
    Get {
        /// The "level" of the query, one of PATIENT, STUDY, SERIES, or IMAGE.
        ///
        /// If not specified, but the "query" argument is, this will default to STUDY.
        #[arg(short, long)]
        query_level: Option<QueryLevel>,

        /// A query parameter for matching, in the form `key=val`, where `key` is a valid tag
        /// signifier. This argument can be supplied multiple times, resulting in a logical AND
        /// effect with all supplied query parameters.
        ///
        /// A valid tag signifier can take the format `GGGGEEEE`, with optional
        /// surrounding parenthesis and optional comma separating the group number from the element
        /// number. A tag signifier can also be a tag name/identifier such as `PatientID` or
        /// `PatientsName`.
        #[arg(short, long, value_parser = parse_key_val)]
        query: Vec<(String, String)>,
    },
}

fn parse_key_val(s: &str) -> Result<(String, String), Box<dyn Error + Send + Sync + 'static>> {
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{s}`"))?;

    let tag = s[..pos].to_string();
    let val = s[pos + 1..].to_string();
    Ok((tag, val))
}
