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
    ColorImage, Context, Margin, SizeHint,
};
use medicom::{
    core::{dcmobject::DicomRoot, read::ParserBuilder},
    dict::stdlookup::STANDARD_DICOM_DICTIONARY,
    load::{
        imgvol::ImageVolume,
        pixeldata::{pdwinlevel::WindowLevel, PixelDataError},
    },
};
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
    sync::Arc,
    thread,
};

use crate::{args::ViewArgs, CommandApplication};

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

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct SliceKey {
    series: SeriesKey,
    slice_index: usize,
}

impl From<&str> for SliceKey {
    fn from(value: &str) -> Self {
        let mut slice_index = 0usize;
        let mut series_uid = value;
        if let Some(lindex) = value.find('[') {
            series_uid = &value[0..lindex];
            if let Some(rindex) = value.rfind(']') {
                let slice = &value[lindex + 1..rindex];
                slice_index = slice.parse::<usize>().unwrap_or_default();
            }
        }
        Self {
            series: SeriesKey::from(series_uid),
            slice_index,
        }
    }
}

impl From<(&str, usize)> for SliceKey {
    fn from(value: (&str, usize)) -> Self {
        Self {
            series: SeriesKey::from(value.0),
            slice_index: value.1,
        }
    }
}

impl From<(&SeriesKey, usize)> for SliceKey {
    fn from(value: (&SeriesKey, usize)) -> Self {
        Self {
            series: value.0.clone(),
            slice_index: value.1,
        }
    }
}

impl std::fmt::Display for SliceKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}[{}]", self.series, self.slice_index)
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct SeriesKey {
    series_uid: String,
}

impl From<&str> for SeriesKey {
    fn from(value: &str) -> Self {
        Self {
            series_uid: value.to_string(),
        }
    }
}

impl From<&String> for SeriesKey {
    fn from(value: &String) -> Self {
        SeriesKey::from(value.as_str())
    }
}

impl From<String> for SeriesKey {
    fn from(value: String) -> Self {
        Self { series_uid: value }
    }
}

impl From<&Path> for SeriesKey {
    fn from(value: &Path) -> Self {
        Self {
            series_uid: value.display().to_string(),
        }
    }
}

impl From<&PathBuf> for SeriesKey {
    fn from(value: &PathBuf) -> Self {
        SeriesKey::from(value.as_path())
    }
}

impl std::fmt::Display for SeriesKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.series_uid)
    }
}

#[derive(Default)]
struct DicomFileImageLoader {
    vol_cache: Mutex<HashMap<SeriesKey, ImageVolume>>,
    image_cache: Mutex<HashMap<SliceKey, Arc<ColorImage>>>,
    failed: Mutex<HashSet<PathBuf>>,
}

impl DicomFileImageLoader {
    fn load_files(&self, ctx: Context, files: &Arc<Mutex<Vec<PathBuf>>>) -> Result<()> {
        // Create a copy of the files list so the files list lock does not need to be held while
        // every file is loaded.
        let files_copy: Vec<PathBuf>;
        {
            let guard = files.lock();
            files_copy = guard.iter().cloned().collect();
            drop(guard);
        }

        // Load all files.
        for path in &files_copy {
            let file = match File::open(path) {
                Err(e) => {
                    self.failed.lock().insert(path.to_owned());
                    eprintln!("Failed to open file: {path:?}: {e:?}");
                    return Err(anyhow!(e));
                }
                Ok(file) => file,
            };

            let dataset = BufReader::with_capacity(1024 * 1024, file);
            let mut parser = ParserBuilder::default().build(dataset, &STANDARD_DICOM_DICTIONARY);
            let Some(dcmroot) = DicomRoot::parse(&mut parser)? else {
                self.failed.lock().insert(path.to_owned());
                eprintln!(
                    "Failed to load {path:?}, {:?}",
                    PixelDataError::MissingPixelData
                );
                ctx.request_repaint();
                continue;
            };

            let series_key = SeriesKey::from(
                dcmroot
                    .series_instance_id()
                    .unwrap_or_else(|_| "<NO SERIES UID>".to_owned()),
            );

            let mut vol_cache = self.vol_cache.lock();
            let imgvol = vol_cache.entry(series_key).or_default();

            if let Err(e) = imgvol.load_slice(dcmroot) {
                self.failed.lock().insert(path.to_owned());
                eprintln!("Failed to load {path:?}: {e:?}");
            }
            ctx.request_repaint();
        }

        // Convert to images only after the full volume has been loaded. Necessary so the proper
        // min/max are computed for window/level.
        let vol_cache = self.vol_cache.lock();
        let mut image_cache = self.image_cache.lock();
        for imgvol in vol_cache.values() {
            for idx in 0..imgvol.slices().len() {
                let key = SliceKey::from((imgvol.series_uid().as_str(), idx));
                image_cache
                    .entry(key)
                    .or_insert_with(|| Arc::new(self.to_image(imgvol, idx)));
            }
        }

        Ok(())
    }

