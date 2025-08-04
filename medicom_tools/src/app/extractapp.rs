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

//! This command extracts pixel data and encodes it as a standard image format.

use anyhow::{anyhow, Result};
use image::{ImageBuffer, Rgb};
use medicom::{
    core::{dcmobject::DicomRoot, defn::ts::TSRef},
    load::{imgvol::ImageVolume, VolAxis},
};

use crate::{app::parse_file, args::ExtractArgs, CommandApplication};

pub struct ExtractApp {
    args: ExtractArgs,
}

impl ExtractApp {
    pub fn new(args: ExtractArgs) -> ExtractApp {
        ExtractApp { args }
    }

    pub(crate) fn is_jpeg(ts: TSRef) -> bool {
        ts.uid().name().contains("JPEG")
    }

    fn extract_image(&self) -> Result<()> {
        let mut output = self.args.output.clone();
        let extension = output
            .extension()
            .and_then(|extension| extension.to_owned().into_string().ok())
            .unwrap_or("png".to_owned());
        output.set_extension("");
        let filename = output
            .file_name()
            .and_then(|filename| filename.to_owned().into_string().ok())
            .unwrap_or("image".to_string());

        let mut parser = parse_file(&self.args.file, true)?;
        if ExtractApp::is_jpeg(parser.ts()) {
            return Err(anyhow!(
                "Unsupported TransferSyntax: {}",
                parser.ts().uid().name()
            ));
        }

        let Some(dcmroot) = DicomRoot::parse(&mut parser)? else {
            return Err(anyhow!("DICOM SOP is missing PixelData"));
        };
        let mut imgvol = ImageVolume::default();
        imgvol.load_slice(dcmroot)?;
        let win = imgvol
            .minmax_winlevel()
            .with_out(f64::from(u8::MIN), f64::from(u8::MAX));

        let axis = VolAxis::Z;
        let axis_dims = imgvol.axis_dims(&axis);

        let mut image: ImageBuffer<Rgb<u8>, Vec<u8>> =
            ImageBuffer::new(u32::try_from(axis_dims.x)?, u32::try_from(axis_dims.y)?);
        for pix in imgvol.slice_iter(&axis, 0) {
            #[allow(clippy::cast_possible_truncation)]
            let val = win.apply(pix.r) as u8;
            image.put_pixel(
                u32::try_from(pix.coord.x)?,
                u32::try_from(pix.coord.y)?,
                Rgb([val, val, val]),
            );
        }
        image.save(format!("{filename}.{extension}"))?;

        Ok(())
    }
}

impl CommandApplication for ExtractApp {
    fn run(&mut self) -> Result<()> {
        self.extract_image()
    }
}
