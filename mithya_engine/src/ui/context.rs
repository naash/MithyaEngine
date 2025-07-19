// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::ui::UiWindow;
use egui_sdl2_gl::egui;

pub struct UiContext<'a> {
    pub(crate) ctx: &'a egui::Context,
}

impl<'a> UiContext<'a> {
    pub fn window(&self, title: &str) -> UiWindow {
        UiWindow {
            title: title.to_string(),
            size: None,
            position: None,
            resizable: true,
            collapsible: true,
        }
    }
}