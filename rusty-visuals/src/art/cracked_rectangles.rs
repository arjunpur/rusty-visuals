use nannou::draw::primitive::polygon::*;
use nannou::prelude::*;
use rand::{thread_rng, Rng};
use rusty_visuals::*;

// Adjust this
const JITTER_FACTOR: f32 = 2.0;

fn main() {
    nannou::app(model).run();
}

struct Model {
    tile_count: u32,
    rect_size: f32,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(1200, 1200)
        .event(event)
        .view(view)
        .build()
        .unwrap();
    Model {
        tile_count: 20,
        rect_size: 60.0,
    }
}

fn view(app: &App, m: &Model, frame: Frame) {
    let draw = app.draw();
    let rect = app.window_rect();

    if app.elapsed_frames() != 1 {
        return;
    }
    draw.background().color(WHITE);
    // ~ Grid generation ~
    //
    // Points in a rectangle:
    // (x4, y4)   (x3, y3)
    //
    // (x1, y1)   (x2, y2)
    //

    for grid_y in 0..m.tile_count {
        for grid_x in 0..m.tile_count {
            let tile_w = rect.w() / m.tile_count as f32;
            let tile_h = rect.h() / m.tile_count as f32;
            let x1 = rect.left() + tile_w * grid_x as f32;
            let y1 = (rect.top() - tile_h) - tile_h * grid_y as f32;
            let x2 = x1 + m.rect_size;
            let y2 = y1;
            let x3 = x1 + m.rect_size;
            let y3 = y1 + m.rect_size;
            let x4 = x1;
            let y4 = y1 + m.rect_size;
            let mut points: Vec<Point2> = Vec::new();

            points.push(pt2(x1, y1));
            points.push(pt2(x2, y2));
            points.push(pt2(x3, y3));
            points.push(pt2(x4, y4));

            let jittered_points = points
                .iter()
                .map(|pt| pt2(pt.x + jitter(tile_w), pt.y + jitter(tile_h)));

            enhance_with_color(grid_x, grid_y, draw.polygon().points(jittered_points));
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

fn jitter(tile_size: f32) -> f32 {
    let mut rng = thread_rng();
    JITTER_FACTOR * tile_size * rng.gen_range(-1.0, 1.0)
}

fn enhance_with_color(grid_x: u32, grid_y: u32, drawing: DrawingPolygon) {
    if (grid_x + grid_y) % 3 == 0 {
        drawing.hsva(0.53, 1.0, 0.64, 0.7);
    } else if (grid_x + grid_y) % 3 == 1 {
        drawing.hsva(0.275, 1.0, 0.64, 0.7);
    } else {
        drawing.hsva(0.138, 0.48, 0.99, 1.0);
    }
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
