// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

mod components;
pub mod systems;
mod physics_config;
mod collision_math;

pub use components::{Collider, ColliderShape, RigidBody};
pub use systems::{PhysicsSystem, CollisionSystem};
pub use physics_config::PhysicsConfig;