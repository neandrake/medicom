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

#![allow(clippy::module_name_repetitions)]

use std::process;

use clap::Parser;

#[cfg(feature = "image")]
use crate::app::imageapp::ImageApp;

#[cfg(feature = "index")]
use crate::app::{indexapp::IndexApp, scpapp::SvcProviderApp};

use crate::{
    app::{
        archiveapp::ArchiveApp, browseapp::BrowseApp, printapp::PrintApp, scuapp::SvcUserApp,
        CommandApplication,
    },
    args::{Arguments, Command},
};

mod app;
mod args;
mod threadpool;

#[cfg(feature = "dhat")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat")]
    let _profiler = dhat::Profiler::new_heap();

    let mut app: Box<dyn CommandApplication> = make_app();
    if let Err(e) = app.run() {
        eprintln!("Error: {e:?}");
        process::exit(1);
    }
}

fn make_app() -> Box<dyn CommandApplication> {
    let args: Arguments = Arguments::parse();

    match args.command {
        Command::Print(args) => Box::new(PrintApp::new(args)),
        Command::Browse(args) => Box::new(BrowseApp::new(args)),
        #[cfg(feature = "image")]
        Command::Image(args) => Box::new(ImageApp::new(args)),
        #[cfg(feature = "index")]
        Command::Index(args) => Box::new(IndexApp::new(args)),
        Command::Archive(args) => Box::new(ArchiveApp::new(args)),
        #[cfg(feature = "index")] // Running SCP service requires the archive database.
        Command::Scp(args) => Box::new(SvcProviderApp::new(args)),
        Command::Scu(args) => Box::new(SvcUserApp::new(args)),
    }
}
