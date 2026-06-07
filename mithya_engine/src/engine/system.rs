// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::{any::Any, collections::HashMap, sync::Arc};

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
    fn initialize(&mut self, world: &mut World);
    fn update(&mut self, update_context: &mut SystemUpdateContext);
    fn cleanup(&mut self, _world: &mut World) {}
    fn as_event_listener_mut(&mut self) -> Option<&mut dyn EngineEventListener>;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SystemPhase {
    Input,
    GameLogic,
    PrePhysics,
    Physics,
    PostPhysics,
    Debug,
}

const PHASE_ORDER: &[SystemPhase] = &[
    SystemPhase::Input,
    SystemPhase::GameLogic,
    SystemPhase::PrePhysics,
    SystemPhase::Physics,
    SystemPhase::PostPhysics,
    SystemPhase::Debug,
];

#[derive(Default)]
pub struct SystemsManager {
    systems: HashMap<SystemPhase, Vec<Box<dyn System>>>,
    rendering_system: Option<RenderingSystem>,
}

impl SystemsManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_rendering_system(&mut self, rendering_system: RenderingSystem) {
        self.rendering_system = Some(rendering_system);
        info!("Rendering System has initialized");
    }

    pub fn add_system<S: System + 'static>(&mut self, system: S, world: &mut World) {
        self.add_system_with_phase(system, world, SystemPhase::GameLogic);
    }

    pub fn add_system_with_phase<S: System + 'static>(&mut self, system: S, world: &mut World, phase: SystemPhase) {
        let mut boxed = Box::new(system);
        boxed.initialize(world);
        self.systems.entry(phase).or_default().push(boxed);
    }

    pub fn handle_event_all(&mut self, event_queue: &mut EngineEventQueue, action_queue: &mut EngineActionQueue, world: &World) {
        if event_queue.len() == 0 {
            return;
        }

        let mut listeners: Vec<&mut dyn EngineEventListener> = Vec::new();

        for systems in self.systems.values_mut() {
            for system in systems {
                if let Some(listener) = system.as_event_listener_mut() {
                    listeners.push(listener);
                }
            }
        }

        if let Some(rendering) = self.rendering_system.as_mut() {
            if let Some(listener) = rendering.as_event_listener_mut() {
                listeners.push(listener);
            }
        }

        event_queue.broadcast_to_listeners(&mut listeners, action_queue, world);
    }

    pub fn update_all(&mut self, update_context: &mut SystemUpdateContext) {
        for &phase in PHASE_ORDER {
            if let Some(systems) = self.systems.get_mut(&phase) {
                for system in systems {
                    system.update(update_context);
                }
            }
        }
    }

    pub fn render(&mut self, window: &Arc<Window>, world: &mut World) {
        if let Some(renderer) = &mut self.rendering_system {
            renderer.render(window, world);
        }
    }

    pub fn get_system_mut<S: System + 'static>(&mut self) -> Option<&mut S> {
        for systems in self.systems.values_mut() {
            for system in systems {
                if let Some(s) = system.as_any_mut().downcast_mut::<S>() {
                    return Some(s);
                }
            }
        }
        None
    }

    pub fn get_rendering_system(&mut self) -> Option<&mut RenderingSystem> {
        self.rendering_system.as_mut()
    }
}

