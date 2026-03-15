// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::{
    engine::system::{System, SystemRenderContext, SystemUpdateContext},
    World,
};

// UI system is stubbed pending migration from egui_sdl2_gl to egui-wgpu
pub struct UISystem;

impl UISystem {
    pub fn new() -> Self {
        Self
    }

    pub fn wants_keyboard_input(&self) -> bool {
        false
    }

    pub fn wants_pointer_input(&self) -> bool {
        false
    }
}

impl System for UISystem {
    fn initialize(&mut self, _world: &mut World) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn update(&mut self, _update_context: &mut SystemUpdateContext) {}

    fn cleanup(&mut self, _world: &mut World) {}
    
    fn as_event_listener_mut(&mut self) -> Option<&mut dyn crate::core::EngineEventListener> {
        None
    }
}