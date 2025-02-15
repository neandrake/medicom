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

//! This command opens a viewer for a DICOM image.

use anyhow::{anyhow, Result};
use egui::{
    generate_loader_id,
    load::{ImageLoader, ImagePoll, LoadError},
    mutex::Mutex,
    ColorImage, Margin, SizeHint,
};
use medicom::core::pixeldata::{
    pdinfo::PixelDataSliceInfo, pdslice::PixelDataSlice, pdwinlevel::WindowLevel,
};
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    sync::Arc,
    thread,
};

use crate::{
    app::{extractapp::ExtractApp, parse_file},
    args::ViewArgs,
    CommandApplication,
};

pub struct ViewApp {
    args: ViewArgs,
}

impl ViewApp {
    pub fn new(args: ViewArgs) -> Self {
        Self { args }
    }
}

impl CommandApplication for ViewApp {
    fn run(&mut self) -> Result<()> {
        ImageViewer::open_viewer(&self.args.input)
    }
}

#[derive(PartialEq, Eq, Hash)]
struct DicomUri {
    path: String,
}

impl DicomUri {}

impl From<&str> for DicomUri {
    fn from(value: &str) -> Self {
        Self {
            path: value.trim_start_matches("dicom://").to_string(),
        }
    }
}

impl From<&Path> for DicomUri {
    fn from(value: &Path) -> Self {
        Self {
            path: value.display().to_string(),
        }
    }
}

impl From<&PathBuf> for DicomUri {
    fn from(value: &PathBuf) -> Self {
        DicomUri::from(value.as_path())
    }
}

struct LoadedSlice {
    file: PathBuf,
    slice_info: PixelDataSlice,
    image: Arc<ColorImage>,
}

#[derive(Default)]
struct DicomFileImageLoader {
    cache: Mutex<HashMap<DicomUri, LoadedSlice>>,
    failed: Mutex<HashSet<DicomUri>>,
}

impl DicomFileImageLoader {
    fn load_files(&self, files: &Arc<Mutex<Vec<PathBuf>>>) {
        let mut files = files.lock();
        for file in files.iter() {
            if let Err(e) = self.load_dicom_file(file) {
                self.failed.lock().insert(DicomUri::from(file));
                eprintln!("Failed to load: {e:?}");
            }
        }

        let mut slices: Vec<&LoadedSlice> = Vec::with_capacity(files.len());
        let cache = self.cache.lock();
        for slice in cache.values() {
            slices.push(slice);
        }
        slices.sort_by(|a, b| {
            let a_pos = a.slice_info.info().image_pos()[2];
            let b_pos = b.slice_info.info().image_pos()[2];
            if a_pos < b_pos {
                Ordering::Less
            } else if a_pos > b_pos {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });

        files.clear();
        files.extend(slices.iter().map(|s| s.file.clone()));
    }

    fn load_dicom_file(&self, file: &Path) -> Result<()> {
        let parser = parse_file(file, true)?;

        if ExtractApp::is_jpeg(parser.ts()) {
            return Err(anyhow!(
                "Unsupported TransferSyntax: {}",
                parser.ts().uid().name()
            ));
        }

        let pixdata_info = PixelDataSliceInfo::process_dcm_parser(parser)?;
        let slice_info = pixdata_info.load_pixel_data()?;
        //dbg!(&pixdata_buffer);

        match slice_info {
            PixelDataSlice::U8(pdslice) => {
                let width = pdslice.info().rows().into();
                let height = pdslice.info().cols().into();
                let image = ColorImage::from_rgb([width, height], pdslice.buffer());
                let entry = LoadedSlice {
                    file: file.to_path_buf(),
                    slice_info: PixelDataSlice::U8(pdslice),
                    image: Arc::new(image),
                };
                self.cache.lock().insert(DicomUri::from(file), entry);
            }
            PixelDataSlice::U16(pdslice) => {
                // WindowLevel to map i16 values into u8.
                let win = WindowLevel::new(
                    String::new(),
                    f64::from(i16::MAX) / 2_f64,
                    f64::from(i16::MAX),
                    f64::from(u8::MIN),
                    f64::from(u8::MAX),
                );
                let width = pdslice.info().rows().into();
                let height = pdslice.info().cols().into();

                let iter = pdslice.pixel_iter_with_win(win).map(|p| p.r as u8);
                let image = ColorImage::from_gray_iter([width, height], iter);
                let entry = LoadedSlice {
                    file: file.to_path_buf(),
                    slice_info: PixelDataSlice::U16(pdslice),
                    image: Arc::new(image),
                };
                self.cache.lock().insert(DicomUri::from(file), entry);
            }
            PixelDataSlice::I16(pdslice) => {
                // WindowLevel to map i16 values into u8.
                let win = pdslice.info().win_levels().last().map_or_else(
                    || {
                        WindowLevel::new(
                            String::new(),
                            0_f64,
                            f64::from(i16::MAX),
                            f64::from(u8::MIN),
                            f64::from(u8::MAX),
                        )
                    },
                    |w| {
                        WindowLevel::new(
                            w.name().to_string(),
                            pdslice.rescale(w.center()),
                            pdslice.rescale(w.width()),
                            f64::from(u8::MIN),
                            f64::from(u8::MAX),
                        )
                    },
                );
                let width = pdslice.info().rows().into();
                let height = pdslice.info().cols().into();

                let iter = pdslice.pixel_iter_with_win(win).map(|p| p.r as u8);
                let image = ColorImage::from_gray_iter([width, height], iter);
                let entry = LoadedSlice {
                    file: file.to_path_buf(),
                    slice_info: PixelDataSlice::I16(pdslice),
                    image: Arc::new(image),
                };
                self.cache.lock().insert(DicomUri::from(file), entry);
            }
            other => {
                return Err(anyhow!("Unsupported PixelData: {other:?}"));
            }
        }

        Ok(())
    }

    fn is_loaded(&self, uri: &DicomUri) -> bool {
        self.cache.lock().contains_key(uri)
    }

    fn is_failed(&self, uri: &DicomUri) -> bool {
        self.failed.lock().contains(uri)
    }
}

impl ImageLoader for DicomFileImageLoader {
    fn id(&self) -> &'static str {
        generate_loader_id!(DicomFileImageLoader)
    }

