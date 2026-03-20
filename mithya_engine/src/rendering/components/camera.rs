// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

#[derive(Debug, Clone)]
pub struct Camera {
    pub size: f32,        // half height in world units
    pub near: f32,
    pub far: f32,
    pub is_active: bool,
}

impl Camera {
    pub fn new(size: f32) -> Self {
        Self {
            size,
            near: -1.0,
            far: 1.0,
            is_active: true,
        }
    }
}