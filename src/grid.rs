//! Spatial grid: splits the screen into square cells so collisions only
//! need to be checked between particles in the same or neighbouring cells.
//!
//! Each frame:
//! 1. `clear()` the grid
//! 2. `insert()` every particle (its index in `World::particles` + its position)
//! 3. for each particle, check only the indexes returned by `neighbours()`

use macroquad::prelude::*;

pub struct Grid {
    /// Width and height of one cell, in pixels.
    /// Must be >= the diameter of the biggest particle.
    cell_size: f32,
    /// Number of columns (cells along x).
    cols: usize,
    /// Number of rows (cells along y).
    rows: usize,
    /// One entry per cell, stored row by row (`cols * rows` entries).
    /// Each cell holds the indexes of the particles inside it.
    cells: Vec<Vec<usize>>,
}

impl Grid {
    /// Creates an empty grid covering a `width` x `height` area.
    pub fn new(width: f32, height: f32, cell_size: f32) -> Self {
        let cols = (width / cell_size).ceil() as usize;
        let rows = (height / cell_size).ceil() as usize;
        Self {
            cell_size,
            cols,
            rows,
            cells: vec![Vec::new(); cols * rows],
        }
    }

    /// Empties every cell (keep the cells themselves, just remove their contents).
    pub fn clear(&mut self) {
        for cell in &mut self.cells {
            cell.clear();
        }
    }

    /// Converts a position in pixels to the (column, row) of its cell.
    /// Must never return a cell outside the grid.
    fn cell_coords(&self, pos: Vec2) -> (usize, usize) {
        let col = (pos.x / self.cell_size).floor() as usize;
        let row = (pos.y / self.cell_size).floor() as usize;
        (col.min(self.cols - 1), row.min(self.rows - 1))
    }

    /// Converts (column, row) to the index of that cell in `cells`.
    fn cell_index(&self, col: usize, row: usize) -> usize {
        row * self.cols + col
    }

    /// Puts the particle number `index`, located at `pos`, in its cell.
    pub fn insert(&mut self, index: usize, pos: Vec2) {
        let (col, row) = self.cell_coords(pos);
        let cell_i = self.cell_index(col, row);
        self.cells[cell_i].push(index)
    }

    /// Returns the indexes of all particles in the cell containing `pos`
    /// and in the 8 cells around it (fewer at the edges of the grid).
    pub fn neighbours(&self, pos: Vec2) -> Vec<usize> {
        let (col, row) = self.cell_coords(pos);
        let col_start = col.saturating_sub(1);
        let col_end = (col + 1).min(self.cols - 1);
        let row_start = row.saturating_sub(1);
        let row_end = (row + 1).min(self.rows - 1);
        let mut res = Vec::new();
        for r in row_start..=row_end {
            for c in col_start..=col_end {
                let cell_i = self.cell_index(c, r);
                res.extend_from_slice(&self.cells[cell_i]);
            }
        }
        res
    }
}
