// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use mithya_engine::{
    engine::{Engine, EngineConfig, EntityBuilder, GameLogic, World},
    player::Player,
    physics::RigidBody,
    physics::collider::{Collider, ColliderShape},
    rendering::mesh::Mesh,
    Render,
    Transform
};
use glam::{Quat, Vec3};

//Sandbox to test engine features
struct Sandbox {
    // Game-specific state
}

impl GameLogic for Sandbox {
    fn initialize(&mut self, world: &mut World) {

        let _player = EntityBuilder::new(&mut world.entity_manager)
            .with(Transform{
                position: Vec3 { x: 0.0, y: 20.0, z: 0.0 },
                rotation: Quat::IDENTITY,
                scale: Vec3::ONE,
            })
            .with(Render {
                mesh: Mesh::new_quad_textured(),
                material_id: world.material_manager
                    .get_material_id_from_name("unlit_texture_circle")
                    .copied()
            })
            .with(Player)
            .with(RigidBody {
                velocity: Vec3::ZERO,
                bounce: 1.0,
                ..Default::default()
            })
            .with(Collider {
                shape: ColliderShape::Circle { radius: 1.0 },
                ..Default::default()
            })
            .build();

        let _floor = EntityBuilder::new(&mut world.entity_manager)
            .with(Transform{
                position: Vec3 { x: 0.0, y: -15.0, z: 0.0 },
                rotation: Quat::IDENTITY,
                scale: Vec3::ONE * 2.0,
            })
            .with(Render {
                mesh: Mesh::new_quad_textured(),
                material_id: world.material_manager
                    .get_material_id_from_name("unlit_texture_default")
                    .copied()
            })
            .with(Collider {
                shape: ColliderShape::Box { width: 2.0, height: 2.0 },
                ..Default::default()
            })
            .build();
    }

    fn update(&mut self, _world: &mut World, _delta_time: f32) {
        // Sandbox Updates go here
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = EngineConfig {
        window_title: "Mithya Sandbox".to_string(),
        window_width: 960,
        window_height: 540,
        ..Default::default()
    };

    let engine = Engine::new(config)?;
    let game = Sandbox { /* initialize game state */ };
    
    engine.run(game)
}
