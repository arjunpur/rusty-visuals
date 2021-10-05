use nannou::geom::path;
use nannou::prelude::*;
use rand::Rng;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(600, 600)
        .run();
}

struct Model {
    positions: Vec<Point2>,
}

fn model(app: &App) -> Model {
    let positions = vec![app.window_rect().xy()];
    Model { positions }
}

fn update(app: &App, m: &mut Model, _update: Update) {
    let rect = app.window_rect();

    // Generate a random velocity and ensure movement won't
    // go off screen
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(-1.0, 1.0);
    let y = rng.gen_range(-1.0, 1.0);
    let last = m.positions.last().unwrap();
    let mut velocity = vec2(x, y) * 8.0;
    if (last.x > rect.right()) || (last.x < rect.left()) {
        velocity.x *= -1.0;
    }
    if (last.y > rect.top()) || (last.y < rect.bottom()) {
        velocity.y *= -1.0;
    }

    m.positions.push(*last + velocity);
}

fn view(app: &App, m: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();
    draw.background().color(SKYBLUE);

    let mut builder = path::Builder::new();
    builder = builder.move_to(m.positions[0]);

    for i in 1..m.positions.len() {
        builder = builder.line_to(m.positions[i]);
    }

    let path = builder.build();

    // Set the start of the path to the original point
    draw.polyline()
        .weight(2.0)
        .color(SEAGREEN)
        .events(path.iter());

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
