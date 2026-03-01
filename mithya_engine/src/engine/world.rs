// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::collections::HashSet;
use winit::keyboard::KeyCode;

use crate::{
    asset::AssetManager, 
    physics::PhysicsConfig, 
    EntityManager
};

#[derive(Default)]
pub struct InputState {
    pub movement: (f32, f32),
    pub keys_pressed: HashSet<KeyCode>,
    pub mouse_position: (i32, i32),
}

pub struct World {
    pub input_state: InputState,
    pub entity_manager: EntityManager,
    pub physics_config: PhysicsConfig,
    pub asset_manager: AssetManager,
    pub fps: f32
}