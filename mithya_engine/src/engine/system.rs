
use crate::{
    core::EntityManager,
    input::managers::InputManager,
};

pub trait System {
    fn update(&mut self, input: &InputManager, entity_manager: &mut EntityManager);
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

    pub fn update_all(&mut self, input: &InputManager, entity_manager: &mut EntityManager) {
        for system in &mut self.systems {
            system.update(input, entity_manager);
        }
    }
}

pub struct MovementSystem;

impl System for MovementSystem {
    fn update(&mut self, input: &InputManager, entity_manager: &mut EntityManager) {
        crate::input::systems::movement_system(input, entity_manager);
    }
}