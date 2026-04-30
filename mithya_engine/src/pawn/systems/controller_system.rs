// Copyright (c) 2025 Nishant Sthalekar
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::any::TypeId;

use glam::Vec2;

use crate::{
    Controller, Movement, NavAgent, core::EngineEventListener, engine::{system::{System, SystemUpdateContext}, world::World}, input::{InputAction, InputActionEvent}
};

pub struct ControllerSystem {
    pending_actions: Vec<InputAction>,
}

impl ControllerSystem {
    pub fn new() -> Self {
        Self { pending_actions: Vec::new() }
    }
}

impl System for ControllerSystem {
    fn initialize(&mut self, _world: &mut World) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn update(&mut self, update_context: &mut SystemUpdateContext) {
        let controller_ids = update_context.world.entity_manager
            .query_component::<Controller>();

        for controller_id in controller_ids {
            let possessed_id = match update_context.world.entity_manager
                .get_component::<Controller>(controller_id)
            {
                Some(c) => c.possessed_entity_id,
                None => continue,
            };

            // Read world-derived intent — written by NavigationSystem each frame.
            let world_intent = update_context.world.entity_manager
                .get_component::<NavAgent>(possessed_id)
                .map(|a| a.move_input)
                .unwrap_or(Vec2::ZERO);

            let intent = match update_context.world.entity_manager
                .get_component_mut::<Controller>(controller_id)
            {
                Some(ctrl) => {
                    ctrl.behavior.on_input_actions(&self.pending_actions);
                    ctrl.behavior.on_world_intent(world_intent);
                    ctrl.behavior.compute_intent()
                }
                None => continue,
            };

            if let Some(movement) = update_context.world.entity_manager
                .get_component_mut::<Movement>(possessed_id)
            {
                movement.intent = intent;
            }
        }

        self.pending_actions.clear();
    }

    fn as_event_listener_mut(&mut self) -> Option<&mut dyn EngineEventListener> {
        Some(self)
    }
}

impl EngineEventListener for ControllerSystem {
    fn interested_events(&self) -> Vec<TypeId> {
        vec![TypeId::of::<InputActionEvent>()]
    }

    fn on_events(
        &mut self,
        events: &crate::core::EngineEventQueue,
        _actions: &mut crate::core::EngineActionQueue,
        _world: &World,
    ) {
        for event in events.iter_type::<InputActionEvent>() {
            self.pending_actions.push(event.action.clone());
        }
    }
}
