
use std::collections::HashMap;
use crate::rendering::ShaderManager;

use super::Material;

// Material Manager for caching and loading
pub struct MaterialManager {
    shader_manager: ShaderManager,
    materials: HashMap<u32, Material>,        // ID -> Material
    material_name_to_id: HashMap<String, u32>,         // Name -> ID (for lookup by name)
    next_id: u32,
    shader_cache: HashMap<String, u32>, // Path -> Program ID
}

impl MaterialManager {
    pub fn new() -> Self {
        Self {
            material_name_to_id: HashMap::new(),
            materials: HashMap::new(),
            next_id: 1,
            shader_manager: ShaderManager::new(),
            shader_cache: HashMap::new(),
        }
    }

    // Create material with auto-generated ID
    pub fn load_material(&mut self, name: &str, vertex_path: &str, fragment_path: &str) -> Result<(), String> {
        let shader_key = format!("{}+{}", vertex_path, fragment_path);
        
        let id = self.next_id;
        self.next_id += 1;

        // Check if shader is already compiled
        let program_key = if let Some(&existing_id) = self.shader_cache.get(&shader_key) {
            existing_id
        } else {
            // Compile new shader program
            let program_key = self.shader_manager.create_program(vertex_path, fragment_path)?;
            self.shader_cache.insert(shader_key, program_key);
            program_key
        };

         println!("Loading Material with Program : {}", program_key);
        let mut material = Material::new(name, vertex_path, fragment_path);
        let program_id =self.shader_manager.get_program(program_key);
        
        //This is ugly, needs cleaup
        material.shader_program_id = Some(program_id.unwrap().id());
        
        self.materials.insert(id, material);
        self.material_name_to_id.insert(name.to_string(), id);
        Ok(())
    }

    pub fn get_material_id_from_name(&self, name: &str) -> Option<&u32> {
        self.material_name_to_id.get(name)
    }

    pub fn get_material(&self, material_id: &u32) -> Option<&Material> {
        self.materials.get(&material_id)
    }

    pub fn get_material_mut(&mut self, material_id: &u32) -> Option<&mut Material> {
        self.materials.get_mut(&material_id)
    }

    pub fn get_material_from_name(&self, name: &str) -> Option<&Material> {
        let material_id = self.material_name_to_id[name];
        self.materials.get(&material_id)
    }

    pub fn get_material_from_name_mut(&mut self, name: &str) -> Option<&mut Material> {
        let material_id = self.material_name_to_id[name];
        self.materials.get_mut(&material_id)
    }

    // Create predefined materials
    pub fn create_default_materials(&mut self) -> Result<(), String> {
        self.load_material("unlit_color",
         include_str!("../../../shaders/unlit_color.vert"),
         include_str!("../../../shaders/unlit_color.frag"))?;
        Ok(())
    }
}