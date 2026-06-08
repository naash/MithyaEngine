// Copyright (c) 2025 Nishant Sthalekar
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::any::{Any, TypeId};

use crate::{
    Movement, PlayerControlled,
    core::{EngineActionQueue, EngineEventListener, EngineEventQueue},
    engine::{system::{System, SystemUpdateContext}, world::World},
    input::{InputAction, InputActionEvent},
};

pub struct PlayerInputSystem {
    pending_actions: Vec<InputAction>,
}

impl PlayerInputSystem {
    pub fn new() -> Self {
        Self { pending_actions: Vec::new() }
    }
}

impl System for PlayerInputSystem {
    fn initialize(&mut self, _world: &mut World) {}

    fn update(&mut self, ctx: &mut SystemUpdateContext) {
        let entities = ctx.world.entity_manager.query_component::<PlayerControlled>();
        for entity_id in entities {
            let mut dx = 0.0_f32;
            let mut dy = 0.0_f32;
            for action in &self.pending_actions {
                match action {
                    InputAction::MoveLeft  => dx -= 1.0,
                    InputAction::MoveRight => dx += 1.0,
                    InputAction::MoveUp    => dy += 1.0,
                    InputAction::MoveDown  => dy -= 1.0,
                    _ => {}
                }
            }
            if let Some(movement) = ctx.world.entity_manager.get_component_mut::<Movement>(entity_id) {
                movement.intent = glam::Vec2::new(dx, dy);
            }
        }
        self.pending_actions.clear();
    }

    fn as_event_listener_mut(&mut self) -> Option<&mut dyn EngineEventListener> {
        Some(self)
    }

    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}

impl EngineEventListener for PlayerInputSystem {
    fn interested_events(&self) -> Vec<TypeId> {
        vec![TypeId::of::<InputActionEvent>()]
    }

    fn on_events(&mut self, events: &EngineEventQueue, _actions: &mut EngineActionQueue, _world: &World) {
        for event in events.iter_type::<InputActionEvent>() {
            self.pending_actions.push(event.action.clone());
        }
    }
}
