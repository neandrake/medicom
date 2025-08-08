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

use std::collections::BTreeMap;

use crate::load::{imgvol::ImageVolume, LoadableKey};

#[derive(Default)]
pub struct Workspace {
    volumes: BTreeMap<LoadableKey, ImageVolume>,
}

impl Workspace {
    pub fn volume(&self, loadable_key: &LoadableKey) -> Option<&ImageVolume> {
        self.volumes.get(loadable_key)
    }

    pub fn volume_mut(&mut self, loadable_key: &LoadableKey) -> Option<&mut ImageVolume> {
        self.volumes.get_mut(loadable_key)
    }

    pub fn volumes(&self) -> impl Iterator<Item = &ImageVolume> {
        self.volumes.values()
    }

    #[must_use]
    pub fn initialize_vol(&mut self, loadable_key: LoadableKey) -> &mut ImageVolume {
        // Remove any existing volume with the same key.
        if let Some(_existing) = self.volumes.remove(&loadable_key) {
            // TODO: Log a warning about replacing an existing volume.
        }
        // This will now always insert a new volume initialized with default.
        self.volumes.entry(loadable_key).or_default()
    }

    pub fn unload(&mut self, loadable_key: &LoadableKey) {
        self.volumes.remove(loadable_key);
    }

    pub fn unload_all(&mut self) {
        self.volumes.clear();
    }
}
