use nannou::color::*;
use crate::grid;

/// GridParams are all the various options provided to a Colorer's
/// `color` function
/// TODO: This should be generic so that the Colorer can color other things
/// and not just grids.
pub struct GridParams<'a> {
    pub cell: &'a grid::Cell,
    pub total_num_cells: &'a grid::CellIndex,
}

/// GridColorer is the trait that all GridColorer's must implement. As long as a struct
/// implements this trait, it can be used to color a ColoredGrid.
pub trait GridColorer {
    fn color(&self, params: GridParams) -> Hsv;

    fn update(&mut self);
}
