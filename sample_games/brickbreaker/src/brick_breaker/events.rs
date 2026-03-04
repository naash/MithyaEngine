// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::any::Any;
use mithya_engine::core::EngineEvent;

#[derive(Debug, Clone)]
pub struct BallLostEvent;

impl EngineEvent for BallLostEvent {
    fn as_any(&self) -> &dyn Any { self }
}

#[derive(Debug, Clone)]
pub struct BrickDestroyedEvent {
    pub entity_id: u32,
    pub points: u32,
}

impl EngineEvent for BrickDestroyedEvent {
    fn as_any(&self) -> &dyn Any { self }
}

#[derive(Debug, Clone)]
pub struct GameOverEvent {
    pub final_score: u32,
}

impl EngineEvent for GameOverEvent {
    fn as_any(&self) -> &dyn Any { self }
}

#[derive(Debug, Clone)]
pub struct GameWonEvent {
    pub final_score: u32,
}

impl EngineEvent for GameWonEvent {
    fn as_any(&self) -> &dyn Any { self }
}

#[derive(Debug, Clone)]
pub struct LoseLifeEvent;

impl EngineEvent for LoseLifeEvent {
    fn as_any(&self) -> &dyn Any { self }
}