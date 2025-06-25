use glam::Vec3;

use crate::core::{EntityManager, Transform};
use crate::rendering::{RenderingSystem, Render, Mesh};

// Updated main display function using the new architecture
pub fn display_window() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 1);

    let window = video_subsystem
        .window("Mithya Engine", 2048, 1024)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    // Initialize systems
    let mut entity_manager = EntityManager::new();
    let mut rendering_system = RenderingSystem::new();
    rendering_system.initialize().expect("Failed to initialize rendering system");

    // Create some entities
    let quad_entity = entity_manager.create_entity();
    entity_manager.add_component(quad_entity, Transform {
        position: Vec3::new(1.0, 0.0, 10.0),
        ..Default::default()
    });
    entity_manager.add_component(quad_entity, Render {
        mesh: Mesh::new_quad_textured(),
        material_id : rendering_system.material_manager.get_material_id_from_name("unlit_texture_default").copied(),
    });

    let triangle_entity = entity_manager.create_entity();
    
    entity_manager.add_component(triangle_entity, Transform {
        position:  Vec3::new(0.0, 10.0, 0.0),
        ..Default::default()
    });
    entity_manager.add_component(triangle_entity, Render {
        mesh: Mesh::new_triangle(),
        material_id : rendering_system.material_manager.get_material_id_from_name("unlit_color").copied(),
    });

    unsafe {
        gl::Viewport(0, 0, 980, 640);
    }

    // Main loop
    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        // Render all entities
        rendering_system.render(&mut entity_manager);
        window.gl_swap_window();
    }
}