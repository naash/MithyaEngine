// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::core::engine_events::{
    KeyPressedEvent, 
    KeyReleasedEvent, 
    MouseButtonReleasedEvent, 
    MouseClickEvent, 
    MouseMoveEvent, 
    MouseWheelEvent, 
    TextInputEvent, 
    WindowResizedEvent
};
use crate::core::EngineEventListener;
use crate::engine::system::{System, SystemRenderContext, SystemUpdateContext};
use crate::ui::UIContext;
use crate::World;
use egui_sdl2_gl::painter::Painter;
use egui_sdl2_gl::egui;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::video::Window;

pub struct UISystem {
    painter: Painter,
    context: egui::Context,
    raw_input: egui::RawInput,
    mouse_pos: egui::Pos2,
    mouse_pressed: [bool; 3],
    keys_pressed: std::collections::HashSet<egui::Key>,
    modifiers: egui::Modifiers,
    window_size: egui::Vec2,
}

impl UISystem {
     pub fn new(window: &Window) -> Self {
        let painter = Painter::new(window, 1.0, egui_sdl2_gl::ShaderVersion::Adaptive);
        let context = egui::Context::default();
        let (width, height) = window.size();
        
        Self {
            painter,
            context,
            raw_input: egui::RawInput::default(),
            mouse_pos: egui::Pos2::ZERO,
            mouse_pressed: [false; 3],
            keys_pressed: std::collections::HashSet::new(),
            modifiers: egui::Modifiers::default(),
            window_size: egui::Vec2::new(width as f32, height as f32),
        }
    }
    
    pub fn wants_keyboard_input(&self) -> bool {
        self.context.wants_keyboard_input()
    }
    
    pub fn wants_pointer_input(&self) -> bool {
        self.context.wants_pointer_input()
    }

    pub fn begin_frame(&mut self, window_size: (u32, u32)) {
        self.painter.update_screen_rect(window_size);
        
        self.raw_input.screen_rect = Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            self.window_size,
        ));
        
        self.raw_input.events.push(egui::Event::PointerMoved(self.mouse_pos));
    }

    // Add a separate method for rendering UI that can be called from the trait
    fn render_ui(&mut self) -> bool {
        let full_output = self.context.run(self.raw_input.take(), |ctx| {
            let ui_ctx = UIContext { ctx };
            
            // Your exact UI code from the main loop
            use crate::ui::UIWindow; // Make sure this import path is correct
            
            UIWindow {
                title: "Debug Info".to_string(),
                size: Some((200.0, 100.0)),
                position: Some((20.0, 20.0)),
                resizable: true,
                collapsible: true,
            }
            .show(&ui_ctx, |ui| {
                ui.label("Hello UI!");
                if ui.button("Test Button").clicked {
                    println!("Button clicked!");
                }
                
                ui.separator();
                
                ui.heading("Test heading");
                
                // Access world data here
                // Find better way to fetch UI data. We shouldn't pass data directly. Rather UI specific components should be created
                //ui.label(&format!("FPS: {:.1}", world.fps)); 
            });
        });
        
        let wants_input = self.context.wants_pointer_input() || self.context.wants_keyboard_input();
        
        let primitives = self.context.tessellate(full_output.shapes, full_output.pixels_per_point);
        
        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Disable(gl::DEPTH_TEST);
            gl::Disable(gl::CULL_FACE);
        }
        
        self.painter.paint_jobs(
            None,
            full_output.textures_delta,
            primitives,
        );
        
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::CULL_FACE);
            gl::Disable(gl::BLEND);
        }
        
        wants_input
    }
}

impl System for UISystem {
    fn initialize(&mut self, _world: &mut World) -> Result<(), Box<dyn std::error::Error>> {
        // Any initialization logic that needs access to the world
        // For now, most initialization is done in new()
        Ok(())
    }

