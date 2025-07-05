// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use mithya_engine::{
    engine::{Engine, EngineConfig, EntityBuilder, GameLogic, World},
    rendering::{mesh::Mesh}
};
use glam::{Quat, Vec2, Vec3};

//Sandbox to test engine features
struct Sandbox {
    // Game-specific state
}

impl GameLogic for Sandbox {
    fn initialize(&mut self, world: &mut World) {

        let _player = EntityBuilder::new(&mut world.entity_manager)
            .with_transform(Vec3 { x: (0.0), y: (20.0), z: (0.0) }, Quat::IDENTITY, Vec3::ONE)
            .with_render(
                Mesh::new_quad_textured(), 
                world.rendering_system.material_manager
                    .get_material_id_from_name("unlit_texture_default")
                    .copied()
            )
            .with_player_control()
            .with_box_collider(10.0, 10.0)
            .with_rigidbody(Vec2::ZERO)
            .build();

        let _floor = EntityBuilder::new(&mut world.entity_manager)
            .with_transform(Vec3 { x: (0.0), y: (-15.0), z: (0.0) }, Quat::IDENTITY, Vec3::ONE * 1.0)
            .with_render(
                Mesh::new_quad_textured(), 
                world.rendering_system.material_manager
                    .get_material_id_from_name("unlit_texture_default")
                    .copied()
            )
            .with_box_collider(10.0, 10.0)
            .build();
    }

    fn update(&mut self, _world: &mut World) {
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
