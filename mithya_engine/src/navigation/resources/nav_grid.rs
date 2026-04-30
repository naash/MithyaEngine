// Copyright (c) 2025 Nishant Sthalekar
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Ordering;
use glam::Vec3;

use crate::navigation::grid_cell::{Direction, GridCell};

/// Internal node stored in the A* open set.
/// The heap is a max-heap by default in Rust, so we reverse the
/// ordering on f_score to make it behave as a min-heap.
#[derive(PartialEq)]
struct AStarNode {
    f_score: f32,
    cell: GridCell,
}

impl Eq for AStarNode {}

impl Ord for AStarNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse so the heap pops the node with the LOWEST f_score first.
        other.f_score
            .partial_cmp(&self.f_score)
            .unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for AStarNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// What a cell contains / how agents may traverse it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellType {
    Wall, // Impassable
    Floor, // Passable
}

impl CellType {
    pub fn is_walkable(self) -> bool {
        matches!(self, CellType::Floor)
    }
}

/// Grid-based navigation map stored as a World resource.
///
/// Coordinates: col 0..cols-1 left-to-right, row 0..rows-1 top-to-bottom.
/// Insert into the world with `world.resources.insert(nav_grid)` before
/// registering `NavigationSystem`.
pub struct NavGrid {
    pub cols: u32,
    pub rows: u32,
    pub cell_size: f32, /// World-space size of one cell (width == height).
    pub origin: Vec3,
    cells: Vec<CellType>,
}

impl NavGrid {
    /// Create a grid filled entirely with `Floor`.
    pub fn new(cols: u32, rows: u32, cell_size: f32, origin: Vec3) -> Self {
        Self {
            cols,
            rows,
            cell_size,
            origin,
            cells: vec![CellType::Floor; (cols * rows) as usize],
        }
    }

    pub fn in_bounds(&self, cell: GridCell) -> bool {
        cell.col >= 0
            && cell.row >= 0
            && cell.col < self.cols as i32
            && cell.row < self.rows as i32
    }

    pub fn get_cell(&self, cell: GridCell) -> Option<CellType> {
        if !self.in_bounds(cell) {
            return None;
        }
        Some(self.cells[cell.row as usize * self.cols as usize + cell.col as usize])
    }

    pub fn set_cell(&mut self, cell: GridCell, cell_type: CellType) {
        if self.in_bounds(cell) {
            self.cells[cell.row as usize * self.cols as usize + cell.col as usize] = cell_type;
        }
    }

    pub fn is_walkable(&self, cell: GridCell) -> bool {
        self.get_cell(cell).map(|c| c.is_walkable()).unwrap_or(false)
    }

    /// Returns all walkable cardinal neighbours together with the direction
    /// needed to reach them.
    pub fn neighbors(&self, cell: GridCell) -> Vec<(GridCell, Direction)> {
        Direction::all()
            .iter()
            .filter_map(|&dir| {
                let neighbor = cell.neighbor(dir);
                if self.is_walkable(neighbor) {
                    Some((neighbor, dir))
                } else {
                    None
                }
            })
            .collect()
    }

    /// World-space centre of a cell.
    pub fn cell_to_world(&self, cell: GridCell) -> Vec3 {
        Vec3::new(
            self.origin.x + cell.col as f32 * self.cell_size + self.cell_size * 0.5,
            self.origin.y - cell.row as f32 * self.cell_size - self.cell_size * 0.5,
            self.origin.z,
        )
    }

    /// Nearest grid cell for a world-space position. Clamped to grid bounds.
    pub fn world_to_cell(&self, world_pos: Vec3) -> GridCell {
        let col = ((world_pos.x - self.origin.x) / self.cell_size).floor() as i32;
        let row = (-(world_pos.y - self.origin.y) / self.cell_size).floor() as i32;
        GridCell::new(
            col.clamp(0, self.cols as i32 - 1),
            row.clamp(0, self.rows as i32 - 1),
        )
    }

    /// A* shortest path from `from` to `to`.
    ///
    /// `heuristic(current, goal)` must be admissible — it must never
    /// overestimate the true cost. Euclidean distance satisfies this for
    /// uniform-cost grids and extends naturally to 3D when GridCell gains a
    /// third axis.
    ///
    /// Returns `None` when no path exists.
    /// The returned vec includes both the start and goal cells.
    pub fn find_path(
        &self,
        from: GridCell,
        to: GridCell,
        heuristic: impl Fn(GridCell, GridCell) -> f32,
    ) -> Option<Vec<GridCell>> {
        if !self.is_walkable(from) || !self.is_walkable(to) {
            return None;
        }
        if from == to {
            return Some(vec![from]);
        }

        // g_score[cell] = cheapest known cost from `from` to `cell`.
        let mut g_score: HashMap<GridCell, f32> = HashMap::new();
        // came_from[cell] = cell we arrived from on the best known path.
        let mut came_from: HashMap<GridCell /*To Cell */, GridCell /* From Cell */> = HashMap::new();
        let mut open_set: BinaryHeap<AStarNode> = BinaryHeap::new();
        // closed set — cells whose cheapest path is confirmed, never revisit.
        let mut closed: HashSet<GridCell> = HashSet::new();

        g_score.insert(from, 0.0);
        open_set.push(AStarNode {
            f_score: heuristic(from, to),
            cell: from,
        });

        while let Some(AStarNode { cell: current, .. }) = open_set.pop() {
            // Stale heap entry — a cheaper path to this cell was already settled.
            if closed.contains(&current) {
                continue;
            }
            closed.insert(current);

            if current == to {
                return Some(Self::reconstruct_path(&came_from, from, to));
            }

            let current_g = *g_score.get(&current).unwrap_or(&f32::MAX);

            for (neighbor, _dir) in self.neighbors(current) {
                // Cheapest path to this neighbour already confirmed — skip.
                if closed.contains(&neighbor) {
                    continue;
                }

                // Each step costs 1.0 (uniform grid).
                // TODO - Add support for cell costs, based on agent type, some cells might cost more?
                let tentative_g = current_g + 1.0;
                //TODO reconsider for parallel pathfinding
                g_score.insert(neighbor, tentative_g);
                came_from.insert(neighbor, current);
                open_set.push(AStarNode {
                    f_score: tentative_g + heuristic(neighbor, to),
                    cell: neighbor,
                });
            }
        }

        None // no path found
    }

    /// Convenience wrapper using Euclidean distance as the heuristic.
    pub fn find_path_default(&self, from: GridCell, to: GridCell) -> Option<Vec<GridCell>> {
        self.find_path(from, to, |a, b| a.euclidean_distance_sq(b).sqrt())
    }

    fn reconstruct_path(
        came_from: &HashMap<GridCell, GridCell>,
        from: GridCell,
        to: GridCell,
    ) -> Vec<GridCell> {
        let mut path = Vec::new();
        let mut current = to;

        loop {
            path.push(current);
            if current == from {
                break;
            }
            match came_from.get(&current) {
                Some(&prev) => current = prev,
                None => break,
            }
        }

        path.reverse();
        path
    }
}