use nannou::prelude::*;
use crate::colorer::Colorer;

/// ModuloColorer invokes the underlying colorer every nth time.
/// Every other time, this will return the `base_color`
pub struct ModuloColorer {
    base_color: Hsv,
    colorer: Box<dyn Colorer>,
    iteration: i32,
    modulo_n: i32,
}

impl Colorer for ModuloColorer {
    fn color(&self) -> Hsv {
        if self.iteration % self.modulo_n == 0 {
            return self.colorer.color();
        }
        self.base_color
    }

    fn update(&mut self) {
        self.iteration += 1
    }
}

impl ModuloColorer {
    pub fn new(colorer: Box<dyn Colorer>, base_color: Hsv, modulo_n: i32) -> Self {
        ModuloColorer {
            base_color,
            colorer,
            modulo_n,
            iteration: 0,
        }
    }
}
