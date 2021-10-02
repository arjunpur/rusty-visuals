use nannou::color::*;
use nannou::draw::primitive::polygon::*;
use nannou::prelude::*;
use rand::{thread_rng, Rng};
use rusty_visuals::*;

fn main() {
    nannou::app(model).run();
}

struct Model {}

fn model(app: &App) -> Model {
    app.new_window()
        .size(1200, 1200)
        .event(event)
        .view(view)
        .build()
        .unwrap();
    Model {}
}

fn view(app: &App, m: &Model, frame: Frame) {
    let draw = app.draw();
    let rect = app.window_rect();

    let hue = map_range(app.mouse.y, 0.0, rect.h(), 0.0, 360.0);
    let background_color = Hsv::new(hue, 0.7, 1.0);
    draw.background().color(background_color);

    let rect_size = abs(app.mouse.x) * 2.0;
    draw.rect()
        .wh(pt2(rect_size, rect_size))
        .color(Hsv::new(360.0 - hue, 0.7, 1.0));
    draw.to_frame(app, &frame).unwrap();
}

fn event(app: &App, _: &mut Model, event: WindowEvent) {
    match event {
        KeyPressed(Key::P) => {
            println!("printing out because P was pressed");
            file_utils::capture_frame_to_path(app);
        }
        KeyPressed(Key::D) => {
            println!("Mouse Position: {}, {}", app.mouse.y, app.mouse.x);
        }
        _other => (),
    }
}
