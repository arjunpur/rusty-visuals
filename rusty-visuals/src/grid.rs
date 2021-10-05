use nannou::color::*;
use nannou::noise::*;
use nannou::prelude::*;

use std::collections::vec_deque::*;

pub struct ColoredGrid {}

impl ColoredGrid {
    // `resolution` is the size of each individual box -- only squares are currently supported
    pub fn draw(draw: &Draw, rect: &Rect, resolution: f32, colorer: &mut dyn Colorer) {
        // This rect is shifted and moved around the grid to help align the rect we're
        // drawing
        let mut aligning_rect = Rect::from_wh(pt2(resolution, resolution))
            .align_left_of(*rect)
            .align_bottom_of(*rect);

        // Total number of rectangles in the grid; We take the ceiling to ensure we cover the whole
        // grid.
        let t_x = (rect.w() / resolution).ceil() as i32;
        let t_y = (rect.h() / resolution).ceil() as i32;

        // Indices of the rectangle within the grid.
        let mut i_x = 0;
        let mut i_y = 0;
        while aligning_rect.y.start <= rect.top() {
            while aligning_rect.x.start <= rect.right() {
                let radius = rect.w();

                // TODO: Clean up this randomization (ajain)
                let mut color = Hsv::new(0.0, 0.0, 1.0);

                // Colors in a circle
                if (aligning_rect.left() + (radius / 2.0)).pow(2.0)
                    + (aligning_rect.bottom() + (radius / 2.0)).pow(2.0)
                    <= radius.pow(2.0)
                {
                    color = colorer.color(i_x, i_y, t_x, t_y);
                } else if random_f32() < 0.70 {
                    color = Hsv::new(200.0, 0.9, 1.0);
                }

                draw.rect()
                    .wh(pt2(resolution, resolution))
                    .xy(aligning_rect.xy())
                    .color(color);
                aligning_rect = aligning_rect.shift_x(resolution);
                i_x += 1
            }
            // Reset x back to the left
            aligning_rect = aligning_rect.align_left_of(*rect);
            aligning_rect = aligning_rect.shift_y(resolution);
            i_x = 0;
            i_y += 1
        }
    }
}

pub trait Colorer {
    // i_x: The column index of the rectangle to color
    // i_y: The row index of the rectangle to color
    // t_x: The total number of columns
    // t_y: The total number of rows
    fn color(&mut self, i_x: i32, i_y: i32, t_x: i32, t_y: i32) -> Hsv;
}

pub struct NoiseColorer {
    base_color: Hsv,
    current_color: Hsv,
    hue_bound: Vector2,
    noise: SuperSimplex,
}

impl Colorer for NoiseColorer {
    fn color(&mut self, i_x: i32, i_y: i32, _t_x: i32, _t_y: i32) -> Hsv {
        // Assume that base_color's Hue is not None
        let current_hue = self.current_color.get_hue().unwrap();
        // .unwrap_or(self.base_color.get_hue().unwrap());

        let current_saturation = self.current_color.saturation;
        let current_value = self.current_color.value;

        // Use noise functions to move the hue, saturation and brigthness around
        let hue_delta = self
            .noise
            .get([i_x as f64, i_y as f64, current_hue.to_radians() as f64])
            / 100.0;
        let saturation_delta = self.noise.get([
            i_x as f64 + 1000.0,
            i_y as f64 + 1000.0,
            current_saturation as f64,
        ]) as f32
            / 100.0;
        let brightness_delta = self.noise.get([
            i_x as f64 + 10000.0,
            i_y as f64 + 10000.0,
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
        let new_color = Hsv::new(
            new_hue.to_degrees(),
            current_saturation + saturation_delta,
            current_value + brightness_delta,
        );
        self.current_color = new_color;
        new_color
    }
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
            current_color: base_color,
            hue_bound,
            noise,
        }
    }
}

pub struct AlternatingColorer {
    colors: VecDeque<Hsv>,
}

impl Colorer for AlternatingColorer {
    fn color(&mut self, _i_x: i32, _i_y: i32, _t_x: i32, _t_y: i32) -> Hsv {
        let color = self.colors.pop_front().unwrap();
        self.colors.push_back(color);
        color
    }
}

impl AlternatingColorer {
    pub fn new(colors: VecDeque<Hsv>) -> Self {
        AlternatingColorer { colors }
    }
}

pub struct InterpolatedColorer {
    color_range: (Hsv, Hsv),
    base_gradient: Gradient<Hsv>,
}

impl Colorer for InterpolatedColorer {
    fn color(&mut self, i_x: i32, i_y: i32, t_x: i32, t_y: i32) -> Hsv {
        let color_for_idx = map_range(i_x, 0, t_x, 0.0, 1.0);
        self.get_gradient(i_x, i_y, t_x, t_y).get(color_for_idx)
    }
}

impl InterpolatedColorer {
    pub fn new(color_range: (Hsv, Hsv)) -> Self {
        let base_gradient = Gradient::new(vec![color_range.0, color_range.1]);
        InterpolatedColorer {
            color_range,
            base_gradient,
        }
    }

    // TODO: This can be precomputed if we know the number of tiles in the grid when the
    // interpolated colorer is constructed.
    // TODO:
    fn get_gradient(&mut self, _i_x: i32, i_y: i32, _t_x: i32, t_y: i32) -> Gradient<Hsv> {
        let y_gradient_start_idx = map_range(i_y, 0, t_y, 0.0, 1.0);
        let y_gradient_start = self.base_gradient.get(y_gradient_start_idx);

        // 30.0 degree step for now. Parametrize this
        let y_gradient_end = y_gradient_start + Hsv::new(30.0, 0.0, 0.0);
        
        Gradient::new(vec![y_gradient_start, y_gradient_end])
    }
}
