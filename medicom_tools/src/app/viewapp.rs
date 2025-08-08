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

use anyhow::Result;
use egui::{
    generate_loader_id,
    load::{ImageLoader, ImagePoll},
    ColorImage, Margin, SizeHint,
};
use medicom::load::{
    imgvol::ImageVolume, pixeldata::LoadError, workspace::Workspace, IndexVec, LoadableChunkKey,
    LoadableKey, Loader, SeriesSource, SeriesSourceLoadResult, VolAxis,
};
use std::{
    fs::File,
    ops::Deref,
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
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
    series: LoadableKey,
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

        // The series_uid component could be a file path and include forward-slashes.
        if let Some((a, b)) = series_uid.rsplit_once('/') {
            series_uid = a;
            slice_index = b.parse::<usize>().unwrap_or_default();
        }

        Self {
            series: LoadableKey::from(series_uid),
            axis,
            slice_index,
        }
    }
}

impl From<(&str, VolAxis, usize)> for SliceKey {
    fn from(value: (&str, VolAxis, usize)) -> Self {
        Self {
            series: LoadableKey::from(value.0),
            axis: value.1,
            slice_index: value.2,
        }
    }
}

impl From<(&LoadableKey, VolAxis, usize)> for SliceKey {
    fn from(value: (&LoadableKey, VolAxis, usize)) -> Self {
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

struct FlatFolderSeriesSource {
    folder: PathBuf,
    progress: RwLock<SeriesSourceLoadResult>,
}

impl FlatFolderSeriesSource {
    pub fn new(folder: PathBuf) -> Result<Self> {
        let mut chunks = Vec::new();
        if folder.is_file() {
            chunks.push(LoadableChunkKey::new(folder.display().to_string()));
        } else {
            let files = folder.read_dir().map_err(LoadError::from)?;
            for file in files {
                let file = file.map_err(LoadError::from)?.path();
                chunks.push(LoadableChunkKey::new(file.display().to_string()));
            }
        }

        let progress = SeriesSourceLoadResult::new(chunks);
        Ok(Self {
            folder,
            progress: RwLock::new(progress),
        })
    }

    fn folder_to_key(folder: &Path) -> LoadableKey {
        LoadableKey::new(folder.display().to_string())
    }

    #[allow(dead_code)]
    fn key_to_folder(key: &LoadableKey) -> PathBuf {
        PathBuf::from(key.key())
    }

    #[allow(dead_code)]
    fn file_to_key(file: &Path) -> LoadableChunkKey {
        LoadableChunkKey::new(file.display().to_string())
    }

    fn key_to_file(key: &LoadableChunkKey) -> PathBuf {
        PathBuf::from(key.chunk_key())
    }
}

impl SeriesSource<File> for FlatFolderSeriesSource {
    fn loadable_key(&self) -> LoadableKey {
        Self::folder_to_key(&self.folder)
    }

    fn chunks(&self) -> std::result::Result<Vec<LoadableChunkKey>, LoadError> {
        if let Ok(progress) = self.progress.read() {
            Ok(progress.total().clone())
        } else {
            Ok(Vec::with_capacity(0))
        }
    }

    fn chunk_stream(&self, chunk_key: &LoadableChunkKey) -> std::result::Result<File, LoadError> {
        let path = Self::key_to_file(chunk_key);
        File::open(path).map_err(LoadError::from)
    }
}

#[derive(Default)]
struct DicomFileImageLoader {
    workspace: RwLock<Workspace>,
}

impl DicomFileImageLoader {
    fn to_image(imgvol: &ImageVolume, axis: &VolAxis, slice_index: usize) -> ColorImage {
        let win = imgvol
            .minmax_winlevel()
            .with_out(f32::from(u8::MIN), f32::from(u8::MAX));

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
        if let Ok(workspace) = self.workspace.read() {
            if let Some(imgvol) = workspace.volume(&slice_key.series) {
                let axis_dims = imgvol.axis_dims(&slice_key.axis);
                if slice_key.slice_index < axis_dims.z {
                    let image = Self::to_image(imgvol, &slice_key.axis, slice_key.slice_index);
                    let image = Arc::new(image);
                    return Ok(ImagePoll::Ready { image });
                }
            }
        }
        Err(egui::load::LoadError::NotSupported)
    }

    fn forget(&self, uri: &str) {
        if let Ok(mut workspace) = self.workspace.write() {
            workspace.unload(&LoadableKey::from(uri));
        }
    }

    fn forget_all(&self) {
        if let Ok(mut workspace) = self.workspace.write() {
            workspace.unload_all();
        }
    }

    fn byte_size(&self) -> usize {
        if let Ok(workspace) = self.workspace.read() {
            workspace.volumes().map(ImageVolume::byte_size).sum()
        } else {
            0
        }
    }
}

const NO_CURRENT_SLICE_SENTINEL: usize = usize::MAX;
struct ImageViewer {
    source: Arc<FlatFolderSeriesSource>,
    current_slice: usize,
    image_loader: Arc<DicomFileImageLoader>,
    view_axis: VolAxis,
}

impl ImageViewer {
    fn new(input: &Path, cc: &eframe::CreationContext<'_>) -> Result<Self> {
        // Start the current image as the middle index. Note that at this point the files list is
        // not sorted at all.
        let loader = Arc::new(DicomFileImageLoader::default());

        let source = Arc::new(FlatFolderSeriesSource::new(input.to_path_buf())?);

        // Create one list of the files, shared to the thread which will load all the images in the
        // background. After loading it modifies the input list of files to be sorted based on the
        // image position.
        let loader_for_loading = loader.clone();
        let source_for_loading = source.clone();
        thread::spawn(move || {
            let loader = Loader::<File>::new();
            if let Err(e) = loader.load_into(
                &*source_for_loading,
                &loader_for_loading.workspace,
                Some(&source_for_loading.progress),
            ) {
                eprintln!("Error loading: {e:?}");
            }
        });

        let loader_for_self = loader.clone();
        cc.egui_ctx.add_image_loader(loader);
        Ok(Self {
            source,
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

            let series_key = self.source.loadable_key();

            let mut finished_loading = false;
            if let Ok(progress) = self.source.progress.try_read() {
                let num_files = progress.num_total();
                let num_failed = progress.num_failed();
                let total_files = num_files - num_failed;
                let loaded_count = progress.num_loaded();
                finished_loading = loaded_count == total_files;
                ui.add(ImageViewer::create_progress(total_files, loaded_count));
            } else {
                ui.add(
                    egui::ProgressBar::new(0_f32)
                        .animate(true)
                        .text("Unable to report progress"),
                );
            }

            let Ok(workspace) = self.image_loader.workspace.try_read() else {
                return;
            };
            let imgvol = workspace.volume(&series_key);
            let Some(imgvol) = imgvol else {
                return;
            };
            ui.label(imgvol.patient_name());
            ui.label(imgvol.patient_id());

            if !finished_loading {
                return;
            }

            let axis = self.view_axis.clone();
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
            ui.label(format!("Series UID: {}", imgvol.series_uid()));

            // Need to manually drop the cache lock before slice/image loading (via adding an image
            // to the ui), otherwise it results in a deadlock.
            drop(workspace);

            let slice_key = SliceKey::from((&series_key, axis, self.current_slice));
            ui.add(egui::Image::from_uri(slice_key.to_string()));
        });
    }
}
