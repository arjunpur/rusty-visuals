/// Inspired by https://github.com/gridbugs/grid-2d/blob/master/src/lib.rs
///
/// API Examples:
///
/// let grid = Grid::new(
///     (width, height),
///     (x, y),
///     (x_tile_count, y_tile_count),
/// );
/// for cell in grid.row_major_iter() {...}
///
use nannou::prelude::*;

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub struct CellIndex {
    pub row: usize,
    pub col: usize,
}
pub struct Cell {
    // Note that the x, y coordinates here are the center coordinates of
    // the cell. Use the `left()` or `bottom()` methods to get the appropriate
    // coordinates.
    pub xy: Vec2,
    pub wh: Vec2,

    // index is the (i,j) row, column index that this cell
    // represents.
    pub index: CellIndex,
}

/// Cell implements functions for a Grid container struct
/// that represents a single block on the Grid.
impl Cell {
    // Width / Height; Coordinates; Index in 2D matix
    fn new(wh: Vec2, xy: Vec2, index: CellIndex) -> Self {
        Cell { wh, xy, index }
    }

    pub fn left(&self) -> f32 {
        self.xy.x - (self.wh.x / 2.0)
    }

    pub fn bottom(&self) -> f32 {
        self.xy.y - (self.wh.y / 2.0)
    }

    pub fn top(&self) -> f32 {
        self.xy.y + (self.wh.y / 2.0)
    }

    pub fn right(&self) -> f32 {
        self.xy.x + (self.wh.x / 2.0)
    }
}

// A Grid is a collection of Cells in a 2D matrix of
// arbitrary and possibly even non-uniform vectors
// of vectors of cells.
pub struct Grid {
    cells: Vec<Vec<Cell>>,
    rect: Rect,
}

// TODO: Compute all the cells on the fly instead of storing
// everything on the heap.
impl Grid {
    /// num_cells: (number of rows, number of columns)
    /// NOTE: All the indexing and dimensions follow the matrix
    /// indexing scheme (i.e (rows, columns)).
    pub fn new(bounding_rect: Rect, num_cells: &CellIndex) -> Self {
        let cell_height = bounding_rect.h() / num_cells.row as f32;
        let cell_width = bounding_rect.w() / num_cells.col as f32;

        // Construct the 2D grid of cells using the bounding rectangle passed into the
        // constructor as an anchor for dimensions and position.
        let cells: Vec<Vec<Cell>> = (0..num_cells.row)
            .map(|row| {
                (0..num_cells.col)
                    .map(|col| {
                        Cell::new(
                            vec2(cell_width, cell_height), // Dimensions
                            vec2(
                                // Anchor from the top left of the grid and
                                // build downwards and to the right.
                                // Note: Nannou grids decrease in x going left
                                // but increase in y going up. Top left is
                                // (-width, height)
                                // The 0.5 is to center the coordinate in the middle
                                // of the cell
                                bounding_rect.top_left().x + (cell_width * (col as f32 + 0.5)),
                                bounding_rect.top_left().y
                                    - (cell_height * (row as f32 + 0.5) as f32),
                            ),
                            CellIndex { row, col },
                        )
                    })
                    .collect()
            })
            .collect(); // Collect on both levels of vector
        Grid {
            cells,
            rect: bounding_rect,
        }
    }

    /// The row major iterator will traverse from the
    /// top left of the matrix down to the bottom right
    /// going cell by cell in each row.
    pub fn row_major_iter(&self) -> GridIterator {
        GridIterator::new(self)
    }

    fn get_cell_by_index(&self, row: usize, col: usize) -> Option<&Cell> {
        let row = self.cells.get(row);
        row.map(|r| r.get(col)).flatten()
    }

    pub fn diagonal_length(&self) -> f32 {
        self.rect.top_left().distance(self.rect.bottom_right())
    }

    pub fn wh(&self) -> Vec2 {
        self.rect.wh()
    }

    pub fn xy(&self) -> Vec2 {
        self.rect.xy()
    }
}

pub struct GridIterator<'a> {
    grid: &'a Grid,
    curr: (usize, usize),
}

impl<'a> GridIterator<'a> {
    fn new(grid: &'a Grid) -> Self {
        GridIterator { grid, curr: (0, 0) }
    }
}

impl<'a> Iterator for GridIterator<'a> {
    type Item = &'a Cell;
    fn next(&mut self) -> Option<Self::Item> {
        let mut col_idx = self.curr.0;
        let mut row_idx = self.curr.1;

        // We're at or past (should never go past) the last row
        // of the grid.
        if row_idx >= self.grid.cells.len() {
            return None;
        }

        // Move through the grid till you've found the next valid
        // cell (i.e skip empty vectors in the middle)
        let mut row = self.grid.cells.get(row_idx).unwrap();
        while col_idx >= row.len() {
            row_idx += 1;
            if row_idx >= self.grid.cells.len() {
                return None;
            }
            row = self.grid.cells.get(row_idx).unwrap();
            col_idx = 0;
        }

        let cell = row.get(col_idx).unwrap();
        col_idx += 1;
        self.curr = (col_idx, row_idx);
        return Some(cell);
    }
}

mod tests {
    use super::{CellIndex, Grid};
    use nannou::prelude::*;

    #[test]
    fn all_indicies_are_correct() {
        let grid_cells = CellIndex { row: 5, col: 5 };
        let rect = geom::Rect::from_x_y_w_h(0.0, 0.0, 5.0, 5.0);
        let grid = Grid::new(&rect, &grid_cells);

        let mut iter = grid.row_major_iter();
        for y in 0..grid_cells.row {
            for x in 0..grid_cells.col {
                let cell = iter.next();
                assert!(cell.is_some());
                assert_eq!(cell.unwrap().index.row, y);
                assert_eq!(cell.unwrap().index.col, x);
            }
        }
    }