    fn update(&mut self, _update_context: &mut SystemUpdateContext) {
        // Update UI frame preparation
        self.painter.update_screen_rect((self.window_size.x as u32, self.window_size.y as u32));
        
        self.raw_input.screen_rect = Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            self.window_size,
        ));
        
        self.raw_input.events.push(egui::Event::PointerMoved(self.mouse_pos));
    }

    fn render(&mut self, _render_context: &mut SystemRenderContext) {
         let _ = self.render_ui();
    }

    fn cleanup(&mut self, _world: &mut World) {
        // Any cleanup logic specific to the UI system
        // The painter and context will be dropped automatically
    }

    fn as_event_listener_mut(&mut self) -> Option<&mut dyn EngineEventListener> {
       Some(self)
    }
}

impl EngineEventListener for UISystem {

    fn interested_events(&self) -> Vec<std::any::TypeId> {
        use std::any::TypeId;
        vec![
            TypeId::of::<MouseMoveEvent>(),
            TypeId::of::<MouseClickEvent>(),
            TypeId::of::<MouseButtonReleasedEvent>(),
            TypeId::of::<MouseWheelEvent>(),
            TypeId::of::<KeyPressedEvent>(),
            TypeId::of::<KeyReleasedEvent>(),
            TypeId::of::<TextInputEvent>(),
            TypeId::of::<WindowResizedEvent>(),
        ]
    }

    fn on_events(
        &mut self,
        events: &crate::core::EngineEventQueue,
        actions: &mut crate::core::EngineActionQueue,
    ) {
        // Handle mouse motion
        for event in events.iter_type::<MouseMoveEvent>() {
            self.mouse_pos = egui::pos2(event.position.x, event.position.y);
        }

        // Handle mouse button down
        for event in events.iter_type::<MouseClickEvent>() {
            if let Some(button) = sdl_to_egui_button(event.button) {
                let button_index = button_to_index(button);
                if button_index < self.mouse_pressed.len() {
                    self.mouse_pressed[button_index] = true;
                }
                
                self.raw_input.events.push(egui::Event::PointerButton {
                    pos: self.mouse_pos,
                    button,
                    pressed: true,
                    modifiers: self.modifiers,
                });
            }
        }

        // Handle mouse button up
        for event in events.iter_type::<MouseButtonReleasedEvent>() {
            if let Some(button) = sdl_to_egui_button(event.button) {
                let button_index = button_to_index(button);
                if button_index < self.mouse_pressed.len() {
                    self.mouse_pressed[button_index] = false;
                }
                
                self.raw_input.events.push(egui::Event::PointerButton {
                    pos: self.mouse_pos,
                    button,
                    pressed: false,
                    modifiers: self.modifiers,
                });
            }
        }

        // Handle mouse wheel
        for event in events.iter_type::<MouseWheelEvent>() {
            self.raw_input.events.push(egui::Event::MouseWheel {
                unit: egui::MouseWheelUnit::Line,
                delta: egui::vec2(0.0, event.delta_y as f32 * 10.0),
                modifiers: self.modifiers,
            });
        }

        // Handle key down
        for event in events.iter_type::<KeyPressedEvent>() {
            
            self.modifiers = event.modifiers.to_egui();
            
            if let Some(key) = sdl_to_egui_key(event.key) {
                self.keys_pressed.insert(key);
                self.raw_input.events.push(egui::Event::Key {
                    key,
                    pressed: true,
                    repeat: false,
                    modifiers: self.modifiers,
                    physical_key: Some(egui::Key::from(key)),
                });
            }
        }

        // Handle key up
        for event in events.iter_type::<KeyReleasedEvent>() {
            
           self.modifiers = event.modifiers.to_egui();
            
            if let Some(key) = sdl_to_egui_key(event.key) {
                self.keys_pressed.remove(&key);
                self.raw_input.events.push(egui::Event::Key {
                    key,
                    pressed: false,
                    repeat: false,
                    modifiers: self.modifiers,
                    physical_key: Some(egui::Key::from(key)),
                });
            }
        }

        // Handle text input
        for event in events.iter_type::<TextInputEvent>() {
            self.raw_input.events.push(egui::Event::Text(event.text.clone()));
        }

        // Handle window resize
        for event in events.iter_type::<WindowResizedEvent>() {
            self.window_size = egui::Vec2::new(event.width as f32, event.height as f32);
        }
    }
}

