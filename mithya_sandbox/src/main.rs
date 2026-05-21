// Copyright (c) 2025 Nishant Sthalekar
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

mod click_to_move_system;
mod maze;

use glam::{Vec2, Vec3};

use mithya_engine::{
    asset::UniformValue,
    engine::{
        system::SystemsManager,
        Engine, EngineConfig, EntityBuilder, GameLogic, World,
    },
    rendering::{Camera, Mesh, Render},
    Controller, GridCell, Movement, NavAgent, NavBehavior, NavGrid, NavigationSystem, Transform,
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

        // Pawn start: room cell at maze position (8, 4) → grid cell (16, 8)
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

        let pawn_id = EntityBuilder::new(&mut world.entity_manager)
            .with(Transform {
                position: start_pos,
                scale: Vec3::new(cell_size, cell_size, 1.0),
                ..Default::default()
            })
            .with(Render { mesh: Mesh::new_quad_textured(), material_id: Some(pawn_material_id), gpu_cache: None })
            .with(NavAgent::new(start_cell))
            .with(Movement::new(3.0))
            .build();

        EntityBuilder::new(&mut world.entity_manager)
            .with(Controller::new(pawn_id, NavBehavior::new()))
            .build();

        systems_manager.add_system(NavigationSystem::new(), world);
        systems_manager.add_system(
            ClickToMoveSystem::new(pawn_id, Vec2::new(960.0, 540.0), 5.0),
            world,
        );
    }

    fn update(&mut self, _world: &mut World) {}
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
