// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use mithya_engine::{
    core::{EngineEventListener, EngineActionQueue, EngineAction, EngineEventQueue, Transform, KeyPressedEvent, DestroyEntityAction},
    engine::{
        system::{System, SystemRenderContext, SystemUpdateContext},
        World,
    },    
    physics::{RigidBody, systems::CollisionEvent},
};
use glam::Vec3;
use super::components::{Ball, Brick, BrickBreakerState};
use std::any::TypeId;
use winit::keyboard::KeyCode;

#[derive(Debug)]
pub struct LaunchBallAction {
    pub ball_id: u32,
}

impl EngineAction for LaunchBallAction {
    fn execute(&mut self, world: &mut World) {
        if let Some(mut rb) = world.entity_manager
                .get_component_mut::<RigidBody>(self.ball_id) {
            rb.velocity = Vec3::new(0.0, 35.0, 0.0);
            rb.is_kinematic = false;
        }
    }
}

#[derive(Debug)]
pub struct BallPaddleCollisionAction {
    pub ball_id: u32,
    pub paddle_id: u32,
}

impl EngineAction for BallPaddleCollisionAction {
    fn execute(&mut self, world: &mut World) {
        let entity_manager = &mut world.entity_manager;

        // Borrow both transforms in one immutable fetch
        let (paddle_t, ball_t) =
            entity_manager.get_two_components::<Transform>(self.paddle_id, self.ball_id);

        let paddle_t = paddle_t.expect("Paddle transform missing");
        let ball_t   = ball_t.expect("Ball transform missing");

        //Here scope of borrows end so we can use mut borrow again
        let dx = ball_t.position.x - paddle_t.position.x;

        let ball_rb = entity_manager.get_component_mut::<RigidBody>(self.ball_id)
            .expect("Ball rigidbody missing");

        // Apply new velocity
        ball_rb.velocity.x = dx * 2.0;
        ball_rb.velocity.y = ball_rb.velocity.y.abs(); // ensure ball goes up
    }
}

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

#[derive(Debug)]
pub struct LoseLifeAction {
    pub game_manager_id: u32,
}

impl EngineAction for LoseLifeAction {
    fn execute(&mut self, world: &mut World) {
        if let Some(state) = world.entity_manager
            .get_component_mut::<BrickBreakerState>(self.game_manager_id)
        {
            if state.lives > 0 {
                state.lives -= 1;
                println!("Lives remaining: {}", state.lives);
            }
            if state.lives == 0 {
                state.game_over = true;
                println!("Game over!");
            }
        }
    }
}

pub struct BrickBreakerSystem {
    pub ball_id: u32,
    pub paddle_id: u32,
    pub game_manager_id: u32,
    pub has_ball_launched: bool,
    pub paddle_start_x: f32,
}

impl BrickBreakerSystem {
    pub fn new(ball_id: u32, paddle_id: u32, game_manager_id: u32, paddle_start_x: f32) -> Self {
        Self {
            ball_id,
            paddle_id,
            game_manager_id,
            has_ball_launched: false,
            paddle_start_x,
        }
    }
}

impl System for BrickBreakerSystem {
    fn initialize(&mut self, _world: &mut World) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn update(&mut self, update_context: &mut SystemUpdateContext) {
        // Check game over state first, skip everything if game over
        let game_over = update_context.world.entity_manager
            .get_component::<BrickBreakerState>(self.game_manager_id)
            .map(|s| s.game_over)
            .unwrap_or(false);

        if game_over {
            return;
        }

        // Check win condition
        let brick_count = update_context.world.entity_manager
            .query_component::<Brick>()
            .len();

        if brick_count == 0 {
            println!("You win! Final score: {}", 
                update_context.world.entity_manager
                    .get_component::<BrickBreakerState>(self.game_manager_id)
                    .map(|s| s.score)
                    .unwrap_or(0)
            );
            if let Some(state) = update_context.world.entity_manager
                .get_component_mut::<BrickBreakerState>(self.game_manager_id)
            {
                state.game_over = true;
            }
            return;
        }

        // Stick ball to paddle before launch
        if !self.has_ball_launched {
            let (paddle_t, ball_t) = update_context.world.entity_manager
                .get_two_components_mut::<Transform>(self.paddle_id, self.ball_id);
            let paddle_t = paddle_t.expect("Paddle transform missing");
            let ball_t = ball_t.expect("Ball transform missing");
            ball_t.position.x = paddle_t.position.x;
            ball_t.position.y = paddle_t.position.y + 1.0;
            return;
        }

        // Check ball death
        let ball_y = update_context.world.entity_manager
            .get_component::<Transform>(self.ball_id)
            .map(|t| t.position.y)
            .unwrap_or(0.0);

        if ball_y < -20.0 {
            self.has_ball_launched = false;

            // Reset ball
            if let Some(rb) = update_context.world.entity_manager
                .get_component_mut::<RigidBody>(self.ball_id)
            {
                rb.velocity = Vec3::ZERO;
                rb.is_kinematic = true;
            }

            // Lose a life
            if let Some(state) = update_context.world.entity_manager
                .get_component_mut::<BrickBreakerState>(self.game_manager_id)
            {
                if state.lives > 0 {
                    state.lives -= 1;
                    println!("Ball lost! Lives remaining: {}", state.lives);
                }
                if state.lives == 0 {
                    state.game_over = true;
                    println!("Game over! Final score: {}", state.score);
                }
            }
        }
    }

    fn render(&mut self, _render_context: &mut SystemRenderContext) {
        // No rendering needed
    }

    fn as_event_listener_mut(&mut self) -> Option<&mut dyn EngineEventListener> {
        Some(self)
    }
}

impl EngineEventListener for BrickBreakerSystem {
    fn interested_events(&self) -> Vec<TypeId> {
        vec![TypeId::of::<CollisionEvent>(), TypeId::of::<KeyPressedEvent>()]
    }

    fn on_events(
        &mut self,
        events: &EngineEventQueue,
        actions: &mut EngineActionQueue,
    ) {
        for ev in events.iter() {
            let ev = ev.as_ref();

            if let Some(col) = ev.as_any().downcast_ref::<CollisionEvent>() {
                if col.entity_a == self.ball_id && col.entity_b != self.paddle_id {
                    actions.push(DestroyEntityAction { entity_id: col.entity_b });
                    actions.push(AddScoreAction { 
                        game_manager_id: self.game_manager_id, 
                        points: 10  // fixed for now, improve later
                    });
                } else if col.entity_a == self.ball_id && col.entity_b == self.paddle_id {
                    actions.push(BallPaddleCollisionAction { 
                        ball_id: self.ball_id, 
                        paddle_id: self.paddle_id 
                    });
                }
            }

            if let Some(key) = ev.as_any().downcast_ref::<KeyPressedEvent>() {
                if key.key == winit::keyboard::KeyCode::Space && !self.has_ball_launched {
                    self.has_ball_launched = true;
                    println!("Launch ball");
                    actions.push(LaunchBallAction { ball_id: self.ball_id });
                }
                continue;
            }
        }
    }
}