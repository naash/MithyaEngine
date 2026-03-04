// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use mithya_engine::{
    core::{EngineEventListener, EngineEvent, EngineActionQueue, EngineAction, EngineEventQueue, Transform, KeyPressedEvent, DestroyEntityAction},
    engine::{
        system::{System, SystemRenderContext, SystemUpdateContext},
        World,
    },    
    physics::{RigidBody, systems::CollisionEvent},
};

use super::actions::{
    LaunchBallAction, BallPaddleCollisionAction,
    ResetGameAction, AddScoreAction,
};

use glam::Vec3;
use super::components::{Ball, Brick, BrickBreakerState};
use crate::brick_breaker::brick_spawner::*;
use std::any::{TypeId, Any};
use winit::keyboard::KeyCode;

pub struct BrickBreakerSystem {
    pub ball_id: u32,
    pub paddle_id: u32,
    pub game_manager_id: u32,
    pub has_ball_launched: bool,
    pub paddle_start_x: f32
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

    fn do_reset(&mut self, update_context: &mut SystemUpdateContext) {
        // Destroy remaining bricks
        let bricks = update_context.world.entity_manager.query_component::<Brick>();
        for brick_id in bricks {
            update_context.world.entity_manager.destroy_entity(brick_id);
        }

        // Respawn bricks
        let config = BrickGridConfig {
            rows: 5,
            columns: 10,
            brick_width: 3.0,
            brick_height: 1.5,
            spacing: 0.2,
            start_position: Vec3::new(0.0, 10.0, 0.0),
        };
        spawn_brick_grid(update_context.world, config);

        // Reset ball
        if let Some(rb) = update_context.world.entity_manager
            .get_component_mut::<RigidBody>(self.ball_id)
        {
            rb.velocity = Vec3::ZERO;
            rb.is_kinematic = true;
        }

        // Reset state
        if let Some(state) = update_context.world.entity_manager
            .get_component_mut::<BrickBreakerState>(self.game_manager_id)
        {
            state.score = 0;
            state.lives = 3;
            state.game_over = false;
            state.needs_reset = false;
            println!("Game reset! Press Space to launch.");
        }

        self.has_ball_launched = false;
    }
}

impl System for BrickBreakerSystem {
    fn initialize(&mut self, _world: &mut World) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn update(&mut self, update_context: &mut SystemUpdateContext) {

        let state = update_context.world.entity_manager.get_component::<BrickBreakerState>(self.game_manager_id);

        if let Some(state) = state {
            if state.game_over {
                return;
            }
            if state.needs_reset {
                self.do_reset(update_context);
                return;
            }
        }

        // Check win condition
        let brick_count = update_context.world.entity_manager
            .query_component::<Brick>()
            .len();

        if brick_count == 0 {
            if let Some(state) = update_context.world.entity_manager
                .get_component_mut::<BrickBreakerState>(self.game_manager_id)
            {
                println!("You win! Final score: {}", state.score);
                state.game_over = true;
                println!("Game over! Press Enter to restart");
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
            if let Some(state) = update_context.world.entity_manager
                .get_component_mut::<BrickBreakerState>(self.game_manager_id)
            {
                // Lose a life
                if state.lives > 0 {
                    state.lives -= 1;
                    println!("Lives remaining: {}", state.lives);
                }
                if state.lives == 0 {
                    state.game_over = true;
                    println!("Game Over! Final score: {}", state.score);
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
        vec![TypeId::of::<CollisionEvent>(), 
        TypeId::of::<KeyPressedEvent>()]
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
                    actions.push(LaunchBallAction { ball_id: self.ball_id });
                }
                if key.key == KeyCode::Enter {
                    self.has_ball_launched = false;
                    actions.push(ResetGameAction {
                        ball_id: self.ball_id,
                        game_manager_id: self.game_manager_id,
                    });
                }
            }
        }
    }
}