// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use mithya_engine::{
    engine::{system::SystemsManager, Engine, EngineConfig, GameLogic, World},
};

//Sandbox to test engine features
struct Sandbox {
    // Game-specific state
}

impl GameLogic for Sandbox {
    fn initialize(&mut self, _world: &mut World, _systems_manager: &mut SystemsManager) {
       
    }

    fn update(&mut self, _world: &mut World, _delta_time: f32) {
        // Sandbox Updates go here
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

    let game = Sandbox { /* initialize game state */ };

    let engine = Engine::new(config, game);

    engine.run()
}
