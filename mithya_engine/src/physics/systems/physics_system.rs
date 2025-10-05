// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use glam::Vec3;

use crate::{
    core::Transform,
    engine::{system::{System, SystemRenderContext, SystemUpdateContext}, World}, 
    physics::RigidBody,
};

pub struct PhysicsSystem;

impl System for PhysicsSystem {
    fn initialize(&mut self, _world: &mut World) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn update(&mut self, update_context: &mut SystemUpdateContext) {
        let dt = update_context.world.physics_config.time_step;
        let gravity = update_context.world.physics_config.gravity;

        // Get all entities with both Transform and RigidBody
        let physics_entities = update_context.world.entity_manager
            .query_two_components::<Transform, RigidBody>();

        for entity_id in physics_entities {
            // Cache velocity and should_update flag
            let (velocity, should_update) = {
                if let Some(rigidbody) = update_context.world.entity_manager.get_component_mut::<RigidBody>(entity_id) {
                    // Skip kinematic bodies
                    if rigidbody.is_kinematic {
                        continue;
                    }

                     // Reset with gravity at start
                    rigidbody.acceleration.y = gravity.y * rigidbody.gravity_scale;

                    //Future feature Apply other forces here

                    // Update velocity
                    rigidbody.velocity.x += rigidbody.acceleration.x * dt;
                    rigidbody.velocity.y += rigidbody.acceleration.y * dt;

                    // Apply drag
                    rigidbody.velocity.x *= 1.0 - (rigidbody.drag * dt);
                    rigidbody.velocity.y *= 1.0 - (rigidbody.drag * dt);

                    // Store velocity for position update
                    let vel = rigidbody.velocity;

                    // Reset acceleration for next frame
                    rigidbody.acceleration = Vec3::ZERO;

                    (vel, true)
                } else {
                    (Vec3::ZERO, false)
                }
            }; // The mutable borrow of entity_manager ends here

            // Update transform position
            if should_update {
                if let Some(transform) = update_context.world.entity_manager.get_component_mut::<Transform>(entity_id) {
                    transform.position.x += velocity.x * dt;
                    transform.position.y += velocity.y * dt;
                }
            }
        }
    }

    fn render(&mut self, _render_context: &mut SystemRenderContext) {
        // Physics system doesn't render anything
    }
}