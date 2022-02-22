use nannou::geom::{Rect, Vec2};
use nannou::noise::*;
use nannou::prelude::*;

const RESOLUTION: f32 = 10.0;
// These smoothers are used to reduce the distance between
// subsequent invocations of noise so that outputs are also
// closer together, giving us the impression that the changes
// are smooth.
const ANGLE_SMOOTHER: f32 = 500.0;
const TIME_SMOOTHER: f64 = 3.0;
const MAGNITUDE_SMOOTHER: f32 = 100.0;
// Since Perlin is deterministic and since values closer to
// each other will be closer even after the noise is applied,
// we offset the magnitude and angle by some large number to ensure
// angle and magnitude vary fairly differently.
const ANGLE_MAGNITUDE_NOISE_OFFSET: f64 = 40000.0;
// How much we scale the final force vector's magnitude.
const MAGNITUDE_SCALE: f32 = 2.0;

pub struct ForceField {
    rect: Rect<f32>,
    force_field: Vec<Vec<Vec2>>,
}

// ForceField is an abstraction coupled with Nannou's Draw API. It allows
// callers to create, update, and display a ForceField with Perlin noise
// adjusted vectors (color, magnitude and angle).
// The ForceField can be queried for an acceleration at any given position
// in a rectangle from `get_acceleration_from_position`. Callers may update
// objects with this acceleration to give the perception of smoothed motion.
impl ForceField {
    pub fn new(rect: Rect<f32>, time: f32) -> ForceField {
        // Calculate the dimensions of the force field
        let num_forces = (
            (rect.w() / RESOLUTION).ceil() as usize,
            (rect.h() / RESOLUTION).ceil() as usize,
        );
        let mut force_field = vec![vec![vec2(0.0, 0.0); num_forces.0]; num_forces.1];
        populate_force_field(rect, time, &mut force_field);
        ForceField { rect, force_field }
    }

    // To be called by a nannou `update` function. Simply updates the vectors
    // in the force field by the next unit in time.
    pub fn update(&mut self, rect: Rect<f32>, time: f32) {
        self.rect = rect;
        populate_force_field(rect, time, &mut self.force_field)
    }

    // Snaps a position to a given vector on the force field and returns the
    // acceleration represented by that force field.
    pub fn get_acceleration_from_position(&self, position: Vec2) -> Vec2 {
        let scaled_mover_position = vec2(
            // We use clamp to ensure we don't return out of bounds indicies
            clamp(
                // Map the position to a particular index. This is essentially doing:
                // floor( (x / width) * num_forces_on_x_axis )
                map_range(
                    position.x,
                    0.0,
                    self.rect.w(),
                    0.0,
                    (self.force_field[0].len() as i32) as f32,
                ),
                0.0,
                (self.force_field[0].len() as i32) as f32,
            ),
            clamp(
                map_range(
                    position.y,
                    0.0,
                    self.rect.h(),
                    0.0,
                    (self.force_field.len() as i32) as f32,
                ),
                0.0,
                (self.force_field.len() as i32) as f32,
            ),
        );
        self.force_field[scaled_mover_position.y as usize]
            [scaled_mover_position.x as usize]
    }

    pub fn display(&self, draw: &Draw, time: f32) {
        let bottom_left = self.rect.bottom_left();
        let noise = SuperSimplex::new();
        for i in 0..self.force_field.len() {
            for j in 0..self.force_field[i].len() {
                let start = bottom_left + pt2(j as f32 * RESOLUTION, i as f32 * RESOLUTION);
                let end = start + self.force_field[i][j];
                // Vary the hue of the vectors with a smoothed noise function
                // so that colors may also change with time.
                let hue = noise.get([
                    (start.x as f64) / 10000.0,
                    (start.y as f64) / 10000.0,
                    time as f64 / 3.0,
                ]);
                draw.line()
                    .points(start, end)
                    .hsl(hue as f32, 0.7, 0.5)
                    .weight(2.0);
            }
        }
    }
}

// The two functions here are used to update the forces on the force
// field with a noise randomized step. We use a bunch of constants (ex. ANGLE_SMOOTHER) to scale down the inputs so that noise
// returns closer values, giving a more smoothed out visual experience.
fn populate_force_field(
    rect: Rect<f32>,
    time: f32,
    force_field: &mut Vec<Vec<Vec2>>,
) {
    let bottom_left = rect.bottom_left();
    for i in 0..force_field.len() {
        for j in 0..force_field[i].len() {
            force_field[i][j] = create_force_from_noise(
                time,
                bottom_left.x + (j as f32 * RESOLUTION),
                bottom_left.y + (i as f32 * RESOLUTION),
            );
        }
    }
}

fn create_force_from_noise(time: f32, x: f32, y: f32) -> Vec2 {
    let noise = Perlin::new();
    let angle = noise.get([
        (x / ANGLE_SMOOTHER) as f64,
        (y / ANGLE_SMOOTHER) as f64,
        time as f64 / TIME_SMOOTHER,
    ]) as f32
        * TAU;
    let magnitude = noise.get([
        (x / MAGNITUDE_SMOOTHER) as f64 + ANGLE_MAGNITUDE_NOISE_OFFSET,
        (y / MAGNITUDE_SMOOTHER) as f64 + ANGLE_MAGNITUDE_NOISE_OFFSET,
        time as f64 / TIME_SMOOTHER,
    ]) as f32;
    Vec2::new(angle.cos(), angle.sin()) * magnitude * MAGNITUDE_SCALE
}
