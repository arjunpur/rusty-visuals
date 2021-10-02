use nannou::color::*;
use nannou::noise::*;
use nannou::prelude::*;
use std::collections::vec_deque::*;

pub struct NoiseColoredGrid {}

impl NoiseColoredGrid {
    pub fn draw(draw: &Draw, rect: &Rect, resolution: f32, colorer: &mut dyn Colorer) {
        // This rect is shifted and moved around the grid to help align the rect we're
        // drawing
        let mut aligning_rect = Rect::from_wh(pt2(resolution, resolution))
            .align_left_of(*rect)
            .align_bottom_of(*rect);

        while aligning_rect.y.start <= rect.top() {
            while aligning_rect.x.start <= rect.right() {
                draw.rect()
                    .wh(pt2(resolution, resolution))
                    .xy(aligning_rect.xy())
                    .color(colorer.color(aligning_rect.x.start, aligning_rect.y.start));
                aligning_rect = aligning_rect.shift_x(resolution);
            }
            // Reset x back to the left
            aligning_rect = aligning_rect.align_left_of(*rect);
            aligning_rect = aligning_rect.shift_y(resolution);
        }
    }
}

pub trait Colorer {
    fn color(&mut self, x: f32, y: f32) -> Hsv;
}

pub struct NoiseColorer {
    base_color: Hsv,
    current_color: Hsv,
    hue_bound: Vector2,
    noise: SuperSimplex,
}

impl Colorer for NoiseColorer {
    fn color(&mut self, x: f32, y: f32) -> Hsv {
        // Assume that base_color's Hue is not None
        let current_hue = self.current_color.get_hue().unwrap();
        // .unwrap_or(self.base_color.get_hue().unwrap());

        let current_saturation = self.current_color.saturation;
        let current_value = self.current_color.value;

        let hue_delta = self
            .noise
            .get([x as f64, y as f64, current_hue.to_radians() as f64])
            / 10.0;
        let saturation_delta = self.noise.get([
            x as f64 + 1000.0,
            y as f64 + 1000.0,
            current_saturation as f64,
        ]) as f32;
        let brightness_delta =
            self.noise
                .get([x as f64 + 10000.0, y as f64 + 10000.0, current_value as f64])
                as f32;

        println!(
            "current hue: {}, hue_delta: {}",
            current_hue.to_radians(),
            hue_delta
        );
        let range_size = self.hue_bound.y - self.hue_bound.x;
        let new_hue = (((current_hue.to_radians() as f32 + hue_delta as f32) - self.hue_bound.x)
            % range_size)
            + self.hue_bound.x;
        let new_color = Hsv::new(
            new_hue.to_degrees(),
            current_saturation + saturation_delta,
            current_value + brightness_delta,
        );
        self.current_color = new_color;
        return new_color;
    }
}

impl NoiseColorer {
    pub fn new(base_color: Hsv, hue_bound: Vector2) -> Self {
        let noise = SuperSimplex::new();
        let current_hue = base_color.get_hue().unwrap().to_degrees();
        if current_hue < hue_bound.x || current_hue > hue_bound.y {
            panic!("Hue of base color out of provided bounds");
        }
        return NoiseColorer {
            base_color,
            current_color: base_color,
            hue_bound,
            noise,
        };
    }
}

pub struct AlternatingColorer {
    colors: VecDeque<Hsv>,
}

impl Colorer for AlternatingColorer {
    fn color(&mut self, x: f32, y: f32) -> Hsv {
        let color = self.colors.pop_front().unwrap();
        self.colors.push_back(color);
        return color;
    }
}

impl AlternatingColorer {
    pub fn new(colors: VecDeque<Hsv>) -> Self {
        return AlternatingColorer { colors };
    }
}
