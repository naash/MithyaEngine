// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::{
    Movement, 
    Transform, 
    engine::{
        system::{System, SystemRenderContext, SystemUpdateContext}, world::World
    }
};


// Reads intent from Movement component, applies to Transform, clears intent
pub struct MovementSystem;

impl System for MovementSystem {
    fn initialize(&mut self, _world: &mut World) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn update(&mut self, update_context: &mut SystemUpdateContext) {
        let entities = update_context.world.entity_manager
            .query_component::<Movement>();

        for entity_id in entities {
            let (speed, intent) = match update_context.world.entity_manager
                .get_component::<Movement>(entity_id)
            {
                Some(m) => (m.speed, m.intent),
                None => continue,
            };

            if intent == glam::Vec2::ZERO {
                continue;
            }

            if let Some(transform) = update_context.world.entity_manager
                .get_component_mut::<Transform>(entity_id)
            {
                transform.position.x += intent.x * speed * update_context.delta_time;
                transform.position.y += intent.y * speed * update_context.delta_time;
            }

            if let Some(movement) = update_context.world.entity_manager
                .get_component_mut::<Movement>(entity_id)
            {
                movement.intent = glam::Vec2::ZERO;
            }
        }
    }

    fn render(&mut self, _render_context: &mut SystemRenderContext) {}
    
    fn as_event_listener_mut(&mut self) -> Option<&mut dyn crate::core::EngineEventListener> {
        None
    }
}