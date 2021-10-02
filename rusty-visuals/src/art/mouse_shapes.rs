/**
 * drawing with a changing shape by dragging the mouse.
 *
 * MOUSE
 * position x          : length
 * position y          : thickness and number of lines
 * drag                : draw
 *
 * KEYS
 * spacebar            : erase
 * s                   : save png
 */
use nannou::prelude::*;
// use std::collections::vec_deque::VecDeque;
use rand::seq::SliceRandom;

use rusty_visuals::*;

fn main() {
    nannou::app(model).run();
}

struct Model {
    clicked: bool,
    clear_background: bool,
    // Use a deque to cycle between colors for each mouse
    // press
    color_scheme: Vec<Hsv>,

    current_color: Hsv,
    // Set when the mouse is pressed. This is used
    // to calculate the radius of the circle
    circle_radius_start: Point2,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(720, 720)
        .view(view)
        .mouse_pressed(mouse_pressed)
        .mouse_released(mouse_released)
        .key_pressed(key_pressed)
        .key_released(key_released)
        .build()
        .unwrap();
    // let base_green_color = hsv(82.0 / 360.0, 0.2607, 0.4196);
    // let base_cream_color = hsv(82.0 / 360.0, 0.2607, 0.4196);
    // let mut color_deque: Vec<Hsv> = vec![];

    // for i in 0..4 {
    //     color_deque.push(base_color + (hsv(0.0, 0.2 * i as f32, 0.0)));
    // }

    // https://colorpalettes.net/color-palette-2246/
    // let color_scheme = vec![
    //     hsv(63.0 / 360.0, 0.330, 0.660),
    //     hsv(48.0 / 360.0, 0.420, 0.93),
    //     hsv(39.0 / 360.0, 0.170, 0.780),
    //     hsv(19.0 / 360.0, 0.320, 0.520),
    //     hsv(17.0 / 360.0, 0.20, 0.27),
    // ];

    // https://colorpalettes.net/color-palette-4291/
    let color_scheme = vec![
        hsv(39.0 / 360.0, 0.760, 0.980),
        hsv(38.0 / 360.0, 0.350, 0.98),
        hsv(116.0 / 360.0, 0.070, 0.780),
        hsv(96.0 / 360.0, 0.310, 0.40),
        hsv(84.0 / 360.0, 0.39, 0.20),
    ];

    let circle_radius_start = pt2(0.0, 0.0);

    let current_color = color_scheme.choose(&mut rand::thread_rng()).unwrap();

    Model {
        clicked: false,
        clear_background: false,
        current_color: *current_color,
        color_scheme,
        circle_radius_start,
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();
    let win = app.window_rect();
    let circle_resolution = map_range(app.mouse.y, win.top(), win.bottom(), 3, 12);
    let angle = TAU / circle_resolution as f32;

    if app.elapsed_frames() == 1 || model.clear_background {
        draw.background().color(WHITE);
    }

    if model.clicked {
        let difference: Vector2<_> = model.circle_radius_start - app.mouse.position();
        let radius = difference.dot(difference).sqrt();
        let mut points = Vec::new();
        for i in 0..circle_resolution {
            let x = (angle * i as f32).cos() * radius;
            let y = (angle * i as f32).sin() * radius;
            points.push(pt2(x, y) + model.circle_radius_start);
        }

        draw.polygon()
            .stroke(rgba(0.0, 0.0, 0.0, 0.1))
            .color(model.current_color)
            .stroke_weight(0.0)
            .points(points);
    }
    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}

fn mouse_pressed(_app: &App, model: &mut Model, _button: MouseButton) {
    model.clicked = true;
    model.circle_radius_start = _app.mouse.position();
    model.current_color = *model.color_scheme.choose(&mut rand::thread_rng()).unwrap();
}
fn mouse_released(_app: &App, model: &mut Model, _button: MouseButton) {
    model.clicked = false;
    // let front = model.color_deque.pop_front().unwrap();
    // model.color_deque.push_back(front);
}
fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Space => {
            model.clear_background = true;
        }
        Key::S => {
            file_utils::capture_frame_to_path(app);
        }
        _other_key => {}
    }
}
fn key_released(_app: &App, model: &mut Model, key: Key) {
    if key == Key::Space {
        model.clear_background = false;
    }
}
