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
    load::{imgvol::ImageVolume, IndexVec, VolAxis},
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
    axis: VolAxis,
    slice_index: usize,
}

impl From<&str> for SliceKey {
    fn from(value: &str) -> Self {
        let mut series_uid = value;
        let mut axis = VolAxis::Z;
        let mut slice_index = 0usize;

        if let Some((_a, b)) = series_uid.split_once("X:") {
            axis = VolAxis::X;
            series_uid = b;
        } else if let Some((_a, b)) = series_uid.split_once("Y:") {
            axis = VolAxis::Y;
            series_uid = b;
        } else if let Some((_a, b)) = series_uid.split_once("Z:") {
            axis = VolAxis::Z;
            series_uid = b;
        }

        if let Some((a, b)) = series_uid.split_once("/") {
            series_uid = a;
            slice_index = b.parse::<usize>().unwrap_or_default();
        }

        Self {
            series: SeriesKey::from(series_uid),
            axis,
            slice_index,
        }
    }
}

impl From<(&str, VolAxis, usize)> for SliceKey {
    fn from(value: (&str, VolAxis, usize)) -> Self {
        Self {
            series: SeriesKey::from(value.0),
            axis: value.1,
            slice_index: value.2,
        }
    }
}

impl From<(&SeriesKey, VolAxis, usize)> for SliceKey {
    fn from(value: (&SeriesKey, VolAxis, usize)) -> Self {
        Self {
            series: value.0.clone(),
            axis: value.1,
            slice_index: value.2,
        }
    }
}

impl std::fmt::Display for SliceKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}/{}", self.axis, self.series, self.slice_index)
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
    failed: Mutex<HashSet<PathBuf>>,
}

impl DicomFileImageLoader {
    fn load_files(&self, _ctx: Context, files: &Arc<Mutex<Vec<PathBuf>>>) -> Result<()> {
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
                    eprintln!("Failed to open file: {}: {e:?}", path.display());
                    return Err(anyhow!(e));
                }
                Ok(file) => file,
            };

            let dataset = BufReader::with_capacity(1024 * 1024, file);
            let mut parser = ParserBuilder::default().build(dataset, &STANDARD_DICOM_DICTIONARY);
            let Some(dcmroot) = DicomRoot::parse(&mut parser)? else {
                self.failed.lock().insert(path.to_owned());
                eprintln!("Missing PixelData: {}", path.display());
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
                eprintln!("Failed to load {}: {e:?}", path.display());
            }
            drop(vol_cache);
        }

        Ok(())
    }

    fn num_z_slices_loaded(&self, uri: &SeriesKey) -> usize {
        if let Some(vol) = self.vol_cache.lock().get(uri) {
            vol.slices().len()
        } else {
            0
        }
    }

    fn to_image(imgvol: &ImageVolume, axis: &VolAxis, slice_index: usize) -> ColorImage {
        let win = imgvol
            .minmax_winlevel()
            .with_out(f64::from(u8::MIN), f64::from(u8::MAX));

        let axis_dims = imgvol.axis_dims(axis);

        #[allow(clippy::cast_possible_truncation)]
        let iter = imgvol
            .slice_iter(axis, slice_index)
            .map(|p| win.apply(p.r) as u8);
        ColorImage::from_gray_iter([axis_dims.x, axis_dims.y], iter)
    }
}

