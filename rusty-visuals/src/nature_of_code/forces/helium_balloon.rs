use nannou::prelude::*;
use rusty_visuals::{force_field::ForceField, mover::Mover};

const BUOYANCY_FORCE: Vector2 = Vector2 { x: 0.0, y: 0.3 };
const GRAVITY: Vector2 = Vector2 { x: 0.0, y: -0.1 };

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
}

fn model(app: &App) -> Model {
    let mover: Mover = Mover::new_with_inherent_force(app.window_rect(), BUOYANCY_FORCE);
    let force_field: ForceField = ForceField::new(app.window_rect(), app.time);
    Model { mover, force_field }
}

fn update(app: &App, m: &mut Model, _update: Update) {
    m.mover.apply_force(GRAVITY);
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
