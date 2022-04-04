use std::collections::VecDeque;
use nannou::prelude::*;
use crate::colorer::{GridColorer, GridParams};

/// RotatingColorer keeps a VecDeque of colorers and will always use the front of the VecDeque
/// as the current colorer. The colorer can be rotated by invoking the `update` method
pub struct RotatingColorer {
    colorers: VecDeque<Box<dyn GridColorer>>,
}

impl GridColorer for RotatingColorer {
    fn color(&self, params: GridParams) -> Hsv {
        let colorer = self.colorers.front().unwrap();
        (*colorer).color(params)
    }

    fn update(&mut self) {
        let front_colorer = self.colorers.pop_front().unwrap();
        self.colorers.push_back(front_colorer);
    }
}

impl RotatingColorer {
    pub fn new(colorers: VecDeque<Box<dyn GridColorer>>) -> Self {
        RotatingColorer { colorers }
    }
}
