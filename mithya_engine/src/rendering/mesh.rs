// Mesh data - the actual geometry
#[derive(Clone, Debug)]
pub struct Mesh {
    pub vertices: Vec<f32>,
    pub indices: Vec<u32>,
    pub vao: Option<gl::types::GLuint>,
    pub vbo: Option<gl::types::GLuint>,
    pub ebo: Option<gl::types::GLuint>,
}

impl Mesh {
    pub fn new_triangle() -> Self {
        Self {
            vertices: vec![-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0],
            indices: vec![0, 1, 2],
            vao: None,
            vbo: None,
            ebo: None,
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
        }
    }
}