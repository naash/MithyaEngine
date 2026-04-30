// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::any::Any;

use crate::{core::{EngineEvent, EntityId}, navigation::grid_cell::GridCell};

#[derive(Debug, Clone)]
pub struct MoveToEvent {
    pub entity_id: EntityId,
    pub target: GridCell
}

impl EngineEvent for MoveToEvent {
    fn as_any(&self) -> &dyn Any { self }
}