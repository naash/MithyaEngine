// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::{any::Any, collections::HashSet};
use glam::{Vec2, Vec3Swizzles};

use crate::{
    core::{EngineEvent, Transform},
    engine::{system::{System, SystemRenderContext, SystemUpdateContext}, World}, 
    physics::{
        collision_config::{DAMPING_THRESHOLD, MIN_SEPARATION, MIN_VELOCITY, VELOCITY_DAMPING},
        collision_utils::{self, CollisionInfo},
        components::{Collider, RigidBody},
    },
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
    fn initialize(&mut self, _world: &mut World) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }

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
                    self.resolve_collision(update_context.world, entity_a, entity_b, collision);

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

    fn render(&mut self, _render_context: &mut SystemRenderContext) {
        // No rendering for collision system
    }
}

impl CollisionSystem {
    /// Check collision and return collision info if colliding
    fn check_collision(&self, world: &World, entity_a: u32, entity_b: u32) -> Option<CollisionInfo> {
        let transform_a = world.entity_manager.get_component::<Transform>(entity_a)?;
        let transform_b = world.entity_manager.get_component::<Transform>(entity_b)?;
        let collider_a = world.entity_manager.get_component::<Collider>(entity_a)?;
        let collider_b = world.entity_manager.get_component::<Collider>(entity_b)?;

        // Get world positions with offsets
        let pos_a = transform_a.position + collider_a.offset;
        let pos_b = transform_b.position + collider_b.offset;

        // Scale the colliders
        let scaled_a = collider_a.get_scaled_shape(transform_a.scale);
        let scaled_b = collider_b.get_scaled_shape(transform_b.scale);

        // Use collision_utils for detection
        collision_utils::detect_collision(pos_a.xy(), &scaled_a, pos_b.xy(), &scaled_b)
    }

    /// Resolve collision between two entities
    fn resolve_collision(&self, world: &mut World, entity_a: u32, entity_b: u32, collision: CollisionInfo) {
        // Skip invalid collisions
        if collision.separation < MIN_SEPARATION {
            return;
        }

        // Check rigidbody status
        let has_rb_a = world.entity_manager.get_component::<RigidBody>(entity_a).is_some();
        let has_rb_b = world.entity_manager.get_component::<RigidBody>(entity_b).is_some();

        // Separate entities
        self.separate_entities(world, entity_a, entity_b, &collision, has_rb_a, has_rb_b);

        // Apply velocity damping if needed
        if collision.separation > DAMPING_THRESHOLD && has_rb_a && has_rb_b {
            self.apply_damping(world, entity_a, entity_b);
        }

        // Reflect velocities
        self.reflect_velocity(world, entity_a, collision.normal, has_rb_a);
        self.reflect_velocity(world, entity_b, -collision.normal, has_rb_b);
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
    fn reflect_velocity(&self, world: &mut World, entity: u32, normal: Vec2, has_rigidbody: bool) {
        if !has_rigidbody {
            return;
        }

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

            let vel_along_normal = velocity.dot(normal);

            // Only reflect if moving into the collision
            if vel_along_normal < 0.0 {
                let reflected = velocity - 2.0 * vel_along_normal * normal;
                rb.velocity.x = reflected.x * rb.bounce;
                rb.velocity.y = reflected.y * rb.bounce;
            }
        }
    }
}