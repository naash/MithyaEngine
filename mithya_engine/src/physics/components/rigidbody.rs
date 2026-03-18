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
    pub bounce: f32, //0.0 to 1.0 | inverts % of velocity on collision
    pub is_kinematic: bool,//if true, such bodies are not affected by external force
    pub max_speed: f32,        // caps velocity
    pub max_acceleration: f32, // caps acceleration
}

impl RigidBody {
    pub fn set_velocity(&mut self, new_velocity: Vec3) {        
        if self.max_speed > 0.0 && new_velocity.length() > self.max_speed {
            self.velocity = new_velocity.normalize() * self.max_speed;
        } else {
            self.velocity = new_velocity;
        }
    }
}

impl Default for RigidBody {
    fn default() -> Self {
        Self {
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,
            drag: 0.01,
            gravity_scale: 1.0,
            bounce: 0.0,
            is_kinematic: false,
            max_speed: 0.0,
            max_acceleration: 0.0
        }
    }
}