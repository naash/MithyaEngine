// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::{
    Movement, 
    Transform, 
    engine::{
        system::{System, SystemUpdateContext}, world::World
    }, 
    physics::RigidBody
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
            let (impulse, intent) = match update_context.world.entity_manager
                .get_component::<Movement>(entity_id)
            {
                Some(m) => (m.impulse, m.intent),
                None => continue,
            };

            //Update acceleration first if controlled entity has rigicbody 'it should'
            if let Some(rb) = update_context.world.entity_manager
                        .get_component_mut::<RigidBody>(entity_id)
            {
                rb.acceleration.x = intent.x * impulse;
                rb.acceleration.y = intent.y * impulse;
            } else {
                // Fallback for entities without RigidBody — move directly
                if let Some(transform) = update_context.world.entity_manager
                    .get_component_mut::<Transform>(entity_id)
                {
                    transform.position.x += intent.x * impulse * update_context.delta_time;
                    transform.position.y += intent.y * impulse * update_context.delta_time;
                }
            }

            if let Some(movement) = update_context.world.entity_manager
                .get_component_mut::<Movement>(entity_id)
            {
                movement.intent = glam::Vec2::ZERO;
            }
        }
    }
    
    fn as_event_listener_mut(&mut self) -> Option<&mut dyn crate::core::EngineEventListener> {
        None
    }
}