// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::any::Any;

use glam::{Vec3, Quat};
use serde::{Deserialize, Serialize};

use crate::Component;

// Transform component for position, rotation, scale
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }
}

impl Component for Transform {
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
        let transform: Transform = serde_json::from_value(value)?;
        Ok(Box::new(transform))
    }
}