impl ImageLoader for DicomFileImageLoader {
    fn id(&self) -> &'static str {
        generate_loader_id!(DicomFileImageLoader)
    }

    fn load(&self, _ctx: &egui::Context, uri: &str, _: SizeHint) -> egui::load::ImageLoadResult {
        let slice_key = SliceKey::from(uri);
        if let Some(imgvol) = self.vol_cache.lock().get(&slice_key.series) {
            let axis_dims = imgvol.axis_dims(&slice_key.axis);
            if slice_key.slice_index < axis_dims.z {
                let image = Self::to_image(imgvol, &slice_key.axis, slice_key.slice_index);
                let image = Arc::new(image);
                return Ok(ImagePoll::Ready { image });
            }
        }
        Err(LoadError::NotSupported)
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

const NO_CURRENT_SLICE_SENTINEL: usize = usize::MAX;
struct ImageViewer {
    image_files: Arc<Mutex<Vec<PathBuf>>>,
    current_slice: usize,
    image_loader: Arc<DicomFileImageLoader>,
    view_axis: VolAxis,
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
            current_slice: NO_CURRENT_SLICE_SENTINEL,
            image_loader: loader_for_self,
            view_axis: VolAxis::Z,
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

    fn create_progress(total: usize, loaded_count: usize) -> egui::ProgressBar {
        if total == 0 {
            return egui::ProgressBar::new(1f32).text("Failed to load any images");
        }

        // Unlikely precision loss since number of files would at max be up in the thousands.
        // Additionally, for reporting progress any loss of precision is fine.
        if loaded_count == total {
            let progress_text = format!("Loaded {loaded_count} images");
            egui::ProgressBar::new(1f32)
                .show_percentage()
                .text(progress_text)
        } else {
            #[allow(clippy::cast_precision_loss)]
            let progress = loaded_count as f32 / total as f32;
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
            egui::MenuBar::new().ui(ui, |ui| {
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

            // File processing progress.
            let num_files = self.image_files.lock().len();
            let num_failed = image_loader.failed.lock().len();
            let total_files = num_files - num_failed;
            let loaded_count = image_loader.num_z_slices_loaded(&series_key);
            let finished_loading = loaded_count == total_files;
            ui.add(ImageViewer::create_progress(total_files, loaded_count));

            let vol_cache = image_loader.vol_cache.lock();
            let imgvol = vol_cache.get(&series_key);
            let Some(imgvol) = imgvol else {
                return;
            };

            ui.label(imgvol.patient_name());
            ui.label(imgvol.patient_id());

            if !finished_loading {
                return;
            }

            let axis = self.view_axis.to_owned();
            let axis_dims = imgvol.axis_dims(&axis);
            let num_slices = axis_dims.z;
            if self.current_slice == NO_CURRENT_SLICE_SENTINEL {
                self.current_slice = num_slices / 2;
            }

            // Modify the image index for iterating.
            if ui.input(|i| i.key_down(egui::Key::ArrowUp) || i.key_down(egui::Key::K)) {
                self.current_slice = self.current_slice.saturating_sub(1);
            } else if ui.input(|i| i.key_down(egui::Key::ArrowDown) || i.key_down(egui::Key::J)) {
                if self.current_slice < num_slices - 1 {
                    self.current_slice += 1;
                }
            } else if ui.input(|i| i.key_pressed(egui::Key::V)) {
                match axis {
                    VolAxis::X => self.view_axis = VolAxis::Y,
                    VolAxis::Y => self.view_axis = VolAxis::Z,
                    VolAxis::Z => self.view_axis = VolAxis::X,
                }
                self.current_slice = NO_CURRENT_SLICE_SENTINEL;
                // Don't finish rendering, let the next render pick up on this axis change.
                return;
            } else if ui.input(|i| i.key_pressed(egui::Key::Q)) {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }

            let mut index_coord = IndexVec::default();
            match axis {
                VolAxis::X => index_coord.x = self.current_slice,
                VolAxis::Y => index_coord.y = self.current_slice,
                VolAxis::Z => index_coord.z = self.current_slice,
            }
            let dcm_pos = imgvol.dims().coordinate(index_coord);
            ui.label(format!(
                "Top-left Loc: {:.2}, {:.2}, {:.2}",
                dcm_pos.x, dcm_pos.y, dcm_pos.z
            ));
            ui.label(format!("Slice Dims: {}x{}", axis_dims.x, axis_dims.y));
            ui.label(imgvol.series_desc());

            ui.label(format!("Slice No: {}/{num_slices}", self.current_slice + 1));
            ui.label(format!("Series UID: {}", series_key.series_uid));

            // Need to manually drop the cache lock before slice/image loading (via adding an image
            // to the ui), otherwise it results in a deadlock.
            drop(vol_cache);

            let slice_key = SliceKey::from((&series_key, axis, self.current_slice));
            ui.add(egui::Image::from_uri(slice_key.to_string()));
        });
    }
}
