// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use glam::Vec2;

pub struct PhysicsConfig {
    pub gravity: Vec2,
    pub time_step: f32,
}

impl Default for PhysicsConfig {
    fn default() -> Self {
        Self {
            gravity: Vec2::new(0.0, -9.81), // Standard gravity
            time_step: 1.0 / 60.0,          // 60 FPS
        }
    }
}