// Copyright (c) Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

pub struct EngineStats {
    pub fps: f32,
    pub frame_count: u64,
}

impl EngineStats {
    pub fn default() -> Self {
        Self {
            fps: 0.0,
            frame_count: 0,
        }
    }
}

pub struct Time {
    pub delta: f32,
    pub elapsed: f32,
}

impl Time {
    pub fn default() -> Self {
        Self {
            delta: 0.0,
            elapsed: 0.0,
        }
    }
}

