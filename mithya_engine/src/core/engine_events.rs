// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::any::Any;
use std::fmt::Debug;

/// Trait for all events in the system
/// Game code can implement this for custom events
pub trait EngineEvent : Debug {
    /// Allows downcasting to concrete types
    fn as_any(&self) -> &dyn Any;
}

/// Trait for all actions in the system
/// Game code can implement this for custom actions
pub trait EngineAction : Debug {
    /// Execute the action, mutating the world
    fn execute(&mut self, world: &mut crate::engine::World);
}

/// Trait for systems that want to listen to events
pub trait EngineEventListener {

    /// Return the TypeIds of events this listener cares about
    fn interested_events(&self) -> Vec<std::any::TypeId>;

    /// Process events and optionally queue actions
    fn on_events(
        &mut self,
        events: &EngineEventQueue,
        actions: &mut EngineActionQueue,
    );
}

//Event Queue system
pub struct EngineEventQueue {
    events: Vec<Box<dyn EngineEvent>>,
}

impl EngineEventQueue {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    /// Push any event that implements the Event trait
    pub fn push<E: EngineEvent + 'static>(&mut self, event: E) {
        self.events.push(Box::new(event));
    }

    /// Iterate over all events (type-erased)
    pub fn iter(&self) -> impl Iterator<Item = &Box<dyn EngineEvent>> {
        self.events.iter()
    }

    /// Get events of a specific type
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
        println!("EventQueue contents ({} events):", self.events.len());
        for (i, event) in self.events.iter().enumerate() {
            // Use Debug formatting to print any event
            println!("  [{}] {:?}", i, event);
        }
    }

    /// Broadcast events to all listeners
    pub fn broadcast_to_listeners(
        &mut self,
        listeners: &mut [&mut dyn EngineEventListener],
        actions: &mut EngineActionQueue,
    ) {
        for listener in listeners.iter_mut() {
            // Check if any events match listener's interests
            let interested_types = listener.interested_events();
            
            let has_relevant_events = self.events.iter().any(|event| {
                let event_type = (*event).as_any().type_id();
                interested_types.contains(&event_type)
            });
            
            if has_relevant_events {
                listener.on_events(self, actions);
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

// Action Queue system
pub struct EngineActionQueue {
    actions: Vec<Box<dyn EngineAction>>,
}

impl EngineActionQueue {
    pub fn new() -> Self {
        Self { actions: Vec::new() }
    }

    /// Push any action that implements the Action trait
    pub fn push<A: EngineAction + 'static>(&mut self, action: A) {
        self.actions.push(Box::new(action));
    }

    /// Execute all queued actions
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

//Core Events for input and stuff

#[derive(Debug, Clone, Copy, Default)]
pub struct KeyModifiers {
    pub alt: bool,
    pub ctrl: bool,
    pub shift: bool,
    pub gui: bool, // Windows key, Command key on Mac
}

impl KeyModifiers {
    pub fn from_sdl(keymod: sdl2::keyboard::Mod) -> Self {
        Self {
            alt: keymod.contains(sdl2::keyboard::Mod::LALTMOD) || keymod.contains(sdl2::keyboard::Mod::RALTMOD),
            ctrl: keymod.contains(sdl2::keyboard::Mod::LCTRLMOD) || keymod.contains(sdl2::keyboard::Mod::RCTRLMOD),
            shift: keymod.contains(sdl2::keyboard::Mod::LSHIFTMOD) || keymod.contains(sdl2::keyboard::Mod::RSHIFTMOD),
            gui: keymod.contains(sdl2::keyboard::Mod::LGUIMOD) || keymod.contains(sdl2::keyboard::Mod::RGUIMOD),
        }
    }

    pub fn to_egui(&self) -> egui::Modifiers {
        egui::Modifiers {
            alt: self.alt,
            ctrl: self.ctrl,
            shift: self.shift,
            mac_cmd: false,
            command: self.gui,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GameQuitEvent;

impl EngineEvent for GameQuitEvent {
    fn as_any(&self) -> &dyn Any { self }
}

#[derive(Debug, Clone)]
pub struct KeyPressedEvent {
    pub key: sdl2::keyboard::Keycode, // You'll want to wrap this too eventually
    pub modifiers: KeyModifiers,
}

#[derive(Debug, Clone)]
pub struct KeyReleasedEvent {
    pub key: sdl2::keyboard::Keycode,
    pub modifiers: KeyModifiers,
}

impl EngineEvent for KeyReleasedEvent {
    fn as_any(&self) -> &dyn Any { self }
}

impl EngineEvent for KeyPressedEvent {
    fn as_any(&self) -> &dyn Any { self }
}

#[derive(Debug, Clone)]
pub struct MouseClickEvent {
    pub position: glam::Vec2,
    pub button: sdl2::mouse::MouseButton,
}

impl EngineEvent for MouseClickEvent {
    fn as_any(&self) -> &dyn Any { self }
}

#[derive(Debug, Clone)]
pub struct MouseMoveEvent {
    pub position: glam::Vec2,
}

impl EngineEvent for MouseMoveEvent {
    fn as_any(&self) -> &dyn Any { self }
}

#[derive(Debug, Clone)]
pub struct MouseButtonReleasedEvent {
    pub position: glam::Vec2,
    pub button: sdl2::mouse::MouseButton,
}

impl EngineEvent for MouseButtonReleasedEvent {
    fn as_any(&self) -> &dyn Any { self }
}

#[derive(Debug, Clone)]
pub struct MouseWheelEvent {
    pub delta_x: i32,
    pub delta_y: i32,
}

impl EngineEvent for MouseWheelEvent {
    fn as_any(&self) -> &dyn Any { self }
}

#[derive(Debug, Clone)]
pub struct TextInputEvent {
    pub text: String,
}

impl EngineEvent for TextInputEvent {
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