// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

pub mod components;
pub mod systems;
pub mod brick_spawner;
pub mod events;
pub mod actions;

pub use components::{Brick, Ball, Paddle, BrickType};
pub use systems::BrickBreakerSystem;
pub use brick_spawner::*;

