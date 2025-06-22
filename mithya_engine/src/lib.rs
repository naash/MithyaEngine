pub mod core;
pub mod rendering;
pub mod components;
pub mod window;

// Re-export commonly used types for easier access
pub use core::{EntityManager, EntityId, Transform};
pub use components::Component;
pub use rendering::{RenderingSystem, Mesh, Render};
pub use window::display_window;

// Prelude module - common imports users will want
pub mod prelude {
    pub use crate::{
        EntityManager, EntityId, Transform,
        Component, RenderingSystem, Mesh, Render,
        display_window
    };
}