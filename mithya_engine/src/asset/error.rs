// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AssetError {
    #[error("Asset directory not found: {0}")]
    DirectoryNotFound(String),
    
    #[error("Asset not found: {0}")]
    AssetNotFound(String),
    
    #[error("Failed to load asset: {0}")]
    LoadError(String),
    
    #[error("Invalid asset format: {0}")]
    InvalidFormat(String),

    #[error("Material error: {0}")]
    MaterialError(MaterialError),

    #[error("Texture error: {0}")]
    TextureLoadError(TextureLoadError),

    #[error("Shader error: {0}")]
    ShaderError(ShaderError),
}

#[derive(Error, Debug)]
pub enum MaterialError {
    #[error("Material not found: {0}")]
    MaterialNotFound(String),

    #[error("Invalid shader program")]
    InvalidShaderProgram,

    #[error("Invalid texture slot: {0}")]
    InvalidTextureSlot(u32),
}

#[derive(Error, Debug)]
pub enum TextureLoadError {
    #[error("Image error: {0}")]
    ImageError(image::ImageError),
    #[error("Unable to import texture: {0}")]
    IoError(std::io::Error),
}

#[derive(Error, Debug)]
pub enum ShaderError {
    #[error("Failed to compile shader: {0}")]
    CompilationError(String),

    #[error("Failed to link program: {0}")]
    LinkError(String),

    #[error("Shader file not found: {0}")]
    FileNotFound(String),

    #[error("Invalid shader source: {0}")]
    InvalidSource(String),

     #[error("Invalid program: {0}")]
    ProgramError(String)
}

impl From<MaterialError> for AssetError { 
    fn from(err: MaterialError) -> Self { 
        AssetError::MaterialError(err) 
    } 
}

impl From<TextureLoadError> for AssetError { 
    fn from(err: TextureLoadError) -> Self { 
        AssetError::TextureLoadError(err) 
    } 
}

impl From<ShaderError> for AssetError { 
    fn from(err: ShaderError) -> Self { 
        AssetError::ShaderError(err) 
    } 
}