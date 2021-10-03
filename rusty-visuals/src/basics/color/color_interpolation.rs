use nannou::color::*;
use nannou::draw::primitive::polygon::*;
use nannou::prelude::*;
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

    let num_boxes_in_width = 200.0;
    let mut colorer =
        grid::InterpolatedColorer::new((Hsv::new(00.0, 1.0, 1.0), Hsv::new(60.0, 1.0, 1.0)));
    let grid = grid::ColoredGrid::draw(&draw, &rect, rect.w() / num_boxes_in_width, &mut colorer);

    draw.background().color(WHITE);
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