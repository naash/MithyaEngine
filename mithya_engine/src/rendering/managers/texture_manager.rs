use std::collections::HashMap;

pub struct TextureManager {
    texture_cache: HashMap<String, u32>, // path -> OpenGL texture ID
}

impl TextureManager {
    pub fn new() -> Self {
        Self {
            texture_cache: HashMap::new(),
        }
    }

    pub fn load_texture(&mut self, path: &str) -> Result<u32, TextureLoadError> {
        // Return cached texture if already loaded
        
        if let Some(&texture_id) = self.texture_cache.get(path) {
            return Ok(texture_id);
        }

        let texture_id = self.create_gl_texture(path)?;
        println!("Generated texture ID: {}", texture_id);
        self.texture_cache.insert(path.to_string(), texture_id);
        Ok(texture_id)
    }

    fn create_gl_texture(&self, texture_name: &str) -> Result<u32, TextureLoadError> {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        //All textures need to be inside textures folder
        path.push("textures");
        path.push(texture_name);

        let img = image::open(&path)
        .map_err(|e| {
            println!("Failed to load image: {:?}", e);
            TextureLoadError::ImageError(e)
        })?
        .to_rgba8();
        
        let (width, height) = img.dimensions();
        let mut texture_id = 0;

        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
            
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                img.as_ptr() as *const std::ffi::c_void,
            );

            gl::GenerateMipmap(gl::TEXTURE_2D);
            
            // Set texture parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
        }

        Ok(texture_id)
    }

    pub fn cleanup(&mut self) {
        unsafe {
            for &texture_id in self.texture_cache.values() {
                gl::DeleteTextures(1, &texture_id);
            }
        }
        self.texture_cache.clear();
    }
}

#[derive(Debug)]
pub enum TextureLoadError {
    ImageError(image::ImageError),
    IoError(std::io::Error),
}