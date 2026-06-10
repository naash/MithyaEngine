// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::{any::Any, collections::HashSet};
use glam::{Vec2, Vec3Swizzles};

use crate::{
        core::{EngineEvent, Transform}, engine::{World, system::{System, SystemUpdateContext}}, physics::{
        collision_math::{self, CollisionInfo}, components::{Collider, RigidBody}, physics_config::{DAMPING_THRESHOLD, MIN_SEPARATION, MIN_VELOCITY, VELOCITY_DAMPING}
    }
};

#[derive(Debug, Clone)]
pub struct CollisionEvent {
    pub entity_a: u32,
    pub entity_b: u32,
}

impl EngineEvent for CollisionEvent {
    fn as_any(&self) -> &dyn Any { self }
}

pub struct CollisionSystem;

impl System for CollisionSystem {
    fn initialize(&mut self, _world: &mut World) {}

    fn update(&mut self, update_context: &mut SystemUpdateContext) {
        let collider_entities = update_context.world.entity_manager.query_two_components::<Transform, Collider>();
        let mut colliding_entities = HashSet::new();

        // Check all pairs of entities
        for i in 0..collider_entities.len() {
            for j in i + 1..collider_entities.len() {
                let entity_a = collider_entities[i];
                let entity_b = collider_entities[j];

                // Check and resolve collision
                if let Some(collision) = self.check_collision(update_context.world, entity_a, entity_b) {
                    // Trigger pairs are detected (event + is_colliding) but never
                    // physically resolved — no separation, no velocity response.
                    let is_trigger_pair = [entity_a, entity_b].iter().any(|&entity| {
                        update_context.world.entity_manager
                            .get_component::<Collider>(entity)
                            .map(|c| c.is_trigger)
                            .unwrap_or(false)
                    });

                    if !is_trigger_pair {
                        self.resolve_collision(update_context.world, entity_a, entity_b, collision);
                    }

                    colliding_entities.insert(entity_a);
                    colliding_entities.insert(entity_b);

                    update_context.events.push(CollisionEvent { entity_a, entity_b });
                }
            }
        }

        // Update collision state
        for &entity in &collider_entities {
            if let Some(collider) = update_context.world.entity_manager.get_component_mut::<Collider>(entity) {
                collider.is_colliding = colliding_entities.contains(&entity);
            }
        }
    }

    fn as_event_listener_mut(&mut self) -> Option<&mut dyn crate::core::EngineEventListener> {
        None
    }

    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}

impl CollisionSystem {
    /// Check collision and return collision info if colliding
    fn check_collision(&self, world: &World, entity_a: u32, entity_b: u32) -> Option<CollisionInfo> {

        let collider_a = world.entity_manager.get_component::<Collider>(entity_a)?;
        let collider_b = world.entity_manager.get_component::<Collider>(entity_b)?;

        if (collider_a.layer & collider_b.mask) == 0 && (collider_b.layer & collider_a.mask) == 0 {
            return None; //Colliders cannot interact
        }
        
        let transform_a = world.entity_manager.get_component::<Transform>(entity_a)?;
        let transform_b = world.entity_manager.get_component::<Transform>(entity_b)?;

        // Get world positions with offsets
        let pos_a = transform_a.position + collider_a.offset;
        let pos_b = transform_b.position + collider_b.offset;

        // Scale the colliders
        let scaled_a = collider_a.get_scaled_shape(transform_a.scale);
        let scaled_b = collider_b.get_scaled_shape(transform_b.scale);

        // Use collision_utils for detection
        collision_math::detect_collision(pos_a.xy(), &scaled_a, pos_b.xy(), &scaled_b)
    }

    /// Resolve collision between two entities
    fn resolve_collision(&self, world: &mut World, entity_a: u32, entity_b: u32, collision: CollisionInfo) {
        // Skip invalid collisions
        if collision.separation < MIN_SEPARATION {
            return;
        }

        // Cache velocities
        let (has_rb_a, velocity_a) = {
            let rb = world.entity_manager.get_component::<RigidBody>(entity_a);
            (rb.is_some(), rb.map(|r| Vec2::new(r.velocity.x, r.velocity.y)).unwrap_or(Vec2::ZERO))
        };

        let (has_rb_b, velocity_b) = {
            let rb = world.entity_manager.get_component::<RigidBody>(entity_b);
            (rb.is_some(), rb.map(|r| Vec2::new(r.velocity.x, r.velocity.y)).unwrap_or(Vec2::ZERO))
        };

        // Separate entities
        self.separate_entities(world, entity_a, entity_b, &collision, has_rb_a, has_rb_b);

        // Apply velocity damping if needed
        if collision.separation > DAMPING_THRESHOLD && has_rb_a == has_rb_b {
            self.apply_damping(world, entity_a, entity_b);
        }

        // Reflect velocities
        if has_rb_a {
            self.reflect_velocity(world, entity_a, collision.normal, velocity_b);
        }
        if has_rb_b {
            self.reflect_velocity(world, entity_b, -collision.normal, velocity_a);
        }        
    }

