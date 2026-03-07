// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::any::Any;
use crate::core::EngineEvent;
use super::actions::InputAction;

#[derive(Debug, Clone)]
pub struct InputActionEvent {
    pub action: InputAction,
}

impl EngineEvent for InputActionEvent {
    fn as_any(&self) -> &dyn Any { self }
}