use crate::colorer::{Colorer, GridColorer, GridParams};
use nannou::prelude::*;
use rand::{thread_rng, Rng};

pub struct PaletteColorer {
    hues: Vec<f32>,
    saturations: Vec<f32>,
    values: Vec<f32>,
}

impl GridColorer for PaletteColorer {
    fn color(&self, _: GridParams) -> Hsv {
        self.color()
    }

    fn update(&mut self) {}
}

impl Colorer for PaletteColorer {
    fn color(&self) -> Hsv {
        self.color()
    }
    fn update(&mut self) {}
}

impl PaletteColorer {
    pub fn new(hues: Vec<f32>, saturations: Vec<f32>, values: Vec<f32>) -> Self {
        if hues.len() == 0 || saturations.len() == 0 || values.len() == 0 {
            panic!("hues or saturations or values must not be empty");
        }
        PaletteColorer {
            hues,
            saturations,
            values,
        }
    }

    fn color(&self) -> Hsv {
        let mut rng = thread_rng();
        let hue_idx = rng.gen_range(0, self.hues.len());
        let saturations_idx = rng.gen_range(0, self.saturations.len());
        let values_idx = rng.gen_range(0, self.values.len());
        hsv(
            self.hues[hue_idx],
            self.saturations[saturations_idx],
            self.values[values_idx],
        )
    }
}

pub struct PastelColorer {
    colorer: Box<dyn Colorer>,
}

impl Colorer for PastelColorer {
    fn color(&self) -> Hsv {
        self.colorer.color()
    }

    fn update(&mut self) {}
}

impl PastelColorer {
    pub fn new() -> Self {
        let colorer = PaletteColorer::new(
            (0..360).map(|n| n as f32 / 360.0).collect(),
            vec![0.6],
            vec![1.0],
        );

        PastelColorer {
            colorer: Box::new(colorer),
        }
    }
}

impl Default for PastelColorer {
    fn default() -> Self {
        PastelColorer::new()
    }
}
