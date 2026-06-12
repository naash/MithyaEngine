// Copyright (c) 2025 Nishant Sthalekar
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use glam::Vec3;

use mithya_engine::{
    asset::UniformValue,
    engine::{EntityBuilder, World},
    rendering::{Mesh, Render},
    CellType, GridCell, NavGrid, Transform,
};

// Iterative DFS maze generation (recursive backtracker).
// Rooms occupy even grid positions; the cell between two rooms is the passage wall to carve.
// Returns the set of floor cells to carve into the NavGrid.
fn generate_maze_cells(cols: u32, rows: u32) -> Vec<GridCell> {
    let maze_cols = (cols / 2) as i32;
    let maze_rows = (rows / 2) as i32;

    let mut floor_cells: Vec<GridCell> = Vec::new();
    let mut visited = vec![false; (maze_cols * maze_rows) as usize];
    let mut stack: Vec<(i32, i32)> = Vec::new();

    const DIRS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    let visit = |mc: i32, mr: i32| -> usize { (mr * maze_cols + mc) as usize };

    visited[visit(0, 0)] = true;
    stack.push((0, 0));
    floor_cells.push(GridCell::new(0, 0));

    // LCG for deterministic pseudo-random neighbour selection
    let mut rng: u32 = 0xBEEF_1234;

    while let Some(&(mc, mr)) = stack.last() {
        let neighbors: Vec<(i32, i32)> = DIRS
            .iter()
            .map(|&(dc, dr)| (mc + dc, mr + dr))
            .filter(|&(nc, nr)| {
                nc >= 0 && nc < maze_cols
                    && nr >= 0 && nr < maze_rows
                    && !visited[visit(nc, nr)]
            })
            .collect();

        if neighbors.is_empty() {
            stack.pop();
        } else {
            rng = rng.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            let (nc, nr) = neighbors[(rng as usize) % neighbors.len()];

            visited[visit(nc, nr)] = true;
            stack.push((nc, nr));

            floor_cells.push(GridCell::new(nc * 2, nr * 2));
            floor_cells.push(GridCell::new(mc * 2 + (nc - mc), mr * 2 + (nr - mr)));
        }
    }

    floor_cells
}

pub fn spawn_maze(world: &mut World, cell_size: f32, cols: u32, rows: u32) {
    // Fill the entire grid with walls, then carve the maze passages
    if let Some(nav_grid) = world.resources.get_mut::<NavGrid>() {
        for row in 0..rows as i32 {
            for col in 0..cols as i32 {
                nav_grid.set_cell(GridCell::new(col, row), CellType::Wall);
            }
        }
    }

    let floor_cells = generate_maze_cells(cols, rows);

    if let Some(nav_grid) = world.resources.get_mut::<NavGrid>() {
        for cell in &floor_cells {
            nav_grid.set_cell(*cell, CellType::Floor);
        }
    }

    let wall_material_id = world
        .asset_manager
        .load_material("wall")
        .expect("failed to create wall material");
    if let Some(mat) = world.asset_manager.get_material_mut(wall_material_id) {
        mat.uniforms.insert("u_color".to_string(), UniformValue::Vec3([0.0, 0.0, 0.0]));
    }

    // Spawn a black quad for every wall cell
    let origin = world.resources.get::<NavGrid>().unwrap().origin;
    let quad_scale = Vec3::new(cell_size, cell_size, 1.0);

    for row in 0..rows as i32 {
        for col in 0..cols as i32 {
            let cell = GridCell::new(col, row);
            let is_wall = world.resources.get::<NavGrid>()
                .map(|g| !g.is_walkable(cell))
                .unwrap_or(false);

            if is_wall {
                let pos = Vec3::new(
                    origin.x + col as f32 * cell_size + cell_size * 0.5,
                    origin.y - row as f32 * cell_size - cell_size * 0.5,
                    0.0,
                );
                EntityBuilder::new(&mut world.entity_manager)
                    .with(Transform { position: pos, scale: quad_scale, ..Default::default() })
                    .with(Render {
                        mesh: Mesh::new_quad(),
                        material_id: Some(wall_material_id),
                        gpu_cache: None,
                        tint: None,
                    })
                    .build();
            }
        }
    }
}
