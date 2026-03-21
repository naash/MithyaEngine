// Copyright (c) 2025 Nishant Sthalekar
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::any::Any;

use crate::{core::EngineEvent, debug::commands::DebugCommand};

#[derive(Debug, Clone)]
pub struct DrawDebugEvent {
    pub commands: Vec<DebugCommand>,
}

impl EngineEvent for DrawDebugEvent {
    fn as_any(&self) -> &dyn Any { self }
}