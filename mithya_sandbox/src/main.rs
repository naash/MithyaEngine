// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use mithya_engine::{
    engine::{Engine, EngineConfig, EntityBuilder, GameLogic, World}, physics::{Collider, ColliderShape, RigidBody}, player::Player, rendering::{Mesh, Render}, Transform
};

use glam::{Quat, Vec3};

//Sandbox to test engine features
struct Sandbox {
    // Game-specific state
}

impl GameLogic for Sandbox {
    fn initialize(&mut self, world: &mut World) {

        //Wall
        // Left Wall
        let _left_wall = EntityBuilder::new(&mut world.entity_manager)
            .with(Transform {
                position: Vec3 { x: -20.0, y: 0.0, z: 0.0 },
                rotation: Quat::IDENTITY,
                scale: Vec3 { x: 1.0, y: 39.0, z: 1.0 },
            })
            .with(Render {
                mesh: Mesh::new_quad_textured(),
                material_id: world.asset_manager
                    .get_material_by_name("unlit_texture_default")
            })
            .with(Collider {
                shape: ColliderShape::Box { width: 1.0, height: 1.0 },
                ..Default::default()
            })
            .build();

        // Right Wall
        let _right_wall = EntityBuilder::new(&mut world.entity_manager)
            .with(Transform {
                position: Vec3 { x: 20.0, y: 0.0, z: 0.0 },
                rotation: Quat::IDENTITY,
                scale: Vec3 { x: 1.0, y: 39.0, z: 1.0 },
            })
            .with(Render {
                mesh: Mesh::new_quad_textured(),
                material_id: world.asset_manager
                    .get_material_by_name("unlit_texture_default")
            })
            .with(Collider {
                shape: ColliderShape::Box { width: 1.0, height: 1.0 },
                ..Default::default()
            })
            .build();

        // Top Wall
        let _top_wall = EntityBuilder::new(&mut world.entity_manager)
            .with(Transform {
                position: Vec3 { x: 0.0, y: 19.0, z: 0.0 },
                rotation: Quat::IDENTITY,
                scale: Vec3 { x: 38.5, y: 1.0, z: 1.0 },
            })
            .with(Render {
                mesh: Mesh::new_quad_textured(),
                material_id: world.asset_manager
                    .get_material_by_name("unlit_texture_default")
            })
            .with(Collider {
                shape: ColliderShape::Box { width: 1.0, height: 1.0 },
                ..Default::default()
            })
            .build();

        let _ball = EntityBuilder::new(&mut world.entity_manager)
            .with(Transform{
                position: Vec3 { x: 0.0, y: 15.0, z: 0.0 },
                rotation: Quat::IDENTITY,
                scale: Vec3::ONE,
            })
            .with(Render {
                mesh: Mesh::new_quad_textured(),
                material_id: world.asset_manager
                    .get_material_by_name("unlit_texture_circle")
            })
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

        let _paddle = EntityBuilder::new(&mut world.entity_manager)
            .with(Transform{
                position: Vec3 { x: 0.0, y: -15.0, z: 0.0 },
                rotation: Quat::IDENTITY,
                scale: Vec3 { x: 5.0, y: 1.0, z: 1.0 },
            })
            .with(Render {
                mesh: Mesh::new_quad_textured(),
                material_id: world.asset_manager
                    .get_material_by_name("unlit_texture_default")
            })
            .with(Collider {
                shape: ColliderShape::Box { width: 1.0, height: 1.0 },
                ..Default::default()
            })
            .with(Player)
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
