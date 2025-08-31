// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::any::Any;

use serde::{Deserialize, Serialize};

use crate::Component;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Player;

impl Component for Player {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    
    fn serialize_to_json(&self) -> Result<serde_json::Value, serde_json::Error> {
        serde_json::to_value(self)  // Serializes to `null` but can add more data in future
    }
    
    fn deserialize_from_json(value: serde_json::Value) -> Result<Box<dyn Component>, serde_json::Error> {
        let _player: Player = serde_json::from_value(value)?;
        Ok(Box::new(Player))
    }
}