    #[test]
    fn cell_has_correct_metadata() {
        let grid_cells = CellIndex { row: 5, col: 5 };
        let rect = geom::Rect::from_x_y_w_h(0.0, 0.0, 5.0, 5.0);
        let grid = Grid::new(&rect, &grid_cells);

        let cell_option = grid.get_cell_by_index(0, 2);
        assert!(cell_option.is_some(), "this cell should exist");
        let cell = cell_option.unwrap();
        assert_eq!(cell.index.row, 0);
        assert_eq!(cell.index.col, 2);
        assert_eq!(cell.wh.x, 1.0, "cell width and height should be 1.0");
        assert_eq!(cell.wh.y, 1.0, "cell width and height should be 1.0");
        assert_eq!(
            cell.xy.y, 2.0,
            "the top row should have a y value of height - cell_height / 2.0"
        );
        assert_eq!(
            cell.xy.x, 0.0,
            "the second column should have an x value of -width + (cell_height / 2.0) * col",
        )
    }

    #[test]
    fn grid_coordinates_are_correct_when_rect_is_offset() {
        let grid_cells = CellIndex { row: 5, col: 5 };
        let rect = geom::Rect::from_x_y_w_h(10.0, 10.0, 10.0, 10.0);
        let grid = Grid::new(&rect, &grid_cells);

        let cell_option = grid.get_cell_by_index(0, 0);
        assert!(cell_option.is_some(), "this cell should exist");
        let cell = cell_option.unwrap();
        assert_eq!(cell.wh.x, 2.0);
        assert_eq!(cell.wh.y, 2.0);
        assert_eq!(cell.index.row, 0);
        assert_eq!(cell.index.col, 0);
        assert_eq!(cell.xy.x, 6.0);
        assert_eq!(cell.xy.y, 14.0);

        let cell_option = grid.get_cell_by_index(5, 5);
        assert!(cell_option.is_none(), "this cell should NOT exist");

        let cell_option = grid.get_cell_by_index(4, 4);
        assert!(cell_option.is_some(), "this cell should exist");
        let cell = cell_option.unwrap();
        assert_eq!(cell.wh.x, 2.0);
        assert_eq!(cell.wh.y, 2.0);
        assert_eq!(cell.index.row, 4);
        assert_eq!(cell.index.col, 4);
        assert_eq!(cell.xy.x, 14.0);
        assert_eq!(cell.xy.y, 6.0);
    }

    #[test]
    fn grid_coordinates_are_correct_when_rect_is_not_square() {
        let grid_cells = CellIndex { row: 10, col: 5 };
        let rect = geom::Rect::from_x_y_w_h(0.0, 0.0, 10.0, 100.0);
        let grid = Grid::new(&rect, &grid_cells);

        let cell_option = grid.get_cell_by_index(0, 0);
        assert!(cell_option.is_some(), "this cell should exist");
        let cell = cell_option.unwrap();
        assert_eq!(cell.wh.x, 2.0);
        assert_eq!(cell.wh.y, 10.0);
        assert_eq!(cell.index.row, 0);
        assert_eq!(cell.index.col, 0);
        assert_eq!(cell.xy.x, -4.0);
        assert_eq!(cell.xy.y, 45.0);

        let cell_option = grid.get_cell_by_index(0, 4);
        assert!(cell_option.is_some(), "this cell should exist");
        let cell = cell_option.unwrap();
        assert_eq!(cell.index.row, 0);
        assert_eq!(cell.index.col, 4);
        assert_eq!(cell.xy.x, 4.0);
        assert_eq!(cell.xy.y, 45.0);

        let cell_option = grid.get_cell_by_index(9, 0);
        assert!(cell_option.is_some(), "this cell should exist");
        let cell = cell_option.unwrap();
        assert_eq!(cell.index.row, 9);
        assert_eq!(cell.index.col, 0);
        assert_eq!(cell.xy.x, -4.0);
        assert_eq!(cell.xy.y, -45.0);

        let cell_option = grid.get_cell_by_index(9, 4);
        assert!(cell_option.is_some(), "this cell should exist");
        let cell = cell_option.unwrap();
        assert_eq!(cell.index.row, 9);
        assert_eq!(cell.index.col, 4);
        assert_eq!(cell.xy.x, 4.0);
        assert_eq!(cell.xy.y, -45.0);
    }

    #[test]
    fn grid_edge_coordinates_are_correct_using_practical_values() {
        let grid_cells = CellIndex { row: 30, col: 30 };
        let rect = geom::Rect::from_x_y_w_h(0.0, 0.0, 1500.0, 1500.0);
        let grid = Grid::new(&rect, &grid_cells);

        let cell_option = grid.get_cell_by_index(0, 0);
        assert!(cell_option.is_some(), "this cell should exist");
        let cell = cell_option.unwrap();
        assert_eq!(cell.wh.x, 50.0);
        assert_eq!(cell.wh.y, 50.0);
        assert_eq!(cell.index.row, 0);
        assert_eq!(cell.index.col, 0);
        assert_eq!(
            cell.xy.x - cell.wh.x / 2.0,
            rect.left(),
            "cell's left coordinate matches bounding rectangle left"
        );
        assert_eq!(
            cell.xy.y + cell.wh.y / 2.0,
            rect.top(),
            "cell's top coordinate matches bounding rectangle top"
        );
    }
}
