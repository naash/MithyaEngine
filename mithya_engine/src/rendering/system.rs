use super::{ShaderManager, Mesh};
use super::components::Renderable;
use crate::core::{EntityManager, Transform};

// Rendering system - handles all rendering logic
pub struct RenderingSystem {
    shader_manager: ShaderManager,
    default_shader_id: Option<u32>,
}

impl RenderingSystem {
    pub fn new() -> Self {
        Self {
            shader_manager: ShaderManager::new(),
            default_shader_id: None,
        }
    }

    pub fn initialize(&mut self) -> Result<(), String> {
        // Create default shader program
        let vert_source = include_str!("../../shaders/triangle.vert");
        let frag_source = include_str!("../../shaders/triangle.frag");
        
        let shader_id = self.shader_manager.create_program(vert_source, frag_source)?;
        self.default_shader_id = Some(shader_id);
        
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::ClearColor(0.0, 0.3, 0.5, 1.0);
        }
        
        Ok(())
    }

    // Prepare mesh for rendering by creating OpenGL buffers
    pub fn prepare_mesh(&self, mesh: &mut Mesh) {
        unsafe {
            let mut vao = 0;
            let mut vbo = 0;
            let mut ebo = 0;

            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            gl::BindVertexArray(vao);

            // Vertex buffer
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (mesh.vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                mesh.vertices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );

            // Element buffer
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (mesh.indices.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
                mesh.indices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );

            // Vertex attributes
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (3 * std::mem::size_of::<f32>()) as gl::types::GLint,
                std::ptr::null(),
            );

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);

            mesh.vao = Some(vao);
            mesh.vbo = Some(vbo);
            mesh.ebo = Some(ebo);
        }
    }

    // Render all entities with renderable components
    pub fn render(&self, entity_manager: &mut EntityManager) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        // Collect entity IDs first to avoid borrowing conflicts
        let entities = entity_manager.get_renderable_entities();
        
        for entity_id in entities {
        if let Some((transform, renderable)) = entity_manager.get_transform_and_renderable_mut(entity_id) {
            self.render_entity(transform, renderable);
        }
    }
    }

    fn render_entity(&self, _transform: &Transform, renderable: &mut Renderable) {
        // Prepare mesh if not already prepared
        if renderable.mesh.vao.is_none() {
            self.prepare_mesh(&mut renderable.mesh);
        }

        // Use shader program
        let shader_id = renderable.shader_program_id.unwrap_or(
            self.default_shader_id.expect("No default shader available")
        );
        
        if let Some(program) = self.shader_manager.get_program(shader_id) {
            program.set_used();
        }

        //TODO use Transform
    
        // Render the mesh
        if let Some(vao) = renderable.mesh.vao {
            unsafe {
                gl::BindVertexArray(vao);
                gl::DrawElements(
                    gl::TRIANGLES,
                    renderable.mesh.indices.len() as i32,
                    gl::UNSIGNED_INT,
                    std::ptr::null(),
                );
                gl::BindVertexArray(0);
            }
        }
    }

    pub fn create_shader_program(&mut self, vert_source: &str, frag_source: &str) -> Result<u32, String> {
        self.shader_manager.create_program(vert_source, frag_source)
    }
}