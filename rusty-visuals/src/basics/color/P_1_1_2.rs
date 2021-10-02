use nannou::prelude::*;
use rusty_visuals::*;

fn main() {
    nannou::app(model).run();
}

struct Model {
    num_segments: u32,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(1200, 1200)
        .event(event)
        .view(view)
        .build()
        .unwrap();
    Model { num_segments: 360 }
}

fn view(app: &App, m: &Model, frame: Frame) {
    let draw = app.draw();
    let rect = app.window_rect();

    draw.background().color(WHITE);

    // Take the minimum of the width and height to figure out the right radius
    let radius = rect.w().min(rect.h()) / 2.0;
    let center = rect.xy();
    let angle_delta = 360.0 / m.num_segments as f32;
    let mut starting_angle = angle_delta;
    let mut starting_pos = pt2(center.x + radius, center.y);

    // We need to add the width and height offsite to the value being mapped because
    // the mouse coordinates are between -width/2 -> +width/2
    let saturation = map_range(app.mouse.x + rect.w(), 0.0, rect.w(), 0.0, 1.0);
    let brightness = map_range(app.mouse.y + rect.h(), 0.0, rect.h(), 0.0, 1.0);

    while starting_angle <= 360.0 {
        let x_coord = center.x + (starting_angle.to_radians().cos() * radius);
        let y_coord = center.y + (starting_angle.to_radians().sin() * radius);
        let next_point = pt2(x_coord, y_coord);
        let color = Hsv::new(starting_angle, saturation, brightness);
        draw.tri()
            .stroke_weight(0.0)
            .color(color)
            .points(center, starting_pos, next_point);
        starting_angle += angle_delta;
        starting_pos = next_point;
    }

    draw.to_frame(app, &frame).unwrap();
}

fn event(app: &App, m: &mut Model, event: WindowEvent) {
    match event {
        KeyPressed(Key::P) => {
            println!("printing out because P was pressed");
            file_utils::capture_frame_to_path(app);
        }
        KeyPressed(Key::Up) => {
            m.num_segments += 5;
        }
        KeyPressed(Key::Down) => {
            m.num_segments -= 5;
        }
        _other => (),
    }
}
