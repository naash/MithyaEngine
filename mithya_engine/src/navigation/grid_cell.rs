// Copyright (c) 2025 Nishant Sthalekar
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use glam::Vec2;

/// A discrete cell address in the navigation grid.
/// col increases rightward, row increases downward.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridCell {
    pub col: i32,
    pub row: i32,
}

impl GridCell {
    pub fn new(col: i32, row: i32) -> Self {
        Self { col, row }
    }

    pub fn neighbor(self, dir: Direction) -> Self {
        let (dc, dr) = dir.offset();
        Self { col: self.col + dc, row: self.row + dr }
    }

    pub fn manhattan_distance(self, other: Self) -> i32 {
        (self.col - other.col).abs() + (self.row - other.row).abs()
    }

    pub fn euclidean_distance_sq(self, other: Self) -> f32 {
        let dc = (self.col - other.col) as f32;
        let dr = (self.row - other.row) as f32;
        dc * dc + dr * dr
    }
}

/// Cardinal movement directions.
/// Row increases downward (screen convention), so Up = row - 1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn all() -> [Direction; 4] {
        [Direction::Up, Direction::Down, Direction::Left, Direction::Right]
    }

    pub fn opposite(self) -> Direction {
        match self {
            Direction::Up    => Direction::Down,
            Direction::Down  => Direction::Up,
            Direction::Left  => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    /// (delta_col, delta_row) — row increases downward.
    pub fn offset(self) -> (i32, i32) {
        match self {
            Direction::Up    => ( 0, -1),
            Direction::Down  => ( 0,  1),
            Direction::Left  => (-1,  0),
            Direction::Right => ( 1,  0),
        }
    }

    /// World-space Vec2 for this direction.
    /// Flips the row axis so Up = +Y in world space.
    pub fn to_vec2(self) -> Vec2 {
        let (dc, dr) = self.offset();
        Vec2::new(dc as f32, -(dr as f32))
    }

    /// Infer direction from two adjacent cells. Returns None if not neighbours.
    pub fn from_cells(from: GridCell, to: GridCell) -> Option<Direction> {
        match (to.col - from.col, to.row - from.row) {
            ( 1,  0) => Some(Direction::Right),
            (-1,  0) => Some(Direction::Left),
            ( 0, -1) => Some(Direction::Up),
            ( 0,  1) => Some(Direction::Down),
            _         => None,
        }
    }
}