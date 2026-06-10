// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::any::Any;
use glam::Vec3;

use crate::{
    core::Transform,
    engine::{World, resources::Time, system::{System, SystemUpdateContext}},
    physics::{PhysicsConfig, RigidBody},
};

pub struct PhysicsSystem;

impl System for PhysicsSystem {
    fn initialize(&mut self, world: &mut World) {
        world.resources.insert(PhysicsConfig::default());
    }

    fn update(&mut self, update_context: &mut SystemUpdateContext) {

        let dt = update_context.world.resources.get::<Time>()
                                .map(|t| t.delta)
                                .unwrap_or_default();

        let gravity = update_context.world.resources.get::<PhysicsConfig>()
                                .map(|s| s.gravity)
                                .unwrap_or_default();

        // Get all entities with both Transform and RigidBody
        let physics_entities = update_context.world.entity_manager
            .query_two_components::<Transform, RigidBody>();

        for entity_id in physics_entities {
            // Cache velocity and should_update flag
            let (velocity, should_update) = {
                if let Some(rigidbody) = update_context.world.entity_manager.get_component_mut::<RigidBody>(entity_id) {
                     // Reset with gravity at start
                    rigidbody.acceleration.y += gravity.y * rigidbody.gravity_scale;
                    //Future feature Apply other forces here

                    //Cap acceleration
                    if rigidbody.max_acceleration > 0.0 && rigidbody.acceleration.length() > rigidbody.max_acceleration {
                        let capped_acceleration = rigidbody.acceleration.normalize() * rigidbody.max_acceleration;
                        rigidbody.acceleration = capped_acceleration;
                    }

                    // Update velocity
                    rigidbody.velocity.x += rigidbody.acceleration.x * dt;
                    rigidbody.velocity.y += rigidbody.acceleration.y * dt;

                    // Apply drag
                    rigidbody.velocity.x *= 1.0 - (rigidbody.drag * dt);
                    rigidbody.velocity.y *= 1.0 - (rigidbody.drag * dt);

                    //Cap velocity
                    if rigidbody.max_speed > 0.0 && rigidbody.velocity.length() > rigidbody.max_speed {
                        let capped_velocity = rigidbody.velocity.normalize() * rigidbody.max_speed;
                        rigidbody.velocity = capped_velocity;
                    }
                    
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
    
    fn as_event_listener_mut(&mut self) -> Option<&mut dyn crate::core::EngineEventListener> {
        None
    }

    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::Vec2;
    use crate::{
        asset::AssetManager,
        core::{EngineEventQueue, EntityManager, Resources},
        engine::EntityBuilder,
        physics::RigidBody,
    };

    fn test_world() -> World {
        let mut world = World {
            entity_manager: EntityManager::new(),
            asset_manager: AssetManager::new(std::path::PathBuf::new()).unwrap(),
            resources: Resources::new(),
        };
        world.resources.insert(Time::default());
        world.resources.insert(PhysicsConfig { gravity: Vec2::ZERO });
        world
    }

    fn step(world: &mut World, delta: f32) {
        world.resources.get_mut::<Time>().unwrap().delta = delta;
        let mut events = EngineEventQueue::new();
        let mut ctx = SystemUpdateContext { world, events: &mut events };
        PhysicsSystem.update(&mut ctx);
    }

    #[test]
    fn integration_scales_with_frame_delta() {
        let mut world = test_world();
        let entity = EntityBuilder::new(&mut world.entity_manager)
            .with(Transform::default())
            .with(RigidBody {
                velocity: Vec3::new(2.0, 0.0, 0.0),
                drag: 0.0,
                gravity_scale: 0.0,
                ..Default::default()
            })
            .build();

        step(&mut world, 0.1);
        let x_after_short = world.entity_manager
            .get_component::<Transform>(entity).unwrap().position.x;
        assert!((x_after_short - 0.2).abs() < 1e-5);

        step(&mut world, 0.3);
        let x_after_long = world.entity_manager
            .get_component::<Transform>(entity).unwrap().position.x;
        assert!((x_after_long - 0.8).abs() < 1e-5);
    }
}