// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use glam::Vec2;

#[derive(Debug, Clone)]
pub struct Collider {
    pub shape: ColliderShape,
    pub is_trigger: bool,   // If true, detects collisions but doesn't resolve them
    pub offset: Vec2,       // Offset from entity position
}

#[derive(Debug, Clone)]
pub enum ColliderShape {
    Circle { radius: f32 },
    Box { width: f32, height: f32 },
}