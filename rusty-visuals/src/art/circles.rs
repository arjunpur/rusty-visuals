use nannou::color::{self, Hsl};
use nannou::prelude::*;
use rusty_visuals::*;

const CIRCLE_SIZE: app::DrawScalar = 10.0;
const NUM_CIRCLES: usize = 100;

// 50% saturation and lightness
const SATURATION: color::DefaultScalar = 0.5;
const SATURATION_WINDOW: color::DefaultScalar = 0.3;

// This is green
const BACKGROUND_HUE: color::DefaultScalar = 120.0;
const BACKGROUND_SATURATION: color::DefaultScalar = 0.4;
const BACKGROUND_LIGHTNESS: color::DefaultScalar = 1.0;

// This is yellow. We will randomize the hue within
// CIRCLE_HUE_CENTER +/- CIRCLE_HUE_WINDOW
const CIRCLE_HUE_CENTER: color::DefaultScalar = 240.0;
const CIRCLE_HUE_WINDOW: color::DefaultScalar = 20.0;

fn main() {
    nannou::app(model).run();
}

struct Model {}

fn model(app: &App) -> Model {
    app.new_window()
        .size(700, 700)
        .event(event)
        .view(view)
        .build()
        .unwrap();
    Model {}
}

fn event(app: &App, _: &mut Model, event: WindowEvent) {
    match event {
        KeyPressed(Key::P) => {
            println!("printing out because P was pressed");
            file_utils::capture_frame_to_path(app);
        }
        _other => (),
    }
}

fn view(app: &App, _: &Model, frame: Frame) {
    let draw = app.draw();
    let rect = app.window_rect();

    draw.background().color(Hsl::new(
        BACKGROUND_HUE,
        BACKGROUND_SATURATION,
        BACKGROUND_LIGHTNESS,
    ));

    for _ in 0..NUM_CIRCLES {
        // Draw a bunch of circles with some color
        let position = pt2(
            random_range(rect.left(), rect.right()),
            random_range(rect.bottom(), rect.h()),
        );
        let hue = colors::random_color_in_range(
            CIRCLE_HUE_CENTER - CIRCLE_HUE_WINDOW,
            CIRCLE_HUE_CENTER + CIRCLE_HUE_WINDOW,
            SATURATION - SATURATION_WINDOW,
            SATURATION + SATURATION_WINDOW,
        );
        draw.ellipse().xy(position).radius(CIRCLE_SIZE).color(hue);
    }

    draw.to_frame(app, &frame).unwrap();
}
