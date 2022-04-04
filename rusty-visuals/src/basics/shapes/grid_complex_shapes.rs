use chrono::Local;
use nannou::prelude::*;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use rusty_visuals::colorer::*;
use rusty_visuals::hi_res_capture::HiResCapture;

fn main() {
    // The `exit` handler makes sure the PNG is captured before exiting
    // the program
    nannou::app(model).update(update).exit(exit).run();
}

const GRID_COL_RESOLUTION: usize = 15;
const GRID_ROW_RESOLUTION: usize = 15;

struct Model {
    frame: Rect,
    colorer: Box<dyn Colorer>,
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
    let frame = geom::Rect::from_w_h((width - 200) as f32, (height - 200) as f32);
    
    
    let pastel_colorer = PastelColorer::new();
    let colorer = ModuloColorer::new(Box::new(pastel_colorer), Hsv::new(0.0, 0.0, 1.0), 3);

    Model {
        frame,
        colorer: Box::new(colorer),
        hi_res_capture,
        save_image: false,
        seed: Local::now().timestamp() as u64,
        drawing_num: 0,
        debug: false,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    // First, reset the `draw` state.
    match model.drawing_num {
        0 => drawing_0(app, model),
        1 => drawing_1(app, model),
        _ => println!("no drawing!"),
    }

    // DRAWING #2:

    if model.save_image {
        model.save_image = false;
    }
}

/// DRAWING #0: Squares rotate inwards with random chance of coloring the box. The sizes of the box
/// also decrease till the angle hits about 180.0 at which point the box size stays the same but
/// keeps rotating 
///
fn drawing_0(app: &App, model: &mut Model) {
    let draw = &model.hi_res_capture.draw;
    draw.background().color(WHITE);

    let mut rng = StdRng::seed_from_u64(model.seed);
    let rotation_angle = 2.0;
    let mut start_angle = 0.0;
    let original_color = Hsva::new(0.0, 0.0, 1.0, 0.0);
    let mut color = original_color;
    let wh = model.frame.wh() * 0.80;
    while start_angle <= 360.0 {
        // Decrease the box size as the angle rotates more and more
        let w = wh.x - map_range(start_angle, 0.0, 180.0, 10.0, wh.x);
        let h = wh.y - map_range(start_angle, 0.0, 180.0, 10.0, wh.y);

        // Decrease the probability of coloring a box as the angle changes
        let color_change_probability = 1.0 - map_range(start_angle, 0.0, 360.0, 0.0, 1.0);
        if color_change_probability > 0.90 {
            let hue = rng.gen_range(0.0, 360.0);
            let new_color = Hsva::new(hue, 0.5, 1.0, 0.30);
            color = new_color;
        }
        draw.rect()
            .xy(model.frame.xy())
            .wh(vec2(w, h))
            .stroke_color(BLACK)
            .stroke_weight(3.0)
            .color(color)
            .rotate(-1.0 * start_angle.to_radians());
        start_angle += rotation_angle;
        color = original_color;
    }

    model.hi_res_capture.update(app, model.save_image);
}

/// DRAWING #1: Grid with solid line along either of the two diagonals, chosen
/// randomly
///
/// For each cell, randomly choose a direction along which to draw a line.
/// Either:
/// 1) Top left to the bottom right
/// 2) Bottom left to top right
fn drawing_1(app: &App, model: &mut Model) {
    let draw = &model.hi_res_capture.draw;
    draw.background().color(WHITE);

    let mut rng = StdRng::seed_from_u64(model.seed);
    let rotation_angle = 2.0;
    let mut start_angle = 0.0;

    // Iteration
    let wh = model.frame.wh() * 0.80;
    while start_angle <= 360.0 {
        let w = wh.x - map_range(start_angle, 0.0, 360.0, 10.0, wh.x);
        let h = wh.y - map_range(start_angle, 0.0, 360.0, 10.0, wh.y);
        let color = model.colorer.color();
        draw.rect()
            .xy(model.frame.xy())
            .wh(vec2(w, h))
            .stroke_color(BLACK)
            .stroke_weight(4.0)
            .color(color)
            .rotate(-1.0 * start_angle.to_radians());
        start_angle += rotation_angle;
    }

    model.hi_res_capture.update(app, model.save_image);
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
        _other => (),
    }
}
