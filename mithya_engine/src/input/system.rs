// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::{collections::HashSet};
use sdl2::keyboard::Keycode;

use crate::{
    core::{
        engine_events::{
        KeyPressedEvent, 
        KeyReleasedEvent
        }, 
        EngineEventListener
        }, 
        engine::system::{
        System, SystemRenderContext, SystemUpdateContext
        },
    World
};

pub struct InputSystem {
    pressed_keys: HashSet<Keycode>,
    just_pressed: HashSet<Keycode>,
    just_released: HashSet<Keycode>,
}

impl InputSystem {
    pub fn new() -> Self {
        Self {
            pressed_keys: HashSet::new(),
            just_pressed: HashSet::new(),
            just_released: HashSet::new(),
        }
    }

    pub fn is_key_pressed(&self, keycode: Keycode) -> bool {
        self.pressed_keys.contains(&keycode)
    }

    pub fn is_key_just_pressed(&self, keycode: Keycode) -> bool {
        self.just_pressed.contains(&keycode)
    }

    pub fn is_key_just_released(&self, keycode: Keycode) -> bool {
        self.just_released.contains(&keycode)
    }

    pub fn get_movement_input(&self) -> (f32, f32) {
        let mut dx = 0.0;
        let mut dy = 0.0;

        if self.is_key_pressed(Keycode::A) || self.is_key_pressed(Keycode::Left) {
            dx -= 1.0;
        }
        if self.is_key_pressed(Keycode::D) || self.is_key_pressed(Keycode::Right) {
            dx += 1.0;
        }
        if self.is_key_pressed(Keycode::W) || self.is_key_pressed(Keycode::Up) {
            dy += 1.0;
        }
        if self.is_key_pressed(Keycode::S) || self.is_key_pressed(Keycode::Down) {
            dy -= 1.0;
        }

        (dx, dy)
    }
}

impl System for InputSystem {
    fn initialize(&mut self, _world: &mut World) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn update(&mut self, update_context: &mut SystemUpdateContext) {

        update_context.world.input_state.movement = self.get_movement_input();
        // Clear the "just pressed" and "just released" states at the end of each frame
        self.just_pressed.clear();
        self.just_released.clear();
    }

    fn render(&mut self, _render_context: &mut SystemRenderContext) {
        // Input system doesn't render
    }
    
    fn cleanup(&mut self, _world: &mut World) {}

    //Has a listener, I don't like this but can't find a better way
    fn as_event_listener_mut(&mut self) -> Option<&mut dyn EngineEventListener> {
        Some(self)
    }
}

impl EngineEventListener for InputSystem {
    fn interested_events(&self) -> Vec<std::any::TypeId> {
        use std::any::TypeId;
        vec![
            TypeId::of::<KeyPressedEvent>(),
            TypeId::of::<KeyReleasedEvent>()
            ]
    }

    fn on_events(
        &mut self,
        events: &crate::core::EngineEventQueue,
        _actions: &mut crate::core::EngineActionQueue,
    ) {
        // Process KeyPressed events
        for event in events.iter_type::<KeyPressedEvent>() {
            if !self.pressed_keys.contains(&event.key) {
                self.just_pressed.insert(event.key);
            }
            self.pressed_keys.insert(event.key);
        }

        // Process KeyReleased events
        for event in events.iter_type::<KeyReleasedEvent>() {
            if self.pressed_keys.contains(&event.key) {
                self.just_released.insert(event.key);
            }
            self.pressed_keys.remove(&event.key);
        }
    }
}