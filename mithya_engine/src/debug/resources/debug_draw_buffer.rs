// Copyright (c) 2025 Nishant Sthalekar
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use glam::{Vec2, Vec3};
use crate::debug::commands::{DebugCommand, DebugLifetime, DebugPrimitive};

pub struct DebugDrawBuffer {
    pub enabled: bool,
    commands: Vec<DebugCommand>,
    next_handle: u32,
}

impl DebugDrawBuffer {
    pub fn new() -> Self {
        Self {
            enabled: true,
            commands: Vec::new(),
            next_handle: 0,
        }
    }

    pub fn circle(&mut self, center: Vec3, radius: f32, color: egui::Color32, seconds: f32) {
        self.push(DebugPrimitive::Circle { center, radius}, color, DebugLifetime::Timed(seconds));
    }

    pub fn circle_permanent(&mut self, center: Vec3, radius: f32, color: egui::Color32) -> u32 {
        let handle = self.alloc_handle();
        self.push(DebugPrimitive::Circle { center, radius }, color, DebugLifetime::Permanent(handle));
        handle
    }

    pub fn box_shape(&mut self, center: Vec3, half_extents: Vec2, color: egui::Color32, seconds: f32) {
        self.push(DebugPrimitive::Box { center, half_extents}, color, DebugLifetime::Timed(seconds));
    }

    pub fn box_shape_permanent(&mut self, center: Vec3, half_extents: Vec2, color: egui::Color32) -> u32 {
        let handle = self.alloc_handle();
        self.push(DebugPrimitive::Box { center, half_extents }, color, DebugLifetime::Permanent(handle));
        handle
    }

    pub fn line(&mut self, start: Vec3, end: Vec3, color: egui::Color32, seconds: f32) {
        self.push(DebugPrimitive::Line { start, end}, color, DebugLifetime::Timed(seconds));
    }

    pub fn line_permanent(&mut self, start: Vec3, end: Vec3, color: egui::Color32) -> u32 {
        let handle = self.alloc_handle();
        self.push(DebugPrimitive::Line { start, end}, color, DebugLifetime::Permanent(handle));
        handle
    }

    pub fn text(&mut self, center: Vec3, label: String, color: egui::Color32, seconds: f32) {
        self.push(DebugPrimitive::Text { position: center, label }, color, DebugLifetime::Timed(seconds));
    }

    pub fn text_permanent(&mut self, center: Vec3, label: String, color: egui::Color32) -> u32 {
        let handle = self.alloc_handle();
        self.push(DebugPrimitive::Text { position: center, label }, color, DebugLifetime::Permanent(handle));
        handle
    }

    pub fn remove(&mut self, handle: u32) {
        self.commands.retain(|cmd| {
            !matches!(cmd.lifetime, DebugLifetime::Permanent(h) if h == handle)
        });
    }

    pub(crate) fn tick(&mut self, dt: f32) {
        self.commands.retain_mut(|cmd| {
            match &mut cmd.lifetime {
                DebugLifetime::Timed(t) => {
                    *t -= dt;
                    *t > 0.0
                }
                DebugLifetime::Permanent(_) => true,
            }
        });
    }

    /// Read-only access to commands for rendering. Called by DebugSystem.
    pub(crate) fn commands(&self) -> &[DebugCommand] {
        &self.commands
    }

    fn push(&mut self, primitive: DebugPrimitive, color: egui::Color32, lifetime: DebugLifetime) {
        if !self.enabled { return; }
        self.commands.push(DebugCommand { primitive, color, lifetime });
    }

    fn alloc_handle(&mut self) -> u32 {
        let handle = self.next_handle;
        self.next_handle += 1;
        handle
    }
}

impl Default for DebugDrawBuffer {
    fn default() -> Self {
        Self::new()
    }
}