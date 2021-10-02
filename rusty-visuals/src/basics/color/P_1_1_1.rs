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

    draw.background().color(WHITE);

    // Map the mouse position to a linear scale of 0.0 -> dimension
    let step_x = map_range(app.mouse.x, rect.left(), rect.right(), 0.0, rect.w()).max(5.0);
    let step_y = map_range(app.mouse.y, rect.bottom(), rect.top(), 0.0, rect.h()).max(5.0);

    // We align a rectangle to the top left of the window and use this rectangle
    // as a guide to construct the other rectangles. We are shifting this rectangle
    // by a vector through the nested loop.
    let r = Rect::from_wh(vec2(step_x, step_y))
        .align_left_of(rect)
        .align_top_of(rect);

    // This code here creates a grid of rectangles using the step sizes defined
    // above as the rectangle size.
    let mut y = 0.0;
    while y <= rect.h() {
        let mut x = 0.0;
        while x <= rect.w() {
            // r is only scoped to this block. After this loop exits,
            // we return to the r set outside the nested loop
            //
            // We shift the rectangle by a vector defined by how far
            // right we've gone and how far down we've gone.
            let r = r.shift(vec2(x, -y));
            let hue = map_range(x, 0.0, rect.w(), 0.0, 360.0);
            let saturation = map_range(y, 0.0, rect.h(), 0.0, 1.0);
            let color = Hsv::new(hue, 1.0 - saturation, 1.0);
            draw.rect().xy(r.xy()).wh(r.wh()).color(color);
            x += step_x;
        }
        y += step_y;
    }

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
