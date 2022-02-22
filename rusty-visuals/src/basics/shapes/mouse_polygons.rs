use nannou::color::*;
use nannou::prelude::*;
use rusty_visuals::colorer::PaletteColorer;
use rusty_visuals::*;

fn main() {
    nannou::app(model).update(update).run()
}

struct Model {
    circle_resolution: i32,
    radius: f32,
    clear_frame: u64,
    colorer: PaletteColorer,
    draw: bool,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(1500, 1500)
        .event(event)
        .view(view)
        .build()
        .unwrap();

    let rect = app.window_rect();

    let pastels = PaletteColorer::new(
        (0..360).map(|n| n as f32 / 360.0).collect(),
        vec![0.6],
        vec![1.0],
    );

    Model {
        circle_resolution: 5,
        radius: rect.w() / 2.0,
        clear_frame: 1,
        colorer: pastels,
        draw: false,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let rect = app.window_rect();
    let circle_resolution = map_range(app.mouse.y, rect.bottom(), rect.top(), 3, 10);
    let radius = map_range(app.mouse.x, rect.left(), rect.right(), 0.5, rect.w() / 2.0);
    model.circle_resolution = circle_resolution;
    model.radius = radius;
}

fn view(app: &App, m: &Model, frame: Frame) {
    let draw = app.draw();

    // Only draw the background on the first frame
    if m.clear_frame == app.elapsed_frames() {
        draw.background().color(BLACK);
    }

    if m.draw {
        draw_polygon(&draw, m);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn draw_polygon(draw: &Draw, m: &Model) {
    let mut points = vec![];
    let angle = TAU / m.circle_resolution as f32;

    for i in 0..m.circle_resolution {
        let x = m.radius * (i as f32 * angle).cos();
        let y = m.radius * (i as f32 * angle).sin();
        points.push(pt2(x, y));
    }

    let color = m.colorer.color();
    let transperant = hsva(
        color.hue.to_positive_degrees() / 360.0,
        color.saturation,
        color.value,
        0.6,
    );
    draw.polygon()
        .stroke(transperant)
        .stroke_weight(0.7)
        .points(points)
        .color(hsva(0.0, 0.0, 0.0, 0.0));
}

fn event(app: &App, m: &mut Model, event: WindowEvent) {
    match event {
        KeyPressed(Key::P) => {
            println!("printing out because P was pressed");
            file_utils::capture_frame_to_path(app);
        }
        KeyPressed(Key::C) => {
            m.clear_frame = app.elapsed_frames() + 1;
        }
        MousePressed(MouseButton::Left) => {
            m.draw = true;
        }
        MouseReleased(MouseButton::Left) => {
            m.draw = false;
        }
        _other => (),
    }
}
