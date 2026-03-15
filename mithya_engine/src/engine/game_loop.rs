// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::sync::Arc;

use crate::{
    World, asset::AssetManager, core::{
        EntityManager, engine_events::{
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
        }
    }, engine::{
        FrameTimer, InputState, system::{
            SystemRenderContext, 
            SystemUpdateContext, 
            SystemsManager
        }
    }, input::{InputMapping, InputSystem}, 
    pawn::{
        ControllerSystem, 
        MovementSystem
    }, 
    physics::{
        CollisionSystem, 
        PhysicsConfig, 
        PhysicsSystem
    }, 
    rendering::RenderingSystem
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
    pub asset_root: std::path::PathBuf,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            window_title: "Mithya Engine".to_string(),
            window_width: 800,
            window_height: 600,
            resizable: true,
            asset_root: std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        }
    }
}

// Separate struct to hold all initialized state
// This is None until winit gives us a window in resumed()
struct EngineState {
    window: Arc<Window>,
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

        let window = Arc::new(event_loop.create_window(window_attrs).expect("Failed to create window"));

        // Initialize wgpu
        let rendering_system = pollster::block_on(RenderingSystem::new(window.clone()));

        let mut world = World {
            input_state: InputState::default(),
            entity_manager: EntityManager::new(),
            physics_config: PhysicsConfig::default(),
            asset_manager: AssetManager::new(self.config.asset_root.clone()).expect("Unable to create Asset Manager"),
            fps: 0.0,
            input_mapping: InputMapping::new()
        };

        let mut systems_manager = SystemsManager::new();
        systems_manager.add_system(InputSystem::new(), &mut world);
        systems_manager.add_system(ControllerSystem::new(), &mut world);
        systems_manager.add_system(MovementSystem, &mut world);
        systems_manager.add_system(PhysicsSystem, &mut world);
        systems_manager.add_system(CollisionSystem, &mut world);
        systems_manager.add_system(rendering_system, &mut world);
        
        self.game.initialize(&mut world, &mut systems_manager);

        self.state = Some(EngineState {
            window,
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
                use winit::keyboard::PhysicalKey;
                let modifiers = state.world.input_state.current_modifiers;

                if let PhysicalKey::Code(keycode) = key_event.physical_key {
                    match key_event.state {
                        ElementState::Pressed => {
                            state.event_queue.push(KeyPressedEvent { key: keycode, modifiers });
                        }
                        ElementState::Released => {
                            state.event_queue.push(KeyReleasedEvent { key: keycode, modifiers });
                        }
                    }
                }
            }
            WindowEvent::MouseInput { state: btn_state, button, .. } => {
                let position = Vec2::new(
                    state.world.input_state.mouse_position.0 as f32,
                    state.world.input_state.mouse_position.1 as f32,
                );
                match btn_state {
                    ElementState::Pressed => {
                        state.event_queue.push(MouseClickEvent { position, button });
                    }
                    ElementState::Released => {
                        state.event_queue.push(MouseButtonReleasedEvent { position, button });
                    }
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                state.world.input_state.mouse_position = (position.x as i32, position.y as i32);
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
            WindowEvent::ModifiersChanged(modifiers) => {
                // Store current modifiers state so keyboard events can read them
                state.world.input_state.current_modifiers = KeyModifiers::from_winit(&modifiers);
            }
            WindowEvent::Resized(size) => {
                
                if let Some(renderer) = state.systems_manager.get_system_mut::<RenderingSystem>() {
                    renderer.resize(size.width, size.height);
                }

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