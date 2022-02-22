use nannou::prelude::*;
use rusty_visuals::{force_field::ForceField, mover::Mover};


fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(600, 600)
        .run();
}

struct Model {
    mover: Mover,
    force_field: ForceField,
    gravity: Vec2,
}

fn model(app: &App) -> Model {
    let buoyancy: Vec2 = vec2(0.0, 0.3);
    let gravity: Vec2 = vec2(0.0, -0.1);
    let mover: Mover = Mover::new_with_inherent_force(app.window_rect(), buoyancy);
    let force_field: ForceField = ForceField::new(app.window_rect(), app.time);
    Model { mover, force_field, gravity }
}

fn update(app: &App, m: &mut Model, _update: Update) {
    m.mover.apply_force(m.gravity);
    let wind = m
        .force_field
        .get_acceleration_from_position(m.mover.position);
    m.mover.apply_force(wind);
    m.mover.update(app.window_rect());
}

fn view(app: &App, m: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    m.mover.display(&draw);
    draw.to_frame(app, &frame).unwrap();
}
