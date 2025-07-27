// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::ui::UIWindow;
use egui_sdl2_gl::egui;

pub struct UIContext<'a> {
    pub(crate) ctx: &'a egui::Context,
}

impl<'a> UIContext<'a> {
    pub fn window(&self, title: &str) -> UIWindow {
        UIWindow {
            title: title.to_string(),
            size: None,
            position: None,
            resizable: true,
            collapsible: true,
        }
    }
}