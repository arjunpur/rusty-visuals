use nannou::color::*;
use nannou::noise::*;
use nannou::prelude::*;

use std::collections::VecDeque;

/// ColorerParams are all the various options provided to a Colorer's
/// `color` function
/// TODO: This should be generic so that the Colorer can color other things
/// and not just grids.
pub struct ColorerParams<'a> {
    pub box_pos: Vector2<i32>,
    pub total_num_boxes: Vector2<i32>,
    pub current_box_rect: &'a Rect,
    pub grid_rect: &'a Rect,
}

/// Colorer is the trait that all Colorer's must implement. As long as a struct
/// implements this trait, it can be used to color a ColoredGrid.
pub trait Colorer {
    fn color(&self, params: ColorerParams) -> Hsv;

    fn update(&mut self);
}

/// InterpolatedColorer will color the grid's first row with the provided Gradient.
/// All subsequent rows are colored starting with the color pointed to by the current row's index
/// into the Gradient. The end of the Gradient is shifted by 30 degrees.
pub struct InterpolatedColorer {
    base_gradient: Gradient<Hsv>,
}

impl Colorer for InterpolatedColorer {
    fn color(&self, params: ColorerParams) -> Hsv {
        let color_for_idx = map_range(params.box_pos.x, 0, params.total_num_boxes.x, 0.0, 1.0);
        self.get_gradient(
            params.box_pos.x,
            params.box_pos.y,
            params.total_num_boxes.x,
            params.total_num_boxes.y,
        )
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
    fn get_gradient(&self, _i_x: i32, i_y: i32, _t_x: i32, t_y: i32) -> Gradient<Hsv> {
        let y_gradient_start_idx = map_range(i_y, 0, t_y, 0.0, 1.0);
        let y_gradient_start = self.base_gradient.get(y_gradient_start_idx);

        // Keep the difference between the new start and end the same by using the original
        // gradient's difference
        let original_difference = self.base_gradient.get(1.0) - self.base_gradient.get(0.0);
        let y_gradient_end = y_gradient_start + original_difference;

        Gradient::new(vec![y_gradient_start, y_gradient_end])
    }
}

/// RotatingColorer keeps a VecDeque of colorers and will always use the front of the VecDeque
/// as the current colorer. The colorer can be rotated by invoking the `update` method
pub struct RotatingColorer {
    colorers: VecDeque<Box<dyn Colorer>>,
}

impl Colorer for RotatingColorer {
    fn color(&self, params: ColorerParams) -> Hsv {
        let colorer = self.colorers.front().unwrap();
        (*colorer).color(params)
    }

    fn update(&mut self) {
        let front_colorer = self.colorers.pop_front().unwrap();
        self.colorers.push_back(front_colorer);
    }
}

impl RotatingColorer {
    pub fn new(colorers: VecDeque<Box<dyn Colorer>>) -> Self {
        RotatingColorer { colorers }
    }
}

pub struct NoiseColorer {
    base_color: Hsv,
    hue_bound: Vector2,
    noise: SuperSimplex,
}

impl Colorer for NoiseColorer {
    fn color(&self, params: ColorerParams) -> Hsv {
        // Assume that base_color's Hue is not None
        let current_hue = self.base_color.get_hue().unwrap();
        // .unwrap_or(self.base_color.get_hue().unwrap());

        let current_saturation = self.base_color.saturation;
        let current_value = self.base_color.value;

        // Use noise functions to move the hue, saturation and brigthness around
        let hue_delta = self.noise.get([
            params.box_pos.x as f64,
            params.box_pos.y as f64,
            current_hue.to_radians() as f64,
        ]) / 100.0;
        let saturation_delta = self.noise.get([
            params.box_pos.x as f64 + 1000.0,
            params.box_pos.y as f64 + 1000.0,
            current_saturation as f64,
        ]) as f32
            / 100.0;
        let brightness_delta = self.noise.get([
            params.box_pos.x as f64 + 10000.0,
            params.box_pos.y as f64 + 10000.0,
            current_value as f64,
        ]) as f32
            / 100.0;

        println!(
            "current hue: {}, hue_delta: {}",
            current_hue.to_radians(),
            hue_delta
        );

        // Move the Hue but within a range only
        // let range_size = self.hue_bound.y - self.hue_bound.x;
        // let new_hue = (((current_hue.to_radians() as f32 + hue_delta as f32) - self.hue_bound.x)
        //     % range_size)
        //     + self.hue_bound.x;
        let new_hue = current_hue.to_radians() as f32 + hue_delta as f32;

        Hsv::new(
            new_hue.to_degrees(),
            current_saturation + saturation_delta,
            current_value + brightness_delta,
        )
    }

    fn update(&mut self) {}
}

impl NoiseColorer {
    pub fn new(base_color: Hsv, hue_bound: Vector2) -> Self {
        let noise = SuperSimplex::new();
        let current_hue = base_color.get_hue().unwrap().to_degrees();
        if current_hue < hue_bound.x || current_hue > hue_bound.y {
            panic!("Hue of base color out of provided bounds");
        }
        NoiseColorer {
            base_color,
            hue_bound,
            noise,
        }
    }
}

pub struct AlternatingColorer {
    colors: Vec<Hsv>,
}

impl Colorer for AlternatingColorer {
    fn color(&self, params: ColorerParams) -> Hsv {
        let position = (params.box_pos.x + params.box_pos.y) % self.colors.len() as i32;
        *self.colors.get(position as usize).unwrap()
    }

    fn update(&mut self) {}
}

impl AlternatingColorer {
    pub fn new(colors: Vec<Hsv>) -> Self {
        AlternatingColorer { colors }
    }
}
