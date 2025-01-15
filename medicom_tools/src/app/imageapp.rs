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

//! The image command extracts pixel data and encodes it as a standard image format.

use anyhow::{anyhow, Result};
use egui::{
    generate_loader_id,
    load::{ImageLoader, ImagePoll, LoadError},
    mutex::Mutex,
    ColorImage, SizeHint,
};
use image::{ImageBuffer, Rgb};
use medicom::core::{
    defn::ts::TSRef,
    pixeldata::{
        pdinfo::PixelDataSliceInfo, pdslice::PixelDataSlice, pdwinlevel::WindowLevel,
        pixel_i16::PixelI16, pixel_u16::PixelU16, pixel_u8::PixelU8,
    },
};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::{app::parse_file, args::ImageArgs, CommandApplication};

pub struct ImageApp {
    args: ImageArgs,
}

impl ImageApp {
    pub fn new(args: ImageArgs) -> ImageApp {
        ImageApp { args }
    }

    fn is_jpeg(ts: TSRef) -> bool {
        ts.uid().name().contains("JPEG")
    }
}

impl CommandApplication for ImageApp {
    fn run(&mut self) -> Result<()> {
        if true {
            ImageViewer::open_viewer(self.args.file.clone())?;
        } else {
            ImageExtractor::extract_image(&self.args.file, &mut self.args.output)?;
        }
        Ok(())
    }
}

struct ImageExtractor;
impl ImageExtractor {
    fn extract_image(input: &Path, output: &mut PathBuf) -> Result<()> {
        let parser = parse_file(input, true)?;

        if ImageApp::is_jpeg(parser.ts()) {
            return Err(anyhow!(
                "Unsupported TransferSyntax: {}",
                parser.ts().uid().name()
            ));
        }

        let pixdata_info = PixelDataSliceInfo::process_dcm_parser(parser)?;
        let pixdata_buffer = pixdata_info.load_pixel_data()?;
        dbg!(&pixdata_buffer);

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

#[derive(Default)]
struct DicomFileImageLoader {
    cache: Mutex<HashMap<String, Arc<ColorImage>>>,
}

impl DicomFileImageLoader {
    pub fn load_dicom(&self, file: &Path) -> Result<()> {
        let parser = parse_file(file, true)?;

        if ImageApp::is_jpeg(parser.ts()) {
            return Err(anyhow!(
                "Unsupported TransferSyntax: {}",
                parser.ts().uid().name()
            ));
        }

        let pixdata_info = PixelDataSliceInfo::process_dcm_parser(parser)?;
        let pixdata_buffer = pixdata_info.load_pixel_data()?;
        dbg!(&pixdata_buffer);

        match pixdata_buffer {
            PixelDataSlice::U8(pdslice) => {
                let width = pdslice.info().rows().into();
                let height = pdslice.info().cols().into();
                let image = ColorImage::from_rgb([width, height], pdslice.buffer());
                self.cache
                    .lock()
                    .insert(format!("dicom://{}", file.display()), Arc::new(image));
            }
            PixelDataSlice::U16(pdslice) => {
                // WindowLevel to map i16 values into u8.
                let win = WindowLevel::new(
                    "".to_string(),
                    f64::from(i16::MAX) / 2_f64,
                    f64::from(i16::MAX),
                    f64::from(u8::MIN),
                    f64::from(u8::MAX),
                );
                let width = pdslice.info().rows().into();
                let height = pdslice.info().cols().into();

                let iter = pdslice.pixel_iter_with_win(win).map(|p| p.r as u8);
                let image = ColorImage::from_gray_iter([width, height], iter);
                self.cache
                    .lock()
                    .insert(format!("dicom://{}", file.display()), Arc::new(image));
            }
            PixelDataSlice::I16(pdslice) => {
                // WindowLevel to map i16 values into u8.
                let win = pdslice
                    .info()
                    .win_levels()
                    .last()
                    .map(|w| {
                        WindowLevel::new(
                            w.name().to_string(),
                            pdslice.rescale(w.center()),
                            pdslice.rescale(w.width()),
                            f64::from(u8::MIN),
                            f64::from(u8::MAX),
                        )
                    })
                    .unwrap_or_else(|| {
                        WindowLevel::new(
                            "".to_string(),
                            0_f64,
                            f64::from(i16::MAX),
                            f64::from(u8::MIN),
                            f64::from(u8::MAX),
                        )
                    });
                let width = pdslice.info().rows().into();
                let height = pdslice.info().cols().into();

                let iter = pdslice.pixel_iter_with_win(win).map(|p| p.r as u8);
                let image = ColorImage::from_gray_iter([width, height], iter);
                self.cache
                    .lock()
                    .insert(format!("dicom://{}", file.display()), Arc::new(image));
            }
            other => {
                return Err(anyhow!("Unsupported PixelData: {other:?}"));
            }
        }

        Ok(())
    }
}

impl ImageLoader for DicomFileImageLoader {
    fn id(&self) -> &str {
        generate_loader_id!(DicomFileImageLoader)
    }

    fn load(&self, _ctx: &egui::Context, uri: &str, _: SizeHint) -> egui::load::ImageLoadResult {
        if let Some(image) = self.cache.lock().get(uri) {
            Ok(ImagePoll::Ready {
                image: image.clone(),
            })
        } else {
            let uri = uri.trim_start_matches("dicom://");
            self.load_dicom(&PathBuf::from(uri))
                .map_err(|e| LoadError::Loading(format!("{e:?}")))?;
            Err(LoadError::NotSupported)
        }
    }

    fn forget(&self, uri: &str) {
        self.cache.lock().remove(uri);
    }

    fn forget_all(&self) {
        self.cache.lock().clear();
    }

    fn byte_size(&self) -> usize {
        self.cache
            .lock()
            .values()
            .map(|image| image.as_raw().len())
            .sum()
    }
}

struct ImageViewer {
    input: PathBuf,
}

impl ImageViewer {
    fn new(input: PathBuf, cc: &eframe::CreationContext<'_>) -> Result<Self> {
        let loader = Arc::new(DicomFileImageLoader::default());
        loader.load_dicom(&input)?;
        cc.egui_ctx.add_image_loader(loader);
        Ok(Self { input })
    }

    fn open_viewer(input: PathBuf) -> Result<()> {
        let native_options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([400.0, 300.0])
                .with_min_inner_size([300.0, 220.0]),
            ..Default::default()
        };

        eframe::run_native(
            "medicom_image_viewer",
            native_options,
            Box::new(|cc| Ok(Box::new(ImageViewer::new(input, cc)?))),
        )?;

        Ok(())
    }
}

impl eframe::App for ImageViewer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    let quit_btn = ui.button("Quit");
                    if quit_btn.clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.add_space(16.0);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Image Viewer");
            ui.add(egui::Image::from_uri(format!(
                "dicom://{}",
                self.input.display()
            )));
        });
    }
}
