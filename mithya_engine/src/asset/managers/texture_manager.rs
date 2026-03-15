// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::collections::HashMap;
use crate::asset::error::TextureLoadError;

pub struct TextureEntry {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
}

pub struct TextureManager {
    path_to_id: HashMap<String, u32>,
    textures_by_id: HashMap<u32, TextureEntry>,
    next_id: u32,
    asset_root: std::path::PathBuf,
}

impl TextureManager {
    pub fn new(asset_root: std::path::PathBuf) -> Self {
        Self {
            path_to_id: HashMap::new(),
            textures_by_id: HashMap::new(),
            next_id: 1,
            asset_root,
        }
    }

    pub fn load_texture(
        &mut self,
        path: &str,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Result<u32, TextureLoadError> {
        // Return cached id if already loaded
        if let Some(&id) = self.path_to_id.get(path) {
            return Ok(id);
        }

        let entry = self.create_wgpu_texture(path, device, queue)?;
        let id = self.next_id;
        self.next_id += 1;

        println!("Loaded texture: {} -> id {}", path, id);
        self.path_to_id.insert(path.to_string(), id);
        self.textures_by_id.insert(id, entry);
        Ok(id)
    }

    pub fn get_texture(&self, id: &u32) -> Option<&TextureEntry> {
        self.textures_by_id.get(id)
    }

    fn create_wgpu_texture(
        &self,
        texture_name: &str,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Result<TextureEntry, TextureLoadError> {
        let path = self.asset_root.join("textures").join(texture_name);

        let img = image::open(&path)
            .map_err(|e| {
                println!("Failed to load image: {:?}", e);
                TextureLoadError::ImageError(e)
            })?
            .to_rgba8();

        let (width, height) = img.dimensions();
        let size = wgpu::Extent3d { width, height, depth_or_array_layers: 1 };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some(texture_name),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &img,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        Ok(TextureEntry { texture, view })
    }

    pub fn cleanup(&mut self) {
        self.textures_by_id.clear();
        self.path_to_id.clear();
    }
}