    fn num_slices_loaded(&self, uri: &SeriesKey) -> usize {
        if let Some(vol) = self.vol_cache.lock().get(uri) {
            vol.slices().len()
        } else {
            0
        }
    }

    fn to_image(&self, imgvol: &ImageVolume, slice_index: usize) -> ColorImage {
        // WindowLevel to map i16 values into u8.
        let min = imgvol.min_val() * imgvol.slope() + imgvol.intercept();
        let max = imgvol.max_val() * imgvol.slope() + imgvol.intercept();
        let width = max - min;
        let center = width / 2f64;
        let win = WindowLevel::new(
            String::new(),
            width,
            center,
            f64::from(u8::MIN),
            f64::from(u8::MAX),
        );
        let width = imgvol.dims().rows().into();
        let height = imgvol.dims().cols().into();

        let iter = imgvol.slice_iter(slice_index, win).map(|p| p.r as u8);
        ColorImage::from_gray_iter([width, height], iter)
    }
}

impl ImageLoader for DicomFileImageLoader {
    fn id(&self) -> &'static str {
        generate_loader_id!(DicomFileImageLoader)
    }

    fn load(&self, _ctx: &egui::Context, uri: &str, _: SizeHint) -> egui::load::ImageLoadResult {
        let mut cache = self.image_cache.lock();
        let slice_key = SliceKey::from(uri);
        if let Some(loaded_slice) = cache.get(&slice_key) {
            Ok(ImagePoll::Ready {
                image: loaded_slice.clone(),
            })
        } else {
            if let Some(imgvol) = self.vol_cache.lock().get(&slice_key.series) {
                if slice_key.slice_index < imgvol.slices().len() {
                    let image = self.to_image(imgvol, slice_key.slice_index);
                    let image = Arc::new(image);
                    cache.insert(slice_key, image.clone());
                    return Ok(ImagePoll::Ready { image });
                }
            }

            Err(LoadError::NotSupported)
        }
    }

    fn forget(&self, uri: &str) {
        self.vol_cache.lock().remove(&SeriesKey::from(uri));
    }

    fn forget_all(&self) {
        self.vol_cache.lock().clear();
    }

    fn byte_size(&self) -> usize {
        self.vol_cache
            .lock()
            .values()
            .map(ImageVolume::byte_size)
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

        // Start the current image as the middle index. Note that at this point the files list is
        // not sorted at all.
        let current_image = image_files.len() / 2;
        let loader = Arc::new(DicomFileImageLoader::default());

        // Create one list of the files, shared to the thread which will load all the images in the
        // background. After loading it modifies the input list of files to be sorted based on the
        // image position.
        let image_files = Arc::new(Mutex::new(image_files));
        let image_files_for_loading = image_files.clone();
        let loader_for_loading = loader.clone();
        let ctx = cc.egui_ctx.clone();
        thread::spawn(move || {
            if let Err(e) = loader_for_loading.load_files(ctx, &image_files_for_loading) {
                eprintln!("Error loading: {e:?}");
            }
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
        series_key: &SeriesKey,
        total: usize,
        image_loader: &Arc<DicomFileImageLoader>,
    ) -> egui::ProgressBar {
        if total == 0 {
            return egui::ProgressBar::new(1f32).text("Failed to load any images");
        }

        let loaded_count = image_loader.num_slices_loaded(series_key);
        // Unlikely precision loss since number of files would at max be up in the thousands.
        // Additionally, for reporting progress any loss of precision is fine.
        if loaded_count == total {
            let progress_text = format!("Loaded {loaded_count} images");
            egui::ProgressBar::new(1f32)
                .show_percentage()
                .text(progress_text)
        } else {
            #[allow(clippy::cast_precision_loss)]
            let progress = (loaded_count / total) as f32;
            let progress_text = format!("Loading images {loaded_count}/{total}...");
            egui::ProgressBar::new(progress)
                .animate(true)
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
            ui.spacing_mut().window_margin = Margin::same(5);

            let img_vol_cache_lock = self.image_loader.vol_cache.lock();
            let Some(series_key) = img_vol_cache_lock.keys().next().cloned() else {
                return;
            };
            drop(img_vol_cache_lock);

            // The loader is used to filter out files that failed parsing.
            let image_loader = self.image_loader.clone();

            let num_images = self.image_files.lock().len();
            let num_failed = image_loader.failed.lock().len();
            let total = num_images - num_failed;

            ui.add(ImageViewer::create_progress(
                &series_key,
                total,
                &image_loader,
            ));

            // Modify the image index for iterating.
            if ui.input(|i| i.key_pressed(egui::Key::ArrowUp) || i.key_down(egui::Key::K)) {
                if self.current_image > 0 {
                    self.current_image -= 1;
                }
            } else if ui.input(|i| i.key_pressed(egui::Key::ArrowDown) || i.key_down(egui::Key::J))
                && self.current_image != total - 1
            {
                self.current_image += 1;
            }

            let slice_key = SliceKey::from((&series_key, self.current_image));
            ui.add(egui::Image::from_uri(slice_key.to_string()));
        });
    }
}
