use nannou::prelude::*;
use rusty_visuals::Mover;

mod force_field;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    mover: Mover,
    force_field: force_field::ForceField,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(1000, 1000)
        .view(view)
        .build()
        .unwrap();
    let force_field = force_field::ForceField::new(app.window_rect(), app.time);
    let mover = Mover::new(app.window_rect());
    Model { mover, force_field }
}

fn update(app: &App, m: &mut Model, _update: Update) {
    let window_rect = app.window_rect();
    m.force_field.update(window_rect, app.time);
    let acceleration = m
        .force_field
        .get_acceleration_from_position(m.mover.position);

    m.mover.update(window_rect, acceleration);
}

fn view(app: &App, m: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();
    draw.background().color(WHITE);
    m.mover.display(&draw);
    m.force_field.display(&draw, app.time);
    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
