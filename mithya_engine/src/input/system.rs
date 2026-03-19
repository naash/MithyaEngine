// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::collections::HashSet;
use winit::keyboard::KeyCode;

use crate::{
    World, core::EngineEventListener, engine::system::{
        System,
        SystemUpdateContext
    }, input::{InputMapping, events::{KeyPressedEvent, KeyReleasedEvent}}
};

use super::mapping::InputActionMode;
use super::events::InputActionEvent;

pub struct InputSystem {
    pressed_keys: HashSet<KeyCode>,
    just_pressed: HashSet<KeyCode>,
    just_released: HashSet<KeyCode>,
}

impl InputSystem {
    pub fn new() -> Self {
        Self {
            pressed_keys: HashSet::new(),
            just_pressed: HashSet::new(),
            just_released: HashSet::new(),
        }
    }

    pub fn is_key_pressed(&self, keycode: KeyCode) -> bool {
        self.pressed_keys.contains(&keycode)
    }

    pub fn is_key_just_pressed(&self, keycode: KeyCode) -> bool {
        self.just_pressed.contains(&keycode)
    }

    pub fn is_key_just_released(&self, keycode: KeyCode) -> bool {
        self.just_released.contains(&keycode)
    }
}

impl System for InputSystem {
    fn initialize(&mut self, _world: &mut World) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn update(&mut self, update_context: &mut SystemUpdateContext) {
        
        if let Some(mapping) = update_context.world.resources.get::<InputMapping>() {
            for (key, binding) in &mapping.bindings {
                let should_fire = match binding.mode {
                    InputActionMode::Continuous => self.pressed_keys.contains(key),
                    InputActionMode::OneShot => self.just_pressed.contains(key),
                };

                if should_fire {
                    update_context.events.push(InputActionEvent {
                        action: binding.action.clone(),
                    });
                }
            }
        }

        self.just_pressed.clear();
        self.just_released.clear();
    }

    fn cleanup(&mut self, _world: &mut World) {}

    fn as_event_listener_mut(&mut self) -> Option<&mut dyn EngineEventListener> {
        Some(self)
    }
}

impl EngineEventListener for InputSystem {
    fn interested_events(&self) -> Vec<std::any::TypeId> {
        use std::any::TypeId;
        vec![
            TypeId::of::<KeyPressedEvent>(),
            TypeId::of::<KeyReleasedEvent>(),
        ]
    }

    fn on_events(
        &mut self,
        events: &crate::core::EngineEventQueue,
        _actions: &mut crate::core::EngineActionQueue,
        _world: &World
    ) {
        for event in events.iter_type::<KeyPressedEvent>() {
            if !self.pressed_keys.contains(&event.key) {
                self.just_pressed.insert(event.key);
            }
            self.pressed_keys.insert(event.key);
        }

        for event in events.iter_type::<KeyReleasedEvent>() {
            if self.pressed_keys.contains(&event.key) {
                self.just_released.insert(event.key);
            }
            self.pressed_keys.remove(&event.key);
        }
    }
}