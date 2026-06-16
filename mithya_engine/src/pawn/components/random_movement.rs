// Copyright (c) 2025 Nishant Sthalekar
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use glam::Vec2;

#[derive(Clone, Debug)]
pub struct RandomMovement {
    pub current_direction: Vec2,
}

impl RandomMovement {
    pub fn new() -> Self {
        Self { current_direction: Vec2::ZERO }
    }
}
