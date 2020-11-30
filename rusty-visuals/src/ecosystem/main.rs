use nannou::noise::*;
use nannou::prelude::*;

mod mover;

const RESOLUTION: f32 = 10.0;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    mover: mover::Mover,
    force_field: Vec<Vec<geom::Vector2>>,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(1000, 1000)
        .view(view)
        .build()
        .unwrap();
    let rect = app.window_rect();
    // Calculate the dimensions of the force field
    let num_forces = (
        (rect.w() / RESOLUTION).ceil() as usize,
        (rect.h() / RESOLUTION).ceil() as usize,
    );
    let mut force_field = vec![vec![vec2(0.0, 0.0); num_forces.0]; num_forces.1];
    let bottom_left = rect.bottom_left();
    for i in 0..force_field.len() {
        for j in 0..force_field[i].len() {
            force_field[i][j] = create_force_from_noise(
                app,
                bottom_left.x + (j as f32 * RESOLUTION),
                bottom_left.y + (i as f32 * RESOLUTION),
            );
        }
    }
    let mover = mover::Mover::new(app.window_rect());
    Model { mover, force_field }
}

fn create_force_from_noise(app: &App, x: f32, y: f32) -> geom::Vector2 {
    let noise = Perlin::new();
    let angle = noise.get([
        (x / 500.0) as f64,
        (y / 500.0) as f64,
        app.time as f64 / 3.0,
    ]) as f32
        * TAU;
    let magnitude = noise.get([
        (x / 100.0) as f64 + 40000.0,
        (y / 100.0) as f64 + 40000.0,
        app.time as f64 / 3.0,
    ]) as f32;
    return geom::Vector2::from_angle(angle) * magnitude * 40.0;
}

fn update(app: &App, m: &mut Model, _update: Update) {
    let window_rect = app.window_rect();
    let bottom_left = window_rect.bottom_left();
    for i in 0..m.force_field.len() {
        for j in 0..m.force_field[i].len() {
            m.force_field[i][j] = create_force_from_noise(
                app,
                bottom_left.x + (j as f32 * RESOLUTION),
                bottom_left.y + (i as f32 * RESOLUTION),
            );
        }
    }
    m.mover.update(app.window_rect());
}

fn view(app: &App, m: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();
    draw.background().color(WHITE);
    let window_rect = app.window_rect();
    let bottom_left = window_rect.bottom_left();
    // m.mover.display(&draw);
    //
    let noise = SuperSimplex::new();
    for i in 0..m.force_field.len() {
        for j in 0..m.force_field[i].len() {
            let start = bottom_left + pt2(j as f32 * RESOLUTION, i as f32 * RESOLUTION);
            let end = start + m.force_field[i][j];
            // draw.line().points(start, end).color(RED);
            // println!("Point: {} {}", start.x as f64, start.y as f64);
            let hue = noise.get([
                (start.x as f64) / 10000.0,
                (start.y as f64) / 10000.0,
                app.time as f64 / 3.0,
            ]);
            // println!("Hue: {}", hue);
            draw.line()
                .points(start, end)
                .hsl(hue as f32, 0.7, 0.5)
                .weight(2.0);
        }
    }

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
