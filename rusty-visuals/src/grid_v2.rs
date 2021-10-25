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

#[derive(Debug, Default, Clone)]
pub struct Cell {
    xy: Vector2,
    wh: Vector2,

    // index is the (i,j) row, column index that this cell
    // represents.
    index: Vector2<usize>,
}

impl Cell {
    fn new(wh: Vector2, xy: Vector2, index: Vector2<usize>) -> Self {
        Cell { wh, xy, index }
    }
}

pub struct Grid {
    cells: Vec<Vec<Cell>>,
}

impl Grid {
    fn new(bounding_rect: &Rect, num_cells: Vector2<usize>) -> Self {
        let cell_width = bounding_rect.w() / num_cells.x as f32;
        let cell_height = bounding_rect.h() / num_cells.y as f32;

        // Construct the 2D grid of cells using the bounding rectangle passed into the
        // constructor as an anchor for dimensions and position.
        let cells: Vec<Vec<Cell>> = (0..num_cells.y)
            .map(|row| {
                (0..num_cells.x)
                    .map(|col| {
                        Cell::new(
                            vec2(cell_width, cell_height),
                            vec2(
                                bounding_rect.top_left().x
                                    + ((cell_width / 2.0) * (col + 1) as f32),
                                bounding_rect.top_left().y
                                    + ((cell_height / 2.0) * (row + 1) as f32),
                            ),
                            vec2(col, row),
                        )
                    })
                    .collect()
            })
            .collect();
        Grid { cells }
    }

    fn row_major_iter(&self) -> GridIterator {
        GridIterator::new(&self)
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
        // We're at or past (should never go past) the last row
        // of the grid.
        let mut col_idx = self.curr.0;
        let mut row_idx = self.curr.1;

        if row_idx >= self.grid.cells.len() {
            return None;
        }
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

// TODO: Test the actual coordinates are correct
// TODO: Test non-square grids
// TODO: Test grids with some empty rows
// TODO: Test grids where the rectangle starts at an offset
// TODO: Test grids where the rectangle is not square
mod tests {
    use super::Grid;
    use nannou::prelude::*;
    #[test]
    fn can_construct_a_grid() {
        let grid_cells = vec2(5, 5);
        let rect = geom::Rect::from_x_y_w_h(0.0, 0.0, 5.0, 5.0);
        let grid = Grid::new(&rect, grid_cells);
        println!("{:?}", grid.cells);
        let mut iter = grid.row_major_iter();
        for y in 0..grid_cells.y {
            for x in 0..grid_cells.x {
                let cell = iter.next();
                assert!(cell.is_some());
                assert_eq!(cell.unwrap().index.x, x);
                assert_eq!(cell.unwrap().index.y, y);
                if cell.is_some() {
                    println!("({}, {}): {:?}", y, x, cell.unwrap());
                }
            }
        }
    }
}
