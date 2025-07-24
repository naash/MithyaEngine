// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

//System
pub mod system;
#[path = "components/player.rs"]
pub mod player;
pub use player::Player;
pub use system::PlayerControlSystem;