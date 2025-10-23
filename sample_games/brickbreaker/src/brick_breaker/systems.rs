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

pub struct BrickBreakerSystem {
    pub ball_id: u32,
    pub paddle_id: u32,
    pub has_ball_launched: bool
}

impl BrickBreakerSystem {
    pub fn new(new_ball_id: u32, new_paddle_id: u32) -> Self {
        Self {
            ball_id: new_ball_id,
            paddle_id: new_paddle_id,
            has_ball_launched: false
        }
    }
}

impl System for BrickBreakerSystem {
    fn initialize(&mut self, _world: &mut World) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn update(&mut self, update_context: &mut SystemUpdateContext) {
        // Check if all bricks are destroyed
        let brick_entities: Vec<u32> = update_context.world.entity_manager
            .query_component::<Brick>();    

        //Check level finished
        if brick_entities.is_empty() {
            println!("Game won!");
        }

        //Let ball stick to the paddle
        if !self.has_ball_launched
        {
            let (option_paddle_transform, mut option_ball_transform) = update_context.world.entity_manager.get_two_components_mut::<Transform>(self.paddle_id, self.ball_id);

            // unwrap early
            let paddle_transform = option_paddle_transform.expect("Paddle transform missing");
            let ball_transform   = option_ball_transform.expect("Ball transform missing");


            ball_transform.position.x = paddle_transform.position.x;
            ball_transform.position.y = paddle_transform.position.y + 1.0; // place above paddle
        }

        // Check if ball fell off screen
        self.check_ball_death(update_context);
    }

    fn render(&mut self, _render_context: &mut SystemRenderContext) {
        // No rendering needed
    }

    fn as_event_listener_mut(&mut self) -> Option<&mut dyn EngineEventListener> {
        Some(self)
    }
}

impl BrickBreakerSystem {
    fn check_ball_death(&self, update_context: &mut SystemUpdateContext) {
        if let Some(transform) = update_context.world.entity_manager
                .get_component::<Transform>(self.ball_id) 
        {
            // Ball fell below paddle area
            if transform.position.y < -20.0 {
                println!("Ball lost! Lives remaining...");
                // TODO: Reset ball position or lose life
            }
        }
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
                println!("Collision detected: {} <-> {}", col.entity_a, col.entity_b);
                if col.entity_b != self.paddle_id && col.entity_a == self.ball_id {
                    actions.push(DestroyEntityAction { entity_id : col.entity_b })
                }
                else if col.entity_a == self.ball_id && col.entity_b == self.paddle_id {
                    actions.push(BallPaddleCollisionAction { ball_id : self.ball_id, paddle_id : self.paddle_id })
                }
                continue;
            }

            if let Some(key) = ev.as_any().downcast_ref::<KeyPressedEvent>() {
               if key.key == sdl2::keyboard::Keycode::Space && !self.has_ball_launched {
                    self.has_ball_launched = true;
                    println!("Launch ball");
                    actions.push(LaunchBallAction { ball_id : self.ball_id })
                    //Launch ball action
                }
                continue;
            }
        }
    }
}