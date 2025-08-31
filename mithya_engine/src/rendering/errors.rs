// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use thiserror::Error;

#[derive(Error, Debug)]
pub enum RenderingError {
    #[error("Renderer not initialized")]
    NotInitialized,
    
    #[error("Invalid render target")]
    InvalidRenderTarget,
    
    #[error("Mesh rendering failed: {0}")]
    MeshRender(String),
    
    #[error("UI rendering failed: {0}")]
    UIRender(String),
    
    #[error("Frame buffer error: {0}")]
    FrameBuffer(String),
    
    #[error("OpenGL error: {0}")]
    OpenGL(#[from] gl::OpenGLError),
}

pub mod gl {
    use thiserror::Error;
    
    #[derive(Error, Debug)]
    pub enum OpenGLError {
        #[error("OpenGL error code: {0}")]
        GLError(u32),
        
        #[error("Failed to load OpenGL function: {0}")]
        FunctionLoad(String),
        
        #[error("Unsupported OpenGL version: required {required}, found {found}")]
        UnsupportedVersion { required: String, found: String },
        
        #[error("Invalid viewport dimensions: width={width}, height={height}")]
        InvalidViewport { width: i32, height: i32 },
        
        #[error("Buffer operation failed: {0}")]
        BufferError(String),
    }
    
    pub fn check_gl_error() -> Result<(), OpenGLError> {
        unsafe {
            let error = gl::GetError();
            if error != gl::NO_ERROR {
                Err(OpenGLError::GLError(error))
            } else {
                Ok(())
            }
        }
    }
}