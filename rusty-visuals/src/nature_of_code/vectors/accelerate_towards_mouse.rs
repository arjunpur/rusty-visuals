// Exercise 1: Create a simulation of a car (or runner) that accelerates when you press the up key and brakes when you press the down key.
// Exercise 2: Accelerate an object towards the mouse

use nannou::prelude::*;

use rusty_visuals::mover::Mover;

const SCALE_ACCELERATION_BY: app::DrawScalar = 0.1;
const NUM_MOVERS: usize = 10;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(600, 600)
        .run();
}

struct Model {
    movers: Vec<Mover>,
}

fn model(app: &App) -> Model {
    let mut movers: Vec<Mover> = Vec::with_capacity(NUM_MOVERS);
    for _ in 0..NUM_MOVERS {
        movers.push(Mover::new(app.window_rect()));
    }
    Model { movers }
}

fn update(app: &App, m: &mut Model, _update: Update) {
    for i in 0..NUM_MOVERS {
        // Since m.movers is an owned vector of owned types, we need to
        // get a mutable reference. The let operator will try to acquire
        // ownership or copy otherwise.
        let mover = &mut m.movers[i];
        let normalized_vec_to_mouse = (app.mouse.position() - mover.position).normalize();
        mover.update(
            app.window_rect(),
            normalized_vec_to_mouse * SCALE_ACCELERATION_BY,
        );
    }
}

fn view(app: &App, m: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    for i in 0..NUM_MOVERS {
        let mover = &m.movers[i];
        mover.display(&draw);
    }
    draw.to_frame(app, &frame).unwrap();
}
