// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::any::Any;
use crate::{core::EngineEvent, input::resources::KeyModifiers};
use super::mapping::InputAction;
use winit::{event::MouseButton, keyboard::KeyCode};

#[derive(Debug, Clone)]
pub struct InputActionEvent {
    pub action: InputAction,
}

impl EngineEvent for InputActionEvent {
    fn as_any(&self) -> &dyn Any { self }
}

#[derive(Debug, Clone)]
pub struct KeyPressedEvent {
    pub key: KeyCode,
    pub modifiers: KeyModifiers,
}

impl EngineEvent for KeyPressedEvent {
    fn as_any(&self) -> &dyn Any { self }
}

#[derive(Debug, Clone)]
pub struct KeyReleasedEvent {
    pub key: KeyCode,
    pub modifiers: KeyModifiers,
}

impl EngineEvent for KeyReleasedEvent {
    fn as_any(&self) -> &dyn Any { self }
}

#[derive(Debug, Clone)]
pub struct MouseClickEvent {
    pub position: glam::Vec2,
    pub button: MouseButton,
}

impl EngineEvent for MouseClickEvent {
    fn as_any(&self) -> &dyn Any { self }
}

#[derive(Debug, Clone)]
pub struct MouseButtonReleasedEvent {
    pub position: glam::Vec2,
    pub button: MouseButton,
}

impl EngineEvent for MouseButtonReleasedEvent {
    fn as_any(&self) -> &dyn Any { self }
}

#[derive(Debug, Clone)]
pub struct MouseMoveEvent {
    pub position: glam::Vec2,
}

impl EngineEvent for MouseMoveEvent {
    fn as_any(&self) -> &dyn Any { self }
}

#[derive(Debug, Clone)]
pub struct MouseWheelEvent {
    pub delta_x: i32,
    pub delta_y: i32,
}

impl EngineEvent for MouseWheelEvent {
    fn as_any(&self) -> &dyn Any { self }
}

#[derive(Debug, Clone)]
pub struct TextInputEvent {
    pub text: String,
}

impl EngineEvent for TextInputEvent {
    fn as_any(&self) -> &dyn Any { self }
}