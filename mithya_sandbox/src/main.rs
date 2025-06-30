use mithya_engine::{
    core::EntityManager,
    engine::{Engine, EngineConfig, EntityBuilder, GameLogic},
    input::{input_manager::InputManager, system::movement_system},
    rendering::{mesh::Mesh, RenderingSystem}
};
use glam::{Vec3, Quat};

//Sandbox to test engine features
struct Sandbox {
    // Game-specific state
}

impl GameLogic for Sandbox {
    fn initialize(&mut self, entity_manager: &mut EntityManager, rendering_system: &mut RenderingSystem) {

        let _player = EntityBuilder::new(entity_manager)
            .with_transform(Vec3::ZERO, Quat::IDENTITY, Vec3::ONE)
            .with_render(
                Mesh::new_quad_textured(), 
                rendering_system.material_manager
                    .get_material_id_from_name("unlit_texture_default")
                    .copied()
            )
            .with_player_control()
            .build();
    }

    fn update(&mut self, input: &InputManager, entity_manager: &mut EntityManager) {
        // Updates go here
        movement_system(input, entity_manager);
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
