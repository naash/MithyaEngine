// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::any::Any;

// Component trait - anything that can be attached to an entity
pub trait Component: Any + Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    // Serialization support
    fn serialize_to_json(&self) -> Result<serde_json::Value, serde_json::Error>;
    fn deserialize_from_json(value: serde_json::Value) -> Result<Box<dyn Component>, serde_json::Error> 
    where Self: Sized;
}

// Automatic implementation for all types that meet the requirements
// impl<T: Any + Send + Sync> Component for T {
//     fn as_any(&self) -> &dyn Any {
//         self
//     }
    
//     fn as_any_mut(&mut self) -> &mut dyn Any {
//         self
//     }
// }