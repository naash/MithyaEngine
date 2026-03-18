// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use glam::Vec3;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum ColliderShape {
    Circle { radius: f32 },
    Box { width: f32, height: f32 },
}