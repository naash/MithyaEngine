use egui_sdl2_gl::egui;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

pub fn sdl_to_egui_button(button: MouseButton) -> Option<egui::PointerButton> {
    match button {
        MouseButton::Left => Some(egui::PointerButton::Primary),
        MouseButton::Right => Some(egui::PointerButton::Secondary),
        MouseButton::Middle => Some(egui::PointerButton::Middle),
        _ => None,
    }
}

pub fn sdl_to_egui_key(keycode: Keycode) -> Option<egui::Key> {
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

pub fn button_to_index(button: egui::PointerButton) -> usize {
    match button {
        egui::PointerButton::Primary => 0,
        egui::PointerButton::Secondary => 1,
        egui::PointerButton::Middle => 2,
        egui::PointerButton::Extra1 => 0,
        egui::PointerButton::Extra2 => 0,
    }
}