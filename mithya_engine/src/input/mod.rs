//System
#[path = "managers/input_manager.rs"]
pub mod input_manager;

pub use input_manager::InputManager;
pub mod system;
pub use system::PlayerControlled;