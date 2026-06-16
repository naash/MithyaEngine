// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::collections::{HashMap, HashSet};
use std::any::TypeId;
use super::Transform;
use super::Component;

use crate::core::NamedEngineAction;
use crate::rendering::Render;
use crate::World;

// Simple entity ID system
pub type EntityId = u32;

#[derive(Debug)]
pub struct DestroyEntityAction {
    pub entity_id: u32,
}

impl NamedEngineAction for DestroyEntityAction {
    fn execute(self: Box<Self>, world: &mut World) {
        world.entity_manager.destroy_entity(self.entity_id);
    }
}


//Archetype is a collection of entities that share the same component types
#[derive(Debug)]
pub struct Archetype {
    pub signature: HashSet<TypeId>,
    pub entities: Vec<EntityId>,
}

impl Archetype {
    pub fn new(signature: HashSet<TypeId>) -> Self {
        Self {
            signature,
            entities: Vec::new(),
        }
    }

    pub fn matches(&self, required_components: &[TypeId]) -> bool {
        required_components.iter().all(|type_id| self.signature.contains(type_id))
    }
}

// Entity manager - stores components for entities
pub struct EntityManager {
    next_entity_id: EntityId,
    entity_components: HashMap<TypeId, HashMap<EntityId, Box<dyn Component>>>,
    archetypes: Vec<Archetype>,
    entity_to_archetype: HashMap<EntityId, usize>,
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            next_entity_id: 0,
            entity_components: HashMap::new(),
            archetypes: Vec::new(),
            entity_to_archetype: HashMap::new(),
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
        
        let mut new_signature = match self.entity_to_archetype.get(&entity_id) {
            Some(&arch_idx) => self.archetypes[arch_idx].signature.clone(),
            None => HashSet::new(),
        };

        // Add new component type to signature
        new_signature.insert(type_id);

        // Find or create matching archetype
        let archetype_idx = self.find_or_create_archetype(new_signature);

        // Move entity to new archetype
        if let Some(old_arch_idx) = self.entity_to_archetype.get(&entity_id) {
            self.archetypes[*old_arch_idx].entities.retain(|&e| e != entity_id);
        }
        
        self.archetypes[archetype_idx].entities.push(entity_id);
        self.entity_to_archetype.insert(entity_id, archetype_idx);
        
