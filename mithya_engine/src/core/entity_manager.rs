// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::collections::HashMap;
use std::any::{Any, TypeId};
use super::Transform;

use crate::components::Component;
use crate::rendering::Render;

// Simple entity ID system
pub type EntityId = u32;

// Entity manager - stores components for entities
pub struct EntityManager {
    next_entity_id: EntityId,
    entity_components: HashMap<TypeId, HashMap<EntityId, Box<dyn Any + Send + Sync>>>,
    archetypes: HashMap<TypeId, Vec<EntityId>>,
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            next_entity_id: 0,
            entity_components: HashMap::new(),
            archetypes: HashMap::new(),
        }
    }

    pub fn create_entity(&mut self) -> EntityId {
        let id = self.next_entity_id;
        self.next_entity_id += 1;
        id
    }

    //Add component of any type
    pub fn add_component<T: Component>(&mut self, entity_id: EntityId, component: T) {
        let type_id = TypeId::of::<T>();
        
        //Store the component
        self.entity_components
            .entry(type_id)
            .or_insert_with(HashMap::new)
            .insert(entity_id, Box::new(component));
        
        //Add entity to this component's archetype
        // This creates the archetype if it doesn't exist
        self.archetypes
            .entry(type_id)
            .or_insert_with(Vec::new)
            .push(entity_id);
    }

    pub fn get_component<T: Component>(&self, entity_id: EntityId) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        self.entity_components
            .get(&type_id)?
            .get(&entity_id)?
            .downcast_ref::<T>()
    }
    
    pub fn get_component_mut<T: Component>(&mut self, entity_id: EntityId) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();
        self.entity_components
            .get_mut(&type_id)?
            .get_mut(&entity_id)?
            .downcast_mut::<T>()
    }

    pub fn remove_component<T: Component>(&mut self, entity_id: EntityId) -> Option<T> {
       let type_id = TypeId::of::<T>();
  
        // Remove from component storage
        let component = self.entity_components
            .get_mut(&type_id)?
            .remove(&entity_id)?
            .downcast::<T>()
            .ok()?;
        
        // Remove from archetype
        if let Some(entities) = self.archetypes.get_mut(&type_id) {
            entities.retain(|&id| id != entity_id);
            // Clean up empty archetypes
            if entities.is_empty() {
                self.archetypes.remove(&type_id);
            }
        }
        
        Some(*component)
    }
    
    pub fn has_component<T: Component>(&self, entity_id: EntityId) -> bool {
        self.get_component::<T>(entity_id).is_some()
    }
    
    pub fn destroy_entity(&mut self, entity_id: EntityId) {
        
        //Remove from all archetypes
        for (_, entities) in self.archetypes.iter_mut() {
            entities.retain(|&id| id != entity_id);
        }

        //Clean up empty archetypes (optional optimization)
        self.archetypes.retain(|_, entities| !entities.is_empty());
        
        //Remove from all component storages
        for (_, components) in self.entity_components.iter_mut() {
            components.remove(&entity_id);
        }
    }

     // Get entities that have ALL specified component types
    pub fn query_entities(&self, component_types: &[TypeId]) -> Vec<EntityId> {
        if component_types.is_empty() {
            return Vec::new();
        }
        
        // Start with entities from the first component type
        let mut result: Vec<EntityId> = self.archetypes
            .get(&component_types[0])
            .cloned()
            .unwrap_or_default();
        
        // Filter by remaining component types
        for &type_id in &component_types[1..] {
            if let Some(entities) = self.archetypes.get(&type_id) {
                result.retain(|entity_id| entities.contains(entity_id));
            } else {
                // If any component type has no entities, result is empty
                return Vec::new();
            }
        }
        
        result
    }
    
    // Convenience method for two component types
    pub fn query_two_components<T1: Component, T2: Component>(&self) -> Vec<EntityId> {
        let types = vec![TypeId::of::<T1>(), TypeId::of::<T2>()];
        self.query_entities(&types)
    }
    
    // Get renderable entities (Transform + Renderable)
    pub fn get_renderable_entities(&self) -> Vec<EntityId> {
        self.query_two_components::<Transform, Render>()
    }
}