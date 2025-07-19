// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::ui::{UiContext, UiElement};
use egui_sdl2_gl::egui;

pub struct UiWindow {
    pub title: String,
    pub size: Option<(f32, f32)>,
    pub position: Option<(f32, f32)>,
    pub resizable: bool,
    pub collapsible: bool,
}

impl UiWindow {
    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.size = Some((width, height));
        self
    }
    
    pub fn position(mut self, x: f32, y: f32) -> Self {
        self.position = Some((x, y));
        self
    }
    
    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }
    
    pub fn collapsible(mut self, collapsible: bool) -> Self {
        self.collapsible = collapsible;
        self
    }
    
    pub fn show<F>(self, ctx: &UiContext, mut content: F) 
    where
    F: FnMut(&mut UiElement),
    {
        let mut window = egui::Window::new(self.title)
            .resizable(self.resizable)
            .collapsible(self.collapsible);
        
        if let Some((width, height)) = self.size {
            window = window.default_size(egui::vec2(width, height));
        }
        
        if let Some((x, y)) = self.position {
            window = window.default_pos(egui::pos2(x, y));
        }
        
        window.show(ctx.ctx, |ui| {
            let mut ui_element = UiElement { ui };
            content(&mut ui_element);
        });
    }
}