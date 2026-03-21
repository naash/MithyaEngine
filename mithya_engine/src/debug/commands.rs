// Copyright (c) 2025 Nishant Sthalekar
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use glam::{Vec2, Vec3};

#[derive(Debug, Clone)]
pub enum DebugPrimitive{
    Circle {
        center: Vec3,
        radius: f32,
    },
    Box {
        center: Vec3,
        half_extents: Vec2,
    },
    Line {
        start: Vec3,
        end: Vec3,
    },
    Text {
        position: Vec3,
        label: String,
    }
}

#[derive(Debug, Clone)]
pub enum DebugLifetime {
    Timed(f32), //Auto removed after n seconds
    Permanent(u32) //To be removed explicitly via handle
}

#[derive(Debug, Clone)]
pub struct DebugCommand {
    pub primitive: DebugPrimitive,
    pub color: egui::Color32,
    pub lifetime: DebugLifetime
}