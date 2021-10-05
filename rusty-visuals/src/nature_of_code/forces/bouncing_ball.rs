use nannou::app::DrawScalar;
use nannou::geom::{Point2, Vector2};
use nannou::noise::*;
use nannou::prelude::*;
use rusty_visuals::mover::Mover;

const GRAVITY: Vector2 = Vector2 { x: 0.0, y: -1.5 };

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(600, 600)
        .run();
}

struct Model {
    mover: Mover,
}

fn model(app: &App) -> Model {
    let mover: Mover = Mover::new(app.window_rect());
    Model { mover }
}

fn update(app: &App, m: &mut Model, _update: Update) {
    let wind = create_wind(app.time, m.mover.position);
    m.mover.apply_force(GRAVITY);
    m.mover.apply_force(wind);
    m.mover.apply_friction();
    m.mover.update(app.window_rect());
}

fn view(app: &App, m: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    m.mover.display(&draw);
    draw.to_frame(app, &frame).unwrap();
}

fn create_wind(time: DrawScalar, xy: Point2) -> Vector2 {
    const DIRECTION_MAGNITUDE_OFFSET: f32 = 5000.0;
    let noise = Perlin::new();
    let direction = if noise.get([xy.x as f64, xy.y as f64, time as f64]) < 0.5 {
        -1.0
    } else {
        1.0
    } as f32;
    let magnitude = noise.get([
        (xy.x + DIRECTION_MAGNITUDE_OFFSET) as f64,
        (xy.y + DIRECTION_MAGNITUDE_OFFSET) as f64,
        time as f64,
    ]);
    vec2(direction * magnitude as f32, 0.0)
}
