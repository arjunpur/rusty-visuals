use nannou::prelude::*;
use crate::colorer::{GridColorer, GridParams};

pub struct AlternatingColorer {
    colors: Vec<Hsv>,
}

impl GridColorer for AlternatingColorer {
    fn color(&self, params: GridParams) -> Hsv {
        let position =
            ((params.cell.index.col + params.cell.index.row) as i32) % self.colors.len() as i32;
        *self.colors.get(position as usize).unwrap()
    }

    fn update(&mut self) {}
}

impl AlternatingColorer {
    pub fn new(colors: Vec<Hsv>) -> Self {
        AlternatingColorer { colors }
    }
}
