// Copyright (c) 2025 Nishant Sthalekar
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use tracing::warn;
use crate::{
    core::engine_events::EngineEventListener,
    debug::{events::DrawDebugEvent, resources::debug_draw_buffer::DebugDrawBuffer},
    engine::{resources::Time, system::{System, SystemUpdateContext}, world::World},
};

pub struct DebugSystem;

impl System for DebugSystem {
    fn initialize(&mut self, world: &mut World) -> Result<(), Box<dyn std::error::Error>> {
        // Register the buffer as a resource if not already present
        if world.resources.get::<DebugDrawBuffer>().is_none() {
            world.resources.insert(DebugDrawBuffer::new());
        }
       
        Ok(())
    }

    fn update(&mut self, context: &mut SystemUpdateContext) {
        let delta_time = context.world.resources.get::<Time>()
        .expect("Time resource must be registered before DebugSystem runs")
        .delta;

        let buffer = match context.world.resources.get_mut::<DebugDrawBuffer>() {
            Some(b) => b,
            None => {
                warn!("DebugDrawBuffer resource not found");
                return;
            }
        };

        // Tick all timed commands down — removes expired ones
        buffer.tick(delta_time);

        // Fire event with current commands for RenderingSystem to consume
        if !buffer.commands().is_empty() {
            context.events.push( DrawDebugEvent {
                commands: buffer.commands().to_vec()
            });
        }
    }
    
    fn cleanup(&mut self, _world: &mut World) {}
    
    fn as_event_listener_mut(&mut self) -> Option<&mut dyn EngineEventListener> {
        None
    }
}