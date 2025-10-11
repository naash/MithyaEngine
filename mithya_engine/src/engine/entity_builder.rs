// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::{Component, EntityManager};

// Helper function for creating common entities
pub struct EntityBuilder<'a> {
    entity_manager: &'a mut EntityManager,
    entity_id: u32,
}

impl<'a> EntityBuilder<'a> {
    pub fn new(entity_manager: &'a mut EntityManager) -> Self {
        let entity_id = entity_manager.create_entity();
        Self { entity_manager, entity_id }
    }

    // Generic method to add any component
    pub fn with<T: Component>(self, component: T) -> Self {
        self.entity_manager.add_component(self.entity_id, component);
        self
    }

    pub fn build(self) -> u32 {
        self.entity_id
    }
}