        // Store component
        self.entity_components
            .entry(type_id)
            .or_insert_with(HashMap::new)
            .insert(entity_id, Box::new(component));
    }

    fn find_or_create_archetype(&mut self, signature: HashSet<TypeId>) -> usize {
        // Try to find existing archetype
        if let Some(idx) = self.archetypes
            .iter()
            .position(|arch| arch.signature == signature) 
        {
            return idx;
        }
        
        // Create new archetype
        let idx = self.archetypes.len();
        self.archetypes.push(Archetype::new(signature));
        idx
    }

    pub fn get_component<T: Component>(&self, entity_id: EntityId) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        self.entity_components
            .get(&type_id)?
            .get(&entity_id)?
            .as_ref()
            .as_any()
            .downcast_ref::<T>()
    }

    pub fn get_two_components<T: Component>(
        & self,
        a: EntityId,
        b: EntityId
    ) -> (Option<&T>, Option<&T>) {
        assert!(a != b);
        (
            self.get_component::<T>(a),
            self.get_component::<T>(b),
        )
    }
    
    pub fn get_component_mut<T: Component>(&mut self, entity_id: EntityId) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();
        self.entity_components
            .get_mut(&type_id)?
            .get_mut(&entity_id)?
            .as_mut()
            .as_any_mut()
            .downcast_mut::<T>()
    }

    pub fn get_two_components_mut<T: Component>(
        &mut self,
        a: EntityId,
        b: EntityId
    ) -> (Option<&mut T>, Option<&mut T>) {
        assert!(a != b);
        let Some(storage) = self.entity_components.get_mut(&TypeId::of::<T>()) else {
            return (None, None);
        };
        let [component_a, component_b] = storage.get_disjoint_mut([&a, &b]);
        (
            component_a.and_then(|c| c.as_mut().as_any_mut().downcast_mut::<T>()),
            component_b.and_then(|c| c.as_mut().as_any_mut().downcast_mut::<T>()),
        )
    }

    pub fn remove_component<T: Component + Clone>(&mut self, entity_id: EntityId) -> Option<T> {
        let type_id = TypeId::of::<T>();

        // Get current archetype
        let current_arch_idx = *self.entity_to_archetype.get(&entity_id)?;
        let current_signature = self.archetypes[current_arch_idx].signature.clone();
        
        // Create new signature without borrowing self
        let mut new_signature = current_signature;
        new_signature.remove(&type_id);
        
        // Store component before archetype modifications
        let component = match self.entity_components.get_mut(&type_id) {
            Some(components) => match components.remove(&entity_id) {
                Some(component) => component,
                None => return None
            },
            None => return None
        };

        // Now handle archetype changes
        let new_arch_idx = self.find_or_create_archetype(new_signature);
        self.archetypes[current_arch_idx].entities.retain(|&e| e != entity_id);
        self.archetypes[new_arch_idx].entities.push(entity_id);
        self.entity_to_archetype.insert(entity_id, new_arch_idx);

        // Downcast and clone the concrete type
        let concrete_component = component
            .as_ref()           // &dyn Component
            .as_any()           // &dyn Any  
            .downcast_ref::<T>()?  // Option<&T>
            .clone();           // T (cloned)

        Some(concrete_component)
    }
    
    pub fn has_component<T: Component>(&self, entity_id: EntityId) -> bool {
        self.get_component::<T>(entity_id).is_some()
    }
    
    pub fn destroy_entity(&mut self, entity_id: EntityId) {
        // Remove from current archetype
        if let Some(&arch_idx) = self.entity_to_archetype.get(&entity_id) {
            self.archetypes[arch_idx].entities.retain(|&e| e != entity_id);
        }

        // Remove archetype mapping
        self.entity_to_archetype.remove(&entity_id);

        // Remove from all component storages
        for (_, components) in self.entity_components.iter_mut() {
            components.remove(&entity_id);
        }

        // Empty archetypes are intentionally kept: entity_to_archetype stores
        // indices into self.archetypes, so removing one would invalidate every
        // mapping that points past it. find_or_create_archetype reuses them.
    }

    pub fn destroy_all_entities(&mut self) {
        // Collect all entity IDs first to avoid borrow issues
        let entity_ids: Vec<EntityId> = self.entity_to_archetype.keys().copied().collect();

        // Destroy each entity
        for entity_id in entity_ids {
            self.destroy_entity(entity_id);
        }
    }

    pub fn destroy_all_entities_except(&mut self, excluded_component_types: &[TypeId]) {
        // Collect entity IDs that DON'T have any of the excluded components
        let entity_ids: Vec<EntityId> = self.entity_to_archetype
            .iter()
            .filter_map(|(&entity_id, &arch_idx)| {
                let signature = &self.archetypes[arch_idx].signature;
                if excluded_component_types.iter().any(|type_id| signature.contains(type_id)) {
                    None
                } else {
                    Some(entity_id)
                }
            })
            .collect();

        // Destroy each entity
        for entity_id in entity_ids {
            self.destroy_entity(entity_id);
        }
    }

     // Get entities that have ALL specified component types
    pub fn query_entities(&self, component_types: &[TypeId]) -> Vec<EntityId> {
        if component_types.is_empty() {
            return Vec::new();
        }
        
        let mut result = Vec::new();
        
        for archetype in &self.archetypes {
            if archetype.matches(component_types) {
                // Verify entities exist in component storages
                let valid_entities: Vec<_> = archetype.entities.iter()
                    .filter(|&&entity_id| {
                        component_types.iter().all(|&type_id| 
                            self.entity_components
                                .get(&type_id)
                                .and_then(|components| components.get(&entity_id))
                                .is_some()
                        )
                    })
                    .copied()
                    .collect();
                
                result.extend(valid_entities);
            }
        }
        
        result
    }
    
    pub fn query_component<T1: Component>(&self) -> Vec<EntityId> {
        let types = vec![TypeId::of::<T1>()];
        self.query_entities(&types)
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

    pub fn get_storage<T: Component>(&self) -> Option<&HashMap<EntityId, Box<dyn Component>>> {
        self.entity_components.get(&TypeId::of::<T>())
    }

    pub fn get_storage_mut<T: Component>(&mut self) -> Option<&mut HashMap<EntityId, Box<dyn Component>>> {
        self.entity_components.get_mut(&TypeId::of::<T>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct CompA(u32);
    struct CompB(u32);
    struct CompC(u32);

    #[test]
    fn destroy_entity_keeps_other_archetype_mappings_valid() {
        let mut em = EntityManager::new();

        let e1 = em.create_entity();
        em.add_component(e1, CompA(1));

        let e2 = em.create_entity();
        em.add_component(e2, CompA(2));
        em.add_component(e2, CompB(2));

        em.destroy_entity(e1);

        em.add_component(e2, CompC(2));

        assert_eq!(em.query_component::<CompA>(), vec![e2]);
        assert_eq!(em.query_two_components::<CompB, CompC>(), vec![e2]);
        assert_eq!(em.get_component::<CompC>(e2).map(|c| c.0), Some(2));
    }

    #[test]
    fn destroyed_entity_disappears_from_queries_and_storage() {
        let mut em = EntityManager::new();

        let e1 = em.create_entity();
        em.add_component(e1, CompA(1));
        em.add_component(e1, CompB(1));

        em.destroy_entity(e1);

        assert!(em.query_component::<CompA>().is_empty());
        assert!(em.get_component::<CompA>(e1).is_none());
        assert!(!em.has_component::<CompB>(e1));
    }

    #[test]
    fn get_two_components_mut_gives_disjoint_borrows() {
        let mut em = EntityManager::new();

        let e1 = em.create_entity();
        em.add_component(e1, CompA(1));
        let e2 = em.create_entity();
        em.add_component(e2, CompA(2));

        let (a, b) = em.get_two_components_mut::<CompA>(e1, e2);
        let (a, b) = (a.unwrap(), b.unwrap());
        a.0 += 10;
        b.0 += 20;

        assert_eq!(em.get_component::<CompA>(e1).map(|c| c.0), Some(11));
        assert_eq!(em.get_component::<CompA>(e2).map(|c| c.0), Some(22));
    }

    #[test]
    fn get_two_components_mut_handles_missing_entries() {
        let mut em = EntityManager::new();

        let e1 = em.create_entity();
        em.add_component(e1, CompA(1));
        let e2 = em.create_entity();

        let (a, b) = em.get_two_components_mut::<CompA>(e1, e2);
        assert!(a.is_some());
        assert!(b.is_none());

        let (a, b) = em.get_two_components_mut::<CompB>(e1, e2);
        assert!(a.is_none());
        assert!(b.is_none());
    }

    #[test]
    fn emptied_archetype_is_reused_for_same_signature() {
        let mut em = EntityManager::new();

        let e1 = em.create_entity();
        em.add_component(e1, CompA(1));
        em.destroy_entity(e1);

        let e2 = em.create_entity();
        em.add_component(e2, CompA(2));

        assert_eq!(em.query_component::<CompA>(), vec![e2]);
        assert_eq!(em.archetypes.len(), 1);
    }
}