// Helper functions
fn sdl_to_egui_button(button: MouseButton) -> Option<egui::PointerButton> {
    match button {
        MouseButton::Left => Some(egui::PointerButton::Primary),
        MouseButton::Right => Some(egui::PointerButton::Secondary),
        MouseButton::Middle => Some(egui::PointerButton::Middle),
        _ => None,
    }
}

fn button_to_index(button: egui::PointerButton) -> usize {
    match button {
        egui::PointerButton::Primary => 0,
        egui::PointerButton::Secondary => 1,
        egui::PointerButton::Middle => 2,
        egui::PointerButton::Extra1 => 3, // Fixed: use proper index
        egui::PointerButton::Extra2 => 4, // Fixed: use proper index
    }
}

fn sdl_to_egui_key(keycode: Keycode) -> Option<egui::Key> {
    match keycode {
        Keycode::Down => Some(egui::Key::ArrowDown),
        Keycode::Left => Some(egui::Key::ArrowLeft),
        Keycode::Right => Some(egui::Key::ArrowRight),
        Keycode::Up => Some(egui::Key::ArrowUp),
        Keycode::Escape => Some(egui::Key::Escape),
        Keycode::Tab => Some(egui::Key::Tab),
        Keycode::Backspace => Some(egui::Key::Backspace),
        Keycode::Return => Some(egui::Key::Enter),
        Keycode::Space => Some(egui::Key::Space),
        Keycode::Insert => Some(egui::Key::Insert),
        Keycode::Delete => Some(egui::Key::Delete),
        Keycode::Home => Some(egui::Key::Home),
        Keycode::End => Some(egui::Key::End),
        Keycode::PageUp => Some(egui::Key::PageUp),
        Keycode::PageDown => Some(egui::Key::PageDown),
        Keycode::A => Some(egui::Key::A),
        Keycode::B => Some(egui::Key::B),
        Keycode::C => Some(egui::Key::C),
        Keycode::D => Some(egui::Key::D),
        Keycode::E => Some(egui::Key::E),
        Keycode::F => Some(egui::Key::F),
        Keycode::G => Some(egui::Key::G),
        Keycode::H => Some(egui::Key::H),
        Keycode::I => Some(egui::Key::I),
        Keycode::J => Some(egui::Key::J),
        Keycode::K => Some(egui::Key::K),
        Keycode::L => Some(egui::Key::L),
        Keycode::M => Some(egui::Key::M),
        Keycode::N => Some(egui::Key::N),
        Keycode::O => Some(egui::Key::O),
        Keycode::P => Some(egui::Key::P),
        Keycode::Q => Some(egui::Key::Q),
        Keycode::R => Some(egui::Key::R),
        Keycode::S => Some(egui::Key::S),
        Keycode::T => Some(egui::Key::T),
        Keycode::U => Some(egui::Key::U),
        Keycode::V => Some(egui::Key::V),
        Keycode::W => Some(egui::Key::W),
        Keycode::X => Some(egui::Key::X),
        Keycode::Y => Some(egui::Key::Y),
        Keycode::Z => Some(egui::Key::Z),
        Keycode::Num0 => Some(egui::Key::Num0),
        Keycode::Num1 => Some(egui::Key::Num1),
        Keycode::Num2 => Some(egui::Key::Num2),
        Keycode::Num3 => Some(egui::Key::Num3),
        Keycode::Num4 => Some(egui::Key::Num4),
        Keycode::Num5 => Some(egui::Key::Num5),
        Keycode::Num6 => Some(egui::Key::Num6),
        Keycode::Num7 => Some(egui::Key::Num7),
        Keycode::Num8 => Some(egui::Key::Num8),
        Keycode::Num9 => Some(egui::Key::Num9),
        _ => None,
    }
}
