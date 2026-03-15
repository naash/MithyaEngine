// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::any::TypeId;

use crate::{
    Controller, Movement, core::EngineEventListener, engine::{
        system::{System, SystemRenderContext, SystemUpdateContext}, world::World
    }, input::{InputAction, InputActionEvent}
};

// Reads input state, finds possessed entity, writes intent on its Movement component
pub struct ControllerSystem
{
    pub pending_dx: f32,
    pub pending_dy: f32
}

impl ControllerSystem {
    pub fn new() -> Self {
        Self {
        pending_dx : 0.0,
        pending_dy : 0.0
        }
    }
}

impl System for ControllerSystem {  
    fn initialize(&mut self, _world: &mut World) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn update(&mut self, update_context: &mut SystemUpdateContext) {
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
                movement.intent.x = self.pending_dx;
                movement.intent.y = self.pending_dy;
            }
        }

        //clear inputs
        self.pending_dx = 0.0;
        self.pending_dy = 0.0;
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
    ) {
        // collect intent from input action events
        // store temporarily, applied in update()
        for event in events.iter_type::<InputActionEvent>() {
            match event.action {
                InputAction::MoveLeft  => self.pending_dx -= 1.0,
                InputAction::MoveRight => self.pending_dx += 1.0,
                InputAction::MoveUp    => self.pending_dy += 1.0,
                InputAction::MoveDown  => self.pending_dy -= 1.0,
                _ => {}
            }
        }
    }
}