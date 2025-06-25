use glam::Mat4;

use super::{ShaderManager, Mesh};
use super::render::Render;
use crate::core::{EntityManager, Transform};
use crate::rendering::MaterialManager;

// Rendering system - handles all rendering logic
pub struct RenderingSystem {
    shader_manager: ShaderManager,
    pub material_manager: MaterialManager,
}

impl RenderingSystem {
    pub fn new() -> Self {
        Self {
            shader_manager: ShaderManager::new(),
            material_manager: MaterialManager::new(),
        }
    }

    pub fn initialize(&mut self) -> Result<(), String> {
        //Create default program
        let _ = self.material_manager.create_default_materials();

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

            println!("Generated VAO: {}", vao);
            self.check_gl_error("GenVertexArrays");
            gl::GenBuffers(1, &mut vbo);

            println!("Generated VBO: {}", vbo);
            self.check_gl_error("GenBuffers VBO");
            gl::GenBuffers(1, &mut ebo);
            println!("Generated EBO: {}", ebo);
            self.check_gl_error("GenBuffers EBO");

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

            // Configure all attributes
            for attr in &mesh.attributes {
                gl::EnableVertexAttribArray(attr.location);
                gl::VertexAttribPointer(
                    attr.location,
                    attr.size,
                    gl::FLOAT,
                    gl::FALSE,
                    mesh.vertex_stride as gl::types::GLint,
                    attr.offset as *const gl::types::GLvoid,
                );
            }

            //Clear buffers. Note that ebo is not cleared as it is part of vao
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);

            mesh.vao = Some(vao);
            mesh.vbo = Some(vbo);
            mesh.ebo = Some(ebo);
        }
    }

    // Render all entities with renderable components
    pub fn render(&mut self, entity_manager: &mut EntityManager) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        // Collect entity IDs first to avoid borrowing conflicts
        let entities = entity_manager.get_renderable_entities();
        
        for entity_id in entities {
            if let (Some(transform), Some(render)) = (
            entity_manager.get_component::<Transform>(entity_id).cloned(), //Need to clone to fix mutability issue
            entity_manager.get_component_mut::<Render>(entity_id)
            ) 
            {
                self.render_entity(&transform, render);
            }
        }
    }

    fn render_entity(&mut self, transform: &Transform, render: &mut Render) {
        // Prepare mesh if not already prepared
        if render.mesh.vao.is_none() {
            self.prepare_mesh(&mut render.mesh);
        }

        // Get material or use default
        let material_id = render.material_id.unwrap_or(
            0
        );

        if let Some(material) = self.material_manager.get_material_mut(&material_id) {

            // Use glam to create the transformation matrix
            let translation = glam::Mat4::from_translation(transform.position);
            let rotation = glam::Mat4::from_quat(transform.rotation);
            let scale = glam::Mat4::from_scale(transform.scale);
            
            // Combine transformations: Translation * Rotation * Scale
            let model_matrix = translation * rotation * scale;

            // TODO: Update when camera system is implemented
            let identity_matrix = [
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ];

            let projection = Mat4::orthographic_rh(-20.0, 20.0, -20.0, 20.0, -1.0, 100.0);
        
            // Update model matrix from transform
            material.set_mat4("u_model", model_matrix.to_cols_array());
            material.set_mat4("u_view", identity_matrix);
            material.set_mat4("u_projection", projection.to_cols_array());

            //Color is static and should be set during initialization
            material.set_vec3("u_color", [0.1, 0.5, 0.5]);

            //Apply material
            material.apply();

            //Render Mesh
            self.render_mesh(&render.mesh);
        }      
    }

    pub fn create_shader_program(&mut self, vert_source: &str, frag_source: &str) -> Result<u32, String> {
        self.shader_manager.create_program(vert_source, frag_source)
    }

    fn render_mesh(&self, mesh: &Mesh) {
        // Render the mesh
        if let Some(vao) = mesh.vao {
            unsafe {
                gl::BindVertexArray(vao);
                gl::DrawElements(
                    gl::TRIANGLES,
                    mesh.indices.len() as i32,
                    gl::UNSIGNED_INT,
                    std::ptr::null(),
                );
                gl::BindVertexArray(0);
            }
        }
    }

    // Add this debugging function to check OpenGL errors
    fn check_gl_error(&self, operation: &str) {
        unsafe {
            let error = gl::GetError();
            if error != gl::NO_ERROR {
                println!("OpenGL error after {}: {}", operation, error);
            }
        }
    }
}