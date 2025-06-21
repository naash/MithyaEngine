use std::collections::HashMap;

use super::render_gl::Program;
use super::render_gl::Shader;

// Shader manager - handles shader programs
pub struct ShaderManager {
    programs: HashMap<u32, Program>,
    next_program_id: u32,
}

impl ShaderManager {
    pub fn new() -> Self {
        Self {
            programs: HashMap::new(),
            next_program_id: 0,
        }
    }

    pub fn create_program(&mut self, vert_source: &str, frag_source: &str) -> Result<u32, String> {
        use std::ffi::CString;
        
        let vert_shader = Shader::from_vert_source(
            &CString::new(vert_source).map_err(|e| e.to_string())?
        ).map_err(|e| format!("Vertex shader error: {}", e))?;

        let frag_shader = Shader::from_frag_source(
            &CString::new(frag_source).map_err(|e| e.to_string())?
        ).map_err(|e| format!("Fragment shader error: {}", e))?;

        let program = Program::from_shaders(&[vert_shader, frag_shader])
            .map_err(|e| format!("Program linking error: {}", e))?;

        let id = self.next_program_id;
        self.next_program_id += 1;
        self.programs.insert(id, program);
        Ok(id)
    }

    pub fn get_program(&self, id: u32) -> Option<&Program> {
        self.programs.get(&id)
    }

    pub fn set_uniform_matrix4fv(&self, id: u32, name: &str, matrix: &[f32; 16]) {
        unsafe {
            let c_name = std::ffi::CString::new(name).unwrap();
            let location = gl::GetUniformLocation(id, c_name.as_ptr());
            if location != -1 {
                gl::UniformMatrix4fv(location, 1, gl::FALSE, matrix.as_ptr());
            }
        }
    }
    
    pub fn set_uniform_1f(&self, id: u32, name: &str, value: f32) {
        unsafe {
            let c_name = std::ffi::CString::new(name).unwrap();
            let location = gl::GetUniformLocation(id, c_name.as_ptr());
            if location != -1 {
                gl::Uniform1f(location, value);
            }
        }
    }
    
    pub fn set_uniform_3f(&self, id: u32, name: &str, x: f32, y: f32, z: f32) {
        unsafe {
            let c_name = std::ffi::CString::new(name).unwrap();
            let location = gl::GetUniformLocation(id, c_name.as_ptr());
            if location != -1 {
                gl::Uniform3f(location, x, y, z);
            }
        }
    }
}