// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::any::Any;
use glam::Vec2;
use crate::Component;

#[derive(Debug, Clone)]
pub struct Movement {
    pub impulse: f32,
    pub intent: Vec2,
}

impl Movement {
    pub fn new(impulse: f32) -> Self {
        Self {
            impulse,
            intent: Vec2::ZERO,
        }
    }
}