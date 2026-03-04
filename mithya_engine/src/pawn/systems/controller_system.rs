// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::{
    Movement, 
    Controller, 
    engine::{
        system::{System, SystemRenderContext, SystemUpdateContext}, world::World
    }
};

// Reads input state, finds possessed entity, writes intent on its Movement component
pub struct ControllerSystem;

impl System for ControllerSystem {
    fn initialize(&mut self, _world: &mut World) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn update(&mut self, update_context: &mut SystemUpdateContext) {
        let (dx, dy) = update_context.world.input_state.movement;

        let controllers = update_context.world.entity_manager
            .query_component::<Controller>();

        for controller_id in controllers {
            let possessed_id = match update_context.world.entity_manager
                .get_component::<Controller>(controller_id)
            {
                Some(c) => c.possessed_entity_id,
                None => continue,
            };

            if let Some(movement) = update_context.world.entity_manager
                .get_component_mut::<Movement>(possessed_id)
            {
                movement.intent.x = dx;
                movement.intent.y = dy;
            }
        }
    }

    fn render(&mut self, _render_context: &mut SystemRenderContext) {}
}