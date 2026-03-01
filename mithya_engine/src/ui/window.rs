// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

// UIWindow is stubbed pending migration to egui-wgpu
pub struct UIWindow {
    pub title: String,
    pub size: Option<(f32, f32)>,
    pub position: Option<(f32, f32)>,
    pub resizable: bool,
    pub collapsible: bool,
}

impl UIWindow {
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

    pub fn show<F>(self, _ctx: &crate::ui::UIContext, _content: F)
    where
        F: FnMut(),
    {
        // Stubbed — no-op until egui-wgpu backend is implemented
    }
}