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

    /// Dispatches queued events to all interested listeners, then clears the queue.
    ///
    /// This is a two-phase design: all listeners receive a read-only `&World` snapshot
    /// during dispatch (read phase), and any world mutations are deferred via `EngineActionQueue`
    /// to be applied after all listeners have run (write phase). This guarantees that every
    /// listener in a broadcast cycle observes the same world state — no listener can poison
    /// another's read by mutating world mid-dispatch.
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

pub trait NamedEngineAction: Debug {
    fn execute(self: Box<Self>, world: &mut World);
}

/// Trait for all actions in the system
pub enum EngineAction {
    /// Named struct action — identifiable in logs and debuggers
    Named(Box<dyn NamedEngineAction>),
    /// Anonymous closure action — for simple one-liners
    Closure(Box<dyn FnOnce(&mut World)>),
}

impl Debug for EngineAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EngineAction::Named(a) => write!(f, "EngineAction::Named({:?})", a),
            EngineAction::Closure(_) => write!(f, "EngineAction::Closure(...)"),
        }
    }
}

pub struct EngineActionQueue {
    actions: Vec<EngineAction>,
}

impl EngineActionQueue {
    pub fn new() -> Self {
        Self { actions: Vec::new() }
    }

    /// Push a named struct action — must implement NamedEngineAction
    pub fn push_named<A: NamedEngineAction + 'static>(&mut self, action: A) {
        self.actions.push(EngineAction::Named(Box::new(action)));
    }

    /// Push an anonymous closure action
    pub fn push_anonymous(&mut self, f: impl FnOnce(&mut World) + 'static) {
        self.actions.push(EngineAction::Closure(Box::new(f)));
    }

    pub fn execute_all(&mut self, world: &mut World) {
        for action in self.actions.drain(..) {
            match action {
                EngineAction::Named(a) => a.execute(world),
                EngineAction::Closure(f) => f(world),
            }
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

// --- Basic Events ---

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