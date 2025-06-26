use sdl2::keyboard::Keycode;

use crate::{EntityManager, InputManager};
use crate::core::Transform;

const SPEED: f32 = 0.1;

#[derive(Debug, Clone, Copy)]
pub struct PlayerControlled;

pub fn movement_system(input: &InputManager, entity_manager: &mut EntityManager) {
    let mut dx = 0.0;
    let mut dy = 0.0;

    // Check keyboard input directly
    if input.is_key_pressed(Keycode::A) || input.is_key_pressed(Keycode::Left) {
        dx -= 1.0;
    }
    if input.is_key_pressed(Keycode::D) || input.is_key_pressed(Keycode::Right) {
        dx += 1.0;
    }
    if input.is_key_pressed(Keycode::W) || input.is_key_pressed(Keycode::Up) {
        dy += 1.0;  // Positive Y = up
    }
    if input.is_key_pressed(Keycode::S) || input.is_key_pressed(Keycode::Down) {
        dy -= 1.0;  // Negative Y = down
    }

    if dx != 0.0 || dy != 0.0 {
        let player_ids = entity_manager.query_two_components::<Transform, PlayerControlled>();
        for entity_id in player_ids {
            if let Some(transform) = entity_manager.get_component_mut::<Transform>(entity_id) {
                transform.position[0] += dx * SPEED;
                transform.position[1] += dy * SPEED;
            }
        }
    }
}