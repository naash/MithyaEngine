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
    pub components: HashMap<TypeId, Vec<Box<dyn Component>>>,
}

impl Archetype {
    pub fn new(signature: HashSet<TypeId>) -> Self {
        Self {
            signature,
            entities: Vec::new(),
            components: HashMap::new()
        }
    }

    pub fn matches(&self, required_components: &[TypeId]) -> bool {
        required_components.iter().all(|type_id| self.signature.contains(type_id))
    }
}

// Entity manager - stores components for entities
pub struct EntityManager {
    next_entity_id: EntityId,
    archetypes: Vec<Archetype>,
    entity_to_archetype: HashMap<EntityId, (usize, usize)>,
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            next_entity_id: 0,
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
            Some(&(arch_idx, _row_idx),) => self.archetypes[arch_idx].signature.clone(),
            None => HashSet::new(),
        };

        // Add new component type to signature
        new_signature.insert(type_id);

        // Find or create matching archetype
        let archetype_idx = self.find_or_create_archetype(new_signature);

        // Move entity to new archetype
        // After finding archetype_idx, before pushing new component
        if let Some((old_arch_idx, old_row_idx)) = self.entity_to_archetype.get(&entity_id).copied() {
            // get the old signature to know which types to migrate
            let old_signature: Vec<TypeId> = self.archetypes[old_arch_idx].signature.iter().copied().collect();
            
            for existing_type_id in old_signature {
                // swap_remove from old archetype's column
                if let Some(column) = self.archetypes[old_arch_idx].components.get_mut(&existing_type_id) {
                    let migrated_component = column.swap_remove(old_row_idx);
                    self.archetypes[archetype_idx].components
                        .entry(existing_type_id)
                        .or_insert_with(Vec::new)
                        .push(migrated_component);
                }
            }
            
            self.archetypes[old_arch_idx].entities.swap_remove(old_row_idx);

            // update displaced entity's row_idx if swap occurred
            if let Some(displaced_entity) = self.archetypes[old_arch_idx].entities.get_mut(old_row_idx) {
                if let Some(entry) = self.entity_to_archetype.get_mut(displaced_entity) {
                    entry.1 = old_row_idx;
                }
            } 
        }
        
        self.archetypes[archetype_idx].entities.push(entity_id);
        let row_idx = self.archetypes[archetype_idx].entities.len() - 1;
        self.entity_to_archetype.insert(entity_id, (archetype_idx, row_idx));
        
        // Store component
        self.archetypes[archetype_idx].components.
        entry(type_id).or_insert_with(Vec::new).push(Box::new(component));
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
        let (arch_idx, row_idx) = *self.entity_to_archetype.get(&entity_id)?;
        self.archetypes[arch_idx].components
            .get(&type_id)?
            .get(row_idx)?
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
        let (arch_idx, row_idx) = *self.entity_to_archetype.get(&entity_id)?;
        self.archetypes[arch_idx].components
            .get_mut(&type_id)?
            .get_mut(row_idx)?
            .as_mut()
            .as_any_mut()
            .downcast_mut::<T>()
    }

    pub fn get_two_components_mut<T: Component>(
        &mut self,
        a: EntityId,
        b: EntityId
    ) -> (Option<&mut T>, Option<&mut T>) {
        let type_id = TypeId::of::<T>();
        assert!(a != b);

        let a_lookup = self.entity_to_archetype.get(&a).copied();
        let b_lookup = self.entity_to_archetype.get(&b).copied();

        match (a_lookup, b_lookup) {
            (None, None) => (None, None),
            (Some((_arch_a, _row_a)), None) => {
                // just look up a
                let ca = self.get_component_mut(a);
                (ca, None)
            },
            (None, Some((_arch_b, _row_b))) => {
                // just look up b
                let cb = self.get_component_mut(b);
                (None, cb)
            },
            (Some((arch_a, row_a)), Some((arch_b, row_b))) => {
                if arch_a == arch_b {
                    // same archetype, same column Vec — use get_disjoint_mut by row
                    let Some(column) = self.archetypes[arch_a].components.get_mut(&type_id) else {
                        return (None, None)
                    };
                    let Ok([ca, cb]) = column.get_disjoint_mut([row_a, row_b]) else {
                        return (None, None);
                    };
                    (
                        ca.as_mut().as_any_mut().downcast_mut::<T>(),
                        cb.as_mut().as_any_mut().downcast_mut::<T>(),
                    )
                }  else {
                    // different archetypes — split_at_mut
                    let (arch_ref_a, arch_ref_b) = if arch_a < arch_b {
                        let (left, right) = self.archetypes.split_at_mut(arch_b);
                        (&mut left[arch_a], &mut right[0])
                    } else {
                        let (left, right) = self.archetypes.split_at_mut(arch_a);
                        (&mut right[0], &mut left[arch_b])
                    };

                    let ca = arch_ref_a.components.get_mut(&type_id)
                        .and_then(|col| col.get_mut(row_a))
                        .and_then(|c| c.as_mut().as_any_mut().downcast_mut::<T>());

                    let cb = arch_ref_b.components.get_mut(&type_id)
                        .and_then(|col| col.get_mut(row_b))
                        .and_then(|c| c.as_mut().as_any_mut().downcast_mut::<T>());

                    (ca, cb)
                }
            }
        }
    }

    pub fn remove_component<T: Component + Clone>(&mut self, entity_id: EntityId) -> Option<T> {
        let type_id = TypeId::of::<T>();

        // Get current archetype
        let (current_arch_idx, old_row_idx) = *self.entity_to_archetype.get(&entity_id)?;
        let current_signature = self.archetypes[current_arch_idx].signature.clone();
        let mut new_signature = current_signature.clone(); // clone here, keep current_signature for looping
        new_signature.remove(&type_id);
        new_signature.remove(&type_id);

        let component = self.archetypes[current_arch_idx].components
            .get_mut(&type_id)?
            .swap_remove(old_row_idx);

        // Find or create new archetype
        let new_arch_idx = self.find_or_create_archetype(new_signature);

        // Split to get two mutable archetype references
        let (old_arch, new_arch) = if current_arch_idx < new_arch_idx {
            let (left, right) = self.archetypes.split_at_mut(new_arch_idx);
            (&mut left[current_arch_idx], &mut right[0])
        } else {
            let (left, right) = self.archetypes.split_at_mut(current_arch_idx);
            (&mut right[0], &mut left[new_arch_idx])
        };

        for existing_type_id in current_signature.iter() {
            if *existing_type_id == type_id { continue; }
            if let Some(column) = old_arch.components.get_mut(existing_type_id) {
                let migrated = column.swap_remove(old_row_idx);
                new_arch.components
                    .entry(*existing_type_id)
                    .or_insert_with(Vec::new)
                    .push(migrated);
            }
        }

        // Migrate entity
        old_arch.entities.swap_remove(old_row_idx);
        let new_row_idx = new_arch.entities.len();
        new_arch.entities.push(entity_id);

        // Update displaced entity
        if let Some(&displaced) = old_arch.entities.get(old_row_idx) {
            if let Some(entry) = self.entity_to_archetype.get_mut(&displaced) {
                entry.1 = old_row_idx;
            }
        }

        // Update entity mapping
        self.entity_to_archetype.insert(entity_id, (new_arch_idx, new_row_idx));

        // Downcast and return
        Some(component.as_ref().as_any().downcast_ref::<T>()?.clone())
    }
    
    pub fn has_component<T: Component>(&self, entity_id: EntityId) -> bool {
        self.get_component::<T>(entity_id).is_some()
    }
    
    pub fn destroy_entity(&mut self, entity_id: EntityId) {
        // Remove from current archetype
        if let Some(&(arch_idx, row_idx)) = self.entity_to_archetype.get(&entity_id) {
            let signature: Vec<TypeId> = self.archetypes[arch_idx].signature.iter().copied().collect();

            for type_id in signature {
                if let Some(column) = self.archetypes[arch_idx].components.get_mut(&type_id) {
                    column.swap_remove(row_idx);
                }
            }

            self.archetypes[arch_idx].entities.swap_remove(row_idx);

            //Update mapping of diplaced entity
            if let Some(&displaced) = self.archetypes[arch_idx].entities.get(row_idx) {
                if let Some(entry) = self.entity_to_archetype.get_mut(&displaced) {
                    entry.1 = row_idx;
                }
            }
        }

        // Remove archetype mapping
        self.entity_to_archetype.remove(&entity_id);
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
            .filter_map(|(&entity_id, &(arch_idx, _row_idx))| {
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
                result.extend(archetype.entities.iter().copied());
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct CompA(u32);

    #[derive(Debug)]
    struct CompB(u32);

    #[derive(Debug)]
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