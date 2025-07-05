// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use glam::Vec2;

use crate::{
    core::{Transform},
    engine::{system::System, World}, 
    physics::RigidBody,
};

pub struct PhysicsSystem;

impl System for PhysicsSystem {
    fn update(&mut self, world: &mut World) {
        let dt = world.physics_config.time_step;
        let gravity = world.physics_config.gravity;

        // Get all entities with both Transform and RigidBody
        let physics_entities = world.entity_manager
            .query_two_components::<Transform, RigidBody>();

        for entity_id in physics_entities {
            // cache velocity and should_update flag
            let (velocity, should_update) = {
                if let Some(rigidbody) = world.entity_manager.get_component_mut::<RigidBody>(entity_id) {
                    // Skip kinematic bodies
                    if rigidbody.is_kinematic {
                        continue;
                    }

                    // Apply gravity
                    rigidbody.acceleration.y += gravity.y * rigidbody.gravity_scale;

                    // Apply drag
                    rigidbody.velocity.x *= 1.0 - (rigidbody.drag * dt);
                    rigidbody.velocity.y *= 1.0 - (rigidbody.drag * dt);

                    // Update velocity
                    rigidbody.velocity.x += rigidbody.acceleration.x * dt;
                    rigidbody.velocity.y += rigidbody.acceleration.y * dt;

                    // Store velocity for position update
                    let vel = rigidbody.velocity;

                    // Reset acceleration for next frame
                    rigidbody.acceleration = Vec2::ZERO;

                    (vel, true)
                } else {
                    (Vec2::ZERO, false)
                }
            }; // The mutable borrow of entity_manager ends here

            // Update transform
            if should_update {
                if let Some(transform) = world.entity_manager.get_component_mut::<Transform>(entity_id) {
                    transform.position.x += velocity.x * dt;
                    transform.position.y += velocity.y * dt;
                }
            }
        }
    }
}