// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use mithya_engine::{
    core::{
        EngineEventListener, EngineActionQueue, EngineEventQueue, 
        Transform, DestroyEntityAction
    },
    engine::{
        system::{System, SystemRenderContext, SystemUpdateContext},
        World,
    },
    input::{
        actions::InputAction,
        InputActionEvent
    },
    physics::{RigidBody, systems::CollisionEvent},
};

use super::actions::{
    LaunchBallAction, ResetGameAction, AddScoreAction,
};

use glam::Vec3;
use super::components::{Brick, BrickBreakerState};
use crate::brick_breaker::{actions::BallPaddleCollisionAction, brick_spawner::*};
use std::{any::TypeId, collections::HashSet};

pub struct BrickBreakerSystem {
    pub ball_id: u32,
    pub paddle_id: u32,
    pub game_manager_id: u32,
    pub has_ball_launched: bool,
    pub paddle_start_x: f32,
    pub brick_ids: HashSet<u32>
}

impl BrickBreakerSystem {
    pub fn new(ball_id: u32, paddle_id: u32, game_manager_id: u32, paddle_start_x: f32, brick_ids: HashSet<u32>) -> Self {
        Self {
            ball_id,
            paddle_id,
            game_manager_id,
            has_ball_launched: false,
            paddle_start_x,
            brick_ids
        }
    }

    fn do_reset(&mut self, update_context: &mut SystemUpdateContext) {
        // Destroy remaining bricks
        for brick_id in self.brick_ids.clone() {
            update_context.world.entity_manager.destroy_entity(brick_id);
        }

        self.brick_ids.clear();

        // Respawn bricks
        let config = BrickGridConfig {
            rows: 5,
            columns: 10,
            brick_width: 3.0,
            brick_height: 1.5,
            spacing: 0.2,
            start_position: Vec3::new(0.0, 10.0, 0.0),
        };
        self.brick_ids = spawn_brick_grid(update_context.world, config);

        // Reset ball
        if let Some(rb) = update_context.world.entity_manager
            .get_component_mut::<RigidBody>(self.ball_id)
        {
            rb.velocity = Vec3::ZERO;
            rb.is_kinematic = true;
        }

        //Reset paddle
        if let Some(rb) = update_context.world.entity_manager
            .get_component_mut::<RigidBody>(self.paddle_id)
        {
            rb.velocity = Vec3::ZERO;
            rb.acceleration = Vec3::ZERO;
        }

        if let Some(transform) = update_context.world.entity_manager
            .get_component_mut::<Transform>(self.paddle_id)
        {
            transform.position.x = self.paddle_start_x;
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

            self.has_ball_launched = false;
        }
    }

    fn as_event_listener_mut(&mut self) -> Option<&mut dyn EngineEventListener> {
        Some(self)
    }
}

impl EngineEventListener for BrickBreakerSystem {
    fn interested_events(&self) -> Vec<TypeId> {
        vec![TypeId::of::<CollisionEvent>(), 
        TypeId::of::<InputActionEvent>()]
    }

    fn on_events(
        &mut self,
        events: &EngineEventQueue,
        actions: &mut EngineActionQueue,
    ) {
        for ev in events.iter() {
            let ev = ev.as_ref();
            if let Some(col) = ev.as_any().downcast_ref::<CollisionEvent>() {
                // Check if ball is involved
                    let is_ball_a = col.entity_a == self.ball_id;
                    let is_ball_b = col.entity_b == self.ball_id;
                    if !is_ball_a && !is_ball_b {
                        continue;
                    }

                    let other_id = if is_ball_a { col.entity_b } else { col.entity_a };

                    if other_id == self.paddle_id {
                        actions.push(BallPaddleCollisionAction {
                            ball_id: self.ball_id,
                            paddle_id: self.paddle_id,
                        });
                    } else if self.brick_ids.contains(&other_id) {
                        actions.push(DestroyEntityAction { entity_id: other_id });
                        actions.push(AddScoreAction {
                            game_manager_id: self.game_manager_id,
                            points: 10,
                        });
                        self.brick_ids.remove(&other_id);
                    }
            }
            if let Some(input) = ev.as_any().downcast_ref::<InputActionEvent>() {
                match input.action {
                    InputAction::Launch => {
                        if !self.has_ball_launched {
                            self.has_ball_launched = true;
                            actions.push(LaunchBallAction { ball_id: self.ball_id });
                        }
                    }
                    InputAction::Confirm => {
                        self.has_ball_launched = false;
                        actions.push(ResetGameAction {
                            ball_id: self.ball_id,
                            game_manager_id: self.game_manager_id,
                        });
                    }
                    _ => {}
                }
            }
        }
    }
}