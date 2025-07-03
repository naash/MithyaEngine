use mithya_engine::{
    engine::{Engine, EngineConfig, EntityBuilder, GameLogic, World},
    rendering::{mesh::Mesh}
};
use glam::{Vec3, Quat};

//Sandbox to test engine features
struct Sandbox {
    // Game-specific state
}

impl GameLogic for Sandbox {
    fn initialize(&mut self, world: &mut World) {

        let _player = EntityBuilder::new(&mut world.entity_manager)
            .with_transform(Vec3::ZERO, Quat::IDENTITY, Vec3::ONE)
            .with_render(
                Mesh::new_quad_textured(), 
                world.rendering_system.material_manager
                    .get_material_id_from_name("unlit_texture_default")
                    .copied()
            )
            .with_player_control()
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
