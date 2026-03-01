// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::sync::Arc;

use crate::{
    asset::AssetManager,
    core::{
        engine_events::{
            EngineActionQueue, 
            EngineEventQueue, 
            KeyModifiers, 
            KeyPressedEvent, 
            KeyReleasedEvent, 
            MouseButtonReleasedEvent, 
            MouseClickEvent, 
            MouseMoveEvent, 
            MouseWheelEvent, 
            TextInputEvent, 
            WindowResizedEvent
        },
        EntityManager
    }, 
    engine::{
        system::{
            SystemRenderContext, 
            SystemUpdateContext, 
            SystemsManager
        }, 
        FrameTimer, 
        InputState
    },
    input::InputSystem, 
    physics::{
        CollisionSystem, 
        PhysicsConfig, 
        PhysicsSystem
    }, 
    player::PlayerControlSystem, 
    rendering::RenderingSystem, 
    World
};

use glam::Vec2;
use winit::{
    application::ApplicationHandler,
    event::{WindowEvent, ElementState, MouseButton},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId, WindowAttributes},
    dpi::LogicalSize,
};

pub struct EngineConfig {
    pub window_title: String,
    pub window_width: u32,
    pub window_height: u32,
    pub resizable: bool,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            window_title: "Mithya Engine".to_string(),
            window_width: 800,
            window_height: 600,
            resizable: true,
        }
    }
}

// Separate struct to hold all initialized state
// This is None until winit gives us a window in resumed()
struct EngineState {
    window: Arc<Window>,
    rendering_system: RenderingSystem,
    systems_manager: SystemsManager,
    world: World,
    frame_timer: FrameTimer,
    action_queue: EngineActionQueue,
    event_queue: EngineEventQueue,
}

pub struct Engine<G: GameLogic> {
    config: EngineConfig,
    game: G,
    state: Option<EngineState>,
}

impl<G: GameLogic> Engine<G> {
    pub fn new(config: EngineConfig, game: G) -> Self {
        Self {
            config,
            game,
            state: None,
        }
    }

    pub fn run(mut self) -> Result<(), Box<dyn std::error::Error>> {
        let event_loop = EventLoop::new()?;
        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.run_app(&mut self)?;
        Ok(())
    }
}

impl<G: GameLogic> ApplicationHandler for Engine<G> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // This is where we create the window and initialize wgpu
        // winit requires window creation to happen here, not in new()
        let window_attrs = WindowAttributes::default()
            .with_title(&self.config.window_title)
            .with_inner_size(LogicalSize::new(self.config.window_width, self.config.window_height))
            .with_resizable(self.config.resizable);

        let window = Arc::new(event_loop.create_window(window_attrs).unwrap());

        // Initialize wgpu
        let rendering_system = pollster::block_on(RenderingSystem::new(window.clone()));

        let mut world = World {
            input_state: InputState::default(),
            entity_manager: EntityManager::new(),
            physics_config: PhysicsConfig::default(),
            asset_manager: AssetManager::new().unwrap(),
            fps: 0.0,
        };

        let mut systems_manager = SystemsManager::new();
        systems_manager.add_system(InputSystem::new());
        systems_manager.add_system(PlayerControlSystem::default());
        systems_manager.add_system(PhysicsSystem);
        systems_manager.add_system(CollisionSystem);
        // Note: RenderingSystem is no longer in SystemsManager
        // It owns wgpu state and is managed directly by Engine

        systems_manager.initialize_all(&mut world);

        rendering_system.initialize_assets(&mut world.asset_manager);
        
        self.game.initialize(&mut world, &mut systems_manager);

        self.state = Some(EngineState {
            window,
            rendering_system,
            systems_manager,
            world,
            frame_timer: FrameTimer::new(),
            action_queue: EngineActionQueue::new(),
            event_queue: EngineEventQueue::new(),
        });
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let state = match self.state.as_mut() {
            Some(s) => s,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::KeyboardInput { event: key_event, .. } => {
                // TODO: map winit key events to your EngineEventQueue
                // Your KeyPressedEvent/KeyReleasedEvent will need keycode mapping from winit
            }
            WindowEvent::MouseInput { state: btn_state, button, .. } => {
                // TODO: map mouse button events
            }
            WindowEvent::CursorMoved { position, .. } => {
                state.event_queue.push(MouseMoveEvent {
                    position: Vec2::new(position.x as f32, position.y as f32),
                });
            }
            WindowEvent::MouseWheel { delta, .. } => {
                use winit::event::MouseScrollDelta;
                if let MouseScrollDelta::LineDelta(x, y) = delta {
                    state.event_queue.push(MouseWheelEvent {
                        delta_x: x as i32,
                        delta_y: y as i32,
                    });
                }
            }
            WindowEvent::Resized(size) => {
                state.rendering_system.resize(size.width, size.height);
                state.event_queue.push(WindowResizedEvent {
                    width: size.width,
                    height: size.height,
                });
            }
            WindowEvent::RedrawRequested => {
                // This is the main loop — winit calls this each frame
                let s = state;
                
                s.frame_timer.update();
                let delta_time = s.frame_timer.get_delta_time();
                s.world.fps = s.frame_timer.get_fps();

                s.systems_manager.handle_event_all(&mut s.event_queue, &mut s.action_queue);
                s.action_queue.execute_all(&mut s.world);

                let mut update_context = SystemUpdateContext {
                    world: &mut s.world,
                    events: &mut s.event_queue,
                    delta_time,
                };
                s.systems_manager.update_all(&mut update_context);
                self.game.update(&mut s.world, delta_time);

                let mut render_context = SystemRenderContext {
                    entity_manager: &mut s.world.entity_manager,
                    asset_manager: &mut s.world.asset_manager,
                };
                s.systems_manager.render_all(&mut render_context);

                // RenderingSystem handles its own frame
                s.rendering_system.render(&mut render_context);

                s.action_queue.clear();
                s.window.request_redraw();
            }
            _ => {}
        }
    }
}

pub trait GameLogic {
    fn initialize(&mut self, world: &mut World, systems_manager: &mut SystemsManager);
    fn update(&mut self, world: &mut World, delta_time: f32);
}