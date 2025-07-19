// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT


#[derive(Debug, Clone, Copy)]
pub struct PlayerControlled;


use std::collections::HashSet;
use sdl2::keyboard::Keycode;

use crate::{engine::system::System, World};

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

    fn handle_event(&mut self, event: &sdl2::event::Event, _world: &mut World) -> bool {
        match event {
            sdl2::event::Event::KeyDown { keycode: Some(keycode), repeat: false, .. } => {
                if !self.pressed_keys.contains(keycode) {
                    self.just_pressed.insert(*keycode);
                }
                self.pressed_keys.insert(*keycode);
                true // Input system always consumes key events
            }
            sdl2::event::Event::KeyUp { keycode: Some(keycode), .. } => {
                if self.pressed_keys.contains(keycode) {
                    self.just_released.insert(*keycode);
                }
                self.pressed_keys.remove(keycode);
                true // Input system always consumes key events
            }
            _ => false
        }
    }

    fn update(&mut self, _world: &mut World, _delta_time: f32) {

        _world.input_state.movement = self.get_movement_input();
        // Clear the "just pressed" and "just released" states at the end of each frame
        self.just_pressed.clear();
        self.just_released.clear();
    }

    fn render(&mut self, _world: &mut World) {
        // Input system doesn't render
    }
}