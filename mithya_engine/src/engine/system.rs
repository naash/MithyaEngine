// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::any::Any;

use crate::{
    asset::AssetManager,
    core::{
        EngineActionQueue, 
        EngineEventListener, 
        EngineEventQueue}, 
    engine::World, 
    EntityManager
};

pub struct SystemUpdateContext<'a> {
    pub world: &'a mut World,
    pub events: &'a mut EngineEventQueue,
    pub delta_time: f32,
}

pub struct SystemRenderContext<'a> {
    pub entity_manager: &'a mut EntityManager,
    pub asset_manager: &'a mut AssetManager
}

pub trait System {
    fn initialize(&mut self, world: &mut World) -> Result<(), Box<dyn std::error::Error>>;
    fn update(&mut self, update_context: &mut SystemUpdateContext);
    fn render(&mut self, render_context: &mut SystemRenderContext);
    fn cleanup(&mut self, _world: &mut World) {}
    
    fn as_event_listener_mut(&mut self) -> Option<&mut dyn EngineEventListener> {
        None  // Default: not a listener
    }
}

// Separate trait for the Any requirement
pub trait SystemAny: System {
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

// Blanket implementation for all types that implement System
impl<T: System + 'static> SystemAny for T {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub struct SystemsManager {
    systems: Vec<Box<dyn SystemAny>>,
}

impl SystemsManager {
    pub fn new() -> Self {
        Self {
            systems: Vec::new(),
        }
    }

    pub fn add_system<S: System + 'static>(&mut self, system: S) {
        self.systems.push(Box::new(system));
    }

    pub fn handle_event_all( &mut self, event_queue: &mut EngineEventQueue, action_queue: &mut EngineActionQueue){

        //Early return when there are no events to handle
        if event_queue.len() == 0 {
            return;
        }

        // Gather all listeners (systems implementing EventListener)
        let mut listeners: Vec<&mut dyn EngineEventListener> = Vec::new();

        for system in &mut self.systems {
            if let Some(listener) = system.as_event_listener_mut() {
                listeners.push(listener);
            }
        }
        
        event_queue.broadcast_to_listeners(&mut listeners, action_queue);
    }

    pub fn initialize_all(&mut self, world: &mut World) {
        for system in &mut self.systems {
            let _ = system.initialize(world);
        }
    }

    pub fn update_all(&mut self, update_context: &mut SystemUpdateContext) {
        for system in &mut self.systems {
            system.update(update_context);
        }
    }

    pub fn render_all(&mut self, render_context: &mut SystemRenderContext) {
        for system in self.systems.iter_mut().rev() {
            system.render(render_context);
        }
    }

}
