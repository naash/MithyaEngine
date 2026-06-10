// Copyright (c) 2025 Nishant Sthalekar
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::any::{Any, TypeId};

use glam::Vec2;
use winit::event::MouseButton;

use mithya_engine::{
    core::{EngineActionQueue, EngineEventListener, EngineEventQueue},
    engine::{
        system::{System, SystemUpdateContext},
        World,
    },
    input::events::MouseClickEvent,
    rendering::Viewport,
    EntityId, MoveToEvent, NavGrid,
};

// Listens for right-clicks, converts screen → world → grid cell, dispatches MoveToEvent.
pub struct ClickToMoveSystem {
    pawn_id: EntityId,
    pending_clicks: Vec<Vec2>,
}

impl ClickToMoveSystem {
    pub fn new(pawn_id: EntityId) -> Self {
        Self { pawn_id, pending_clicks: Vec::new() }
    }
}

impl System for ClickToMoveSystem {
    fn initialize(&mut self, _world: &mut World) {}

    fn update(&mut self, ctx: &mut SystemUpdateContext) {
        if self.pending_clicks.is_empty() {
            return;
        }
        let (nav_grid, viewport) = match (
            ctx.world.resources.get::<NavGrid>(),
            ctx.world.resources.get::<Viewport>(),
        ) {
            (Some(grid), Some(viewport)) => (grid, viewport),
            _ => { self.pending_clicks.clear(); return; }
        };
        let clicks = std::mem::take(&mut self.pending_clicks);
        for screen_pos in clicks {
            let world_pos = viewport.screen_to_world(screen_pos);
            let target_cell = nav_grid.world_to_cell(world_pos);
            ctx.events.push(MoveToEvent { entity_id: self.pawn_id, target: target_cell });
        }
    }

    fn as_event_listener_mut(&mut self) -> Option<&mut dyn EngineEventListener> {
        Some(self)
    }

    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}

impl EngineEventListener for ClickToMoveSystem {
    fn interested_events(&self) -> Vec<TypeId> {
        vec![TypeId::of::<MouseClickEvent>()]
    }

    fn on_events(&mut self, events: &EngineEventQueue, _: &mut EngineActionQueue, _: &World) {
        for e in events.iter_type::<MouseClickEvent>() {
            if e.button == MouseButton::Right {
                self.pending_clicks.push(e.position);
            }
        }
    }
}
