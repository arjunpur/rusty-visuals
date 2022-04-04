use chrono::Local;
use geo::Line;
use line_intersection::LineInterval;
use nannou::prelude::*;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use rusty_visuals::grid::*;
use rusty_visuals::hi_res_capture::HiResCapture;

fn main() {
    // The `exit` handler makes sure the PNG is captured before exiting
    // the program
    nannou::app(model).update(update).exit(exit).run();
}

const GRID_COL_RESOLUTION: usize = 15;
const GRID_ROW_RESOLUTION: usize = 15;

struct Model {
    grid: Grid,
    hi_res_capture: HiResCapture,
    // save_image tells the `update` handler to write the
    // texture to PNG
    save_image: bool,
    seed: u64,
    drawing_num: u8,
    debug: bool,
}

fn model(app: &App) -> Model {
    // Create the window.
    let [width, height] = [2500, 2500];
    let [win_w, win_h] = [width / 2, height / 2];
    let w_id = app
        .new_window()
        .size(win_w, win_h)
        .event(event)
        .view(view)
        .build()
        .unwrap();

    let hi_res_capture = HiResCapture::new(app, [width, height], w_id);
    let r = geom::Rect::from_w_h((width - 200) as f32, (height - 200) as f32);
    let grid = Grid::new(
        r,
        &CellIndex {
            row: GRID_ROW_RESOLUTION,
            col: GRID_COL_RESOLUTION,
        },
    );

    Model {
        grid,
        hi_res_capture,
        save_image: false,
        seed: Local::now().timestamp() as u64,
        drawing_num: 4,
        debug: false,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    // First, reset the `draw` state.
    match model.drawing_num {
        0 => drawing_0(app, model),
        1 => drawing_1(app, model),
        2 => drawing_2(app, model),
        3 => drawing_3(app, model),
        4 => drawing_4(app, model),
        _ => println!("no drawing!"),
    }

    // DRAWING #2:

    if model.save_image {
        model.save_image = false;
    }
}

/// DRAWING #0: Grid with solid line along either of the two diagonals, chosen
/// randomly
///
/// For each cell, randomly choose a direction along which to draw a line.
/// Either:
/// 1) Top left to the bottom right
/// 2) Bottom left to top right
fn drawing_0(app: &App, model: &mut Model) {
    let draw = &model.hi_res_capture.draw;
    draw.background().color(WHITE);

    // https://rust-random.github.io/book/guide-start.html
    let mut rng = StdRng::seed_from_u64(model.seed);
    for cell in model.grid.row_major_iter() {
        let line_direction = rng.gen_range(0.0, 2.0).floor() as i32;
        if line_direction == 0 {
            draw.line()
                .caps_round()
                .start(pt2(cell.left(), cell.top()))
                .end(pt2(cell.right(), cell.bottom()))
                .weight(40.0);
        } else {
            draw.line()
                .caps_round()
                .start(pt2(cell.left(), cell.bottom()))
                .end(pt2(cell.right(), cell.top()))
                .weight(40.0);
        }
    }

    model.hi_res_capture.update(app, model.save_image);
}

/// DRAWING #1: Same as #0 except with a shadow
fn drawing_1(app: &App, model: &mut Model) {
    let draw = &model.hi_res_capture.draw;
    draw.background().color(WHITE);

    // https://rust-random.github.io/book/guide-start.html
    let mut rng = StdRng::seed_from_u64(model.seed);
    for cell in model.grid.row_major_iter() {
        let line_direction = rng.gen_range(0.0, 2.0).floor() as i32;
        let front_color = Hsv::new(0.0, 0.0, 0.0);
        let back_color = Hsva::new(0.0, 0.0, 0.1, 0.7);
        if line_direction == 0 {
            draw.line()
                .caps_round()
                .start(pt2(cell.left(), cell.top()))
                .end(pt2(cell.right(), cell.bottom()))
                .weight(80.0)
                .join_miter()
                .color(back_color);
            draw.line()
                .start(pt2(cell.left(), cell.top()))
                .end(pt2(cell.right(), cell.bottom()))
                .weight(40.0)
                .color(front_color)
                .caps_round();
        } else {
            draw.line()
                .caps_round()
                .start(pt2(cell.left(), cell.bottom()))
                .end(pt2(cell.right(), cell.top()))
                .weight(80.0)
                .join_miter()
                .color(back_color);
            draw.line()
                .start(pt2(cell.left(), cell.bottom()))
                .end(pt2(cell.right(), cell.top()))
                .weight(40.0)
                .color(front_color)
                .caps_round();
        }
    }

    model.hi_res_capture.update(app, model.save_image);
}

/// DRAWING #2: Olive Circles are positioned on a square within the cell and are pulled
/// towards the mouse
fn drawing_2(app: &App, model: &mut Model) {
    let draw = &model.hi_res_capture.draw;
    // https://rust-random.github.io/book/guide-start.html
    // let mut rng = StdRng::seed_from_u64(model.seed);
    draw.background().color(WHITE);
    let mouse_pos = app.mouse.position();
    for cell in model.grid.row_major_iter() {
        let distance_from_mouse = mouse_pos.distance(cell.xy);
        // Fade the color of the circles by the radius of a bounding circle around the grid'sure
        // rectangle
        let alpha_front = map_range(
            distance_from_mouse,
            0.0,
            model.grid.diagonal_length() / 2.0,
            0.0,
            0.9,
        );
        let alpha_back = map_range(
            distance_from_mouse,
            0.0,
            model.grid.diagonal_length() / 2.0,
            0.0,
            0.4,
        );
        let color_front = Hsva::new(58.0, 0.88, 0.64, alpha_front);
        let color_back = Hsva::new(58.0, 0.88, 0.64, alpha_back);

        draw.ellipse().xy(cell.xy).wh(cell.wh).color(color_back);
        let circle_size = cell.wh * 0.9;
        let rect = Rect::from_xy_wh(cell.xy, circle_size * 0.7);
        let intersection_point = find_intersection_point(mouse_pos, &rect);
        draw.ellipse()
            .xy(intersection_point)
            .wh(circle_size)
            .color(color_front);

        // Debug shapes
        if model.debug {
            draw.rect()
                .xy(cell.xy)
                .wh(cell.wh)
                .stroke_weight(1.0)
                .color(Rgba::new(0.0, 0.0, 0.0, 0.0));
            draw.rect()
                .xy(cell.xy)
                .wh(circle_size)
                .stroke_weight(1.0)
                .stroke_color(RED)
                .color(Rgba::new(0.0, 0.0, 0.0, 0.0));
            draw.ellipse()
                .xy(intersection_point)
                .wh(vec2(8.0, 8.0))
                .color(BLACK);
            draw.line()
                .points(vec2(app.mouse.x, app.mouse.y), cell.xy)
                .stroke_weight(1.0);
        }
    }
    model.hi_res_capture.update(app, model.save_image);
}

/// DRAWING #3: Olive Circles are positioned on a circle within the cell and are pulled
/// towards the mouse
fn drawing_3(app: &App, model: &mut Model) {
    let draw = &model.hi_res_capture.draw;
    // https://rust-random.github.io/book/guide-start.html
    // let mut rng = StdRng::seed_from_u64(model.seed);
    draw.background().color(WHITE);
    let mouse_pos = app.mouse.position();
    for cell in model.grid.row_major_iter() {
        let distance_from_mouse = mouse_pos.distance(cell.xy);
        // Fade the color of the circles by the radius of a bounding circle around the grid'sure
        // rectangle
        let alpha_front = map_range(
            distance_from_mouse,
            0.0,
            model.grid.diagonal_length() / 2.0,
            0.0,
            0.9,
        );
        let alpha_back = map_range(
            distance_from_mouse,
            0.0,
            model.grid.diagonal_length() / 2.0,
            0.0,
            0.4,
        );
        let color_front = Hsva::new(58.0, 0.88, 0.64, alpha_front);
        let color_back = Hsva::new(58.0, 0.88, 0.64, alpha_back);

        draw.ellipse().xy(cell.xy).wh(cell.wh).color(color_back);
        let circle_size = cell.wh * 0.9;

        let position_circle_size = circle_size * 0.7;
        let mouse_unit_vector = (mouse_pos - cell.xy).normalize();
        let pos_on_circle = cell.xy + (mouse_unit_vector * (position_circle_size / 2.0));

        draw.ellipse()
            .xy(pos_on_circle)
            .wh(circle_size)
            .color(color_front);

        // Debug shapes
        if model.debug {
            draw.ellipse()
                .xy(cell.xy)
                .wh(position_circle_size)
                .stroke_weight(1.0)
                .stroke_color(RED)
                .color(Rgba::new(0.0, 0.0, 0.0, 0.0));
            draw.rect()
                .xy(cell.xy)
                .wh(cell.wh)
                .stroke_weight(1.0)
                .color(Rgba::new(0.0, 0.0, 0.0, 0.0));
            draw.ellipse()
                .xy(pos_on_circle)
                .wh(vec2(8.0, 8.0))
                .color(BLACK);
            draw.line()
                .points(vec2(app.mouse.x, app.mouse.y), cell.xy)
                .stroke_weight(1.0);
        }
    }
    model.hi_res_capture.update(app, model.save_image);
}

/// DRAWING #4: Circles are positioned on a circle within the cell and are pulled
/// towards the mouse. Hue and alpha change away from the mouse.
fn drawing_4(app: &App, model: &mut Model) {
    let draw = &model.hi_res_capture.draw;
    // https://rust-random.github.io/book/guide-start.html
    // let mut rng = StdRng::seed_from_u64(model.seed);
    draw.background().color(WHITE);
    let mouse_pos = app.mouse.position();
    for cell in model.grid.row_major_iter() {
        let distance_from_mouse = mouse_pos.distance(cell.xy);
        // Fade the color of the circles by the radius of a bounding circle around the grid'sure
        // rectangle
        let alpha_front = map_range(
            distance_from_mouse,
            0.0,
            model.grid.diagonal_length() / 2.0,
            0.0,
            0.9,
        );
        let alpha_back = map_range(
            distance_from_mouse,
            0.0,
            model.grid.diagonal_length() / 2.0,
            0.0,
            0.4,
        );
        let hue_front = map_range(
            distance_from_mouse,
            0.0,
            model.grid.diagonal_length() / 2.0,
            0.0,
            360.0,
        );
        let hue_back = map_range(
            distance_from_mouse,
            0.0,
            model.grid.diagonal_length() / 2.0,
            0.0,
            360.0,
        );
        let color_front = Hsva::new(hue_front, 0.88, 0.64, alpha_front);
        let color_back = Hsva::new(hue_back, 0.88, 0.64, alpha_back);

        draw.ellipse().xy(cell.xy).wh(cell.wh).color(color_back);
        let circle_size = cell.wh * 0.9;

        let position_circle_size = circle_size * 0.7;
        let mouse_unit_vector = (mouse_pos - cell.xy).normalize();
        let pos_on_circle = cell.xy + (mouse_unit_vector * (position_circle_size / 2.0));

        draw.ellipse()
            .xy(pos_on_circle)
            .wh(circle_size)
            .color(color_front);

        // Debug shapes
        if model.debug {
            draw.ellipse()
                .xy(cell.xy)
                .wh(position_circle_size)
                .stroke_weight(1.0)
                .stroke_color(RED)
                .color(Rgba::new(0.0, 0.0, 0.0, 0.0));
            draw.ellipse()
                .xy(mouse_pos)
                .wh(vec2(10.0, 10.0))
                .stroke_weight(1.0)
                .stroke_color(BLUE)
                .color(Rgba::new(0.0, 0.0, 0.0, 0.0));
            draw.rect()
                .xy(cell.xy)
                .wh(cell.wh)
                .stroke_weight(1.0)
                .color(Rgba::new(0.0, 0.0, 0.0, 0.0));
            draw.ellipse()
                .xy(pos_on_circle)
                .wh(vec2(8.0, 8.0))
                .color(BLACK);
            draw.line()
                .points(vec2(app.mouse.x, app.mouse.y), cell.xy)
                .stroke_weight(1.0);
        }
    }
    model.hi_res_capture.update(app, model.save_image);
}

/// find_intersection_point will find the point of intersection between any
/// of the cell's bounding box line segments, and the line segment between the
/// mouse position and the cell position.
fn find_intersection_point(mouse_pos: Vec2, rect: &Rect) -> Vec2 {
    let top_segment = LineInterval::line_segment(Line {
        start: (rect.top_left().x, rect.top_left().y).into(),
        end: (rect.top_right().x, rect.top_right().y).into(),
    });

    let bottom_segment = LineInterval::line_segment(Line {
        start: (rect.bottom_left().x, rect.bottom_left().y).into(),
        end: (rect.bottom_right().x, rect.bottom_right().y).into(),
    });

    let right_segment = LineInterval::line_segment(Line {
        start: (rect.top_right().x, rect.top_right().y).into(),
        end: (rect.bottom_right().x, rect.bottom_right().y).into(),
    });

    let left_segment = LineInterval::line_segment(Line {
        start: (rect.top_left().x, rect.top_left().y).into(),
        end: (rect.bottom_left().x, rect.bottom_left().y).into(),
    });

    let mouse_cell_segment = LineInterval::line_segment(Line {
        start: (rect.xy().x, rect.xy().y).into(),
        end: (mouse_pos.x, mouse_pos.y).into(),
    });

    let intersection = top_segment
        .relate(&mouse_cell_segment)
        .unique_intersection();
    if let Some(p) = intersection {
        return vec2(p.x(), p.y());
    }

    let intersection = right_segment
        .relate(&mouse_cell_segment)
        .unique_intersection();
    if let Some(p) = intersection {
        return vec2(p.x(), p.y());
    }
    let intersection = bottom_segment
        .relate(&mouse_cell_segment)
        .unique_intersection();
    if let Some(p) = intersection {
        return vec2(p.x(), p.y());
    }

    let intersection = left_segment
        .relate(&mouse_cell_segment)
        .unique_intersection();
    if let Some(p) = intersection {
        return vec2(p.x(), p.y());
    }

    rect.xy()
}

// Draw the state of your `Model` into the given `Frame` here.
fn view(_app: &App, model: &Model, frame: Frame) {
    // Sample the texture and write it to the frame.
    let mut encoder = frame.command_encoder();
    model
        .hi_res_capture
        .texture_reshaper
        .encode_render_pass(frame.texture_view(), &mut *encoder);
}

// Wait for capture to finish.
fn exit(app: &App, model: Model) {
    println!("Waiting for PNG writing to complete...");
    let window = app.main_window();
    let device = window.device();
    model
        .hi_res_capture
        .texture_capturer
        .await_active_snapshots(device)
        .unwrap();
    println!("Done!");
}

fn event(_app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        KeyPressed(Key::P) => {
            model.save_image = true;
        }
        KeyPressed(Key::R) => {
            model.seed = Local::now().timestamp() as u64;
        }
        KeyPressed(Key::D) => {
            if model.debug {
                model.debug = false;
                return;
            }
            model.debug = true;
        }
        KeyPressed(Key::Key0) => {
            model.drawing_num = 0;
        }
        KeyPressed(Key::Key1) => {
            model.drawing_num = 1;
        }
        KeyPressed(Key::Key2) => {
            model.drawing_num = 2;
        }
        KeyPressed(Key::Key3) => {
            model.drawing_num = 3;
        }
        KeyPressed(Key::Key4) => {
            model.drawing_num = 4;
        }
        _other => (),
    }
}
