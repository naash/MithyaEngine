// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

#[derive(Default)]
pub struct EngineStats {
    pub fps: f32,
    pub frame_count: u64,
}

#[derive(Default)]
pub struct Time {
    pub delta: f32,
    pub elapsed: f32,
}

