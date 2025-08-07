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

/// Represents a Window/Level that can be applied to adjust values from one scale to another.
/// Referto Part 3, Section C.11.2, specifically C.11.2.1.2 Window Center and Window Width.
#[derive(Debug)]
pub struct WindowLevel {
    name: String,
    center: f32,
    width: f32,
    out_min: f32,
    out_max: f32,
}

impl WindowLevel {
    #[must_use]
    pub fn new(name: String, center: f32, width: f32, out_min: f32, out_max: f32) -> Self {
        Self {
            name,
            center,
            width,
            out_min,
            out_max,
        }
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    #[must_use]
    pub fn center(&self) -> f32 {
        self.center
    }

    pub fn set_center(&mut self, center: f32) {
        self.center = center;
    }

    #[must_use]
    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn set_width(&mut self, width: f32) {
        self.width = width;
    }

    #[must_use]
    pub fn out_min(&self) -> f32 {
        self.out_min
    }

    pub fn set_out_min(&mut self, out_min: f32) {
        self.out_min = out_min;
    }

    #[must_use]
    pub fn out_max(&self) -> f32 {
        self.out_max
    }

    pub fn set_out_max(&mut self, out_max: f32) {
        self.out_max = out_max;
    }

    #[must_use]
    pub fn with_out(&self, out_min: f32, out_max: f32) -> Self {
        Self::new(
            self.name().to_string(),
            self.center(),
            self.width(),
            out_min,
            out_max,
        )
    }

    /// Converts the given value to this window/level, per Part 3, Section C.11.2.1.2.1.
    #[must_use]
    pub fn apply(&self, value: f32) -> f32 {
        let center = self.center - 0.5_f32;
        let width = self.width - 1_f32;
        let half_width = width / 2_f32;
        if value <= center - half_width {
            self.out_min
        } else if value > center + half_width {
            self.out_max
        } else {
            ((value - center) / width + 0.5_f32) * (self.out_max - self.out_min) + self.out_min
        }
    }
}

#[cfg(test)]
mod tests {
    use super::WindowLevel;

    #[test]
    pub fn test_winlevel() {
        let wl = WindowLevel::new(
            String::new(),
            100_f32,
            200_f32,
            f32::from(u8::MIN),
            f32::from(u8::MAX),
        );

        let v = wl.apply(0_f32) as u8;
        assert_eq!(u8::MIN, v);
        let v = wl.apply(200_f32) as u8;
        assert_eq!(u8::MAX, v);
        let v = wl.apply(100_f32) as u8;
        assert_eq!(u8::MAX / 2 + 1, v);
    }
}
