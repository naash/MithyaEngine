
use std::collections::HashMap;

// Material data - How should the geometry be displayed, which shaders should it use
#[derive(Clone, Debug)]
pub struct Material {
    pub name: String,
    pub shader_program_id: Option<u32>,
    pub vertex_shader_path: String,
    pub fragment_shader_path: String,
    pub uniforms: HashMap<String, UniformValue>,
    pub textures: HashMap<String, TextureBinding>,
    pub render_state: RenderState,
}

// Uniform value types
#[derive(Clone, Debug)]
pub enum UniformValue {
    Float(f32),
    Vec2([f32; 2]),
    Vec3([f32; 3]),
    Vec4([f32; 4]),
    Int(i32),
    Mat4([f32; 16]),
    Bool(bool),
}

// Texture binding information
#[derive(Clone, Debug)]
pub struct TextureBinding {
    pub texture_id: u32,
    pub slot: u32, // Texture unit
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
    pub fn new(name: &str, vertex_path: &str, fragment_path: &str) -> Self {
        Self {
            name: name.to_string(),
            shader_program_id: None,
            vertex_shader_path: vertex_path.to_string(),
            fragment_shader_path: fragment_path.to_string(),
            uniforms: HashMap::new(),
            textures: HashMap::new(),
            render_state: RenderState::default(),
        }
    }

    // Uniform setters
    pub fn set_float(&mut self, name: &str, value: f32) {
        self.uniforms.insert(name.to_string(), UniformValue::Float(value));
    }

    pub fn set_vec3(&mut self, name: &str, value: [f32; 3]) {
        self.uniforms.insert(name.to_string(), UniformValue::Vec3(value));
    }

    pub fn set_vec4(&mut self, name: &str, value: [f32; 4]) {
        self.uniforms.insert(name.to_string(), UniformValue::Vec4(value));
    }

    pub fn set_mat4(&mut self, name: &str, value: [f32; 16]) {
        self.uniforms.insert(name.to_string(), UniformValue::Mat4(value));
    }

    pub fn set_int(&mut self, name: &str, value: i32) {
        self.uniforms.insert(name.to_string(), UniformValue::Int(value));
    }

    pub fn set_bool(&mut self, name: &str, value: bool) {
        self.uniforms.insert(name.to_string(), UniformValue::Bool(value));
    }

    // Texture binding
    pub fn bind_texture(&mut self, uniform_name: &str, texture_id: u32, slot: u32) {
        self.textures.insert(
            uniform_name.to_string(),
            TextureBinding { texture_id, slot },
        );
    }

    // Get uniform value
    pub fn get_uniform(&self, name: &str) -> Option<&UniformValue> {
        self.uniforms.get(name)
    }

    // Apply material state (called before rendering)
    pub fn apply(&self) {
        if let Some(program_id) = self.shader_program_id {
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
        if let Some(program_id) = self.shader_program_id {
            for (name, value) in &self.uniforms {
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
        for (uniform_name, binding) in &self.textures {
            unsafe {
                gl::ActiveTexture(gl::TEXTURE0 + binding.slot);
                gl::BindTexture(gl::TEXTURE_2D, binding.texture_id);
                
                if let Some(program_id) = self.shader_program_id {
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