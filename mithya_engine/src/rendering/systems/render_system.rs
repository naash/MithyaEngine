// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use glam::Mat4;
use sdl2::event::Event;
use sdl2::video::Window;

use crate::{
    asset::{managers::AssetManager, MaterialData, UniformValue}, 
    core::Transform, engine::system::{System, SystemRenderContext, SystemUpdateContext}, rendering::components::{ Mesh, Render}, World
};

// Rendering system - handles all rendering logic
pub struct RenderingSystem {
    pub aspect_ratio: f32,
}

impl RenderingSystem {
    pub fn new(window: &Window) -> Self {

        let (width, height) = window.size();

        Self {
            aspect_ratio: width as f32 / height as f32
        }
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

    fn render_entity(&mut self, transform: &Transform, render: &mut Render, asset_manager: &mut AssetManager) {
        // Prepare mesh if not already prepared

        if render.mesh.vao.is_none() {
            self.prepare_mesh(&mut render.mesh);
        }

        // Get material or use default
        let material_id = render.material_id.unwrap_or(
            0
        );

        if let Some(material) = asset_manager.get_material_mut(material_id) {

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

            let projection = Mat4::orthographic_rh(-20.0 * self.aspect_ratio, 20.0 * self.aspect_ratio, -20.0, 20.0, -1.0, 1.0);

            material.uniforms.insert("u_model".to_string(), UniformValue::Mat4(model_matrix.to_cols_array()));
            material.uniforms.insert("u_view".to_string(), UniformValue::Mat4(identity_matrix));
            material.uniforms.insert("u_projection".to_string(), UniformValue::Mat4(projection.to_cols_array()));
            material.uniforms.insert("u_color".to_string(), UniformValue::Vec3([0.1, 0.5, 0.5]));

            // Apply the material
            self.apply_material(material);
            self.render_mesh(&render.mesh);
        }      
    }

    fn apply_material(&self, material: &MaterialData) {
        // Apply shader program
        if let Some(program_id) = material.shader_program_id {
            unsafe {
                gl::UseProgram(program_id);
                //self.debug_shader_uniforms(program_id);
            }
        }

        // Apply uniforms
        for (name, value) in &material.uniforms {
            unsafe {
                
                let location = gl::GetUniformLocation(
                    material.shader_program_id.unwrap(),
                    std::ffi::CString::new(name.as_str()).unwrap().as_ptr()
                );

                if location != -1 {
                    match value {
                        UniformValue::Mat4(v) => gl::UniformMatrix4fv(location, 1, gl::FALSE, v.as_ptr()),
                        UniformValue::Vec2(v) => gl::Uniform2fv(location, 1, v.as_ptr()),
                        UniformValue::Vec3(v) => gl::Uniform3fv(location, 1, v.as_ptr()),
                        UniformValue::Vec4(v) => gl::Uniform4fv(location, 1, v.as_ptr()),
                        UniformValue::Float(v) => gl::Uniform1f(location, *v),
                        UniformValue::Int(v) => gl::Uniform1i(location, *v),
                        UniformValue::Bool(v) => gl::Uniform1i(location, if *v { 1 } else { 0 }),
                    }
                }
            }
        }

        // Apply textures
        for (_name, binding) in &material.textures {
            unsafe {
                gl::ActiveTexture(gl::TEXTURE0 + binding.slot);
                gl::BindTexture(gl::TEXTURE_2D, binding.texture_id);
            }
        }
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

    fn debug_shader_uniforms(&self, program_id: u32) {
    unsafe {
        let mut uniform_count = 0;
        gl::GetProgramiv(program_id, gl::ACTIVE_UNIFORMS, &mut uniform_count);
        println!("Shader program {} has {} active uniforms:", program_id, uniform_count);
        
        for i in 0..uniform_count {
            let mut name = vec![0u8; 256];
            let mut length = 0;
            let mut size = 0;
            let mut uniform_type = 0;
            
            gl::GetActiveUniform(
                program_id,
                i as u32,
                256,
                &mut length,
                &mut size,
                &mut uniform_type,
                name.as_mut_ptr() as *mut i8,
            );
            
            name.truncate(length as usize);
            let uniform_name = String::from_utf8_lossy(&name);
            let location = gl::GetUniformLocation(
                program_id,
                std::ffi::CString::new(uniform_name.as_ref()).unwrap().as_ptr()
            );
            
            println!("  Uniform {}: '{}' (location: {})", i, uniform_name, location);
        }
    }
}
}

impl System for RenderingSystem
{
    fn initialize(&mut self, _world: &mut World) -> Result<(), Box<dyn std::error::Error>> {
        
        //Create default program
        let _ = _world.asset_manager.create_default_materials();

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::ClearColor(0.0, 0.3, 0.5, 1.0);
        }

        Ok(())
    }

    fn update(&mut self, _update_context: &mut SystemUpdateContext) {
        //Nothing to update just render
    }

    fn render(&mut self, render_context: &mut SystemRenderContext) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        // Collect entity IDs first to avoid borrowing conflicts
        let entities = render_context.entity_manager.get_renderable_entities();
        
        for entity_id in entities {
            if let (Some(transform), Some(render)) = (
            render_context.entity_manager.get_component::<Transform>(entity_id).cloned(), //Need to clone to fix mutability issue
            render_context.entity_manager.get_component_mut::<Render>(entity_id)
            ) 
            {
                self.render_entity(&transform, render, &mut render_context.asset_manager);
            }
        }
    }

    fn cleanup(&mut self, _world: &mut World) {
        // Any cleanup logic specific to the UI system
        // The painter and context will be dropped automatically
    }
}