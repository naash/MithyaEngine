// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use mithya_engine::{
    core::{
        EngineEventListener, EngineActionQueue, EngineEventQueue, 
        Transform
    },
    engine::{
        system::{System, SystemUpdateContext},
        World,
    },
    input::{
        mapping::InputAction,
        InputActionEvent
    },
    physics::{RigidBody, systems::CollisionEvent},
};

use super::actions::{
    LaunchBallAction, ResetGameAction
};

use glam::Vec3;
use super::components::{
    Brick, 
    BrickBreakerState
};
use crate::brick_breaker::{
    actions::{
        BallPaddleCollisionAction, 
        DestroyBrickAction
    }, 
    brick_spawner::*, 
    components::GameState
};
use std::any::TypeId;

pub struct BrickBreakerSystem {
    pub ball_id: u32,
    pub paddle_id: u32,
    pub game_manager_id: u32,
    pub paddle_start_x: f32
}

impl BrickBreakerSystem {
    pub fn new(ball_id: u32, paddle_id: u32, game_manager_id: u32, paddle_start_x: f32) -> Self {
        Self {
            ball_id,
            paddle_id,
            game_manager_id,
            paddle_start_x
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
            start_position: Vec3::new(0.0, 12.0, 0.0),
        };
        
        spawn_brick_grid(update_context.world, config);

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
            state.state = GameState::WaitingToLaunch;
        }
    }
}

impl System for BrickBreakerSystem {
    fn initialize(&mut self, _world: &mut World) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn update(&mut self, update_context: &mut SystemUpdateContext) {

        // Read current game state
        let game_state = update_context.world.entity_manager
            .get_component::<BrickBreakerState>(self.game_manager_id)
            .map(|s| s.state.clone());

        match game_state {
            Some(GameState::GameOver) | Some(GameState::Won) => return,
            Some(GameState::Resetting) => {
                self.do_reset(update_context);
                return;
            }
            Some(GameState::WaitingToLaunch) => {
                // Stick ball to paddle
                let (paddle_t, ball_t) = update_context.world.entity_manager
                    .get_two_components_mut::<Transform>(self.paddle_id, self.ball_id);
                let paddle_t = paddle_t.expect("Paddle transform missing");
                let ball_t = ball_t.expect("Ball transform missing");
                ball_t.position.x = paddle_t.position.x;
                ball_t.position.y = paddle_t.position.y + 1.0;
                return;
            }
            Some(GameState::Playing) => {}
            None => return,
        }

        // Check win condition
        let brick_count = update_context.world.entity_manager
            .query_component::<Brick>()
            .len();

        if brick_count == 0 {
            if let Some(state) = update_context.world.entity_manager
                .get_component_mut::<BrickBreakerState>(self.game_manager_id)
            {
                state.state = GameState::Won;
            }
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
                if state.lives > 0 {
                    state.lives -= 1;
                }
                if state.lives == 0 {
                    state.state = GameState::GameOver;
                } else {
                    state.state = GameState::WaitingToLaunch;
                }
            }
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
        world: &World
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
                        actions.push_named(BallPaddleCollisionAction {
                            ball_id: self.ball_id,
                            paddle_id: self.paddle_id,
                        });
                    } else if world.entity_manager.get_component::<Brick>(other_id).is_some() {
                        actions.push_named(DestroyBrickAction {
                            brick_id: other_id,
                            game_manager_id: self.game_manager_id,
                        });
                    }
            }
            //Button actions to launch ball and reset game
            if let Some(input) = ev.as_any().downcast_ref::<InputActionEvent>() {
                match input.action {
                InputAction::Launch => {
                    actions.push_named(LaunchBallAction { 
                        ball_id: self.ball_id,
                        game_manager_id: self.game_manager_id,
                    });
                }
                InputAction::Confirm => {
                    actions.push_named(ResetGameAction {
                        game_manager_id: self.game_manager_id,
                    });
                }
                _ => {}
                }
            }
        }
    }
}