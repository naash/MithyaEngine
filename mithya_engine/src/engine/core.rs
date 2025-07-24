// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::{
    core::EntityManager, engine::system::{MovementSystem, SystemsManager}, input::{InputSystem, PlayerControlled}, physics::{
        collider::{Collider, ColliderShape}, 
        CollisionSystem, 
        PhysicsConfig, 
        PhysicsSystem, 
        RigidBody
    }, rendering::{MaterialManager, RenderingSystem}, ui::UiSystem, Component, Mesh, Render, Transform
};

use sdl2::{video::Window, EventPump, Sdl, event::Event};
use gl;
use glam::{Quat, Vec3};
use std::{collections::HashSet, time::Instant};

pub struct EngineConfig {
    pub window_title: String,
    pub window_width: u32,
    pub window_height: u32,
    pub gl_major_version: u8,
    pub gl_minor_version: u8,
    pub resizable: bool,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            window_title: "Mithya Engine".to_string(),
            window_width: 800,
            window_height: 600,
            gl_major_version: 4,
            gl_minor_version: 1,
            resizable: true,
        }
    }
}

pub struct Engine {
    _sdl: Sdl,
    window: Window,
    _gl_context: sdl2::video::GLContext,
    event_pump: EventPump,
    pub systems_manager: SystemsManager,
    pub world: World,
    frame_timer: FrameTimer,
}

pub struct World {
    pub input_state: InputState,
    pub entity_manager: EntityManager,
    pub physics_config: PhysicsConfig,
    pub material_manager: MaterialManager,
    pub fps: f32
}

#[derive(Default)]
pub struct InputState {
    pub movement: (f32, f32),
    pub keys_pressed: HashSet<sdl2::keyboard::Keycode>,
    pub mouse_position: (i32, i32),
}

// Helper struct for FPS calculation
struct FrameTimer {
    last_frame_time: Instant,
    last_fps_time: Instant,
    frame_count: u32,
    fps: f32,
    delta_time: f32,
}

impl FrameTimer {
    fn new() -> Self {
        let now = Instant::now();
        Self {
            last_frame_time: now,
            last_fps_time: now,
            frame_count: 0,
            fps: 0.0,
            delta_time: 0.0,
        }
    }
    
    fn update(&mut self) {
        let current_time = Instant::now();
        
        // Calculate delta time from last frame
        self.delta_time = current_time.duration_since(self.last_frame_time).as_secs_f32();
        self.last_frame_time = current_time;
        
        // Update FPS counter
        self.frame_count += 1;
        let fps_elapsed = current_time.duration_since(self.last_fps_time);
        
        if fps_elapsed.as_secs_f32() >= 1.0 {
            self.fps = self.frame_count as f32 / fps_elapsed.as_secs_f32();
            self.frame_count = 0;
            self.last_fps_time = current_time;
        }
    }
    
    fn get_fps(&self) -> f32 {
        self.fps
    }
    
    fn get_delta_time(&self) -> f32 {
        self.delta_time
    }
    
    // Optional: Cap delta time to prevent large jumps (useful for physics)
    fn get_capped_delta_time(&self, max_delta: f32) -> f32 {
        self.delta_time.min(max_delta)
    }
}

impl Engine {
    pub fn new(config: EngineConfig) -> Result<Self, Box<dyn std::error::Error>> {
        // Initialize SDL2
        let sdl = sdl2::init()?;
        let video_subsystem = sdl.video()?;

        // Set up OpenGL attributes
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(config.gl_major_version, config.gl_minor_version);

        // Create window
        let window = if config.resizable {
            video_subsystem
                .window(&config.window_title, config.window_width, config.window_height)
                .opengl()
                .resizable()
                .build()?
        } else {
            video_subsystem
                .window(&config.window_title, config.window_width, config.window_height)
                .opengl()
                .build()?
        };

        let gl_context = window.gl_create_context()?;
        let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

        // Initialize systems
        let mut systems_manager = SystemsManager::new();

        let mut world = World {
            input_state: InputState::default(),
            entity_manager: EntityManager::new(),
            physics_config: PhysicsConfig::default(),
            material_manager: MaterialManager::new(),
            fps: 0.0
        };

        // For input
        systems_manager.add_system(UiSystem::new(&window)); //UI will be at the top to consume event if required
        systems_manager.add_system(InputSystem::new()); //Input manager is then followed so that it caches input state on the world... There should be a better way?
        systems_manager.add_system(MovementSystem::default());
        
        // For rendering
        systems_manager.add_system(RenderingSystem::new(&window));
        
        // For physics
        systems_manager.add_system(PhysicsSystem);
        systems_manager.add_system(CollisionSystem); 
        

        systems_manager.initialize_all(&mut world);
        
        // Set viewport
        unsafe {
            gl::Viewport(0, 0, config.window_width as i32, config.window_height as i32);
        }

        // Get SDL2 event pump
        let event_pump = sdl.event_pump()?;

        Ok(Engine {
            _sdl: sdl,
            window,
            _gl_context: gl_context,
            event_pump,
            systems_manager,
            world,
            frame_timer: FrameTimer::new(),
        })
    }

    pub fn run<G: GameLogic>(mut self, mut game: G) -> Result<(), Box<dyn std::error::Error>> {
        // Let the game initialize itself
        game.initialize(&mut self.world);

        // Main game loop
        'main: loop {
            // Update frame timer
            self.frame_timer.update();
            let delta_time = self.frame_timer.get_delta_time();
           
            self.world.fps = self.frame_timer.fps;

            // Handle events
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'main,
                    _ => {
                        //Event order is fixed -> UI -> Input -> Movement
                        self.systems_manager.handle_event_all(&event, &mut self.world);
                    }
                }
            }

            // Update systems
            self.systems_manager.update_all(&mut self.world, delta_time);

            // Update game logic
            game.update(&mut self.world, delta_time);

            // Clear the screen
            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            }

            //Render all systems. NOTE: Rendering is done in reverse so that UI is rendered at the top
            self.systems_manager.render_all(&mut self.world);

            // Swap buffers
            self.window.gl_swap_window();
        }

        Ok(())
    }
}

pub trait GameLogic {
    fn initialize(&mut self, world: &mut World);
    fn update(&mut self, world: &mut World, delta_time: f32);
}

// Helper function for creating common entities
pub struct EntityBuilder<'a> {
    entity_manager: &'a mut EntityManager,
    entity_id: u32,
}

impl<'a> EntityBuilder<'a> {
    pub fn new(entity_manager: &'a mut EntityManager) -> Self {
        let entity_id = entity_manager.create_entity();
        Self { entity_manager, entity_id }
    }

    // Generic method to add any component
    pub fn with<T: Component>(self, component: T) -> Self {
        self.entity_manager.add_component(self.entity_id, component);
        self
    }

    pub fn build(self) -> u32 {
        self.entity_id
    }
}