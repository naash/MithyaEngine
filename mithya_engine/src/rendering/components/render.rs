// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::any::Any;

use serde::Deserialize;

use crate::{rendering::components::mesh::MeshType, Component};

use super::Mesh;

// Renderable component - marks an entity as something that should be rendered
#[derive(Debug)]
pub struct Render {
    pub mesh: Mesh,
    pub material_id: Option<u32>,
}

impl Component for Render {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    
    fn serialize_to_json(&self) -> Result<serde_json::Value, serde_json::Error> {
        let mesh_type = MeshType::from_mesh(&self.mesh);
        serde_json::to_value(serde_json::json!({
            "mesh_type": mesh_type,
            "material_id": self.material_id
        }))
    }
    
    fn deserialize_from_json(value: serde_json::Value) -> Result<Box<dyn Component>, serde_json::Error> {
        #[derive(Deserialize)]
        struct RenderData {
            mesh_type: MeshType,
            material_id: Option<u32>,
        }
        
        let render_data: RenderData = serde_json::from_value(value)?;
        
        let render = Render {
            mesh: render_data.mesh_type.create_mesh(),
            material_id: render_data.material_id,
        };
        
        Ok(Box::new(render))
    }
}