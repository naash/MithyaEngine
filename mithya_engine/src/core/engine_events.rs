// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::any::Any;
use std::fmt::Debug;

use tracing::debug;

use crate::World;

/// Trait for all events in the system
pub trait EngineEvent : Debug {
    fn as_any(&self) -> &dyn Any;
}

/// Trait for all actions in the system
pub trait EngineAction : Debug {
    fn execute(&mut self, world: &mut crate::engine::World);
}

/// Trait for systems that want to listen to events
pub trait EngineEventListener {
    fn interested_events(&self) -> Vec<std::any::TypeId>;
    fn on_events(
        &mut self,
        events: &EngineEventQueue,
        actions: &mut EngineActionQueue,
        world: &World, //Read only access
    );
}

pub struct EngineEventQueue {
    events: Vec<Box<dyn EngineEvent>>,
}

impl EngineEventQueue {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn push<E: EngineEvent + 'static>(&mut self, event: E) {
        self.events.push(Box::new(event));
    }

    pub fn iter(&self) -> impl Iterator<Item = &Box<dyn EngineEvent>> {
        self.events.iter()
    }

    pub fn iter_type<E: EngineEvent + 'static>(&self) -> impl Iterator<Item = &E> {
        self.events.iter().filter_map(|event| {
            event.as_any().downcast_ref::<E>()
        })
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    pub fn debug_print(&self) {
        debug!("EventQueue contents ({} events):", self.events.len());
        for (i, event) in self.events.iter().enumerate() {
            debug!("  [{}] {:?}", i, event);
        }
    }

    pub fn broadcast_to_listeners(
        &mut self,
        listeners: &mut [&mut dyn EngineEventListener],
        actions: &mut EngineActionQueue,
        world: &World
    ) {
        for listener in listeners.iter_mut() {
            let interested_types = listener.interested_events();
            let has_relevant_events = self.events.iter().any(|event| {
                let event_type = (*event).as_any().type_id();
                interested_types.contains(&event_type)
            });
            if has_relevant_events {
                listener.on_events(self, actions, world);
            }
        }
        self.events.clear();
    }
}

impl Default for EngineEventQueue {
    fn default() -> Self {
        Self::new()
    }
}

pub struct EngineActionQueue {
    actions: Vec<Box<dyn EngineAction>>,
}

impl EngineActionQueue {
    pub fn new() -> Self {
        Self { actions: Vec::new() }
    }

    pub fn push<A: EngineAction + 'static>(&mut self, action: A) {
        self.actions.push(Box::new(action));
    }

    pub fn execute_all(&mut self, world: &mut crate::engine::World) {
        for mut action in self.actions.drain(..) {
            action.execute(world);
        }
    }

    pub fn clear(&mut self) {
        self.actions.clear();
    }

    pub fn len(&self) -> usize {
        self.actions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.actions.is_empty()
    }
}

impl Default for EngineActionQueue {
    fn default() -> Self {
        Self::new()
    }
}

// --- Events ---

#[derive(Debug, Clone)]
pub struct GameQuitEvent;

impl EngineEvent for GameQuitEvent {
    fn as_any(&self) -> &dyn Any { self }
}

#[derive(Debug, Clone)]
pub struct WindowResizedEvent {
    pub width: u32,
    pub height: u32,
}

impl EngineEvent for WindowResizedEvent {
    fn as_any(&self) -> &dyn Any { self }
}