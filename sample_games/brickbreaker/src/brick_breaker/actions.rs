// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use mithya_engine::{
    core::{NamedEngineAction, Transform},
    engine::World,
    physics::RigidBody,
};
use glam::{Vec2, Vec3};
use crate::brick_breaker::components::GameState;

use super::components::{Brick, BrickBreakerState};

//Action added when space is pressed
#[derive(Debug)]
pub struct LaunchBallAction {
    pub ball_id: u32,
    pub game_manager_id: u32
}

impl NamedEngineAction for LaunchBallAction {
    fn execute(self: Box<Self>, world: &mut World) {
        // Set game state to playing
        if let Some(state) = world.entity_manager
            .get_component_mut::<BrickBreakerState>(self.game_manager_id)
        {
            if state.state != GameState::WaitingToLaunch {
                return;  // only launch if waiting
            }
            state.state = GameState::Playing;
        }

        if let Some(rb) = world.entity_manager
            .get_component_mut::<RigidBody>(self.ball_id)
        {
            rb.is_kinematic = false;
            rb.velocity = Vec3::new(0.0, 5.0, 0.0);
        }
    }
}

//Actions when paddle collides
#[derive(Debug)]
pub struct BallPaddleCollisionAction {
    pub ball_id: u32,
    pub paddle_id: u32,
}

impl NamedEngineAction for BallPaddleCollisionAction {
    fn execute(self: Box<Self>, world: &mut World) {
        let entity_manager = &mut world.entity_manager;
        let (paddle_t, ball_t) =
            entity_manager.get_two_components::<Transform>(self.paddle_id, self.ball_id);
        let paddle_t = paddle_t.expect("Paddle transform missing");
        let ball_t = ball_t.expect("Ball transform missing");
        
        let hit_offset = ball_t.position.x - paddle_t.position.x;

        let ball_rb = entity_manager.get_component_mut::<RigidBody>(self.ball_id)
            .expect("Ball rigidbody missing");

        let speed = Vec2::new(ball_rb.velocity.x, ball_rb.velocity.y).length();
        let direction = Vec2::new(hit_offset, 1.0).normalize();
        ball_rb.set_velocity((direction * speed).extend(ball_rb.velocity.z));
    }
}

//Action when brick is destroyed
#[derive(Debug)]
pub struct DestroyBrickAction {
    pub brick_id: u32,
    pub game_manager_id: u32,
}

impl NamedEngineAction for DestroyBrickAction {
    fn execute(self: Box<Self>, world: &mut World) {

         // Apply damage
        if let Some(brick) = world.entity_manager
            .get_component_mut::<Brick>(self.brick_id)
        {
            brick.take_damage();
        }

        // Check destruction and get points in one call
        let (is_destroyed, points) = world.entity_manager
            .get_component::<Brick>(self.brick_id)
            .map(|b| (b.is_destroyed(), b.points))
            .unwrap_or((false, 0));

        if is_destroyed {
            if let Some(state) = world.entity_manager
                .get_component_mut::<BrickBreakerState>(self.game_manager_id)
            {
                state.score += points;
            }

            world.entity_manager.destroy_entity(self.brick_id);
        }
    }
}

//Action when game resets
#[derive(Debug)]
pub struct ResetGameAction {
    pub game_manager_id: u32
}

impl NamedEngineAction for ResetGameAction {
    fn execute(self: Box<Self>, world: &mut World) {
        
        //Only do the action if state is valid
        if let Some(state) = world.entity_manager
            .get_component_mut::<BrickBreakerState>(self.game_manager_id)
        {
            if state.state == GameState::Playing {
                return;  // only reset not playing
            }

            state.state = GameState::Resetting;
        }
    }
}