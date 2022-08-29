use chrono::Local;
use nannou::prelude::*;
use nannou::geom::path;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use rusty_visuals::colorer::*;
use rusty_visuals::hi_res_capture::HiResCapture;

fn main() {
    nannou::app(model).update(update).exit(exit).run();
}

struct Model {
    agent: Agent,
    debug: bool,
    drawing_num: u8,
    frame: Rect,
    hi_res_capture: HiResCapture,
    seed: u64,
    save_image: bool,
}

struct Agent {
    center: Point2,
    radius: f32,
}

impl Agent {
    fn new(center: Point2, radius: f32) -> Agent {

        Agent{
            center,
            radius
        }
    }
}

fn model(app: &App) -> Model {
    // Create the window that the underlying wgpu Texture Capturer is writing to,
    // and the window that program actually displays (downsized to fit on monitors).
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
    let frame = geom::Rect::from_w_h(width as f32, height as f32).pad(200.0);
    let seed = Local::now().timestamp() as u64;

    let center = pt2(
        StdRng::seed_from_u64(seed).gen_range(frame.left(), frame.right()),
        StdRng::seed_from_u64(seed).gen_range(frame.bottom(), frame.top())
    );
    let radius = 60.0;
    let agent = Agent::new(center, radius);

    Model {
        agent,
        debug: false,
        drawing_num: 0,
        frame,
        hi_res_capture,
        seed,
        save_image: false,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    match model.drawing_num {
        0 => drawing_0(app, model),
        _ => println!("no drawing!"),
    }

    if model.save_image {
        model.save_image = false;
    }
}

/// DRAWING #0:
fn drawing_0(app: &App, model: &mut Model) {
    let draw = &model.hi_res_capture.draw;
    draw.background().color(WHITE);

    let mut _rng = StdRng::seed_from_u64(model.seed);

    let control = pt2(
        StdRng::seed_from_u64(model.seed).gen_range(model.frame.left(), model.frame.right()),
        StdRng::seed_from_u64(model.seed).gen_range(model.frame.bottom(), model.frame.top())
    );
    let end = pt2(
        StdRng::seed_from_u64(model.seed).gen_range(model.frame.left(), model.frame.right()),
        StdRng::seed_from_u64(model.seed).gen_range(model.frame.bottom(), model.frame.top())
    );
    let mut builder = path::Builder::new();
    builder = builder.begin(pt2(0.0, 0.0));
    builder = builder.quadratic_bezier_to(
        pt2(100.0, 500.0),
        pt2(200.0, 200.0),
    );

    builder.cubic_bezier_to(ctrl1, ctrl2, to)
    let path = builder.build();

    draw.polyline()
        .weight(10.0)
        .color(SEAGREEN)
        .events(path.iter());

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
        _other => (),
    }
}
