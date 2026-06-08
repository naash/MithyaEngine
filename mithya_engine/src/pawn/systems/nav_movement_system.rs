// Copyright (c) 2025 Nishant Sthalekar
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::any::Any;

use crate::{
    Movement,
    core::EngineEventListener,
    engine::{system::{System, SystemUpdateContext}, world::World},
    navigation::NavAgent,
};

pub struct NavMovementSystem;

impl System for NavMovementSystem {
    fn initialize(&mut self, _world: &mut World) {}

    fn update(&mut self, ctx: &mut SystemUpdateContext) {
        let entities = ctx.world.entity_manager.query_component::<NavAgent>();
        for entity_id in entities {
            let move_input = match ctx.world.entity_manager.get_component::<NavAgent>(entity_id) {
                Some(agent) => agent.move_input,
                None => continue,
            };
            if let Some(movement) = ctx.world.entity_manager.get_component_mut::<Movement>(entity_id) {
                movement.intent = move_input;
            }
        }
    }

    fn as_event_listener_mut(&mut self) -> Option<&mut dyn EngineEventListener> { None }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}
