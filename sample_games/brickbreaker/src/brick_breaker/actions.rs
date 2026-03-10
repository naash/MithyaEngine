// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use mithya_engine::{
    core::{EngineAction, Transform, DestroyEntityAction},
    engine::World,
    physics::RigidBody,
};
use glam::Vec3;
use super::components::{Brick, BrickBreakerState};
use super::brick_spawner::{spawn_brick_grid};

//Action added when space is pressed
#[derive(Debug)]
pub struct LaunchBallAction {
    pub ball_id: u32,
}

impl EngineAction for LaunchBallAction {
    fn execute(&mut self, world: &mut World) {
        if let Some(rb) = world.entity_manager.get_component_mut::<RigidBody>(self.ball_id) {
            rb.velocity = Vec3::new(0.0, 5.0, 0.0);
            rb.is_kinematic = false;
        }
    }
}

//Actions when paddle collides
#[derive(Debug)]
pub struct BallPaddleCollisionAction {
    pub ball_id: u32,
    pub paddle_id: u32,
}

impl EngineAction for BallPaddleCollisionAction {
    fn execute(&mut self, world: &mut World) {
        let entity_manager = &mut world.entity_manager;
        let (paddle_t, ball_t) =
            entity_manager.get_two_components::<Transform>(self.paddle_id, self.ball_id);
        let paddle_t = paddle_t.expect("Paddle transform missing");
        let ball_t = ball_t.expect("Ball transform missing");
        let dx = ball_t.position.x - paddle_t.position.x;
        let ball_rb = entity_manager.get_component_mut::<RigidBody>(self.ball_id)
            .expect("Ball rigidbody missing");
        ball_rb.velocity.x = dx * 2.0;
        ball_rb.velocity.y = ball_rb.velocity.y.abs();
    }
}

//Action when ball resets
#[derive(Debug)]
pub struct ResetBallAction {
    pub ball_id: u32,
}

impl EngineAction for ResetBallAction {
    fn execute(&mut self, world: &mut World) {
        if let Some(rb) = world.entity_manager.get_component_mut::<RigidBody>(self.ball_id) {
            rb.velocity = Vec3::ZERO;
            rb.is_kinematic = true;
        }
    }
}

//Action when score increases
#[derive(Debug)]
pub struct AddScoreAction {
    pub game_manager_id: u32,
    pub points: u32,
}

impl EngineAction for AddScoreAction {
    fn execute(&mut self, world: &mut World) {
        if let Some(state) = world.entity_manager
            .get_component_mut::<BrickBreakerState>(self.game_manager_id)
        {
            state.score += self.points;
            println!("Score: {}", state.score);
        }
    }
}

//Action when brick is destroyed
#[derive(Debug)]
pub struct BrickDestroyedAction {
    pub entity_id: u32,
    pub game_manager_id: u32,
}

impl EngineAction for BrickDestroyedAction {
    fn execute(&mut self, world: &mut World) {
        let points = world.entity_manager
            .get_component::<Brick>(self.entity_id)
            .map(|b| b.points)
            .unwrap_or(10);

        if let Some(state) = world.entity_manager
            .get_component_mut::<BrickBreakerState>(self.game_manager_id)
        {
            state.score += points;
            println!("Brick destroyed! +{} points. Score: {}", points, state.score);
        }

        world.entity_manager.destroy_entity(self.entity_id);
    }
}

//Action when game resets
#[derive(Debug)]
pub struct ResetGameAction {
    pub game_manager_id: u32,
    pub ball_id: u32
}

impl EngineAction for ResetGameAction {
    fn execute(&mut self, world: &mut World) {
        // Reset ball
        if let Some(rb) = world.entity_manager.get_component_mut::<RigidBody>(self.ball_id) {
            rb.velocity = Vec3::ZERO;
            rb.is_kinematic = true;
        }

        // Reset game state
        if let Some(state) = world.entity_manager
            .get_component_mut::<BrickBreakerState>(self.game_manager_id)
        {
            state.score = 0;
            state.lives = 3;
            state.game_over = false;
            state.needs_reset = true;
            println!("Game reset! Press Space to launch.");
        }
    }
}