// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::collections::HashMap;
use winit::keyboard::KeyCode;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InputAction {
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    Launch,
    Confirm,
}

#[derive(Debug, Clone)]
pub enum InputActionMode {
    Continuous,
    OneShot,
}

#[derive(Debug, Clone)]
pub struct InputBinding {
    pub action: InputAction,
    pub mode: InputActionMode,
}

impl InputBinding {
    pub fn continuous(action: InputAction) -> Self {
        Self { action, mode: InputActionMode::Continuous }
    }

    pub fn one_shot(action: InputAction) -> Self {
        Self { action, mode: InputActionMode::OneShot }
    }
}

pub struct InputMapping {
    pub bindings: HashMap<KeyCode, InputBinding>,
}

impl InputMapping {
    pub fn new() -> Self {
        Self { bindings: HashMap::new() }
    }

    pub fn bind(&mut self, key: KeyCode, binding: InputBinding) -> &mut Self {
        self.bindings.insert(key, binding);
        self
    }
}