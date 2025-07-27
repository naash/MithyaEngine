// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::{
    engine::{core::World, system::System}, Player, Transform
};

pub struct PlayerControlSystem {
    pub movement_speed: f32,
}

impl Default for PlayerControlSystem {
    fn default() -> Self {
        Self { movement_speed: 20.0 }
    }
}

impl System for PlayerControlSystem {
    fn initialize(&mut self, _world: &mut World) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn handle_event(&mut self, _event: &sdl2::event::Event, _world: &mut World) -> bool {
        false // Player control system doesn't handle events directly
    }

    fn update(&mut self, world: &mut World, delta_time: f32) {
        let (dx, dy) = world.input_state.movement;

        if dx != 0.0 || dy != 0.0 {
            let player_ids = world.entity_manager
                .query_two_components::<Transform, Player>();
            
            for entity_id in player_ids {
                if let Some(transform) = world.entity_manager.get_component_mut::<Transform>(entity_id) {
                    transform.position[0] += dx * self.movement_speed * delta_time;
                    transform.position[1] += dy * self.movement_speed * delta_time;
                }
            }
        }
    }

    fn render(&mut self, _world: &mut World) {
        // Player control system doesn't render anything
    }
}