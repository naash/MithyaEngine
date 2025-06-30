use crate::{
    components::*,
    core::EntityManager,
    engine::{Engine, EngineConfig, GameLogic, EntityBuilder},
    input::{managers::InputManager, systems::movement_system},
    rendering::{mesh::Mesh, RenderingSystem},
};
use glam::{Vec3, Quat};

pub struct MithyaGame;

//Old code for reference before refactor, displays triangle and movable quad
impl GameLogic for MithyaGame {
    fn initialize(&mut self, entity_manager: &mut EntityManager, rendering_system: &mut RenderingSystem) {
        
        let _player_entity = EntityBuilder::new(entity_manager)
            .with_transform(
                Vec3::new(0.0, 0.0, 0.0),
                Quat::IDENTITY,
                Vec3::new(1.0, 1.0, 1.0)
            )
            .with_render(
                Mesh::new_quad_textured(),
                rendering_system.material_manager
                    .get_material_id_from_name("unlit_texture_default")
                    .copied()
            )
            .with_player_control()
            .build();

        let _triangle_entity = EntityBuilder::new(entity_manager)
            .with_transform(
                Vec3::new(0.0, 10.0, 0.0),
                Quat::IDENTITY,
                Vec3::new(1.0, 1.0, 1.0)
            )
            .with_render(
                Mesh::new_triangle(),
                rendering_system.material_manager
                    .get_material_id_from_name("unlit_color")
                    .copied()
            )
            .build();
    }

    fn update(&mut self, input: &InputManager, entity_manager: &mut EntityManager) {
        movement_system(input, entity_manager);
    }
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config = EngineConfig {
        window_title: "Mithya Engine".to_string(),
        window_width: 2048,
        window_height: 1024,
        ..Default::default()
    };

    let engine = Engine::new(config)?;
    let game = MithyaGame;
    
    engine.run(game)
}