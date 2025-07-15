// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::{
    core::EntityManager,
    engine::system::{MovementSystem, SystemsManager},
    input::{input_manager::InputManager, PlayerControlled}, 
    physics::{collider::{Collider, ColliderShape}, CollisionSystem, PhysicsConfig, PhysicsSystem, RigidBody},
    rendering::RenderingSystem, Mesh, Render, Transform
};

use sdl2::{video::Window, EventPump, Sdl};
use gl;
use glam::{Quat, Vec3};
use egui_sdl2_gl::{painter::Painter, ShaderVersion};
use egui_sdl2_gl::egui; // Use egui from the same crate

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
    pub world: World
}

pub struct World {
    pub input_manager: InputManager,
    pub entity_manager: EntityManager,
    pub rendering_system: RenderingSystem,
    pub physics_config: PhysicsConfig,
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
            input_manager: InputManager::new(),
            entity_manager: EntityManager::new(),
            rendering_system: RenderingSystem::new(),
            physics_config: PhysicsConfig::default()
        };
        
        world.rendering_system.initialize(config.window_width, config.window_height)?;

        //For input
        systems_manager.add_system(MovementSystem);
        //For physics
        systems_manager.add_system(PhysicsSystem);
        systems_manager.add_system(CollisionSystem); 
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
            world
        })
    }

    pub fn run<G: GameLogic>(mut self, mut game: G) -> Result<(), Box<dyn std::error::Error>> {
        // Let the game initialize itself
        game.initialize(&mut self.world);

        // In your initialization:
        let mut painter = Painter::new(&self.window, 1.0, ShaderVersion::Default);
        let egui_ctx = egui::Context::default();

        // Main game loop
        'main: loop {
            // Handle events
            for event in self.event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit { .. } => break 'main,
                    sdl2::event::Event::KeyDown { keycode: Some(keycode), .. } => {
                        self.world.input_manager.handle_key_down(keycode);
                    }
                    sdl2::event::Event::KeyUp { keycode: Some(keycode), .. } => {
                        self.world.input_manager.handle_key_up(keycode);
                    }
                    _ => {}
                }
            }

            //Update systems
            let _ = &self.systems_manager.update_all(&mut self.world);

            // Update game logic
            game.update(&mut self.world);

            // Clear the screen
            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            }

            // Render all entities
            self.world.rendering_system.render(&mut self.world.entity_manager);

            // Get window size
            let (window_width, window_height) = self.window.drawable_size();

            let raw_input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                   egui::vec2(window_width as f32, window_height as f32),
                )),
                ..Default::default()
            };

            // Run egui - Fix me
            let full_output = egui_ctx.run(raw_input, |ctx| {
                egui::Window::new("Debug Info")
                    .default_size(egui::vec2(200.0, 100.0))
                    .default_pos(egui::pos2(20.0, 20.0))
                    .show(ctx, |ui| {
                        ui.label("Hello egui!");
                        ui.label("This should be visible!");
                        if ui.button("Test Button").clicked() {
                            println!("Button clicked!");
                        }
                    });
            });

            // Convert shapes to primitives
            let primitives = egui_ctx.tessellate(full_output.shapes, full_output.pixels_per_point);

            // Paint egui
            painter.paint_jobs(
                None, // Background color
                full_output.textures_delta,
                primitives,
            );

            unsafe {
                let error = gl::GetError();
                if error != gl::NO_ERROR {
                    println!("OpenGL error after egui: {}", error);
                }
            }

            // Swap buffers
            self.window.gl_swap_window();

            self.world.input_manager.clear();
        }

        Ok(())
    }
}

pub trait GameLogic {
    fn initialize(&mut self, world: &mut World);
    fn update(&mut self, world: &mut World);
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

    pub fn with_transform(self, position: Vec3, rotation: Quat, scale: Vec3) -> Self {
        self.entity_manager.add_component(self.entity_id, Transform {
            position,
            rotation,
            scale,
        });
        self
    }

    pub fn with_render(self, mesh: Mesh, material_id: Option<u32>) -> Self {
        self.entity_manager.add_component(self.entity_id, Render {
            mesh,
            material_id,
        });
        self
    }

    pub fn with_player_control(self) -> Self {
        self.entity_manager.add_component(self.entity_id, PlayerControlled);
        self
    }

    pub fn with_rigidbody(self, velocity: Vec3) -> Self {
        self.entity_manager.add_component(
            self.entity_id,
            RigidBody {
                velocity,
                ..Default::default()
            }
        );
        self
    }

    pub fn with_circle_collider(self, radius: f32) -> Self {
        self.entity_manager.add_component(
            self.entity_id,
            Collider {
                shape: ColliderShape::Circle { radius },
                ..Default::default()
            }
        );
        self
    }

    pub fn with_box_collider(self, width: f32, height: f32) -> Self {
        self.entity_manager.add_component(
            self.entity_id,
            Collider {
                shape: ColliderShape::Box { width, height },
                ..Default::default()
            }
        );
        self
    }

    pub fn build(self) -> u32 {
        self.entity_id
    }
}