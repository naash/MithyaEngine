// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use glam::Vec2;

// Configuration constants
pub const SEPARATION_BUFFER: f32 = 0.001;
pub const MIN_SEPARATION: f32 = 0.001;
pub const VELOCITY_DAMPING: f32 = 0.95;
pub const MIN_VELOCITY: f32 = 0.01;
pub const DAMPING_THRESHOLD: f32 = 0.5;

pub struct PhysicsConfig {
    pub gravity: Vec2,
}

impl Default for PhysicsConfig {
    fn default() -> Self {
        Self {
            gravity: Vec2::new(0.0, -9.81), // Standard gravity
        }
    }
}