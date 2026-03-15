// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::any::Any;

use glam::Vec3;
use serde::{Serialize, Deserialize};

use crate::Component;

#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl Component for RigidBody {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    
    fn serialize_to_json(&self) -> Result<serde_json::Value, serde_json::Error> {
        serde_json::to_value(self)
    }
    
    fn deserialize_from_json(value: serde_json::Value) -> Result<Box<dyn Component>, serde_json::Error> {
        let rigidbody: RigidBody = serde_json::from_value(value)?;
        Ok(Box::new(rigidbody))
    }
}