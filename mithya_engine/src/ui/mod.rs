// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

pub mod system;
pub mod context;
pub mod window;
pub mod element;
pub mod events;
pub mod types;

// Re-export the main types users will interact with
pub use system::UiSystem;
pub use context::UiContext;
pub use window::UiWindow;
pub use element::UiElement;
pub use types::*;