// Copyright (c) 2025 Nishant Sthalekar
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use glam::Vec2;
use crate::{core::EntityId, input::InputAction};

/// Pluggable logic that computes a movement intent each frame.
///
/// Implement this to add new controller types (player, AI, LLM, etc.)
/// without changing ControllerSystem.
pub trait ControllerBehavior: Send + Sync {
    /// Called with buffered input actions — player-driven behaviors use this.
    fn on_input_actions(&mut self, _actions: &[InputAction]) {}

    /// Called with intent derived from world state — nav/AI/LLM behaviors use this.
    /// ControllerSystem reads NavAgent.move_input (or equivalent) and passes it here.
    fn on_world_intent(&mut self, _intent: Vec2) {}

    /// Returns the final movement intent for this frame.
    fn compute_intent(&mut self) -> Vec2;
}

// --- Player ---

/// Driven by player input events.
pub struct PlayerBehavior {
    pending_dx: f32,
    pending_dy: f32,
}

impl PlayerBehavior {
    pub fn new() -> Self {
        Self { pending_dx: 0.0, pending_dy: 0.0 }
    }
}

impl ControllerBehavior for PlayerBehavior {
    fn on_input_actions(&mut self, actions: &[InputAction]) {
        for action in actions {
            match action {
                InputAction::MoveLeft  => self.pending_dx -= 1.0,
                InputAction::MoveRight => self.pending_dx += 1.0,
                InputAction::MoveUp    => self.pending_dy += 1.0,
                InputAction::MoveDown  => self.pending_dy -= 1.0,
                _ => {}
            }
        }
    }

    fn compute_intent(&mut self) -> Vec2 {
        let intent = Vec2::new(self.pending_dx, self.pending_dy);
        self.pending_dx = 0.0;
        self.pending_dy = 0.0;
        intent
    }
}

// --- Nav ---

/// Driven by NavAgent.move_input — fed by ControllerSystem each frame.
pub struct NavBehavior {
    current_intent: Vec2,
}

impl NavBehavior {
    pub fn new() -> Self {
        Self { current_intent: Vec2::ZERO }
    }
}

impl ControllerBehavior for NavBehavior {
    fn on_world_intent(&mut self, intent: Vec2) {
        self.current_intent = intent;
    }

    fn compute_intent(&mut self) -> Vec2 {
        self.current_intent
    }
}

pub struct Controller {
    pub possessed_entity_id: EntityId,
    pub behavior: Box<dyn ControllerBehavior>,
}

impl Controller {
    pub fn new(possessed_entity_id: EntityId, behavior: impl ControllerBehavior + 'static) -> Self {
        Self {
            possessed_entity_id,
            behavior: Box::new(behavior),
        }
    }
}
