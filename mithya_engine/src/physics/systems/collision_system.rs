// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use glam::Vec2;

use crate::{
    core::Transform,
    engine::{system::System, World}, physics::{collider::{Collider, ColliderShape}, RigidBody},
};

pub struct CollisionSystem;

impl System for CollisionSystem {
    fn update(&mut self, world: &mut World) {
        let collider_entities = world.entity_manager
        .query_two_components::<Transform, Collider>();

        // Check all pairs of entities
        for i in 0..collider_entities.len() {
            for j in i + 1..collider_entities.len() {
                let entity_a = collider_entities[i];
                let entity_b = collider_entities[j];

                if let (Some(transform_a), Some(collider_a)) = (
                    world.entity_manager.get_component::<Transform>(entity_a),
                    world.entity_manager.get_component::<Collider>(entity_a)
                ) {
                    if let (Some(transform_b), Some(collider_b)) = (
                        world.entity_manager.get_component::<Transform>(entity_b),
                        world.entity_manager.get_component::<Collider>(entity_b)
                    ) {
                        if check_collision(transform_a, collider_a, transform_b, collider_b) {
                            resolve_collision(world, entity_a, entity_b);
                        }
                    }
                }
            }
        }
    }
}

fn check_collision(
    transform_a: &Transform,
    collider_a: &Collider,
    transform_b: &Transform,
    collider_b: &Collider,
) -> bool {
    let pos_a = Vec2::new(transform_a.position.x, transform_a.position.y) + collider_a.offset;
    let pos_b = Vec2::new(transform_b.position.x, transform_b.position.y) + collider_b.offset;

    match (&collider_a.shape, &collider_b.shape) {
        (ColliderShape::Circle { radius: r1 }, ColliderShape::Circle { radius: r2 }) => {
            let distance = (pos_a - pos_b).length();
            distance < r1 + r2
        }
        (ColliderShape::Box { width: w1, height: h1 }, ColliderShape::Box { width: w2, height: h2 }) => {
            let half_w1 = w1 / 2.0;
            let half_h1 = h1 / 2.0;
            let half_w2 = w2 / 2.0;
            let half_h2 = h2 / 2.0;

            pos_a.x - half_w1 < pos_b.x + half_w2 &&
            pos_a.x + half_w1 > pos_b.x - half_w2 &&
            pos_a.y - half_h1 < pos_b.y + half_h2 &&
            pos_a.y + half_h1 > pos_b.y - half_h2
        }
        // Circle-Box collision
        _ => false, //Todo
    }
}

fn resolve_collision(world: &mut World, entity_a: u32, entity_b: u32) {
    println!("Collision between {} and {}", entity_a, entity_b);

    // Get the positions and colliders again to calculate separation
    let (pos_a, pos_b, separation_distance) = {
        let transform_a = world.entity_manager.get_component::<Transform>(entity_a).unwrap();
        let transform_b = world.entity_manager.get_component::<Transform>(entity_b).unwrap();
        let collider_a = world.entity_manager.get_component::<Collider>(entity_a).unwrap();
        let collider_b = world.entity_manager.get_component::<Collider>(entity_b).unwrap();
        
        let pos_a = Vec2::new(transform_a.position.x, transform_a.position.y) + collider_a.offset;
        let pos_b = Vec2::new(transform_b.position.x, transform_b.position.y) + collider_b.offset;
        
        // Calculate how far apart they should be
        let separation = match (&collider_a.shape, &collider_b.shape) {
            (ColliderShape::Circle { radius: r1 }, ColliderShape::Circle { radius: r2 }) => {
                r1 + r2 + 0.1 // Small buffer to prevent immediate re-collision
            }
            (ColliderShape::Box { width: w1, height: h1 }, ColliderShape::Box { width: w2, height: h2 }) => {
                // Simple approximation - use the larger dimension
                ((w1 + w2) / 2.0).max((h1 + h2) / 2.0) + 0.1
            }
            _ => 1.0, // Default separation
        };
        
        (pos_a, pos_b, separation)
    };
    
    // Calculate direction from A to B
    let direction = (pos_b - pos_a).normalize();
    let current_distance = (pos_b - pos_a).length();
    
    // Check which objects have rigidbodies (only move objects with rigidbodies)
    let has_rigidbody_a = world.entity_manager.get_component::<RigidBody>(entity_a).is_some();
    let has_rigidbody_b = world.entity_manager.get_component::<RigidBody>(entity_b).is_some();
    
    // Calculate how much to move each object
    let move_distance = separation_distance - current_distance;
    //This is not ideal. Use impact normal instead to push in the opposite direction
    if has_rigidbody_a && has_rigidbody_b {
        // Both have rigidbodies, split the movement
        let half_move = move_distance / 2.0;
        
        if let Some(transform_a) = world.entity_manager.get_component_mut::<Transform>(entity_a) {
            transform_a.position.x -= direction.x * half_move;
            transform_a.position.y -= direction.y * half_move;
        }
        
        if let Some(transform_b) = world.entity_manager.get_component_mut::<Transform>(entity_b) {
            transform_b.position.x += direction.x * half_move;
            transform_b.position.y += direction.y * half_move;
        }
    } else if has_rigidbody_a {
        // Only A has rigidbody, move A away from B
        if let Some(transform_a) = world.entity_manager.get_component_mut::<Transform>(entity_a) {
            transform_a.position.x -= direction.x * move_distance;
            transform_a.position.y -= direction.y * move_distance;
        }
    } else if has_rigidbody_b {
        // Only B has rigidbody, move B away from A
        if let Some(transform_b) = world.entity_manager.get_component_mut::<Transform>(entity_b) {
            transform_b.position.x += direction.x * move_distance;
            transform_b.position.y += direction.y * move_distance;
        }
    }
}