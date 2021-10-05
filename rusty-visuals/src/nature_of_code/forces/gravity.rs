
use nannou::prelude::*;
use rusty_visuals::mover::Mover;

const GRAVITATIONAL_CONSTANT: app::DrawScalar = 2.0;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(600, 600)
        .run();
}

struct Model {
    mover: Mover,
    attractor: Mover,
}

fn model(app: &App) -> Model {
    let mover: Mover = Mover::new(app.window_rect());
    let attractor: Mover = Mover::new(app.window_rect());
    Model { mover, attractor }
}

fn update(app: &App, m: &mut Model, _update: Update) {
    let distance = m.attractor.position - m.mover.position;
    let vector = distance.normalize();
    let mag = distance.magnitude();
    let mag_clamped = clamp_min(mag, 0.1);
    let gravity_mag =
        (GRAVITATIONAL_CONSTANT * m.mover.mass * m.attractor.mass) / (mag_clamped * mag_clamped);
    let gravity = vector * gravity_mag;
    m.mover.apply_force(gravity);
    m.mover.update(app.window_rect());
}

fn view(app: &App, m: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    m.mover.display(&draw);
    m.attractor.display(&draw);
    draw.to_frame(app, &frame).unwrap();
}
