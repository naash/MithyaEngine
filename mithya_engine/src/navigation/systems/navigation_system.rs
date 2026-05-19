// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::any::{Any, TypeId};

use glam::Vec2;

use crate::{Direction, EntityId, GridCell, MoveToEvent, NavAgent, NavGrid, Transform, World, core::EngineEventListener, engine::system::{System, SystemUpdateContext}};

pub struct NavigationSystem
{
    pending_move_requests: Vec<(EntityId, GridCell)>
}

impl NavigationSystem {
    pub fn new() -> Self {
        Self {
            pending_move_requests: Vec::new(),
        }
    }
}

impl System for NavigationSystem {  
    fn initialize(&mut self, _world: &mut World) {}

    fn update(&mut self, update_context: &mut SystemUpdateContext) {

        let nav_grid = match update_context.world.resources.get::<NavGrid>() {
            Some(g) => g,
            None => return,
        };

        for (entity_id, target) in self.pending_move_requests.drain(..) {
            let current_cell = match update_context.world.entity_manager
                .get_component::<NavAgent>(entity_id)
            {
                Some(a) => a.current_cell,
                None => continue,
            };

            //Default path finding uses euclidean distance as heuristics function
            if let Some(mut path) = nav_grid.find_path_default(current_cell, target) {
                // Index 0 is current_cell — skip it, agent is already there
                if path.len() > 1 {
                    path.remove(0);
                }
                
                if let Some(agent) = update_context.world.entity_manager
                    .get_component_mut::<NavAgent>(entity_id)
                {
                    agent.path = path;
                    agent.target_cell = None;
                    agent.move_input = Vec2::ZERO;
                }
            }
        }

        let nav_grid = match update_context.world.resources.get::<NavGrid>() {
            Some(g) => g,
            None => return,
        };

        let nav_agent_ids = update_context.world.entity_manager
            .query_component::<NavAgent>();

        for entity_id in nav_agent_ids {
            // Arrival check — read transform position, then update agent state.
            let transform_pos = update_context.world.entity_manager
                .get_component::<Transform>(entity_id)
                .map(|t| t.position);

            if let Some(pos) = transform_pos {
                if let Some(agent) = update_context.world.entity_manager
                    .get_component_mut::<NavAgent>(entity_id)
                {
                    agent.try_arrive(pos, nav_grid);
                }
            }

            // Pop next step if no active target
            let agent = match update_context.world.entity_manager
                .get_component_mut::<NavAgent>(entity_id)
            {
                Some(a) => a,
                None => continue,
            };

            if agent.target_cell.is_none() {
                if agent.path.is_empty() {
                    agent.move_input = Vec2::ZERO;
                    continue;
                }
                agent.target_cell = Some(agent.path.remove(0));
            }

            // Compute direction toward target_cell and write to move_input
            if let Some(target) = agent.target_cell {
                agent.move_input = Direction::from_cells(agent.current_cell, target)
                    .map(|d| d.to_vec2())
                    .unwrap_or(Vec2::ZERO);
            }
        }
    }

    fn as_event_listener_mut(&mut self) -> Option<&mut dyn EngineEventListener> {
        Some(self)
    }

    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}

impl EngineEventListener for NavigationSystem {
    fn interested_events(&self) -> Vec<TypeId> {
        vec![TypeId::of::<MoveToEvent>()]
    }

    fn on_events(
        &mut self,
        events: &crate::core::EngineEventQueue,
        _actions: &mut crate::core::EngineActionQueue,
        _world: &World
    ) {
        for event in events.iter_type::<MoveToEvent>() {
            self.pending_move_requests.push((event.entity_id, event.target));
        }
    }
}