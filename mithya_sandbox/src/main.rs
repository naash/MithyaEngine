// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use mithya_engine::{
    engine::{system::SystemsManager, Engine, EngineConfig, EntityBuilder, GameLogic, World},
    physics::{Collider, ColliderShape, RigidBody}, 
    player::Player, rendering::{Mesh, Render}, 
    Transform
};

use glam::{Quat, Vec3};

//Sandbox to test engine features
struct Sandbox {
    // Game-specific state
}

impl GameLogic for Sandbox {
    fn initialize(&mut self, world: &mut World, _systems_manager: &mut SystemsManager) {
       
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
