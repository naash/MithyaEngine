// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use super::Mesh;

#[derive(Debug)]
pub struct RenderGpuCache {
    // Transform changes every frame — buffer stays alive, we write_buffer into it
    pub transform_buffer: wgpu::Buffer,
    pub transform_bind_group: wgpu::BindGroup,

    // Material data — created once, only rebuilt if material changes
    pub material_bind_group: wgpu::BindGroup,
}

// Renderable component - marks an entity as something that should be rendered
#[derive(Debug)]
pub struct Render {
    pub mesh: Mesh,
    pub material_id: Option<u32>,
    pub gpu_cache: Option<RenderGpuCache>,
}