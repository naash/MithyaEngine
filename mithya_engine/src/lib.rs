// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

pub mod asset;
pub mod core;
pub mod rendering;
pub mod engine;
pub mod input;
pub mod pawn;
pub mod physics;
pub mod debug;
pub mod navigation;

// Re-export commonly used types for easier access
pub use core::{EntityManager, EntityId, Component, Transform};
pub use rendering::{RenderingSystem, Mesh, Render};
pub use input::InputSystem;
pub use pawn::{
    PlayerControlled, RandomMovement,
    Movement,
    PlayerInputSystem, NavMovementSystem, RandomMovementSystem, MovementSystem,
};
pub use engine::{Engine, World, WorldConfig};
pub use physics::PhysicsSystem;
pub use debug::resources::debug_draw_buffer::DebugDrawBuffer;
pub use debug::systems::debug_system::DebugSystem;
pub use navigation::{
    GridCell, Direction,
    MoveToEvent,
    NavAgent,
    NavGrid, CellType,
    NavigationSystem,
    NavGridDebugSystem,
};

pub use egui;

// Prelude module - common imports users will want
pub mod prelude {
    pub use crate::{
        EntityManager, EntityId, Transform,
        Component,
        RenderingSystem, Mesh, Render,
        InputSystem,
        PhysicsSystem,
        DebugDrawBuffer,
        DebugSystem,
    };
}