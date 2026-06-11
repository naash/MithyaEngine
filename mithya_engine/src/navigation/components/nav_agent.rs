// Copyright (c) 2025 Nishant Sthalekar
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use glam::{Vec2, Vec3};
use crate::navigation::{grid_cell::GridCell, resources::nav_grid::NavGrid};

/// Component attached to any entity that navigates the grid.
#[derive(Debug, Clone)]
pub struct NavAgent {
    pub current_cell: GridCell,
    pub target_cell: Option<GridCell>,
    pub path: Vec<GridCell>,

    /// Normalised direction toward the current target_cell.
    /// Written by NavigationSystem each frame; read by ControllerSystem to
    /// drive Movement.intent for NavBehavior controllers.
    pub move_input: Vec2,

    /// True when the entity has just snapped to a cell centre and that cell
    /// has more than 2 walkable neighbours (i.e. a junction).
    /// AI controllers poll this to decide when to pick a new direction.
    pub at_intersection: bool,

    /// Distance from a cell's world-space centre at which the agent is
    /// considered to have arrived. Smaller values require the agent to be
    /// closer to the centre before the next step is issued; a value near 0
    /// relies entirely on the overshoot check and gives the tightest
    /// grid alignment. Set to -1.0 (default) to use `cell_size * 0.5`.
    pub arrival_radius: f32,
}

impl NavAgent {
    pub fn new(start: GridCell, arrival_radius: Option<f32>) -> Self {
        Self {
            current_cell: start,
            target_cell: None,
            path: Vec::new(),
            move_input: Vec2::ZERO,
            at_intersection: false,
            arrival_radius: arrival_radius.unwrap_or(-1.0),
        }
    }

    /// True when the agent has no target and no queued path steps.
    pub fn is_idle(&self) -> bool {
        self.target_cell.is_none() && self.path.is_empty()
    }

    /// Check whether the entity has reached its current target cell.
    /// Call this each frame from ControllerSystem with the entity's world position.
    /// On arrival: snaps current_cell, clears target_cell and move_input,
    /// and updates at_intersection.
    pub fn try_arrive(&mut self, transform_pos: Vec3, nav_grid: &NavGrid) {
        let target = match self.target_cell {
            Some(t) => t,
            None => return,
        };

        let target_world = nav_grid.cell_to_world(target);

        // Arrive when within half a cell (normal case) OR when the agent has
        // already passed the centre (overshoot from a delta-time spike).
        let arrived = {
            let dist_sq = (transform_pos.x - target_world.x).powi(2)
                + (transform_pos.y - target_world.y).powi(2);
            let r = if self.arrival_radius < 0.0 { nav_grid.cell_size * 0.5 } else { self.arrival_radius };
            let threshold = r * r;

            let overshot = match self.move_input {
                v if v.x > 0.0 => transform_pos.x >= target_world.x,
                v if v.x < 0.0 => transform_pos.x <= target_world.x,
                v if v.y > 0.0 => transform_pos.y >= target_world.y,
                v if v.y < 0.0 => transform_pos.y <= target_world.y,
                _ => false,
            };

            dist_sq <= threshold || overshot
        };

        if arrived {
            self.current_cell    = target;
            self.target_cell     = None;
            self.move_input      = Vec2::ZERO;
            self.at_intersection = nav_grid.neighbors(target).len() > 2;
        }
    }
}