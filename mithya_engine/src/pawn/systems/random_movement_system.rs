// Copyright (c) 2025 Nishant Sthalekar
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::any::Any;

use glam::Vec2;
use rand::Rng;

use crate::{
    Movement, MoveToEvent,
    core::EngineEventListener,
    engine::{system::{System, SystemUpdateContext}, world::World},
    navigation::{GridCell, NavAgent, NavGrid},
    pawn::components::random_movement::RandomMovement,
};

pub struct RandomMovementSystem;

impl System for RandomMovementSystem {
    fn initialize(&mut self, _world: &mut World) {}

    fn update(&mut self, ctx: &mut SystemUpdateContext) {
        let has_nav_grid = ctx.world.resources.get::<NavGrid>().is_some();
        let mut rng = rand::thread_rng();

        let entities = ctx.world.entity_manager.query_component::<RandomMovement>();
        for entity_id in entities {
            if has_nav_grid {
                let is_idle = ctx.world.entity_manager
                    .get_component::<NavAgent>(entity_id)
                    .map(|a| a.is_idle())
                    .unwrap_or(true);

                if is_idle {
                    let new_target = {
                        let nav_grid = ctx.world.resources.get::<NavGrid>().unwrap();
                        pick_random_floor_cell(nav_grid, &mut rng)
                    };
                    if let Some(target) = new_target {
                        ctx.events.push(MoveToEvent { entity_id, target });
                    }
                }
            } else {
                let dir = {
                    let rm = match ctx.world.entity_manager.get_component_mut::<RandomMovement>(entity_id) {
                        Some(r) => r,
                        None => continue,
                    };
                    if rm.current_direction == Vec2::ZERO || rng.gen_bool(0.015) {
                        let angle = rng.gen_range(0.0_f32..std::f32::consts::TAU);
                        rm.current_direction = Vec2::new(angle.cos(), angle.sin());
                    }
                    rm.current_direction
                };
                if let Some(movement) = ctx.world.entity_manager.get_component_mut::<Movement>(entity_id) {
                    movement.intent = dir;
                }
            }
        }
    }

    fn as_event_listener_mut(&mut self) -> Option<&mut dyn EngineEventListener> { None }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}

fn pick_random_floor_cell(nav_grid: &NavGrid, rng: &mut impl Rng) -> Option<GridCell> {
    let walkable: Vec<GridCell> = (0..nav_grid.cols * nav_grid.rows)
        .map(|i| GridCell::new((i % nav_grid.cols) as i32, (i / nav_grid.cols) as i32))
        .filter(|&cell| nav_grid.is_walkable(cell))
        .collect();

    if walkable.is_empty() {
        None
    } else {
        Some(walkable[rng.gen_range(0..walkable.len())])
    }
}
