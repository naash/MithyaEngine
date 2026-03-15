// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use mithya_engine::core::Component;

use std::any::Any;

use serde::{Deserialize, Serialize};

/// Marks an entity as a brick
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Brick {
    pub brick_type: BrickType,
    pub health: u32,
    pub points: u32,
}

impl Brick {
    pub fn new(brick_type: BrickType) -> Self {
        let (health, points) = match brick_type {
            BrickType::Normal => (1, 10),
            BrickType::Strong => (2, 20),
            BrickType::Unbreakable => (999, 0),
        };
        
        Self {
            brick_type,
            health,
            points,
        }
    }
    
    pub fn take_damage(&mut self) -> bool {
        if self.health > 0 {
            self.health -= 1;
        }
        println!("Health {}", self.health);
        self.health <= 0
    }
    
    pub fn is_destroyed(&self) -> bool {
        self.health <= 0
    }
}

impl Component for Brick {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    
    fn serialize_to_json(&self) -> Result<serde_json::Value, serde_json::Error> {
        serde_json::to_value(self)
    }
    
    fn deserialize_from_json(value: serde_json::Value) -> Result<Box<dyn Component>, serde_json::Error> {
        let brick: Brick = serde_json::from_value(value)?;
        Ok(Box::new(brick))
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BrickType {
    Normal,
    Strong,
    Unbreakable,
}

/// Marks an entity as the ball
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Ball;

impl Component for Ball {
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
        let ball: Ball = serde_json::from_value(value)?;
        Ok(Box::new(ball))
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GameState {
    WaitingToLaunch,
    Playing,
    GameOver,
    Won,
    Resetting,
}

/// Game state component (attached to a game manager entity)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrickBreakerState {
    pub score: u32,
    pub lives: u32,
    pub state: GameState,
}

impl Default for BrickBreakerState {
    fn default() -> Self {
        Self {
            score: 0,
            lives: 3,
            state : GameState::WaitingToLaunch
        }
    }
}

impl Component for BrickBreakerState {
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
        let brickbreakerstate: BrickBreakerState = serde_json::from_value(value)?;
        Ok(Box::new(brickbreakerstate))
    }
}