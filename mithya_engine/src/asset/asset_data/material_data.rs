// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MaterialData {
    pub name: String,
    pub shader_program_id: Option<u32>,
    pub uniforms: HashMap<String, UniformValue>,
    pub textures: HashMap<String, TextureBinding>,
}

#[derive(Debug, Clone)]
pub enum UniformValue {
    Float(f32),
    Vec2([f32; 2]),
    Vec3([f32; 3]),
    Vec4([f32; 4]),
    Mat4([f32; 16]),
    Int(i32),
    Bool(bool),
}

#[derive(Debug, Clone)]
pub struct TextureBinding {
    pub texture_id: u32,
    pub slot: u32,
}

impl MaterialData {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            shader_program_id: None,
            uniforms: HashMap::new(),
            textures: HashMap::new(),
        }
    }

    pub fn set_shader_program(&mut self, program_id: u32) {
        self.shader_program_id = Some(program_id);
    }

    pub fn bind_texture(&mut self, uniform_name: &str, texture_id: u32, slot: u32) {
        self.textures.insert(
            uniform_name.to_string(),
            TextureBinding { texture_id, slot },
        );
    }
}