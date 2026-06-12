// Copyright (c) 2025 Nishant Sthalekar
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

mod click_to_move_system;
mod maze;

use glam::Vec3;
use winit::keyboard::KeyCode;

use mithya_engine::{
    engine::{
        system::SystemsManager,
        Engine, EngineConfig, EntityBuilder, GameLogic, World,
    },
    input::{InputAction, InputBinding, InputMapping},
    rendering::{Camera, Mesh, Render},
    NavMovementSystem, PlayerControlled, PlayerInputSystem, RandomMovement, RandomMovementSystem,
    GridCell, Movement, NavAgent, NavGrid, NavigationSystem, NavGridDebugSystem, Transform,
};

use click_to_move_system::ClickToMoveSystem;

struct Sandbox {}

impl GameLogic for Sandbox {
    fn initialize(&mut self, world: &mut World, systems_manager: &mut SystemsManager) {
        let cell_size = 0.5_f32;
        let cols = 34_u32;
        let rows = 20_u32;

        EntityBuilder::new(&mut world.entity_manager)
            .with(Transform::default())
            .with(Camera::new(5.0))
            .build();

        world.resources.insert(NavGrid::new(
            cols, rows, cell_size,
            Vec3::new(-(cols as f32 * cell_size) / 2.0, (rows as f32 * cell_size) / 2.0, 0.0),
        ));

        maze::spawn_maze(world, cell_size, cols, rows);

        if let Some(mapping) = world.resources.get_mut::<InputMapping>() {
            mapping.bind(KeyCode::ArrowLeft,  InputBinding::continuous(InputAction::MoveLeft));
            mapping.bind(KeyCode::ArrowRight, InputBinding::continuous(InputAction::MoveRight));
            mapping.bind(KeyCode::ArrowUp,    InputBinding::continuous(InputAction::MoveUp));
            mapping.bind(KeyCode::ArrowDown,  InputBinding::continuous(InputAction::MoveDown));
        }

        let start_cell = GridCell::new(16, 8);
        let start_pos = world.resources.get::<NavGrid>().unwrap().cell_to_world(start_cell);

        if let Some(renderer) = systems_manager.get_rendering_system() {
            renderer.load_assets(&mut world.asset_manager, |assets, device, queue| {
                assets
                    .load_texture_for_material("pacman", "pacman.png", device, queue)
                    .expect("Failed to load pacman.png");
            });
        }

        let pawn_material_id = world.asset_manager.load_material("pacman").unwrap();

        // Nav-driven pawn (click to move)
        let pawn_id = EntityBuilder::new(&mut world.entity_manager)
            .with(Transform {
                position: start_pos,
                scale: Vec3::new(cell_size, cell_size, 1.0),
                ..Default::default()
            })
            .with(Render { mesh: Mesh::new_quad_textured(), material_id: Some(pawn_material_id), gpu_cache: None, tint: None })
            .with(NavAgent::new(start_cell, Some(0.05)))
            .with(Movement::new(3.0))
            .build();

        // Player-controlled entity (arrow keys)
        EntityBuilder::new(&mut world.entity_manager)
            .with(Transform {
                position: Vec3::new(1.0, 1.0, 0.0),
                scale: Vec3::new(cell_size, cell_size, 1.0),
                ..Default::default()
            })
            .with(Render { mesh: Mesh::new_quad_textured(), material_id: Some(pawn_material_id), gpu_cache: None, tint: None })
            .with(PlayerControlled)
            .with(Movement::new(3.0))
            .build();

        // Random-movement entities — use grid cells near the pawn start so positions are guaranteed walkable
        let random_cell_1 = GridCell::new(12, 8);
        let random_pos_1 = world.resources.get::<NavGrid>().unwrap().cell_to_world(random_cell_1);
        EntityBuilder::new(&mut world.entity_manager)
            .with(Transform {
                position: random_pos_1,
                scale: Vec3::new(cell_size, cell_size, 1.0),
                ..Default::default()
            })
            .with(Render { mesh: Mesh::new_quad_textured(), material_id: Some(pawn_material_id), gpu_cache: None, tint: None })
            .with(NavAgent::new(random_cell_1, Some(0.05)))
            .with(RandomMovement::new())
            .with(Movement::new(2.0))
            .build();

        let random_cell_2 = GridCell::new(20, 8);
        let random_pos_2 = world.resources.get::<NavGrid>().unwrap().cell_to_world(random_cell_2);
        EntityBuilder::new(&mut world.entity_manager)
            .with(Transform {
                position: random_pos_2,
                scale: Vec3::new(cell_size, cell_size, 1.0),
                ..Default::default()
            })
            .with(Render { mesh: Mesh::new_quad_textured(), material_id: Some(pawn_material_id), gpu_cache: None, tint: None })
            .with(NavAgent::new(random_cell_2, Some(0.05)))
            .with(RandomMovement::new())
            .with(Movement::new(2.0))
            .build();

        systems_manager.add_system(NavigationSystem::new(), world);
        systems_manager.add_system(NavMovementSystem, world);
        systems_manager.add_system(PlayerInputSystem::new(), world);
        systems_manager.add_system(RandomMovementSystem, world);
        systems_manager.add_system(ClickToMoveSystem::new(pawn_id), world);
        systems_manager.add_system(NavGridDebugSystem::new(), world);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = EngineConfig {
        window_title: "Sandbox".to_string(),
        window_width: 960,
        window_height: 540,
        asset_root: std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")),
        ..Default::default()
    };
    Engine::new(config, Sandbox {}).run()
}
