// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::{
    engine::core::World, input::PlayerControlled, Transform
};

pub trait System {
    fn initialize(&mut self, world: &mut World) -> Result<(), Box<dyn std::error::Error>>;
    fn handle_event(&mut self, event: &sdl2::event::Event, world: &mut World) -> bool; // Returns true if consumed
    fn update(&mut self, world: &mut World, delta_time: f32);
    fn render(&mut self, world: &mut World);
    fn cleanup(&mut self, world: &mut World) {}
}

//Todo make use of SystemManager
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

// MovementSystem - add some basic configuration
pub struct MovementSystem {
    pub speed: f32,
}

impl Default for MovementSystem {
    fn default() -> Self {
        Self { speed: 20.0 }
    }
}

impl System for MovementSystem {
    fn initialize(&mut self, _world: &mut World) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn handle_event(&mut self, _event: &sdl2::event::Event, _world: &mut World) -> bool {
        false // Movement system doesn't handle events directly
    }

    fn update(&mut self, world: &mut World, delta_time: f32) {
        let (dx, dy) = world.input_state.movement;

        if dx != 0.0 || dy != 0.0 {
            // Get the IDs first with a separate borrow
            let player_ids = {
                let entity_manager = &world.entity_manager;
                entity_manager.query_two_components::<Transform, PlayerControlled>()
            }; // This borrow ends here
            
            // Now get mutable access
            for entity_id in player_ids {
                if let Some(transform) = world.entity_manager.get_component_mut::<Transform>(entity_id) {
                    transform.position[0] += dx * self.speed * delta_time;
                    transform.position[1] += dy * self.speed * delta_time;
                }
            }
        }
    }

    fn render(&mut self, _world: &mut World) {
        // Movement system doesn't render anything
    }
}