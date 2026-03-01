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
}

impl AssetManager {
    pub fn new() -> Result<Self, AssetError> {
        Ok(Self {
            texture_manager: TextureManager::new(),
            material_manager: MaterialManager::new(),
            loaded_materials: HashMap::new(),
        })
    }

    pub fn load_material(&mut self, name: &str) -> Result<u32, AssetError> {
        if let Some(&id) = self.loaded_materials.get(name) {
            return Ok(id);
        }

        let material_id = self.material_manager.create_material(name)?;
        self.loaded_materials.insert(name.to_string(), material_id);
        Ok(material_id)
    }

    pub fn create_default_materials(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Result<(), AssetError> {
        // Create materials — no shader paths needed, pipelines are in RenderingSystem
        self.load_material("unlit_color")?;
        self.load_material("unlit_texture_green")?;
        self.load_material("unlit_texture_orange")?;
        self.load_material("unlit_texture_blue")?;
        self.load_material("unlit_texture_red")?;
        self.load_material("unlit_texture_yellow")?;
        self.load_material("unlit_texture_purple")?;
        self.load_material("unlit_texture_circle")?;

        // Load textures
        let green_id   = self.texture_manager.load_texture("green_texture.png", device, queue)?;
        let red_id     = self.texture_manager.load_texture("red_texture.png", device, queue)?;
        let blue_id    = self.texture_manager.load_texture("blue_texture.png", device, queue)?;
        let yellow_id  = self.texture_manager.load_texture("yellow_texture.png", device, queue)?;
        let orange_id  = self.texture_manager.load_texture("orange_texture.png", device, queue)?;
        let purple_id  = self.texture_manager.load_texture("purple_texture.png", device, queue)?;
        let circle_id  = self.texture_manager.load_texture("pinkCircle.png", device, queue)?;

        // Bind textures to materials
        let bindings = [
            ("unlit_texture_green",  green_id),
            ("unlit_texture_red",    red_id),
            ("unlit_texture_blue",   blue_id),
            ("unlit_texture_yellow", yellow_id),
            ("unlit_texture_orange", orange_id),
            ("unlit_texture_purple", purple_id),
            ("unlit_texture_circle", circle_id),
        ];

        for (material_name, texture_id) in bindings {
            if let Some(material_id) = self.material_manager.get_material_id(material_name) {
                self.material_manager.add_texture_to_material(
                    material_id,
                    "u_texture",
                    texture_id,
                    0,
                )?;
            }
        }

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