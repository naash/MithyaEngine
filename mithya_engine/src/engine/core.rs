// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::{
    asset::AssetManager,
    core::{
        engine_events::{EngineActionQueue, EngineEventQueue, KeyModifiers, KeyPressedEvent, KeyReleasedEvent, MouseButtonReleasedEvent, MouseClickEvent, MouseMoveEvent, MouseWheelEvent, TextInputEvent, WindowResizedEvent
        },
        EntityManager
    }, 
    engine::system::{SystemRenderContext, SystemUpdateContext, SystemsManager},
    input::InputSystem, physics::{
        CollisionSystem, 
        PhysicsConfig, 
        PhysicsSystem
    }, player::PlayerControlSystem, rendering::RenderingSystem, ui::UISystem, Component
};

use glam::Vec2;
use sdl2::{event::Event, video::Window, EventPump, Sdl};
use gl;
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
    pub action_queue: EngineActionQueue,    
    pub event_queue: EngineEventQueue,
}

pub struct World {
    pub input_state: InputState,
    pub entity_manager: EntityManager,
    pub physics_config: PhysicsConfig,
    pub asset_manager: AssetManager,
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
  
    fn get_delta_time(&self) -> f32 {
        self.delta_time
    }
}

impl Engine {

    pub fn process_sdl_events(&mut self, should_quit : &mut bool ) {
        for sdl_event in self.event_pump.poll_iter() {
            match sdl_event {
                Event::Quit { .. } => {
                    //No need to create event here, simply quit
                    *should_quit = true;
                }
                Event::KeyDown { keycode: Some(key), keymod, repeat: false, .. } => {
                    self.event_queue.push(KeyPressedEvent { key, modifiers: KeyModifiers::from_sdl(keymod) });
                }
                Event::KeyUp { keycode: Some(key), keymod, .. } => {
                    self.event_queue.push(KeyReleasedEvent { key, modifiers: KeyModifiers::from_sdl(keymod) });
                }
                Event::MouseButtonDown { x, y, mouse_btn, .. } => {
                    self.event_queue.push(MouseClickEvent {
                        position: Vec2::new(x as f32, y as f32),
                        button: mouse_btn,
                    });
                }
                Event::MouseButtonUp { x, y, mouse_btn, .. } => {
                    self.event_queue.push(MouseButtonReleasedEvent {
                        position: Vec2::new(x as f32, y as f32),
                        button: mouse_btn,
                    });
                }
                Event::MouseMotion { x, y, .. } => {
                    self.event_queue.push(MouseMoveEvent {
                        position: Vec2::new(x as f32, y as f32),
                    });
                }
                Event::MouseWheel { x, y, .. } => {
                    self.event_queue.push(MouseWheelEvent {
                        delta_x: x,
                        delta_y: y,
                    });
                }
                Event::TextInput { text, .. } => {
                    self.event_queue.push(TextInputEvent { text });
                }
                Event::Window { win_event: sdl2::event::WindowEvent::Resized(width, height), .. } => {
                    self.event_queue.push(WindowResizedEvent {
                        width: width as u32,
                        height: height as u32,
                    });
                }
                _ => {}
            }
        }
    }

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
            asset_manager: AssetManager::new()?,
            fps: 0.0
        };

        // For input
        systems_manager.add_system(UISystem::new(&window)); //UI will be at the top to consume event if required
        systems_manager.add_system(InputSystem::new()); //Input manager is then followed so that it caches input state on the world... There should be a better way?
        systems_manager.add_system(PlayerControlSystem::default());
        
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
            event_queue: EngineEventQueue::new(),
            action_queue: EngineActionQueue::new()
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

            let mut should_quit = false;

            self.process_sdl_events(&mut should_quit);

            if should_quit {
                break 'main;
            }

            //Uncomment to debug events
            //self.event_queue.debug_print();

            self.systems_manager.handle_event_all(&mut self.event_queue, &mut self.action_queue);

            self.action_queue.execute_all(&mut self.world);

            // Create update context
            let mut update_context = SystemUpdateContext {
                world: &mut self.world,
                events: &mut self.event_queue,
                delta_time: delta_time,
            };

            // Update systems
            self.systems_manager.update_all(&mut update_context);

            // Update game logic
            game.update(&mut self.world, delta_time);

            // Clear the screen
            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            }

            //Create Render context
            let mut render_context = SystemRenderContext {
                entity_manager : &mut self.world.entity_manager,
                asset_manager : &mut self.world.asset_manager
            };
            
            //Render all systems. NOTE: Rendering is done in reverse so that UI is rendered at the top
            self.systems_manager.render_all(&mut render_context);

            // Swap buffers
            self.window.gl_swap_window();

            //Clear actions
            self.action_queue.clear();
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