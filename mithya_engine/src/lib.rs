/*extern crate gl;
extern crate sdl2;

pub mod render_gl;

pub fn display_window() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 1);

    let window = video_subsystem
        .window("Game", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let _gl =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    // set up shader program
    use std::ffi::CString;
    let vert_shader =
        render_gl::Shader::from_vert_source(&CString::new(include_str!("triangle.vert")).unwrap())
            .unwrap();

    let frag_shader =
        render_gl::Shader::from_frag_source(&CString::new(include_str!("triangle.frag")).unwrap())
            .unwrap();

    let shader_program = render_gl::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

    // Quad vertex data (4 vertices forming a square)
    let vertices: Vec<f32> = vec![
        // Bottom left
        -0.5, -0.5, 0.0,
        // Bottom right  
         0.5, -0.5, 0.0,
        // Top right
         0.5,  0.5, 0.0,
        // Top left
        -0.5,  0.5, 0.0,
    ];

    // Indices for drawing two triangles to form a quad
    let indices: Vec<u32> = vec![
        0, 1, 2,  // First triangle (bottom-left, bottom-right, top-right)
        2, 3, 0   // Second triangle (top-right, top-left, bottom-left)
    ];

    let mut vbo: gl::types::GLuint = 0;
    let mut ebo: gl::types::GLuint = 0; // Element Buffer Object for indices

    unsafe {
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);
    }

    unsafe {
        // Set up vertex buffer
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );

        // Set up element buffer
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
            indices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
    }

    // set up vertex array object
    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
        
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo); // Bind EBO to VAO
        
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

        gl::Viewport(0, 0, 900, 700);
        gl::ClearColor(0.0, 0.3, 0.5, 1.0);
    }

    // main loop
    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        shader_program.set_used();
        unsafe {
            gl::BindVertexArray(vao);
            // Draw quad using indexed drawing
            gl::DrawElements(
                gl::TRIANGLES,
                6, // number of indices (2 triangles * 3 vertices each)
                gl::UNSIGNED_INT, // type of indices
                std::ptr::null(), // offset
            );
        }

        window.gl_swap_window();
    }
}
*/

pub mod core;
pub mod rendering;
pub mod components;
pub mod window;

// Re-export commonly used types for easier access
pub use core::{EntityManager, EntityId, Transform};
pub use components::Component;
pub use rendering::{RenderingSystem, Mesh, Renderable};
pub use window::display_window;

// Prelude module - common imports users will want
pub mod prelude {
    pub use crate::{
        EntityManager, EntityId, Transform,
        Component, RenderingSystem, Mesh, Renderable,
        display_window
    };
}