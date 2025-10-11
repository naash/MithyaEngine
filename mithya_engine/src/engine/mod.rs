// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

pub mod game_loop;
pub mod system;
pub mod entity_builder;
pub mod frame_timer;
pub mod world;

pub use entity_builder::EntityBuilder;
pub use frame_timer::FrameTimer;
pub use world::{World, InputState};
pub use game_loop::{Engine, EngineConfig, GameLogic};