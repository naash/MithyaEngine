// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::{any::Any, sync::Arc};

use tracing::info;
use winit::window::Window;

use crate::{
    EntityManager, RenderingSystem, asset::AssetManager, core::{
        EngineActionQueue, 
        EngineEventListener, 
        EngineEventQueue}, engine::World
};

pub struct SystemUpdateContext<'a> {
    pub world: &'a mut World,
    pub events: &'a mut EngineEventQueue,
}

pub struct SystemRenderContext<'a> {
    pub entity_manager: &'a mut EntityManager,
    pub asset_manager: &'a mut AssetManager
}

pub trait System {
    fn initialize(&mut self, world: &mut World) -> Result<(), Box<dyn std::error::Error>>;
    fn update(&mut self, update_context: &mut SystemUpdateContext);
    fn cleanup(&mut self, _world: &mut World) {}
    
    fn as_event_listener_mut(&mut self) -> Option<&mut dyn EngineEventListener>;
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
    rendering_system: Option<RenderingSystem>,
}

impl SystemsManager {
    pub fn new() -> Self {
        Self {
            systems: Vec::new(),
            rendering_system : None,
        }
    }

    pub fn set_rendering_system(&mut self, rendering_system: RenderingSystem) {
        self.rendering_system = Some(rendering_system);
        info!("Rendering System has initialized");
    }

    pub fn add_system<S: System + 'static>(&mut self, system: S, world: &mut World) {
        let mut boxed = Box::new(system);
        let _ = boxed.initialize(world);
        self.systems.push(boxed);
    }

    pub fn handle_event_all( &mut self, event_queue: &mut EngineEventQueue, action_queue: &mut EngineActionQueue, world: &World){

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
        
        event_queue.broadcast_to_listeners(&mut listeners, action_queue, world);
    }

    pub fn update_all(&mut self, update_context: &mut SystemUpdateContext) {
        for system in &mut self.systems {
            system.update(update_context);
        }
    }

    pub fn render(&mut self, window: &Arc<Window>, world: &mut World) {
        if let Some(renderer) = &mut self.rendering_system {
            renderer.render(window, world);
        }
    }

    pub fn get_system_mut<S: System + 'static>(&mut self) -> Option<&mut S> {
        for system in &mut self.systems {
            if let Some(s) = system.as_any_mut().downcast_mut::<S>() {
                return Some(s);
            }
        }
        None
    }

    pub fn get_rendering_system(&mut self) -> Option<&mut RenderingSystem> {
        self.rendering_system.as_mut()
    }

}

impl Default for SystemsManager {
    fn default() -> Self {
        Self::new()
    }
}
