// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

//System
#[path = "managers/input_manager.rs"]
pub mod input_manager;

pub use input_manager::InputManager;
pub mod system;
pub use system::PlayerControlled;