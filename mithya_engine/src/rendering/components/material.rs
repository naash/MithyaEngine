// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::asset::asset_data::{MaterialData, UniformValue};

// Material - How should the geometry be displayed, which shaders should it use
#[derive(Clone, Debug)]
pub struct Material {
    pub data: MaterialData,
    pub render_state: RenderState,
}

// Render state for material
#[derive(Clone, Debug)]
pub struct RenderState {
    pub blend_enabled: bool,
    pub depth_test: bool,
    pub depth_write: bool,
    pub cull_face: CullFace,
    pub blend_mode: BlendMode,
}

#[derive(Clone, Debug)]
pub enum CullFace {
    None,
    Front,
    Back,
    FrontAndBack,
}

#[derive(Clone, Debug)]
pub enum BlendMode {
    None,
    Alpha,
    Additive,
    Multiply,
}

impl Default for RenderState {
    fn default() -> Self {
        Self {
            blend_enabled: false,
            depth_test: true,
            depth_write: true,
            cull_face: CullFace::Back,
            blend_mode: BlendMode::None,
        }
    }
}

impl Material {
    pub fn from_data(data: MaterialData) -> Self {
        Self {
            data,
            render_state: RenderState::default(),
        }
    }

    // Uniform setters
    pub fn set_float(&mut self, name: &str, value: f32) {
        self.data.uniforms.insert(name.to_string(), UniformValue::Float(value));
    }

    pub fn set_vec3(&mut self, name: &str, value: [f32; 3]) {
        self.data.uniforms.insert(name.to_string(), UniformValue::Vec3(value));
    }

    pub fn set_vec4(&mut self, name: &str, value: [f32; 4]) {
        self.data.uniforms.insert(name.to_string(), UniformValue::Vec4(value));
    }

    pub fn set_mat4(&mut self, name: &str, value: [f32; 16]) {
        self.data.uniforms.insert(name.to_string(), UniformValue::Mat4(value));
    }

    pub fn set_int(&mut self, name: &str, value: i32) {
        self.data.uniforms.insert(name.to_string(), UniformValue::Int(value));
    }

    pub fn set_bool(&mut self, name: &str, value: bool) {
        self.data.uniforms.insert(name.to_string(), UniformValue::Bool(value));
    }

    // Get uniform value
    pub fn get_uniform(&self, name: &str) -> Option<&UniformValue> {
        self.data.uniforms.get(name)
    }

    // Apply material state (called before rendering)
    pub fn apply(&self) {
        if let Some(program_id) = self.data.shader_program_id {
            unsafe {
                gl::UseProgram(program_id);
            }
        }

        // Apply uniforms
        self.apply_uniforms();
        
        // Apply textures
        self.apply_textures();
        
        // Apply render state
        self.apply_render_state();
    }

    fn apply_uniforms(&self) {
        if let Some(program_id) = self.data.shader_program_id {
            for (name, value) in &self.data.uniforms {
                let location = unsafe {
                    let c_name = std::ffi::CString::new(name.as_str()).unwrap();
                    gl::GetUniformLocation(program_id, c_name.as_ptr())
                };

                if location != -1 {
                    unsafe {
                        match value {
                            UniformValue::Float(v) => gl::Uniform1f(location, *v),
                            UniformValue::Vec2(v) => gl::Uniform2fv(location, 1, v.as_ptr()),
                            UniformValue::Vec3(v) => gl::Uniform3fv(location, 1, v.as_ptr()),
                            UniformValue::Vec4(v) => gl::Uniform4fv(location, 1, v.as_ptr()),
                            UniformValue::Int(v) => gl::Uniform1i(location, *v),
                            UniformValue::Mat4(v) => gl::UniformMatrix4fv(location, 1, gl::FALSE, v.as_ptr()),
                            UniformValue::Bool(v) => gl::Uniform1i(location, if *v { 1 } else { 0 }),
                        }
                    }
                }
            }
        }
    }

    fn apply_textures(&self) {
        for (uniform_name, binding) in &self.data.textures {
            unsafe {
                gl::ActiveTexture(gl::TEXTURE0 + binding.slot);
                gl::BindTexture(gl::TEXTURE_2D, binding.texture_id);
                
                if let Some(program_id) = self.data.shader_program_id {
                    let c_name = std::ffi::CString::new(uniform_name.as_str()).unwrap();
                    let location = gl::GetUniformLocation(program_id, c_name.as_ptr());
                    if location != -1 {
                        gl::Uniform1i(location, binding.slot as i32);
                    }
                }
            }
        }
    }

    fn apply_render_state(&self) {
        unsafe {
            // Depth testing
            if self.render_state.depth_test {
                gl::Enable(gl::DEPTH_TEST);
            } else {
                gl::Disable(gl::DEPTH_TEST);
            }

            // Depth writing
            gl::DepthMask(if self.render_state.depth_write { gl::TRUE } else { gl::FALSE });

            // Face culling
            match self.render_state.cull_face {
                CullFace::None => gl::Disable(gl::CULL_FACE),
                CullFace::Front => {
                    gl::Enable(gl::CULL_FACE);
                    gl::CullFace(gl::FRONT);
                }
                CullFace::Back => {
                    gl::Enable(gl::CULL_FACE);
                    gl::CullFace(gl::BACK);
                }
                CullFace::FrontAndBack => {
                    gl::Enable(gl::CULL_FACE);
                    gl::CullFace(gl::FRONT_AND_BACK);
                }
            }

            // Blending
            if self.render_state.blend_enabled {
                gl::Enable(gl::BLEND);
                match self.render_state.blend_mode {
                    BlendMode::Alpha => gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA),
                    BlendMode::Additive => gl::BlendFunc(gl::ONE, gl::ONE),
                    BlendMode::Multiply => gl::BlendFunc(gl::DST_COLOR, gl::ZERO),
                    _ => {}
                }
            } else {
                gl::Disable(gl::BLEND);
            }
        }
    }
}