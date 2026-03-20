// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::collections::HashSet;

use mithya_engine::{
    Transform, 
    engine::{ Engine, EngineConfig, EntityBuilder, GameLogic, World, system::SystemsManager}, 
    input::{ InputMapping, mapping::{ InputAction, InputBinding}}, 
    pawn::{ Controller, Movement }, 
    physics::{ Collider, ColliderShape, RigidBody}, 
    rendering::{ Mesh, Render, Camera }
};

mod brick_breaker;

use crate::brick_breaker::{
    brick_spawner::*, components::{
        Ball, BrickBreakerState, BrickType, GameState
    }, layers::{LAYER_BALL, LAYER_BRICK, LAYER_PADDLE, LAYER_WALL}, systems::BrickBreakerSystem
};
use winit::keyboard::KeyCode;

use glam::Vec3;

//Brickbreaker
struct Brickbreaker {
    // Game-specific state
}

impl GameLogic for Brickbreaker {
    fn initialize(&mut self, world: &mut World, systems_manager: &mut SystemsManager) {
        // === TEXTURES ===
        // load_assets
        if let Some(renderer) = systems_manager.get_rendering_system() {
            renderer.load_assets(&mut world.asset_manager, |assets, device, queue| {
                assets.load_texture_for_material("unlit_texture_green", "green_texture.png", device, queue).expect("Failed to get texture green_texture.png");
                assets.load_texture_for_material("unlit_texture_red", "red_texture.png", device, queue).expect("Failed to get texture red_texture.png");
                assets.load_texture_for_material("unlit_texture_blue", "blue_texture.png", device, queue).expect("Failed to get texture blue_texture.png");
                assets.load_texture_for_material("unlit_texture_yellow", "yellow_texture.png", device, queue).expect("Failed to get texture yellow_texture.png");
                assets.load_texture_for_material("unlit_texture_orange", "orange_texture.png", device, queue).expect("Failed to get texture orange_texture.png");
                assets.load_texture_for_material("unlit_texture_purple", "purple_texture.png", device, queue).expect("Failed to get texture purple_texture.png");
                assets.load_texture_for_material("unlit_texture_circle", "pinkCircle.png", device, queue).expect("Failed to get texture pinkCircle.png");
            });
        }
        
        // === BALL ===
        let ball_id = spawn_ball(world);
        
        // === PADDLE ===
        let paddle_id = spawn_paddle(world);
        
        // === BRICKS ===
        spawn_bricks(world);

        // === WALLS ===
        spawn_walls(world);

        // === Camera ===
        spawn_camera(world);
        
        let game_manager_id = spawn_game_manager(world);

        let brick_breaker_system = BrickBreakerSystem::new(ball_id, paddle_id, game_manager_id, 0.0);
        systems_manager.add_system(brick_breaker_system, world);

        //Bindings
        if let Some(mapping) = world.resources.get_mut::<InputMapping>() {
            mapping
                .bind(KeyCode::KeyA,       InputBinding::continuous(InputAction::MoveLeft))
                .bind(KeyCode::ArrowLeft,  InputBinding::continuous(InputAction::MoveLeft))
                .bind(KeyCode::KeyD,       InputBinding::continuous(InputAction::MoveRight))
                .bind(KeyCode::ArrowRight, InputBinding::continuous(InputAction::MoveRight))
                .bind(KeyCode::Space,      InputBinding::one_shot(InputAction::Launch))
                .bind(KeyCode::Enter,      InputBinding::one_shot(InputAction::Confirm));
        }

        //For UI
        if let Some(renderer) = systems_manager.get_rendering_system() {
            //We set a closure 'similar to lambda' that executes every render frame and pulls the relevant data
            renderer.ui_draw_fn = Some(Box::new(move |ctx, world| {
            let state = match world.entity_manager
                .get_component::<BrickBreakerState>(game_manager_id) 
            {
                Some(s) => s,
                None => return,
            };

            // Style — dark transparent panels
            let mut style = (*ctx.style()).clone();
            style.visuals.panel_fill = egui::Color32::from_rgba_premultiplied(0, 0, 0, 180);
            style.visuals.override_text_color = Some(egui::Color32::WHITE);
            ctx.set_style(style);

            // Top panel — always visible
            egui::TopBottomPanel::top("hud").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.heading(format!("Score: {}", state.score));
                    ui.separator();
                    ui.heading(format!("Lives: {}", state.lives));
                });
            });

            match state.state {
                GameState::GameOver => {
                    egui::Window::new("game_over")
                        .title_bar(false)
                        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                        .resizable(false)
                        .collapsible(false)
                        .show(ctx, |ui| {
                            ui.vertical_centered(|ui| {
                                ui.heading("GAME OVER");
                                ui.add_space(10.0);
                                ui.label(format!("Final Score: {}", state.score));
                                ui.add_space(10.0);
                                ui.label("Press Enter to restart");
                            });
                        });
                }
                GameState::Won => {
                    egui::Window::new("game_won")
                        .title_bar(false)
                        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                        .resizable(false)
                        .collapsible(false)
                        .show(ctx, |ui| {
                            ui.vertical_centered(|ui| {
                                ui.heading("YOU WIN!");
                                ui.add_space(10.0);
                                ui.label(format!("Final Score: {}", state.score));
                                ui.add_space(10.0);
                                ui.label("Press Enter to restart");
                            });
                        });
                }
                GameState::WaitingToLaunch => {
                    egui::TopBottomPanel::bottom("launch_hint").show(ctx, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.label("Press Space to launch");
                        });
                    });
                }
                GameState::Playing | GameState::Resetting => {}
            }
        }));
        }
    }

    fn update(&mut self, _world: &mut World) {
        // Brickbreaker Updates go here
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = EngineConfig {
        window_title: "Brick breaker".to_string(),
        window_width: 830,
        window_height: 790,
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
            position: Vec3::new(-20.5, 0.0, 0.0),
            scale: Vec3::new(0.5, 40.0, 1.0),
            ..Default::default()
        })
        .with(Render {
            mesh: Mesh::new_quad_textured(),
            material_id: world.asset_manager
                .get_material_by_name("unlit_texture_orange"),
            gpu_cache: None,
        })
        .with(Collider {
            shape: ColliderShape::Box { width: 1.0, height: 1.0 },
            layer: LAYER_WALL,
            mask: LAYER_BALL,
            ..Default::default()
        })
        .build();

    // Right Wall
    EntityBuilder::new(&mut world.entity_manager)
        .with(Transform {
            position: Vec3::new(20.5, 0.0, 0.0),
            scale: Vec3::new(0.5, 40.0, 1.0),
            ..Default::default()
        })
        .with(Render {
            mesh: Mesh::new_quad_textured(),
            material_id: world.asset_manager
                .get_material_by_name("unlit_texture_orange"),
             gpu_cache: None,
        })
        .with(Collider {
            shape: ColliderShape::Box { width: 1.0, height: 1.0 },
            layer: LAYER_WALL,
            mask: LAYER_BALL,
            ..Default::default()
        })
        .build();

    //Top Wall
    EntityBuilder::new(&mut world.entity_manager)
        .with(Transform {
            position: Vec3::new(0.0, 19.5, 0.0),
            scale: Vec3::new(41.0, 0.5, 1.0),
            ..Default::default()
        })
        .with(Render {
            mesh: Mesh::new_quad_textured(),
            material_id: world.asset_manager
                .get_material_by_name("unlit_texture_orange"),
            gpu_cache: None,
        })
        .with(Collider {
            shape: ColliderShape::Box { width: 1.0, height: 1.0 },
            layer: LAYER_WALL,
            mask: LAYER_BALL,
            ..Default::default()
        })
        .build();
}

