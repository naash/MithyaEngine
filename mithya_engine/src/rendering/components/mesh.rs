// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serde::{Serialize, Deserialize};

#[derive(Clone, Debug)]
pub struct VertexAttribute {
    pub location: u32,
    pub size: i32,        // Number of components (1, 2, 3, or 4)
    pub offset: usize,    // Offset in bytes from start of vertex
}

// Mesh data - the actual geometry
#[derive(Clone, Debug)]
pub struct Mesh {
    pub vertices: Vec<f32>,
    pub indices: Vec<u32>,
    pub vao: Option<gl::types::GLuint>,
    pub vbo: Option<gl::types::GLuint>,
    pub ebo: Option<gl::types::GLuint>,
    pub vertex_stride: usize,           // Size of one vertex in bytes
    pub attributes: Vec<VertexAttribute>, // Flexible attribute definition
}

impl Mesh {
    pub fn new_triangle() -> Self {
        Self {
            vertices: vec![-5.0, -5.0, 0.0, 5.0, -5.0, 0.0, 0.0, 0.5, 0.0],
            indices: vec![0, 1, 2],
            vao: None,
            vbo: None,
            ebo: None,
            vertex_stride: 3 * std::mem::size_of::<f32>(),
            attributes: vec![
                VertexAttribute { location: 0, size: 3, offset: 0 }, // Position only
            ],
        }
    }

    pub fn new_quad() -> Self {
        Self {
            vertices: vec![
                -0.5, -0.5, 0.0,  // Bottom left
                 0.5, -0.5, 0.0,  // Bottom right
                 0.5,  0.5, 0.0,  // Top right
                -0.5,  0.5, 0.0,  // Top left
            ],
            indices: vec![0, 1, 2, 2, 3, 0],
            vao: None,
            vbo: None,
            ebo: None,
            vertex_stride: 3 * std::mem::size_of::<f32>(),
            attributes: vec![
                VertexAttribute { location: 0, size: 3, offset: 0 }, // Position only
            ],
        }
    }

    // Textured versions
    pub fn new_triangle_textured() -> Self {
        Self {
            vertices: vec![
                // Position   UV
                -0.5, -0.5, 0.0,  0.0, 0.0,  // Bottom left
                 0.5, -0.5, 0.0,  1.0, 0.0,  // Bottom right
                 0.0,  0.5, 0.0,  0.5, 1.0,  // Top center
            ],
            indices: vec![0, 1, 2],
            vao: None,
            vbo: None,
            ebo: None,
            vertex_stride: 5 * std::mem::size_of::<f32>(),
            attributes: vec![
                VertexAttribute { location: 0, size: 3, offset: 0 }, // Position
                VertexAttribute { location: 1, size: 2, offset: 3 * std::mem::size_of::<f32>() }, // UV
            ],
        }
    }

    pub fn new_quad_textured() -> Self {
        Self {
            vertices: vec![
                // Position    UV
                -0.5, -0.5, 0.0,  0.0, 0.0,  // Bottom left
                 0.5, -0.5, 0.0,  1.0, 0.0,  // Bottom right
                 0.5,  0.5, 0.0,  1.0, 1.0,  // Top right
                -0.5,  0.5, 0.0,  0.0, 1.0,  // Top left
            ],
            indices: vec![0, 1, 2, 2, 3, 0],
            vao: None,
            vbo: None,
            ebo: None,
            vertex_stride: 5 * std::mem::size_of::<f32>(),
            attributes: vec![
                VertexAttribute { location: 0, size: 3, offset: 0 }, // Position
                VertexAttribute { location: 1, size: 2, offset: 3 * std::mem::size_of::<f32>() }, // UV
            ],
        }
    }
}

// Enum to identify mesh types for serialization. Using this as a helper
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MeshType {
    Triangle,
    Quad, 
    TriangleTextured,
    QuadTextured,
    // Future mesh types can be added here
    Custom { name: String }, // For when you add 3D model loading
}

impl MeshType {
    // Helper to create mesh from type
    pub fn create_mesh(&self) -> Mesh {
        match self {
            MeshType::Triangle => Mesh::new_triangle(),
            MeshType::Quad => Mesh::new_quad(),
            MeshType::TriangleTextured => Mesh::new_triangle_textured(),
            MeshType::QuadTextured => Mesh::new_quad_textured(),
            MeshType::Custom { name: _ } => {
                // For now, default to quad_textured
                // Later you can load actual 3D models by name
                Mesh::new_quad_textured()
            }
        }
    }
    
    // Helper to detect mesh type from existing mesh (for serialization)
    pub fn from_mesh(mesh: &Mesh) -> Self {
        // Simple heuristic based on vertex count and stride
        match (mesh.vertices.len(), mesh.vertex_stride) {
            (9, 12) => MeshType::Triangle,      // 3 verts * 3 components, stride 12
            (12, 12) => MeshType::Quad,         // 4 verts * 3 components, stride 12  
            (15, 20) => MeshType::TriangleTextured, // 3 verts * 5 components, stride 20
            (20, 20) => MeshType::QuadTextured,     // 4 verts * 5 components, stride 20
            _ => MeshType::Custom { name: "unknown".to_string() },
        }
    }
}