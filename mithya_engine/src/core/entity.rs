use std::collections::HashMap;
use super::Transform;
use crate::rendering::Renderable;

// Simple entity ID system
pub type EntityId = u32;

// Entity manager - stores components for entities
pub struct EntityManager {
    next_entity_id: EntityId,
    transforms: HashMap<EntityId, Transform>,
    renderables: HashMap<EntityId, Renderable>,
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            next_entity_id: 0,
            transforms: HashMap::new(),
            renderables: HashMap::new(),
        }
    }

    pub fn create_entity(&mut self) -> EntityId {
        let id = self.next_entity_id;
        self.next_entity_id += 1;
        id
    }

    pub fn add_transform(&mut self, entity: EntityId, transform: Transform) {
        self.transforms.insert(entity, transform);
    }

    pub fn add_renderable(&mut self, entity: EntityId, renderable: Renderable) {
        self.renderables.insert(entity, renderable);
    }

    pub fn get_transform(&self, entity: EntityId) -> Option<&Transform> {
        self.transforms.get(&entity)
    }

    pub fn get_renderable(&self, entity: EntityId) -> Option<&Renderable> {
        self.renderables.get(&entity)
    }

    pub fn get_renderable_mut(&mut self, entity: EntityId) -> Option<&mut Renderable> {
        self.renderables.get_mut(&entity)
    }

    pub fn get_transform_and_renderable_mut(&mut self, entity_id: EntityId) 
    -> Option<(&Transform, &mut Renderable)> {
        if let (Some(transform), Some(renderable)) = (
            self.transforms.get(&entity_id),
            self.renderables.get_mut(&entity_id)
        ) {
            Some((transform, renderable))
        } else {
            None
        }
    }

    // Get all entities that have both transform and renderable components
    pub fn get_renderable_entities(&self) -> Vec<EntityId> {
        self.renderables
            .keys()
            .filter(|&&entity| self.transforms.contains_key(&entity))
            .copied()
            .collect()
    }
}