// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::any::Any;

use glam::Vec3;
use serde::{Serialize, Deserialize};

use crate::Component;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collider {
    pub shape: ColliderShape,
    pub is_trigger: bool,   // If true, detects collisions but doesn't resolve them
    pub offset: Vec3,       // Offset from entity position
    pub is_colliding: bool, // Shows if collider is colliding in a frame. This is a very simplistic version and should work for now. Incase we need more info, we can add collision events on the colliding entity.
}

impl Collider {
    pub fn get_scaled_shape(&self, scale: Vec3) -> ColliderShape {
        match self.shape {
            ColliderShape::Circle {radius } => {
                ColliderShape::Circle{radius: radius * scale.x.max(scale.y)}
            }
            ColliderShape::Box { width, height } => {
                ColliderShape::Box {
                    width: width * scale.x,
                    height: height * scale.y,
                }
            }
        }
    }
}

impl Default for Collider {
    fn default() -> Self {
        Self {
            shape: ColliderShape::Circle { radius: 10.0 },
            is_trigger: false,
            is_colliding: false,
            offset: Vec3::ONE,
        }
    }
}

impl Component for Collider {
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
        let collider: Collider = serde_json::from_value(value)?;
        Ok(Box::new(collider))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ColliderShape {
    Circle { radius: f32 },
    Box { width: f32, height: f32 },
}