    fn load(&self, _ctx: &egui::Context, uri: &str, _: SizeHint) -> egui::load::ImageLoadResult {
        let cache = self.cache.lock();
        if let Some(loaded_slice) = cache.get(&DicomUri::from(uri)) {
            Ok(ImagePoll::Ready {
                image: loaded_slice.image.clone(),
            })
        } else {
            // Must release the lock used to check the cache before it can be used in
            // load_dicom_file, otherwise this inflicts a deadlock.
            drop(cache);
            let uri = DicomUri::from(uri).path;
            self.load_dicom_file(&PathBuf::from(uri))
                .map_err(|e| LoadError::Loading(format!("{e:?}")))?;
            Err(LoadError::NotSupported)
        }
    }

    fn forget(&self, uri: &str) {
        self.cache.lock().remove(&DicomUri::from(uri));
    }

    fn forget_all(&self) {
        self.cache.lock().clear();
    }

    fn byte_size(&self) -> usize {
        self.cache
            .lock()
            .values()
            .map(|slice| slice.image.as_raw().len())
            .sum()
    }
}

struct ImageViewer {
    image_files: Arc<Mutex<Vec<PathBuf>>>,
    current_image: usize,
    image_loader: Arc<DicomFileImageLoader>,
}

impl ImageViewer {
    fn new(input: &Path, cc: &eframe::CreationContext<'_>) -> Result<Self> {
        let mut image_files = Vec::new();
        if input.is_dir() {
            let files = input.read_dir()?;
            for file in files {
                let file = file?.path();
                image_files.push(file);
            }
        } else if input.is_file() {
            image_files.push(input.to_path_buf());
        }

        // TODO: Sort by position
        let current_image = image_files.len() / 2;
        let loader = Arc::new(DicomFileImageLoader::default());

        let image_files = Arc::new(Mutex::new(image_files));
        let image_files_for_loading = image_files.clone();
        let loader_for_loading = loader.clone();
        thread::spawn(move || {
            loader_for_loading.load_files(&image_files_for_loading);
        });

        let loader_for_self = loader.clone();
        cc.egui_ctx.add_image_loader(loader);
        Ok(Self {
            image_files,
            current_image,
            image_loader: loader_for_self,
        })
    }

    fn open_viewer(input: &Path) -> Result<()> {
        let native_options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([1024.0, 768.0])
                .with_min_inner_size([1024.0, 768.0]),
            ..Default::default()
        };

        eframe::run_native(
            "Medicom Image Viewer",
            native_options,
            Box::new(|cc| Ok(Box::new(ImageViewer::new(input, cc)?))),
        )?;

        Ok(())
    }

    fn create_progress(
        image_files: &mut Vec<PathBuf>,
        image_loader: &Arc<DicomFileImageLoader>,
    ) -> egui::ProgressBar {
        // Remove non-DICOM files from the list, and do not count for progress.
        image_files.retain(|f| !image_loader.is_failed(&DicomUri::from(f)));
        let mut loaded_count = 0_f32;
        for file in image_files.iter() {
            if image_loader.is_loaded(&DicomUri::from(file)) {
                loaded_count += 1_f32;
            }
        }
        let progress = loaded_count / image_files.len() as f32;
        if progress < 1_f32 {
            let progress_text = format!(
                "Loading images {}/{}...",
                loaded_count as usize,
                image_files.len()
            );
            egui::ProgressBar::new(progress)
                .animate(true)
                .show_percentage()
                .text(progress_text)
        } else {
            let progress_text = format!("Loaded {} images", loaded_count as usize);
            egui::ProgressBar::new(progress)
                .show_percentage()
                .text(progress_text)
        }
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
            ui.spacing_mut().window_margin = Margin::same(5.0);

            let mut image_files = self.image_files.lock();
            let image_loader = self.image_loader.clone();
            ui.add(ImageViewer::create_progress(
                &mut image_files,
                &image_loader,
            ));
            if ui.input(|i| i.key_pressed(egui::Key::ArrowUp) || i.key_down(egui::Key::K)) {
                if self.current_image > 0 {
                    self.current_image -= 1;
                }
            } else if ui.input(|i| i.key_pressed(egui::Key::ArrowDown) || i.key_down(egui::Key::J))
                && self.current_image != image_files.len() - 1
            {
                self.current_image += 1;
            }
            let cur_image_uri = format!("dicom://{}", image_files[self.current_image].display());
            ui.add(egui::Image::from_uri(cur_image_uri));
        });
    }
}
