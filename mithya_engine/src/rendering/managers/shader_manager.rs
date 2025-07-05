// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::collections::HashMap;

use gl;
use std;
use std::ffi::{CStr, CString};

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
        
        println!("Creating Shaders");
        
        //Create Vertex shader
        let vert_shader = Shader::from_vert_source(
            &CString::new(vert_source).map_err(|e| e.to_string())?
        ).
        map_err(|e| 
        {
            println!("Shader compilation error: {}", e);
            format!("Vertex shader error: {}", e)
        })?;

        println!("Create Shader: {}", vert_shader.id());
        self.check_gl_error("Create Shader");

        //Create Fragment shader
        let frag_shader = Shader::from_frag_source(
            &CString::new(frag_source).map_err(|e| e.to_string())?
        ).
        map_err(|e| 
        {
            println!("Shader compilation error: {}", e);
            format!("Fragment shader error: {}", e)
        })?;

        println!("Create Shader: {}", frag_shader.id());
        self.check_gl_error("Create Shader");


        //Create program from those shaders
        let program = Program::from_shaders(&[vert_shader, frag_shader]).
        map_err(|e| 
        {
            println!("Create Program compilation error: {}", e);
            format!("Create Program error: {}", e)
        })?;

        println!("Create Program: {}", program.id());
        self.check_gl_error("Create Program");

        let id = self.next_program_id;
        self.next_program_id += 1;
        self.programs.insert(id, program);
        Ok(id)
    }

    pub fn get_program(&self, id: u32) -> Option<&Program> {
        self.programs.get(&id)
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