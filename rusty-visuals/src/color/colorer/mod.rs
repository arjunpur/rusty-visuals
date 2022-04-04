mod grid_colorer;
pub use grid_colorer::*;

mod alternating_colorer;
pub use alternating_colorer::*;

mod interpolated_colorer;
pub use interpolated_colorer::*;

mod modulo_colorer;
pub use modulo_colorer::*;

mod palette_colorer;
pub use palette_colorer::*;

mod rotating_colorer;
pub use rotating_colorer::*;

use nannou::color::*;

/// Colorer is a more generic version of the GridColorer. The GridColorer is expected to color
/// grids, however this Colorer is more stateless in that there are no arguments being
/// fed to it.
/// TODO: I wonder if we could use generics to implement a GridColorer using the Colorer 
pub trait Colorer {
    fn color(&self) -> Hsv;

    fn update(&mut self);
}
