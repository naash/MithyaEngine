// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::ui::types::ButtonResponse;
use egui_sdl2_gl::egui;

pub struct UiElement<'a> {
    pub(crate) ui: &'a mut egui::Ui,
}

impl<'a> UiElement<'a> {
    pub fn label(&mut self, text: &str) {
        self.ui.label(text);
    }
    
    pub fn button(&mut self, text: &str) -> ButtonResponse {
        let response = self.ui.button(text);
        ButtonResponse {
            clicked: response.clicked(),
            hovered: response.hovered(),
        }
    }
    
    pub fn text_edit(&mut self, text: &mut String) {
        self.ui.text_edit_singleline(text);
    }
    
    pub fn slider(&mut self, value: &mut f32, range: std::ops::RangeInclusive<f32>) {
        self.ui.add(egui::Slider::new(value, range));
    }
    
    pub fn checkbox(&mut self, checked: &mut bool, text: &str) {
        self.ui.checkbox(checked, text);
    }
    
    pub fn separator(&mut self) {
        self.ui.separator();
    }
    
    pub fn heading(&mut self, text: &str) {
        self.ui.heading(text);
    }
    
    pub fn horizontal<F>(&mut self, content: F) 
    where
        F: FnOnce(&mut UiElement),
    {
        self.ui.horizontal(|ui| {
            let mut ui_element = UiElement { ui };
            content(&mut ui_element);
        });
    }
    
    pub fn vertical<F>(&mut self, content: F) 
    where
        F: FnOnce(&mut UiElement),
    {
        self.ui.vertical(|ui| {
            let mut ui_element = UiElement { ui };
            content(&mut ui_element);
        });
    }
}