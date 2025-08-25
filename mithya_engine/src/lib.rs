// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

pub mod asset;
pub mod core;
pub mod rendering;
pub mod engine;
pub mod input;
pub mod player;
pub mod physics;
pub mod ui;

// Re-export commonly used types for easier access
pub use core::{EntityManager, EntityId, Component, Transform};
pub use rendering::{RenderingSystem, Mesh, Render};
pub use input::InputSystem;
pub use player::{Player, PlayerControlSystem};
pub use engine::{Engine, World};
pub use physics::PhysicsSystem;

// Prelude module - common imports users will want
pub mod prelude {
    pub use crate::{
        EntityManager, EntityId, Transform,
        Component, RenderingSystem, Mesh, Render,
        InputSystem,
        PhysicsSystem
    };
}