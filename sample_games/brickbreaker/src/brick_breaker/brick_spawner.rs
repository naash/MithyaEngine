// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::collections::HashSet;

use mithya_engine::{
    engine::{EntityBuilder, World},
    rendering::{Mesh, Render},
    physics::{Collider, ColliderShape},
    Transform,
};
use glam::{Quat, Vec3};
use crate::brick_breaker::layers::{LAYER_BALL, LAYER_BRICK};

use super::components::{Brick, BrickType};

pub struct BrickGridConfig {
    pub rows: usize,
    pub columns: usize,
    pub brick_width: f32,
    pub brick_height: f32,
    pub spacing: f32,
    pub start_position: Vec3,
}

impl Default for BrickGridConfig {
    fn default() -> Self {
        Self {
            rows: 5,
            columns: 10,
            brick_width: 1.0,
            brick_height: 1.0,
            spacing: 0.2,
            start_position: Vec3::new(-15.0, 10.0, 0.0),
        }
    }
}

/// Spawn a grid of bricks
pub fn spawn_brick_grid(world: &mut World, config: BrickGridConfig) -> HashSet<u32> {
    let total_width = (config.brick_width + config.spacing) * config.columns as f32;
    
    // Center the grid
    let start_x = config.start_position.x + (total_width / 2.0) - (config.brick_width / 2.0);
    let start_y = config.start_position.y;
    
    let mut brick_ids = HashSet::new();

    for row in 0..config.rows {
        for col in 0..config.columns {
            // Calculate position
            let x = start_x - (col as f32 * (config.brick_width + config.spacing));
            let y = start_y - (row as f32 * (config.brick_height + config.spacing));
            
            // Determine brick type based on row
            let brick_type = match row {
                0 => BrickType::Strong,    // Top row is strong
                _ => BrickType::Normal,    // Middle rows
            };
            
            // Choose material based on row
            let material_name = match row {
                0 => "unlit_texture_red",
                1 => "unlit_texture_orange", 
                2 => "unlit_texture_green",
                3 => "unlit_texture_blue",
                4 => "unlit_texture_purple",
                _ => "unlit_texture_orange",
            };
            
            let brick_id = spawn_brick(
                world,
                Vec3::new(x, y, 0.0),
                config.brick_width,
                config.brick_height,
                brick_type,
                material_name,
            );

            brick_ids.insert(brick_id);
        }
    }

    brick_ids
}

/// Spawn a single brick
pub fn spawn_brick(
    world: &mut World,
    position: Vec3,
    width: f32,
    height: f32,
    brick_type: BrickType,
    material_name: &str,
) -> u32 {
    EntityBuilder::new(&mut world.entity_manager)
        .with(Transform {
            position,
            rotation: Quat::IDENTITY,
            scale: Vec3::new(width, height, 1.0),
        })
        .with(Render {
            mesh: Mesh::new_quad_textured(),
            material_id: world.asset_manager
                .get_material_by_name(material_name),
            gpu_cache: None,
        })
        .with(Collider {
            shape: ColliderShape::Box { width: 1.0, height: 1.0 },
            layer: LAYER_BRICK,
            mask: LAYER_BALL,
            ..Default::default()
        })
        .with(Brick::new(brick_type))
        .build()
}