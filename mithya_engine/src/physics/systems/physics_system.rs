// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use glam::Vec3;

use crate::{
    core::{Transform},
    engine::{system::System, World}, 
    physics::RigidBody,
};

pub struct PhysicsSystem;

impl System for PhysicsSystem {
    fn initialize(&mut self, _world: &mut World) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn handle_event(&mut self, _event: &sdl2::event::Event, _world: &mut World) -> bool {
        false
    }

    fn update(&mut self, world: &mut World, _delta_time: f32) {
        let dt = world.physics_config.time_step;
        let gravity = world.physics_config.gravity;

        // Get all entities with both Transform and RigidBody
        let physics_entities = world.entity_manager
            .query_two_components::<Transform, RigidBody>();

        for entity_id in physics_entities {
            // Cache velocity and should_update flag
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
                    rigidbody.acceleration = Vec3::ZERO;

                    (vel, true)
                } else {
                    (Vec3::ZERO, false)
                }
            }; // The mutable borrow of entity_manager ends here

            // Update transform position
            if should_update {
                if let Some(transform) = world.entity_manager.get_component_mut::<Transform>(entity_id) {
                    transform.position.x += velocity.x * dt;
                    transform.position.y += velocity.y * dt;
                }
            }
        }
    }

    fn render(&mut self, _world: &mut World) {
        // Physics system doesn't render anything
    }
}