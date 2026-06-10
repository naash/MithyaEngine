// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::collections::HashMap;
use crate::asset::error::AssetError;
use crate::asset::MaterialData;
use crate::asset::managers::{TextureManager, MaterialManager};
use crate::asset::managers::texture_manager::TextureEntry;

pub struct AssetManager {
    texture_manager: TextureManager,
    material_manager: MaterialManager,
    loaded_materials: HashMap<String, u32>,
    default_material_id: u32,
}

impl AssetManager {
    pub fn new(asset_root: std::path::PathBuf) -> Result<Self, AssetError> {
        let mut manager = Self {
            texture_manager: TextureManager::new(asset_root),
            material_manager: MaterialManager::new(),
            loaded_materials: HashMap::new(),
            default_material_id: 0,
        };
        manager.default_material_id = manager.load_material("unlit_color")?; //Support unlit color material as default
        Ok(manager)
    }

    pub fn default_material_id(&self) -> u32 {
        self.default_material_id
    }

    pub fn load_material(&mut self, name: &str) -> Result<u32, AssetError> {
        if let Some(&id) = self.loaded_materials.get(name) {
            return Ok(id);
        }
        let material_id = self.material_manager.create_material(name)?;
        self.loaded_materials.insert(name.to_string(), material_id);
        Ok(material_id)
    }

    pub fn load_texture_for_material(
    &mut self,
    material_name: &str,
    texture_file: &str,
    device: &wgpu::Device,
    queue: &wgpu::Queue) -> Result<(), AssetError> {
        let material_id = self.load_material(material_name)?;
        let texture_id = self.texture_manager.load_texture(texture_file, device, queue)?;
        self.material_manager.add_texture_to_material(material_id, "u_texture", texture_id, 0)?;
        Ok(())
    }

    pub fn get_material(&self, material_id: u32) -> Option<&MaterialData> {
        self.material_manager.get_material(material_id)
    }

    pub fn get_material_mut(&mut self, material_id: u32) -> Option<&mut MaterialData> {
        self.material_manager.get_material_mut(material_id)
    }

    pub fn get_material_by_name(&self, name: &str) -> Option<u32> {
        self.material_manager.get_material_id(name)
    }

    pub fn get_texture(&self, texture_id: &u32) -> Option<&TextureEntry> {
        self.texture_manager.get_texture(texture_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_material_id_resolves_to_a_material() {
        let manager = AssetManager::new(std::path::PathBuf::new()).unwrap();
        let id = manager.default_material_id();

        assert!(manager.get_material(id).is_some());
        assert_eq!(manager.get_material_by_name("unlit_color"), Some(id));
        // Material ids start at 1 — the old hardcoded fallback of 0 never resolved.
        assert!(manager.get_material(0).is_none());
    }
}