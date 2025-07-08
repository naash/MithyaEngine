// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use glam::Vec3;

#[derive(Debug, Clone)]
pub struct RigidBody {
    pub velocity: Vec3,
    pub acceleration: Vec3,
    pub gravity_scale: f32,
    pub drag: f32,
    pub is_kinematic: bool
    //More physical properties can be added here like drag etc
}

impl Default for RigidBody {
    fn default() -> Self {
        Self {
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,
            drag: 0.01,
            gravity_scale: 1.0,
            is_kinematic: false,
        }
    }
}