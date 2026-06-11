// Copyright (c) 2025 Nishant Sthalekar
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::any::Any;

use glam::{Vec2, Vec3};

use crate::{
    core::EngineEventListener,
    debug::resources::debug_draw_buffer::DebugDrawBuffer,
    engine::{system::{System, SystemUpdateContext}, World},
    navigation::{grid_cell::GridCell, resources::nav_grid::NavGrid},
    NavAgent,
};

pub struct NavGridDebugSystem {
    pub enabled: bool,
}

impl NavGridDebugSystem {
    pub fn new() -> Self {
        Self { enabled: true }
    }
}

impl Default for NavGridDebugSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl System for NavGridDebugSystem {
    fn initialize(&mut self, _world: &mut World) {}

    fn update(&mut self, ctx: &mut SystemUpdateContext) {
        if !self.enabled {
            return;
        }

        let (cell_draws, half_extents) = {
            let Some(grid) = ctx.world.resources.get::<NavGrid>() else { return; };
            let half = Vec2::splat(grid.cell_size * 0.46);
            let mut cells = Vec::with_capacity((grid.cols * grid.rows) as usize);
            for row in 0..grid.rows {
                for col in 0..grid.cols {
                    let cell = GridCell::new(col as i32, row as i32);
                    cells.push((grid.cell_to_world(cell), !grid.is_walkable(cell)));
                }
            }
            (cells, half)
        };

        let path_lines: Vec<(Vec3, Vec3)> = {
            let Some(grid) = ctx.world.resources.get::<NavGrid>() else { return; };
            let agent_ids = ctx.world.entity_manager.query_component::<NavAgent>();
            agent_ids.iter().flat_map(|&entity_id| -> Vec<(Vec3, Vec3)> {
                let agent = match ctx.world.entity_manager.get_component::<NavAgent>(entity_id) {
                    Some(a) => a,
                    None => return Vec::new(),
                };
                let mut chain = Vec::with_capacity(agent.path.len() + 2);
                chain.push(agent.current_cell);
                if let Some(t) = agent.target_cell { chain.push(t); }
                chain.extend_from_slice(&agent.path);
                chain.windows(2)
                    .map(|w| (grid.cell_to_world(w[0]), grid.cell_to_world(w[1])))
                    .collect()
            }).collect()
        };

        let Some(buffer) = ctx.world.resources.get_mut::<DebugDrawBuffer>() else { return; };

        let wall_color  = egui::Color32::from_rgba_unmultiplied(200, 60,  60,  180);
        let floor_color = egui::Color32::from_rgba_unmultiplied(80,  180, 80,  40);
        let path_color  = egui::Color32::from_rgba_unmultiplied(255, 220, 0,   220);

        for (center, is_wall) in cell_draws {
            let color = if is_wall { wall_color } else { floor_color };
            buffer.box_shape(center, half_extents, color, 0.05);
        }

        for (start, end) in path_lines {
            buffer.line(start, end, path_color, 0.05);
        }
    }

    fn as_event_listener_mut(&mut self) -> Option<&mut dyn EngineEventListener> {
        None
    }

    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}
