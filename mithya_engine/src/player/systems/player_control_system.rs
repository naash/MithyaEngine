// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::{
    engine::{
        world::World, 
        system::{
            System, 
            SystemRenderContext, 
            SystemUpdateContext
        }
    }, 
    Player, 
    Transform
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

    fn update(&mut self, update_context: &mut SystemUpdateContext) {
        let (dx, dy) = update_context.world.input_state.movement;

        if dx != 0.0 || dy != 0.0 {
            let player_ids = update_context.world.entity_manager
                .query_two_components::<Transform, Player>();
            
            for entity_id in player_ids {
                if let Some(transform) = update_context.world.entity_manager.get_component_mut::<Transform>(entity_id) {
                    transform.position[0] += dx * self.movement_speed * update_context.delta_time;
                    transform.position[1] += dy * self.movement_speed * update_context.delta_time;
                }
            }
        }
    }

    fn render(&mut self, _render_context: &mut SystemRenderContext) {
        // Player control system doesn't render anything
    }
}