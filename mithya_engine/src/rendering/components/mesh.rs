// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use wgpu::util::DeviceExt;


#[derive(Clone, Debug)]
pub struct VertexAttribute {
    pub location: u32,
    pub size: i32,        // Number of components (1, 2, 3, or 4)
    pub offset: usize,    // Offset in bytes from start of vertex
}

// Mesh data - the actual geometry
#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<f32>,
    pub indices: Vec<u32>,
    pub vertex_buffer: Option<wgpu::Buffer>,
    pub index_buffer: Option<wgpu::Buffer>,
    pub vertex_stride: usize,
    pub attributes: Vec<VertexAttribute>,
    pub wgpu_attributes: Vec<wgpu::VertexAttribute>
}

impl Mesh {
    pub fn upload(&mut self, device: &wgpu::Device) {
        self.build_wgpu_attributes();
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&self.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&self.indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        self.vertex_buffer = Some(vertex_buffer);
        self.index_buffer = Some(index_buffer);
    }

    pub fn is_uploaded(&self) -> bool {
        self.vertex_buffer.is_some()
    }

    pub fn build_wgpu_attributes(&mut self) {
        self.wgpu_attributes = self.attributes.iter().map(|a| {
            wgpu::VertexAttribute {
                shader_location: a.location,
                offset: a.offset as u64,
                format: match a.size {
                    1 => wgpu::VertexFormat::Float32,
                    2 => wgpu::VertexFormat::Float32x2,
                    3 => wgpu::VertexFormat::Float32x3,
                    4 => wgpu::VertexFormat::Float32x4,
                    _ => panic!("Unsupported vertex attribute size: {}", a.size),
                },
            }
        }).collect();
    }

    pub fn vertex_buffer_layout(&self) -> wgpu::VertexBufferLayout<'_> {
        wgpu::VertexBufferLayout {
            array_stride: self.vertex_stride as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &self.wgpu_attributes,  // reference to stored vec, no leak
        }
    }

    pub fn new_triangle() -> Self {
        Self {
            vertices: vec![-5.0, -5.0, 0.0, 5.0, -5.0, 0.0, 0.0, 0.5, 0.0],
            indices: vec![0, 1, 2],
            vertex_buffer: None,
            index_buffer: None,
            vertex_stride: 3 * std::mem::size_of::<f32>(),
            attributes: vec![
                VertexAttribute { location: 0, size: 3, offset: 0 },
            ],
            wgpu_attributes : Vec::new()
        }
    }

    pub fn new_quad() -> Self {
        Self {
            vertices: vec![
                -0.5, -0.5, 0.0,
                 0.5, -0.5, 0.0,
                 0.5,  0.5, 0.0,
                -0.5,  0.5, 0.0,
            ],
            indices: vec![0, 1, 2, 2, 3, 0],
            vertex_buffer: None,
            index_buffer: None,
            vertex_stride: 3 * std::mem::size_of::<f32>(),
            attributes: vec![
                VertexAttribute { location: 0, size: 3, offset: 0 },
            ],
            wgpu_attributes : Vec::new()
        }
    }

    pub fn new_triangle_textured() -> Self {
        Self {
            vertices: vec![
                -0.5, -0.5, 0.0,  0.0, 0.0,
                 0.5, -0.5, 0.0,  1.0, 0.0,
                 0.0,  0.5, 0.0,  0.5, 1.0,
            ],
            indices: vec![0, 1, 2],
            vertex_buffer: None,
            index_buffer: None,
            vertex_stride: 5 * std::mem::size_of::<f32>(),
            attributes: vec![
                VertexAttribute { location: 0, size: 3, offset: 0 },
                VertexAttribute { location: 1, size: 2, offset: 3 * std::mem::size_of::<f32>() },
            ],
            wgpu_attributes : Vec::new()
        }
    }

    pub fn new_quad_textured() -> Self {
        Self {
            vertices: vec![
                -0.5, -0.5, 0.0,  0.0, 1.0,
                 0.5, -0.5, 0.0,  1.0, 1.0,
                 0.5,  0.5, 0.0,  1.0, 0.0,
                -0.5,  0.5, 0.0,  0.0, 0.0,
            ],
            indices: vec![0, 1, 2, 2, 3, 0],
            vertex_buffer: None,
            index_buffer: None,
            vertex_stride: 5 * std::mem::size_of::<f32>(),
            attributes: vec![
                VertexAttribute { location: 0, size: 3, offset: 0 },
                VertexAttribute { location: 1, size: 2, offset: 3 * std::mem::size_of::<f32>() },
            ],
            wgpu_attributes : Vec::new()
        }
    }
}
