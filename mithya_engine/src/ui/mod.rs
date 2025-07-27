// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

mod systems;
pub mod context;
pub mod window;
pub mod element;
pub mod events;
pub mod types;

// Re-export the main types users will interact with
pub use systems::UISystem;
pub use context::UIContext;
pub use window::UIWindow;
pub use element::UIElement;
pub use types::*;