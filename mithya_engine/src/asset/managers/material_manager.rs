// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::collections::HashMap;
use crate::asset::error::MaterialError;
use crate::asset::asset_data::MaterialData;

pub struct MaterialManager {
    materials: HashMap<u32, MaterialData>,
    material_name_to_id: HashMap<String, u32>,
    next_id: u32,
}

impl MaterialManager {
    pub fn new() -> Self {
        Self {
            materials: HashMap::new(),
            material_name_to_id: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn create_material(&mut self, name: &str) -> Result<u32, MaterialError> {
        let id = self.next_id;
        self.next_id += 1;

        let material_data = MaterialData::new(name);
        self.materials.insert(id, material_data);
        self.material_name_to_id.insert(name.to_string(), id);
        
        Ok(id)
    }

    pub fn get_material(&self, material_id: u32) -> Option<&MaterialData> {
        self.materials.get(&material_id)
    }

    pub fn get_material_mut(&mut self, material_id: u32) -> Option<&mut MaterialData> {
        self.materials.get_mut(&material_id)
    }

    pub fn get_material_id(&self, name: &str) -> Option<u32> {
        self.material_name_to_id.get(name).copied()
    }

    pub fn add_texture_to_material(
        &mut self,
        material_id: u32,
        uniform_name: &str,
        texture_id: u32,
        slot: u32,
    ) -> Result<(), MaterialError> {
        if let Some(material) = self.materials.get_mut(&material_id) {
            material.bind_texture(uniform_name, texture_id, slot);
            Ok(())
        } else {
            Err(MaterialError::MaterialNotFound(material_id.to_string()))
        }
    }
}