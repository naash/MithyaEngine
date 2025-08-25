// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::collections::HashMap;
use std::rc::Rc;
use gl;
use std;
use std::ffi::{CStr, CString};

use crate::asset::error::ShaderError;

#[derive(Debug, Clone)]
pub struct Program {
    id: gl::types::GLuint,
}

pub struct Shader {
    id: gl::types::GLuint,
}

impl Program {
    pub fn from_shaders(shaders: &[Shader]) -> Result<Program, String> {
        let program_id = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe {
                gl::AttachShader(program_id, shader.id());
            }
        }

        unsafe {
            gl::LinkProgram(program_id);
        }

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }

            return Err(error.to_string_lossy().into_owned());
        }

        for shader in shaders {
            unsafe {
                gl::DetachShader(program_id, shader.id());
            }
        }

        Ok(Program { id: program_id })
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

impl Shader {
    pub fn from_source(source: &CStr, kind: gl::types::GLenum) -> Result<Shader, String> {
        let id = shader_from_source(source, kind)?;
        Ok(Shader { id })
    }

    pub fn from_vert_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::VERTEX_SHADER)
    }

    pub fn from_frag_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::FRAGMENT_SHADER)
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

fn shader_from_source(source: &CStr, kind: gl::types::GLenum) -> Result<gl::types::GLuint, String> {
    let id = unsafe { gl::CreateShader(kind) };
    unsafe {
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
    }

    let mut success: gl::types::GLint = 1;
    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }

        let error = create_whitespace_cstring_with_len(len as usize);

        unsafe {
            gl::GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar,
            );
        }

        return Err(error.to_string_lossy().into_owned());
    }

    Ok(id)
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}

// Shader manager - handles shader programs
pub struct ShaderManager {
    programs: HashMap<String, Rc<Program>>,
}

impl ShaderManager {
    pub fn new() -> Self {
        Self {
            programs: HashMap::new()
        }
    }

