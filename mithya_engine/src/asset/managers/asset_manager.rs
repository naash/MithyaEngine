// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::collections::HashMap;
use crate::asset::error::AssetError;
use crate::asset::MaterialData;
use crate::asset::managers::{ShaderManager, TextureManager, MaterialManager};

pub struct AssetManager {
    shader_manager: ShaderManager,
    texture_manager: TextureManager,
    material_manager: MaterialManager,
    loaded_materials: HashMap<String, u32>,
}

impl AssetManager {
    pub fn new() -> Result<Self, AssetError> {
        Ok(Self {
            shader_manager: ShaderManager::new(),
            texture_manager: TextureManager::new(),
            material_manager: MaterialManager::new(),
            loaded_materials: HashMap::new(),
        })
    }

    pub fn load_material(&mut self, name: &str, vert_path: &str, frag_path: &str) -> Result<u32, AssetError> {
        if let Some(&id) = self.loaded_materials.get(name) {
            return Ok(id);
        }

        // Create material first
        let material_id = self.material_manager.create_material(name)?;

        let shader_key = format!("{}+{}", vert_path, frag_path);
        println!("Creating Shaders from paths in load material: {} and {}", vert_path, frag_path);
        let program_id = if let Some(existing_program) = self.shader_manager.get_program(shader_key) {
            // Program already exists, get its ID
            existing_program.id()
        } else {
            // Create new shader program
            self.shader_manager.create_program(vert_path, frag_path)?.id()
        };

        println!("shader id {}",program_id);

        self.material_manager.set_shader_program(material_id, program_id)?;

        self.loaded_materials.insert(name.to_string(), material_id);
        Ok(material_id)
    }

    pub fn create_default_materials(&mut self) -> Result<(), AssetError> {
        // Load core materials
        self.load_material("unlit_color",
         "shaders/unlit_color.vert",
         "shaders/unlit_color.frag")?;

        self.load_material("unlit_texture_default",
         "shaders/unlit_texture.vert",
         "shaders/unlit_texture.frag")?;

        self.load_material("unlit_texture_circle",
         "shaders/unlit_texture.vert",
         "shaders/unlit_texture.frag")?;

        // Add specific textures
        let default_texture_id = self.texture_manager.load_texture("test_texture.png")?;
        let circle_texture_id = self.texture_manager.load_texture("pinkCircle.png")?;

        if let Some(material_id) = self.material_manager.get_material_id("unlit_texture_default") {
            self.material_manager.add_texture_to_material(
                material_id,
                "u_texture",
                default_texture_id,
                0
            )?;
        }

        if let Some(material_id) = self.material_manager.get_material_id("unlit_texture_circle") {
            self.material_manager.add_texture_to_material(
                material_id,
                "u_texture",
                circle_texture_id,
                0
            )?;
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
        self.material_manager
            .get_material_id(name)

    }
}