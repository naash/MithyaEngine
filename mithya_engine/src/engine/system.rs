// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::{
    engine::core::World
};

pub trait System {
    fn initialize(&mut self, world: &mut World) -> Result<(), Box<dyn std::error::Error>>;
    fn handle_event(&mut self, event: &sdl2::event::Event, world: &mut World) -> bool; // Returns true if consumed
    fn update(&mut self, world: &mut World, delta_time: f32);
    fn render(&mut self, world: &mut World);
    fn cleanup(&mut self, world: &mut World) {}
}

pub struct SystemsManager {
    systems: Vec<Box<dyn System>>,
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

    pub fn handle_event_all(&mut self, event: &sdl2::event::Event, world: &mut World){
        for system in &mut self.systems {
            let _event_consumed = system.handle_event(event, world);

            if _event_consumed {
                break;
            }
        }
    }

    pub fn initialize_all(&mut self, world: &mut World) {
        for system in &mut self.systems {
            system.initialize(world);
        }
    }

    pub fn update_all(&mut self, world: &mut World, delta_time: f32) {
        for system in &mut self.systems {
            system.update(world, delta_time);
        }
    }

    pub fn render_all(&mut self, world: &mut World) {
        for system in self.systems.iter_mut().rev() {
            system.render(world);
        }
    }

}
