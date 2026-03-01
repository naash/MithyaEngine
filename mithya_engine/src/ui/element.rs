// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

// UIElement stubbed pending migration to egui-wgpu

use crate::ui::types::ButtonResponse;

pub struct UIElement;

impl UIElement {
    pub fn label(&mut self, _text: &str) {}

    pub fn button(&mut self, _text: &str) -> ButtonResponse {
        ButtonResponse { clicked: false, hovered: false }
    }

    pub fn text_edit(&mut self, _text: &mut String) {}

    pub fn slider(&mut self, _value: &mut f32, _range: std::ops::RangeInclusive<f32>) {}

    pub fn checkbox(&mut self, _checked: &mut bool, _text: &str) {}

    pub fn separator(&mut self) {}

    pub fn heading(&mut self, _text: &str) {}

    pub fn horizontal<F>(&mut self, _content: F) where F: FnOnce(&mut UIElement) {}

    pub fn vertical<F>(&mut self, _content: F) where F: FnOnce(&mut UIElement) {}
}