// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use super::Mesh;

#[derive(Debug)]
pub struct TransformGpuCache {
    // Transform changes every frame — buffer stays alive, we write_buffer into it
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup
}

#[derive(Debug)]
pub struct MaterialGpuCache {
    pub bind_group: wgpu::BindGroup,
    pub cached_id: Option<u32>,
    pub has_textures: bool,
    pub tint_buffer: Option<wgpu::Buffer>,
}


#[derive(Debug)]
pub struct RenderGpuCache {
    pub transform: TransformGpuCache,
    pub material: MaterialGpuCache,
}

// Renderable component - marks an entity as something that should be rendered
#[derive(Debug)]
pub struct Render {
    pub mesh: Mesh,
    pub material_id: Option<u32>,
    pub tint: Option<[f32; 4]>,
}