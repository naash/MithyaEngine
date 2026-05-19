// Copyright (c) 2025 Nishant Sthalekar
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::any::{Any, TypeId};

use glam::{Vec2, Vec3};
use winit::event::MouseButton;

use mithya_engine::{
    core::{
        EngineActionQueue, EngineEventListener, EngineEventQueue,
        engine_events::WindowResizedEvent,
    },
    engine::{
        system::{System, SystemUpdateContext},
        World,
    },
    input::events::MouseClickEvent,
    EntityId, MoveToEvent, NavGrid,
};

// Listens for right-clicks, converts screen → world → grid cell, dispatches MoveToEvent.
pub struct ClickToMoveSystem {
    pawn_id: EntityId,
    viewport: Vec2,
    camera_half_height: f32,
    pending_clicks: Vec<Vec2>,
}

impl ClickToMoveSystem {
    pub fn new(pawn_id: EntityId, viewport: Vec2, camera_half_height: f32) -> Self {
        Self { pawn_id, viewport, camera_half_height, pending_clicks: Vec::new() }
    }

    fn screen_to_world(&self, screen: Vec2) -> Vec3 {
        let aspect = self.viewport.x / self.viewport.y;
        let ndc_x = (screen.x / self.viewport.x) * 2.0 - 1.0;
        let ndc_y = 1.0 - (screen.y / self.viewport.y) * 2.0;
        Vec3::new(
            ndc_x * self.camera_half_height * aspect,
            ndc_y * self.camera_half_height,
            0.0,
        )
    }
}

impl System for ClickToMoveSystem {
    fn initialize(&mut self, _world: &mut World) {}

    fn update(&mut self, ctx: &mut SystemUpdateContext) {
        if self.pending_clicks.is_empty() {
            return;
        }
        let nav_grid = match ctx.world.resources.get::<NavGrid>() {
            Some(g) => g,
            None => { self.pending_clicks.clear(); return; }
        };
        let clicks = std::mem::take(&mut self.pending_clicks);
        for screen_pos in clicks {
            let world_pos = self.screen_to_world(screen_pos);
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
        vec![TypeId::of::<MouseClickEvent>(), TypeId::of::<WindowResizedEvent>()]
    }

    fn on_events(&mut self, events: &EngineEventQueue, _: &mut EngineActionQueue, _: &World) {
        for e in events.iter_type::<MouseClickEvent>() {
            if e.button == MouseButton::Right {
                self.pending_clicks.push(e.position);
            }
        }
        for e in events.iter_type::<WindowResizedEvent>() {
            self.viewport = Vec2::new(e.width as f32, e.height as f32);
        }
    }
}
