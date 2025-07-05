// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::collections::HashSet;
use sdl2::keyboard::Keycode;

pub struct InputManager {
    pressed_keys: HashSet<Keycode>,
    just_pressed: HashSet<Keycode>,
    just_released: HashSet<Keycode>,
}

impl InputManager {
    pub fn new() -> Self {
        Self {
            pressed_keys: HashSet::new(),
            just_pressed: HashSet::new(),
            just_released: HashSet::new(),
        }
    }

    pub fn handle_key_down(&mut self, keycode: Keycode) {
        if !self.pressed_keys.contains(&keycode) {
            self.just_pressed.insert(keycode);
        }
        self.pressed_keys.insert(keycode);
    }

    pub fn handle_key_up(&mut self, keycode: Keycode) {
        if self.pressed_keys.contains(&keycode) {
            self.just_released.insert(keycode);
        }
        self.pressed_keys.remove(&keycode);
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

    pub fn clear(&mut self) {
        self.just_pressed.clear();
        self.just_released.clear();
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