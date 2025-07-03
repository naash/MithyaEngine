
use crate::{
    engine::core::World, input::PlayerControlled, Transform
};

pub trait System {
    fn update(&mut self, world: &mut World);
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

    pub fn update_all(&mut self, world: &mut World) {
        for system in &mut self.systems {
            system.update(world);
        }
    }
}

pub struct MovementSystem;

const SPEED: f32 = 0.1;

impl System for MovementSystem {
    fn update(&mut self, world: &mut World) {
    
        let (dx, dy) = world.input_manager.get_movement_input();

        if dx != 0.0 || dy != 0.0 {
            //Get the IDs first with a separate borrow
            let player_ids = {
                let entity_manager = &world.entity_manager;
                entity_manager.query_two_components::<Transform, PlayerControlled>()
            }; // This borrow ends here
            
            // Now get mutable access
            for entity_id in player_ids {
                if let Some(transform) = world.entity_manager.get_component_mut::<Transform>(entity_id) {
                    transform.position[0] += dx * SPEED;
                    transform.position[1] += dy * SPEED;
                }
            }
        }
    }
}