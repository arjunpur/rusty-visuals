// Exercise 1: Create a simulation of a car (or runner) that accelerates when you press the up key and brakes when you press the down key.
// Exercise 2: Accelerate an object towards the mouse


use nannou::prelude::*;

use rusty_visuals::mover::Mover;

const ACCELERATION_INCREMENT: app::DrawScalar = 0.01;

// TODO: How do I expose this import?
pub fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    curr_acceleration: Vector2,
    mover: Mover,
}

fn model(app: &App) -> Model {
    let rect = Rect::from_w_h(640.0, 360.0);
    app.new_window()
        .size(rect.w() as u32, rect.h() as u32)
        .event(event)
        .view(view)
        .build()
        .unwrap();
    let mover = Mover::new(app.window_rect());
    let curr_acceleration = vec2(0.0, 0.0);
    Model {
        curr_acceleration,
        mover,
    }
}

fn update(app: &App, m: &mut Model, _update: Update) {
    m.mover.apply_force(m.curr_acceleration);
    m.mover.update(app.window_rect());
}

fn event(_: &App, m: &mut Model, event: WindowEvent) {
    match event {
        KeyPressed(Key::Up) => {
            m.curr_acceleration += vec2(ACCELERATION_INCREMENT, 0.0);
        }
        KeyPressed(Key::Down) => {
            m.curr_acceleration -= vec2(ACCELERATION_INCREMENT, 0.0);
        }
        KeyPressed(Key::P) => println!(
            "current acceleration: ({}, {})",
            m.curr_acceleration.x, m.curr_acceleration.y
        ),
        _other => (),
    }
}

fn view(app: &App, m: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    m.mover.display(&draw);
    draw.to_frame(app, &frame).unwrap();
}
