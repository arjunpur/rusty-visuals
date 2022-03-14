use chrono::Local;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use nannou::prelude::*;
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
    let r = geom::Rect::from_w_h(width as f32, height as f32);
    let grid = Grid::new(&r, &CellIndex{row: GRID_ROW_RESOLUTION, col: GRID_COL_RESOLUTION});

    Model {
        grid,
        hi_res_capture,
        save_image: false,
        seed: Local::now().timestamp() as u64, 
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    // First, reset the `draw` state.
    let draw = &model.hi_res_capture.draw;

    draw.background().color(WHITE);

    // https://rust-random.github.io/book/guide-start.html 
    let mut rng = StdRng::seed_from_u64(model.seed);

    // For each cell, randomly choose a direction along which to draw a line.
    // Either:
    // 1) Top left to the bottom right
    // 2) Bottom left to top right
    //
    for cell in model.grid.row_major_iter() {
        let line_direction = rng.gen_range(0.0, 2.0).floor() as i32;
        if line_direction == 0 {
            draw.line().caps_round().start(pt2(cell.left(), cell.top())).end(pt2(cell.right(), cell.bottom())).weight(40.0);
        } else {
            draw.line().caps_round().start(pt2(cell.left(), cell.bottom())).end(pt2(cell.right(), cell.top())).weight(40.0);
        }
    }

    model.hi_res_capture.update(app, model.save_image);

    if model.save_image {
        model.save_image = false;
    }

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
        _other => (),
    }
}
