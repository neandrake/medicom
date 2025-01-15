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
use medicom::core::{
    defn::ts::TSRef,
    pixeldata::{
        pdinfo::PixelDataSliceInfo, pdslice::PixelDataSlice, pixel_i16::PixelI16,
        pixel_u16::PixelU16, pixel_u8::PixelU8,
    },
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
        let parser = parse_file(&self.args.file, true)?;

        if ExtractApp::is_jpeg(parser.ts()) {
            return Err(anyhow!(
                "Unsupported TransferSyntax: {}",
                parser.ts().uid().name()
            ));
        }

        let pixdata_info = PixelDataSliceInfo::process_dcm_parser(parser)?;
        let pixdata_buffer = pixdata_info.load_pixel_data()?;
        dbg!(&pixdata_buffer);

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

        match pixdata_buffer {
            PixelDataSlice::U8(pdslice) => {
                let mut image: ImageBuffer<Rgb<u8>, Vec<u8>> =
                    ImageBuffer::new(pdslice.info().cols().into(), pdslice.info().rows().into());
                let mut last_z = 0;
                for PixelU8 { x, y, z, r, g, b } in pdslice.pixel_iter() {
                    if z != last_z {
                        image.save(format!("{filename}.{last_z}.{extension}"))?;
                        image = ImageBuffer::new(
                            pdslice.info().cols().into(),
                            pdslice.info().rows().into(),
                        );
                    }
                    last_z = z;
                    image.put_pixel(u32::try_from(x)?, u32::try_from(y)?, Rgb([r, g, b]));
                }
                image.save(format!("{filename}.{last_z}.{extension}"))?;
            }
            PixelDataSlice::U16(pdslice) => {
                let mut image: ImageBuffer<Rgb<u16>, Vec<u16>> =
                    ImageBuffer::new(pdslice.info().cols().into(), pdslice.info().rows().into());
                let mut last_z = 0;
                for PixelU16 { x, y, z, r, g, b } in pdslice.pixel_iter() {
                    if z != last_z {
                        image.save(format!("{filename}.{last_z}.{extension}"))?;
                        image = ImageBuffer::new(
                            pdslice.info().cols().into(),
                            pdslice.info().rows().into(),
                        );
                    }
                    last_z = z;
                    image.put_pixel(u32::try_from(x)?, u32::try_from(y)?, Rgb([r, g, b]));
                }
                image.save(format!("{filename}.{last_z}.{extension}"))?;
            }
            PixelDataSlice::I16(pdslice) => {
                let mut image: ImageBuffer<Rgb<u16>, Vec<u16>> =
                    ImageBuffer::new(pdslice.info().cols().into(), pdslice.info().rows().into());
                let mut last_z = 0;
                for PixelU16 { x, y, z, r, g, b } in
                    // The "image" crate does not support i16 pixel values.
                    pdslice.pixel_iter().map(|PixelI16 { x, y, z, r, g, b }| {
                            let r = PixelDataSlice::shift_i16(r);
                            let g = PixelDataSlice::shift_i16(g);
                            let b = PixelDataSlice::shift_i16(b);
                            PixelU16 { x, y, z, r, g, b }
                        })
                {
                    if z != last_z {
                        image.save(format!("{filename}.{last_z}.{extension}"))?;
                        image = ImageBuffer::new(
                            pdslice.info().cols().into(),
                            pdslice.info().rows().into(),
                        );
                    }
                    last_z = z;
                    image.put_pixel(u32::try_from(x)?, u32::try_from(y)?, Rgb([r, g, b]));
                }
                image.save(format!("{filename}.{last_z}.{extension}"))?;
            }
            other => {
                return Err(anyhow!("Unsupported PixelData: {other:?}"));
            }
        }

        Ok(())
    }
}

impl CommandApplication for ExtractApp {
    fn run(&mut self) -> Result<()> {
        self.extract_image()
    }
}
