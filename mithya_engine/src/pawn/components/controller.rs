// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::any::Any;
use serde::{Deserialize, Serialize};
use crate::Component;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Controller {
    pub possessed_entity_id: u32,
}

impl Controller {
    pub fn new(possessed_entity_id: u32) -> Self {
        Self { possessed_entity_id }
    }
}

impl Component for Controller {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }

    fn serialize_to_json(&self) -> Result<serde_json::Value, serde_json::Error> {
        serde_json::to_value(self)
    }

    fn deserialize_from_json(value: serde_json::Value) -> Result<Box<dyn Component>, serde_json::Error> {
        let c: Controller = serde_json::from_value(value)?;
        Ok(Box::new(c))
    }
}