    pub fn create_program(&mut self, vert_path: &str, frag_path: &str) -> Result<Rc<Program>, ShaderError> {
        use std::ffi::CString;

        // Build absolute paths using CARGO_MANIFEST_DIR (where Cargo.toml is)
        let project_root = env!("CARGO_MANIFEST_DIR");
        let vert_absolute = format!("{}/{}", project_root, vert_path);
        let frag_absolute = format!("{}/{}", project_root, frag_path);
    
        let vert_source = match std::fs::read_to_string(vert_absolute) {
            Ok(content) => {
                println!("Successfully read vertex shader, {} bytes", content.len());
                content
            },
            Err(e) => {
                println!("ERROR reading vertex shader: {:?}", e);
                return Err(ShaderError::ProgramError(format!("Failed to read vertex shader '{}': {}", vert_path, e)));
            }
        };

        println!("About to read fragment shader file...");
        let frag_source = match std::fs::read_to_string(frag_absolute) {
            Ok(content) => {
                println!("Successfully read fragment shader, {} bytes", content.len());
                content
            },
            Err(e) => {
                println!("ERROR reading fragment shader: {:?}", e);
                return Err(ShaderError::ProgramError(format!("Failed to read fragment shader '{}': {}", frag_path, e)));
            }
        };

        println!("Creating Shaders {} {}", vert_source, frag_source);
        //Create Vertex shader
        let vert_shader = Shader::from_vert_source(
            &CString::new(vert_source)
                .map_err(|e| ShaderError::ProgramError(format!("CString error: {}", e)))?
        )
        .map_err(|e| {
            println!("Shader compilation error: {}", e);
            ShaderError::ProgramError(format!("Vertex shader error: {}", e))
        })?;

        println!("Created Shader: {}", vert_shader.id());
        self.check_gl_error("Create Shader");

        // Add this debug function call:
        unsafe {
            let mut success = 0;
            gl::GetShaderiv(vert_shader.id(), gl::COMPILE_STATUS, &mut success);
            println!("Vertex shader compile status: {} (1=success, 0=failure)", success);
            
            // Always check the info log, even on success
            let mut log_length = 0;
            gl::GetShaderiv(vert_shader.id(), gl::INFO_LOG_LENGTH, &mut log_length);
            if log_length > 1 {  // > 1 because there's usually a null terminator
                let mut log = vec![0u8; log_length as usize];
                gl::GetShaderInfoLog(vert_shader.id(), log_length, std::ptr::null_mut(), log.as_mut_ptr() as *mut i8);
                let log_msg = String::from_utf8_lossy(&log);
                println!("Vertex shader info log: '{}'", log_msg.trim());
            }
        }

        //Create Fragment shader
        let frag_shader = Shader::from_frag_source(
            &CString::new(frag_source)
                .map_err(|e| ShaderError::ProgramError(format!("CString error: {}", e)))?
        )
        .map_err(|e| {
            println!("Shader compilation error: {}", e);
            ShaderError::ProgramError(format!("Fragment shader error: {}", e))
        })?;

        println!("Create Shader: {}", frag_shader.id());
        self.check_gl_error("Create Shader");

        // Check for issues
        unsafe {
            let mut success = 0;
            gl::GetShaderiv(frag_shader.id(), gl::COMPILE_STATUS, &mut success);
            println!("Fragment shader compile status: {} (1=success, 0=failure)", success);
            
            let mut log_length = 0;
            gl::GetShaderiv(frag_shader.id(), gl::INFO_LOG_LENGTH, &mut log_length);
            if log_length > 1 {
                let mut log = vec![0u8; log_length as usize];
                gl::GetShaderInfoLog(frag_shader.id(), log_length, std::ptr::null_mut(), log.as_mut_ptr() as *mut i8);
                let log_msg = String::from_utf8_lossy(&log);
                println!("Fragment shader info log: '{}'", log_msg.trim());
            }
        }

        //Create program from those shaders
        let program = Rc::new(Program::from_shaders(&[vert_shader, frag_shader])
        .map_err(|e| {
            println!("Create Program compilation error: {}", e);
            ShaderError::ProgramError(format!("Create Program error: {}", e))
        })?);

        println!("Create Program: {}", program.id());
        self.check_gl_error("Create Program");

        // Check for issues
        unsafe {
            let mut success = 0;
            gl::GetProgramiv(program.id(), gl::LINK_STATUS, &mut success);
            println!("Program link status: {} (1=success, 0=failure)", success);
            
            let mut log_length = 0;
            gl::GetProgramiv(program.id(), gl::INFO_LOG_LENGTH, &mut log_length);
            if log_length > 1 {
                let mut log = vec![0u8; log_length as usize];
                gl::GetProgramInfoLog(program.id(), log_length, std::ptr::null_mut(), log.as_mut_ptr() as *mut i8);
                let log_msg = String::from_utf8_lossy(&log);
                println!("Program link info log: '{}'", log_msg.trim());
            }
        }

        let key = format!("{}+{}", vert_path, frag_path);

        self.programs.insert(key.clone(), program.clone());

        Ok(program)
    }

    pub fn get_program(&self, key: String) -> Option<&Program> {
        if let Some(program) = self.programs.get(&key) {         
            // Verify the program still exists in OpenGL
            unsafe {
                if gl::IsProgram(program.id()) == gl::FALSE {
                    println!("WARNING: Program {} no longer exists in OpenGL!", program.id());
                } else {
                    let mut uniform_count = 0;
                    gl::GetProgramiv(program.id(), gl::ACTIVE_UNIFORMS, &mut uniform_count);
                    
                    if uniform_count <= 0 {
                        println!("WARNING Program {} currently has {} uniforms", program.id(), uniform_count);
                    }                    
                }
            }
            
            Some(program)
        } else {
            println!("No program found with key: {}", key);
            None
        }
    }

    // Add this debugging function to check OpenGL errors
    fn check_gl_error(&self, operation: &str) {
        unsafe {
            let error = gl::GetError();
            if error != gl::NO_ERROR {
                println!("OpenGL error after {}: {}", operation, error);
            }
        }
    }
}