// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::sync::Arc;

use crate::{
    DebugSystem, MovementSystem, World, asset::AssetManager, core::{
        EntityManager, Resources, engine_events::{
            EngineActionQueue, 
            EngineEventQueue,
            WindowResizedEvent
        }
    }, engine::{
        EngineStats, FrameTimer, resources::Time, system::{
            SystemPhase,
            SystemUpdateContext,
            SystemsManager
        }
    }, input::{InputMapping, InputState, InputSystem, events::{KeyPressedEvent, KeyReleasedEvent, MouseButtonReleasedEvent, MouseClickEvent, MouseMoveEvent, MouseWheelEvent}, resources::KeyModifiers}, pawn::ControllerSystem, physics::{
        CollisionSystem,
        PhysicsSystem
    }, rendering::RenderingSystem
};

use glam::Vec2;
use tracing::info;
use winit::{
    application::ApplicationHandler,
    event::{
        WindowEvent, 
        ElementState
    },
    event_loop::{
        ActiveEventLoop, 
        ControlFlow, 
        EventLoop
    },
    window::{
        Window, 
        WindowId, 
        WindowAttributes
    },
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
        tracing_subscriber::fmt()
            .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("mithya_engine=debug,warn"))
            )
            .init();

        info!("Mithya Engine initializing...");
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
            entity_manager: EntityManager::new(),
            asset_manager: AssetManager::new(self.config.asset_root.clone()).expect("Unable to create Asset Manager"),
            resources: Resources::new()
        };

        world.resources.insert(InputState::default());
        world.resources.insert(InputMapping::new());
        world.resources.insert(EngineStats::default());
        world.resources.insert(Time::default());

        let mut systems_manager = SystemsManager::new();
        systems_manager.add_system_with_phase(InputSystem::new(), &mut world, SystemPhase::Input);
        systems_manager.add_system_with_phase(ControllerSystem::new(), &mut world, SystemPhase::GameLogic);
        systems_manager.add_system_with_phase(MovementSystem, &mut world, SystemPhase::Physics);
        systems_manager.add_system_with_phase(PhysicsSystem, &mut world, SystemPhase::Physics);
        systems_manager.add_system_with_phase(CollisionSystem, &mut world, SystemPhase::Physics);
        systems_manager.add_system_with_phase(DebugSystem, &mut world, SystemPhase::Debug);
        systems_manager.set_rendering_system(rendering_system); //Special system that is stored seperate from other systems.
        
        self.game.initialize(&mut world, &mut systems_manager);

        self.state = Some(EngineState {
            window,
            systems_manager,
            world,
            frame_timer: FrameTimer::new(),
            action_queue: EngineActionQueue::new(),
            event_queue: EngineEventQueue::new(),
        });

        info!("Mithya Engine has initialized");
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
                let modifiers = state.world.resources.get::<InputState>()
                                .map(|s| s.current_modifiers)
                                .unwrap_or_default();

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
                let position = state.world.resources.get::<InputState>()
                                    .map(|s| Vec2::new(s.mouse_position.0 as f32, s.mouse_position.1 as f32))
                                    .unwrap_or(Vec2::ZERO);
                
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
                    if let Some(input_state) = state.world.resources.get_mut::<InputState>() {
                        input_state.mouse_position = (position.x as i32, position.y as i32);
                    }
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
                    if let Some(input_state) = state.world.resources.get_mut::<InputState>() {
                        input_state.current_modifiers = KeyModifiers::from_winit(&modifiers);
                    }
            }
            WindowEvent::Resized(size) => {
                
                if let Some(renderer) = state.systems_manager.get_rendering_system() {
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

                if let Some(stats) = s.world.resources.get_mut::<EngineStats>() {
                    stats.fps = s.frame_timer.get_fps();
                    stats.frame_count += 1;
                }

                if let Some(stats) = s.world.resources.get_mut::<Time>() {
                    stats.delta = delta_time;
                    stats.elapsed += delta_time;
                }

                s.systems_manager.handle_event_all(&mut s.event_queue, &mut s.action_queue, &s.world);
                s.action_queue.execute_all(&mut s.world);

                let mut update_context = SystemUpdateContext {
                    world: &mut s.world,
                    events: &mut s.event_queue,
                };
                s.systems_manager.update_all(&mut update_context);

                //Renders entities and ui
                s.systems_manager.render(&s.window, &mut s.world);

                s.action_queue.clear();
                s.window.request_redraw();
            }
            _ => {}
        }
    }
}

pub trait GameLogic {
    fn initialize(&mut self, world: &mut World, systems_manager: &mut SystemsManager);
}