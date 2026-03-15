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
    let total_height = (config.brick_height + config.spacing) * config.rows as f32;
    
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
                4 => BrickType::Normal,    // Bottom row
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
    
    println!("Spawned {} bricks in a {}x{} grid", 
             config.rows * config.columns, config.rows, config.columns);

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
        })
        .with(Collider {
            shape: ColliderShape::Box { width: 1.0, height: 1.0 },
            ..Default::default()
        })
        .with(Brick::new(brick_type))
        .build()
}

/// Create some test patterns
pub mod patterns {
    use super::*;
    
    /// Spawn a pyramid pattern
    pub fn spawn_pyramid(world: &mut World) {
        let brick_width = 3.0;
        let brick_height = 1.5;
        let spacing = 0.2;
        
        let rows = 5;
        for row in 0..rows {
            let bricks_in_row = rows - row;
            let start_x = -(bricks_in_row as f32 * (brick_width + spacing)) / 2.0;
            let y = 10.0 - (row as f32 * (brick_height + spacing));
            
            for col in 0..bricks_in_row {
                let x = start_x + (col as f32 * (brick_width + spacing));
                let brick_id = spawn_brick(
                    world,
                    Vec3::new(x, y, 0.0),
                    brick_width,
                    brick_height,
                    BrickType::Normal,
                    "unlit_texture_orange",
                );
            }
        }
    }
    
    /// Spawn a checkerboard pattern
    pub fn spawn_checkerboard(world: &mut World) {
        let config = BrickGridConfig::default();
        
        for row in 0..config.rows {
            for col in 0..config.columns {
                // Skip every other brick in checkerboard pattern
                if (row + col) % 2 == 0 {
                    let x = config.start_position.x + (col as f32 * (config.brick_width + config.spacing));
                    let y = config.start_position.y - (row as f32 * (config.brick_height + config.spacing));
                    
                    spawn_brick(
                        world,
                        Vec3::new(x, y, 0.0),
                        config.brick_width,
                        config.brick_height,
                        BrickType::Normal,
                        if row % 2 == 0 { "unlit_texture_red" } else { "unlit_texture_blue" },
                    );
                }
            }
        }
    }
}