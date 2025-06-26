use std::collections::HashMap;
use winit::event::VirtualKeyCode;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum Action {
    MoveLeft,
    MoveRight,
    Jump,
    Fire,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum Axis {
    MoveHorizontal,
    MoveVertical,
}

pub struct InputBindings {
    pub action_map: HashMap<VirtualKeyCode, Action>,
    pub axis_map: HashMap<VirtualKeyCode, (Axis, f32)>,
}

impl InputBindings {
    pub fn new_default() -> Self {
        let mut action_map = HashMap::new();
        action_map.insert(VirtualKeyCode::Space, Action::Jump);
        action_map.insert(VirtualKeyCode::LControl, Action::Fire);

        let mut axis_map = HashMap::new();
        axis_map.insert(VirtualKeyCode::A, (Axis::MoveHorizontal, -1.0));
        axis_map.insert(VirtualKeyCode::D, (Axis::MoveHorizontal, 1.0));
        axis_map.insert(VirtualKeyCode::W, (Axis::MoveVertical, 1.0));
        axis_map.insert(VirtualKeyCode::S, (Axis::MoveVertical, -1.0));

        Self {
            action_map,
            axis_map,
        }
    }
}