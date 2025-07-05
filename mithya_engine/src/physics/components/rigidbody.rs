// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use glam::Vec2;

#[derive(Debug, Clone)]
pub struct RigidBody {
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub gravity_scale: f32,
    pub drag: f32,
    pub is_kinematic: bool
    //More physical properties can be added here like drag etc
}