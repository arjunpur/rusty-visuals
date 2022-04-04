use nannou::prelude::*;
use nannou::color::*;
use crate::colorer::{GridColorer, GridParams};

// mod grid_colorer;
// use grid_colorer::{GridColorer, GridParams};

/// InterpolatedColorer will color the grid's first row with the provided Gradient.
/// All subsequent rows are colored starting with the color pointed to by the current row's index
/// into the Gradient. The end of the Gradient is shifted by 30 degrees.
pub struct InterpolatedColorer {
    base_gradient: Gradient<Hsv>,
}

impl GridColorer for InterpolatedColorer {
    fn color(&self, params: GridParams) -> Hsv {
        // Choose the color on the x gradient plane
        let color_for_idx = map_range(
            params.cell.index.col,
            0,
            params.total_num_cells.col,
            0.0,
            1.0,
        );
        // Create the plane for the y axis and then select the
        // x color plane from this gradient.
        self.get_gradient(params.cell.index.row, params.total_num_cells.row)
            .get(color_for_idx)
    }

    fn update(&mut self) {}
}

impl InterpolatedColorer {
    pub fn new(color_range: (Hsv, Hsv)) -> Self {
        let base_gradient = Gradient::new(vec![color_range.0, color_range.1]);
        InterpolatedColorer { base_gradient }
    }

    // TODO: This can be precomputed if we know the number of tiles in the grid when the
    // interpolated colorer is constructed.
    fn get_gradient(&self, row: usize, num_rows: usize) -> Gradient<Hsv> {
        let y_gradient_start_idx = map_range(row, 0, num_rows, 0.0, 1.0);
        let y_gradient_start = self.base_gradient.get(y_gradient_start_idx);

        // Keep the difference between the new start and end the same by using the original
        // gradient's difference
        let original_difference = self.base_gradient.get(1.0) - self.base_gradient.get(0.0);
        let y_gradient_end = y_gradient_start + original_difference;

        Gradient::new(vec![y_gradient_start, y_gradient_end])
    }
}
