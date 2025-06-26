use std::collections::{HashMap, HashSet};
use winit::event::VirtualKeyCode;

use super::{Action, Axis, InputBindings};

pub struct InputState {
    pub held_actions: HashSet<Action>,
    pub axis_values: HashMap<Axis, f32>,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            held_actions: HashSet::new(),
            axis_values: HashMap::new(),
        }
    }

    pub fn handle_key_event(&mut self, key: VirtualKeyCode, is_down: bool, bindings: &InputBindings) {
        if let Some(action) = bindings.action_map.get(&key) {
            if is_down {
                self.held_actions.insert(*action);
            } else {
                self.held_actions.remove(action);
            }
        }

        if let Some((axis, value)) = bindings.axis_map.get(&key) {
            self.axis_values
                .entry(*axis)
                .and_modify(|v| *v += if is_down { *value } else { -*value })
                .or_insert(if is_down { *value } else { 0.0 });
        }
    }

    pub fn is_action_pressed(&self, action: Action) -> bool {
        self.held_actions.contains(&action)
    }

    pub fn get_axis(&self, axis: Axis) -> f32 {
        *self.axis_values.get(&axis).unwrap_or(&0.0)
    }

    pub fn clear(&mut self) {
        self.axis_values.clear();
        self.held_actions.clear();
    }
}