    /// Separate overlapping entities
    fn separate_entities(&self, world: &mut World, entity_a: u32, entity_b: u32, collision: &CollisionInfo, has_rb_a: bool, has_rb_b: bool) {
        match (has_rb_a, has_rb_b) {
            (true, true) => {
                // Both dynamic: split movement
                let half_move = collision.separation / 2.0;
                self.move_entity(world, entity_a, collision.normal * half_move);
                self.move_entity(world, entity_b, -collision.normal * half_move);
            }
            (true, false) => {
                // Only A is dynamic: move A away from B
                self.move_entity(world, entity_a, collision.normal * collision.separation);
            }
            (false, true) => {
                // Only B is dynamic: move B away from A
                self.move_entity(world, entity_b, -collision.normal * collision.separation);
            }
            (false, false) => {
                // Both static: no movement needed
            }
        }
    }

    /// Move an entity by offset
    fn move_entity(&self, world: &mut World, entity: u32, offset: Vec2) {
        if let Some(transform) = world.entity_manager.get_component_mut::<Transform>(entity) {
            transform.position.x += offset.x;
            transform.position.y += offset.y;
        }
    }

    /// Apply velocity damping to both entities
    fn apply_damping(&self, world: &mut World, entity_a: u32, entity_b: u32) {        
        for &entity in &[entity_a, entity_b] {
            if let Some(rb) = world.entity_manager.get_component_mut::<RigidBody>(entity) {
                rb.velocity *= VELOCITY_DAMPING;
            }
        }
    }

    /// Reflect velocity for an entity
    fn reflect_velocity(&self, world: &mut World, entity: u32, normal: Vec2, other_velocity : Vec2) {

        if let Some(rb) = world.entity_manager.get_component_mut::<RigidBody>(entity) {
            // Skip kinematic bodies
            if rb.is_kinematic {
                return;
            }

            let velocity = Vec2::new(rb.velocity.x, rb.velocity.y);

            // Stop if velocity is too small
            if velocity.length() < MIN_VELOCITY {
                rb.velocity = Vec2::ZERO.extend(rb.velocity.z);
                return;
            }

            let relative_velocity = velocity - other_velocity;
            let vel_along_normal = relative_velocity.dot(normal.normalize());
            if vel_along_normal < 0.0 {
                let speed = velocity.length();
                let reflected_relative = relative_velocity - 2.0 * vel_along_normal * normal.normalize();
                let direction = (reflected_relative + other_velocity).normalize();
                rb.velocity = (direction * speed * rb.bounce).extend(rb.velocity.z);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::Vec3;
    use crate::{
        asset::AssetManager,
        core::{EngineEventQueue, EntityManager, Resources},
        engine::EntityBuilder,
        physics::components::ColliderShape,
    };

    fn test_world() -> World {
        World {
            entity_manager: EntityManager::new(),
            asset_manager: AssetManager::new(std::path::PathBuf::new()).unwrap(),
            resources: Resources::new(),
        }
    }

    fn spawn_circle(world: &mut World, x: f32, is_trigger: bool) -> u32 {
        EntityBuilder::new(&mut world.entity_manager)
            .with(Transform { position: Vec3::new(x, 0.0, 0.0), ..Default::default() })
            .with(Collider {
                shape: ColliderShape::Circle { radius: 1.0 },
                is_trigger,
                ..Default::default()
            })
            .with(RigidBody::default())
            .build()
    }

    fn positions(world: &World, a: u32, b: u32) -> (Vec3, Vec3) {
        (
            world.entity_manager.get_component::<Transform>(a).unwrap().position,
            world.entity_manager.get_component::<Transform>(b).unwrap().position,
        )
    }

    #[test]
    fn solid_overlap_is_separated_and_reported() {
        let mut world = test_world();
        let a = spawn_circle(&mut world, 0.0, false);
        let b = spawn_circle(&mut world, 1.0, false);

        let mut events = EngineEventQueue::new();
        let mut ctx = SystemUpdateContext { world: &mut world, events: &mut events };
        CollisionSystem.update(&mut ctx);

        let (pos_a, pos_b) = positions(&world, a, b);
        assert!((pos_b - pos_a).length() >= 2.0, "overlapping solids must be pushed apart");
        assert_eq!(events.iter_type::<CollisionEvent>().count(), 1);
        assert!(world.entity_manager.get_component::<Collider>(a).unwrap().is_colliding);
        assert!(world.entity_manager.get_component::<Collider>(b).unwrap().is_colliding);
    }

    #[test]
    fn trigger_overlap_is_reported_but_not_resolved() {
        let mut world = test_world();
        let a = spawn_circle(&mut world, 0.0, true);
        let b = spawn_circle(&mut world, 1.0, false);

        let mut events = EngineEventQueue::new();
        let mut ctx = SystemUpdateContext { world: &mut world, events: &mut events };
        CollisionSystem.update(&mut ctx);

        let (pos_a, pos_b) = positions(&world, a, b);
        assert_eq!(pos_a, Vec3::new(0.0, 0.0, 0.0), "trigger overlap must not move entities");
        assert_eq!(pos_b, Vec3::new(1.0, 0.0, 0.0), "trigger overlap must not move entities");
        assert_eq!(events.iter_type::<CollisionEvent>().count(), 1);
        assert!(world.entity_manager.get_component::<Collider>(a).unwrap().is_colliding);
        assert!(world.entity_manager.get_component::<Collider>(b).unwrap().is_colliding);
    }
}