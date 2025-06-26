use glam::{Quat, Vec3};

use crate::core::{EntityManager, Transform};
use crate::input::system::movement_system;
use crate::rendering::{RenderingSystem, Render, Mesh};
use crate::input::{InputManager, PlayerControlled}; 

pub fn run() {
    // Initialize SDL2
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    // Set up OpenGL attributes
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 1);

    // Create window
    let window = video_subsystem
        .window("Mithya Engine", 2048, 1024)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

      // Initialize input manager
    let mut input_manager = InputManager::new();

    // Initialize systems
    let mut entity_manager = EntityManager::new();
    let mut rendering_system = RenderingSystem::new();
    rendering_system.initialize().expect("Failed to initialize rendering system");

    // Create some entities
    // Create PLAYER entity (this one will move)
    let player_entity = entity_manager.create_entity();
    entity_manager.add_component(player_entity, Transform {
        position: Vec3::new(0.0, 0.0, 0.0), // Start at center
        rotation: Quat::IDENTITY,
        scale: Vec3::new(1.0, 1.0, 1.0)
    });
    entity_manager.add_component(player_entity, Render {
        mesh: Mesh::new_quad_textured(),
        material_id: rendering_system.material_manager
            .get_material_id_from_name("unlit_texture_default")
            .copied(),
    });
    // This is the key - mark it as player controlled
    entity_manager.add_component(player_entity, PlayerControlled);

    let triangle_entity = entity_manager.create_entity();
    entity_manager.add_component(triangle_entity, Transform {
        position: Vec3::new(0.0, 10.0, 0.0),
        ..Default::default()
    });
    entity_manager.add_component(triangle_entity, Render {
        mesh: Mesh::new_triangle(),
        material_id: rendering_system.material_manager.get_material_id_from_name("unlit_color").copied(),
    });

    // Set viewport
    unsafe {
        gl::Viewport(0, 0, 2048, 1024); // Match your window size
    }

    // Get SDL2 event pump
    let mut event_pump = sdl.event_pump().unwrap();

    // Main game loop
    'main: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                sdl2::event::Event::KeyDown { keycode: Some(keycode), .. } => {
                    input_manager.handle_key_down(keycode);
                }
                sdl2::event::Event::KeyUp { keycode: Some(keycode), .. } => {
                    input_manager.handle_key_up(keycode);
                }
                _ => {}
            }
        }

        // Updates go here
        movement_system(&input_manager, &mut entity_manager);
        
        // Clear the screen
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        // Render all entities
        rendering_system.render(&mut entity_manager);
        
        // Swap buffers
        window.gl_swap_window();

        input_manager.clear();
    }
}