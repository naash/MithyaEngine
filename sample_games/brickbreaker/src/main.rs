// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use mithya_engine::{
    engine::{system::SystemsManager, Engine, EngineConfig, EntityBuilder, GameLogic, World},
    physics::{Collider, ColliderShape, RigidBody},
    rendering::RenderingSystem,
    player::Player, rendering::{Mesh, Render}, 
    Transform
};

mod brick_breaker;

use crate::brick_breaker::{components::{Brick, Ball, Paddle, BrickType, BrickBreakerState},
    systems::BrickBreakerSystem,
    brick_spawner::*
};

use glam::{Quat, Vec3};

//Brickbreaker
struct Brickbreaker {
    // Game-specific state
}

impl GameLogic for Brickbreaker {
    fn initialize(&mut self, world: &mut World, systems_manager: &mut SystemsManager) {
        println!("Initializing Brick Breaker...");
        // === TEXTURES ===
        // load_assets
        if let Some(renderer) = systems_manager.get_system_mut::<RenderingSystem>() {
            renderer.load_assets(&mut world.asset_manager, |assets, device, queue| {
                assets.load_texture_for_material("unlit_texture_green", "green_texture.png", device, queue).unwrap();
                assets.load_texture_for_material("unlit_texture_red", "red_texture.png", device, queue).unwrap();
                assets.load_texture_for_material("unlit_texture_blue", "blue_texture.png", device, queue).unwrap();
                assets.load_texture_for_material("unlit_texture_yellow", "yellow_texture.png", device, queue).unwrap();
                assets.load_texture_for_material("unlit_texture_orange", "orange_texture.png", device, queue).unwrap();
                assets.load_texture_for_material("unlit_texture_purple", "purple_texture.png", device, queue).unwrap();
                assets.load_texture_for_material("unlit_texture_circle", "pinkCircle.png", device, queue).unwrap();
            });
        }

        // === WALLS ===
        spawn_walls(world);
        
        // === BALL ===
        let ball_id = spawn_ball(world);
        
        // === PADDLE ===
        let paddle_id = spawn_paddle(world);
        
        // === BRICKS ===
        spawn_bricks(world);
        
        let game_manager_id = spawn_game_manager(world);

        let mut brickerBreakerSystem = BrickBreakerSystem::new(ball_id, paddle_id, game_manager_id, 0.0);
        systems_manager.add_system(brickerBreakerSystem);

        println!("Brick Breaker ready!");
    }

    fn update(&mut self, _world: &mut World, _delta_time: f32) {
        // Brickbreaker Updates go here
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = EngineConfig {
        window_title: "Brick breaker".to_string(),
        window_width: 960,
        window_height: 540,
        asset_root: std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")),
        ..Default::default()
    };

    let game = Brickbreaker { /* initialize game state */ };

    let engine = Engine::new(config, game);

    engine.run()
}


fn spawn_walls(world: &mut World) {
    // Left Wall
    EntityBuilder::new(&mut world.entity_manager)
        .with(Transform {
            position: Vec3::new(-20.0, 0.0, 0.0),
            rotation: Quat::IDENTITY,
            scale: Vec3::new(1.0, 39.0, 1.0),
        })
        .with(Render {
            mesh: Mesh::new_quad_textured(),
            material_id: world.asset_manager
                .get_material_by_name("unlit_texture_orange")
        })
        .with(Collider {
            shape: ColliderShape::Box { width: 1.0, height: 1.0 },
            ..Default::default()
        })
        .build();

    // Right Wall
    EntityBuilder::new(&mut world.entity_manager)
        .with(Transform {
            position: Vec3::new(20.0, 0.0, 0.0),
            rotation: Quat::IDENTITY,
            scale: Vec3::new(1.0, 39.0, 1.0),
        })
        .with(Render {
            mesh: Mesh::new_quad_textured(),
            material_id: world.asset_manager
                .get_material_by_name("unlit_texture_orange")
        })
        .with(Collider {
            shape: ColliderShape::Box { width: 1.0, height: 1.0 },
            ..Default::default()
        })
        .build();

    //Top Wall
    EntityBuilder::new(&mut world.entity_manager)
        .with(Transform {
            position: Vec3::new(0.0, 19.0, 0.0),
            rotation: Quat::IDENTITY,
            scale: Vec3::new(38.5, 1.0, 1.0),
        })
        .with(Render {
            mesh: Mesh::new_quad_textured(),
            material_id: world.asset_manager
                .get_material_by_name("unlit_texture_orange")
        })
        .with(Collider {
            shape: ColliderShape::Box { width: 1.0, height: 1.0 },
            ..Default::default()
        })
        .build();
}

fn spawn_ball(world: &mut World) -> u32 {
    EntityBuilder::new(&mut world.entity_manager)
        .with(Transform {
            position: Vec3::new(0.0, -14.0, 0.0),  // Start at center
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        })
        .with(Render {
            mesh: Mesh::new_quad_textured(),
            material_id: world.asset_manager
                .get_material_by_name("unlit_texture_circle")
        })
        .with(RigidBody {
            bounce: 1.0,
            is_kinematic: true, //Set kinematic to false when ball should be launched
            ..Default::default()
        })
        .with(Collider {
            shape: ColliderShape::Circle { radius: 0.5 },  // Smaller radius
            ..Default::default()
        })
        .with(Ball)  // Mark as ball for game logic
        .build()
}

fn spawn_paddle(world: &mut World) -> u32 {
    EntityBuilder::new(&mut world.entity_manager)
        .with(Transform {
            position: Vec3::new(0.0, -15.0, 0.0),
            rotation: Quat::IDENTITY,
            scale: Vec3::new(5.0, 1.0, 1.0),
        })
        .with(Render {
            mesh: Mesh::new_quad_textured(),
            material_id: world.asset_manager
                .get_material_by_name("unlit_texture_green")
        })
        .with(Collider {
            shape: ColliderShape::Box { width: 1.0, height: 1.0 },
            ..Default::default()
        })
        .with(RigidBody {
            velocity: Vec3::ZERO,
            bounce: 0.0,
            is_kinematic: true,
            ..Default::default()
        })
        .with(Player)
        .build()
}

fn spawn_bricks(world: &mut World) {
    // Option 1: Spawn a full grid
    let config = BrickGridConfig {
        rows: 5,
        columns: 10,
        brick_width: 3.0,
        brick_height: 1.5,
        spacing: 0.2,
        start_position: Vec3::new(0.0, 10.0, 0.0),
    };
    spawn_brick_grid(world, config);
    
    // Option 2: Spawn a pattern (uncomment to try)
    
    // Option 3: Spawn individual test bricks (for debugging);
}

fn spawn_game_manager(world: &mut World) -> u32 {
    EntityBuilder::new(&mut world.entity_manager)
        .with(BrickBreakerState::default())
        .build()
}

// Helper function for testing individual bricks
#[allow(dead_code)]
fn spawn_test_bricks(world: &mut World) {
    // Just a few bricks for testing
    for i in 0..3 {
        spawn_brick(
            world,
            Vec3::new(-6.0 + (i as f32 * 3.5), 10.0, 0.0),
            3.0,
            1.5,
            BrickType::Normal,
            "unlit_texture_red",
        );
    }
}