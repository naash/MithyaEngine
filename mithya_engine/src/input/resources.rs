// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::collections::HashSet;
use winit::keyboard::KeyCode;

#[derive(Default)]
pub struct InputState {
    pub movement: (f32, f32),
    pub keys_pressed: HashSet<KeyCode>,
    pub mouse_position: (i32, i32),
    pub current_modifiers: KeyModifiers,
}

// --- Key modifiers ---

#[derive(Debug, Clone, Copy, Default)]
pub struct KeyModifiers {
    pub alt: bool,
    pub ctrl: bool,
    pub shift: bool,
    pub gui: bool,
}

impl KeyModifiers {
    pub fn from_winit(mods: &winit::event::Modifiers) -> Self {
        let state = mods.state();
        Self {
            alt: state.alt_key(),
            ctrl: state.control_key(),
            shift: state.shift_key(),
            gui: state.super_key(),
        }
    }
}