fn spawn_ball(world: &mut World) -> u32 {
    EntityBuilder::new(&mut world.entity_manager)
        .with(Transform {
            position: Vec3::new(0.0, -14.0, 0.0),  // Start at center
            scale: Vec3::new(1.25, 1.25, 1.0),
            ..Default::default()
        })
        .with(Render {
            mesh: Mesh::new_quad_textured(),
            material_id: world.asset_manager
                .get_material_by_name("unlit_texture_circle"),
             gpu_cache: None,
        })
        .with(RigidBody {
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,
            drag: 0.0,
            gravity_scale: 0.0,
            bounce: 1.0,
            is_kinematic: true,
            max_acceleration: 5.0,
            max_speed: 5.0
        })
        .with(Collider {
            shape: ColliderShape::Circle { radius: 0.5 },  // Smaller radius
            layer: LAYER_BALL,
            mask: LAYER_WALL | LAYER_PADDLE | LAYER_BRICK,
            ..Default::default()
        })
        .with(Ball)  // Mark as ball for game logic
        .build()
}

fn spawn_paddle(world: &mut World) -> u32 {
    // Spawn the paddle entity
    let paddle_id = EntityBuilder::new(&mut world.entity_manager)
        .with(Transform {
            position: Vec3::new(0.0, -17.0, 0.0),
            scale: Vec3::new(6.0, 0.6, 1.0),
            ..Default::default()
        })
        .with(Render {
            mesh: Mesh::new_quad_textured(),
            material_id: world.asset_manager
                .get_material_by_name("unlit_texture_green"),
             gpu_cache: None,
        })
        .with(Collider {
            shape: ColliderShape::Box { width: 1.0, height: 1.0 },
            layer: LAYER_PADDLE,
            mask: LAYER_BALL | LAYER_WALL,
            ..Default::default()
        })
        .with(RigidBody {
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,
            bounce: 0.0,
            gravity_scale: 0.0,
            is_kinematic: true,
            max_acceleration: 100.0,
            max_speed: 20.0,
            drag: 5.0
        })
        .with(Movement::new(100.0))
        .build();

    // Spawn a separate controller entity that possesses the paddle
    EntityBuilder::new(&mut world.entity_manager)
        .with(Controller::new(paddle_id))
        .build();

    paddle_id
}

fn spawn_bricks(world: &mut World) -> HashSet<u32> {
    // Option 1: Spawn a full grid
    let config = BrickGridConfig {
        rows: 5,
        columns: 10,
        brick_width: 3.0,
        brick_height: 1.2,
        spacing: 0.3,
        start_position: Vec3::new(0.0, 12.0, 0.0),
    };
    spawn_brick_grid(world, config)
}

fn spawn_camera(world: &mut World) {
    EntityBuilder::new(&mut world.entity_manager)
        .with(Transform::default())
        .with(Camera::new(20.0))
        .build();
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