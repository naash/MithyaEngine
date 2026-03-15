// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::any::Any;
use glam::Vec2;
use serde::{Deserialize, Serialize};
use crate::Component;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Movement {
    pub impulse: f32,
    #[serde(skip)]
    pub intent: Vec2,
}

impl Movement {
    pub fn new(impulse: f32) -> Self {
        Self {
            impulse,
            intent: Vec2::ZERO,
        }
    }
}

impl Component for Movement {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }

    fn serialize_to_json(&self) -> Result<serde_json::Value, serde_json::Error> {
        serde_json::to_value(self)
    }

    fn deserialize_from_json(value: serde_json::Value) -> Result<Box<dyn Component>, serde_json::Error> {
        let m: Movement = serde_json::from_value(value)?;
        Ok(Box::new(m))
    }
}