use chrono::Local;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use nannou::prelude::*;
use rusty_visuals::file_utils;
use rusty_visuals::grid::*;

fn main() {
    // The `exit` handler makes sure the PNG is captured before exiting 
    // the program
    nannou::app(model).update(update).exit(exit).run();
}

const GRID_COL_RESOLUTION: usize = 15;
const GRID_ROW_RESOLUTION: usize = 15;

struct Model {
    grid: Grid,
    hi_res: HiResBoilerplate,
    // save_image tells the `update` handler to write the c
    // texture to PNG
    save_image: bool,
    seed: u64,
}

struct HiResBoilerplate {
    // The texture that we will draw to.
    texture: wgpu::Texture,
    // Create a `Draw` instance for drawing to our texture.
    draw: nannou::Draw,
    // The type used to render the `Draw` vertices to our texture.
    renderer: nannou::draw::Renderer,
    // The type used to capture the texture.
    texture_capturer: wgpu::TextureCapturer,
    // The type used to resize our texture to the window texture.
    texture_reshaper: wgpu::TextureReshaper,

    snapshot: Option<wgpu::TextueSnapshot>,
}

impl HiResBoilerplate {

    fn update(&mut self, app: &App) {
        // Render our drawing to the texture.
        let window = app.main_window();
        let device = window.device();
        let ce_desc = wgpu::CommandEncoderDescriptor {
            label: Some("texture renderer"),
        };
        let mut encoder = device.create_command_encoder(&ce_desc);
        self 
            .renderer
            .render_to_texture(device, &mut encoder, &self.draw, &self.texture);

        // Take a snapshot of the texture. The capturer will do the following:
        //
        // 1. Resolve the texture to a non-multisampled texture if necessary.
        // 2. Convert the format to non-linear 8-bit sRGBA ready for image storage.
        // 3. Copy the result to a buffer ready to be mapped for reading.
        let snapshot = self
            .texture_capturer
            .capture(device, &mut encoder, &self.texture);

        self.snapshot = Some(snapshot);

        // Submit the commands for our drawing and texture capture to the GPU.
        window.queue().submit(Some(encoder.finish()));
    }

    fn save_image(self, app: &App) {
        let path = file_utils::get_timestamp_path_for_output(app);
        println!("Saving image to {}", path.as_path().to_str().unwrap());
        // Submit a function for writing our snapshot to a PNG.
        //
        // NOTE: It is essential that the commands for capturing the snapshot are `submit`ted before we
        // attempt to read the snapshot - otherwise we will read a blank texture!
        self.snapshot.unwrap()
            .read(move |result| {
                let image = result.expect("failed to map texture memory").to_owned();
                image
                    .save(&path)
                    .expect("failed to save texture to png image");
            })
            .unwrap();
    }
}

fn bootstrap_high_resolution_capture_boilerplate(app: &App, [width, height]: [u32; 2]) -> HiResBoilerplate {
    // Create the window.
    let [win_w, win_h] = [width / 2, height / 2];

    let w_id = app
        .new_window()
        .size(win_w, win_h)
        .event(event)
        .view(view)
        .build()
        .unwrap();

    let window = app.window(w_id).unwrap();

    // Retrieve the wgpu device.
    let device = window.device();

    // Create our custom texture.
    let sample_count = window.msaa_samples();
    let texture = wgpu::TextureBuilder::new()
        .size([width, height])
        // Our texture will be used as the RENDER_ATTACHMENT for our `Draw` render pass.
        // It will also be SAMPLED by the `TextureCapturer` and `TextureResizer`.
        .usage(wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING)
        // Use nannou's default multisampling sample count.
        .sample_count(sample_count)
        // Use a spacious 16-bit linear sRGBA format suitable for high quality drawing.
        .format(wgpu::TextureFormat::Rgba16Float)
        // Build it!
        .build(device);

    // Create our `Draw` instance and a renderer for it.
    let draw = nannou::Draw::new();
    let descriptor = texture.descriptor();
    let renderer =
        nannou::draw::RendererBuilder::new().build_from_texture_descriptor(device, descriptor);

    // Create the texture capturer.
    let texture_capturer = wgpu::TextureCapturer::default();

    // Create the texture reshaper.
    let texture_view = texture.view().build();
    let texture_sample_type = texture.sample_type();
    let dst_format = Frame::TEXTURE_FORMAT;
    let texture_reshaper = wgpu::TextureReshaper::new(
        device,
        &texture_view,
        sample_count,
        texture_sample_type,
        sample_count,
        dst_format,
    );

    // Create the appropriate output directory if neccessary
    file_utils::create_app_output_dir_all(app);

    HiResBoilerplate {
        texture,
        draw,
        renderer,
        texture_capturer,
        texture_reshaper,
        snapshot: None,
    }
}

fn model(app: &App) -> Model {
    let size = [2500, 2500];
    let hi_res = bootstrap_high_resolution_capture_boilerplate(app, size);

    let r = geom::Rect::from_w_h(size[0] as f32, size[1] as f32);
    let grid = Grid::new(&r, &CellIndex{row: GRID_ROW_RESOLUTION, col: GRID_COL_RESOLUTION});

    Model {
        grid,
        hi_res,
        save_image: false,
        seed: Local::now().timestamp() as u64, 
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    // First, reset the `draw` state.
    let draw = &model.hi_res.draw;

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

    model.hi_res.update(app);

    if model.save_image {
        model.hi_res.save_image(app);
        model.save_image = false;
    }

}

// Draw the state of your `Model` into the given `Frame` here.
fn view(_app: &App, model: &Model, frame: Frame) {
    // Sample the texture and write it to the frame.
    let mut encoder = frame.command_encoder();
    model
        .hi_res
        .texture_reshaper
        .encode_render_pass(frame.texture_view(), &mut *encoder);
}

// Wait for capture to finish.
fn exit(app: &App, model: Model) {
    println!("Waiting for PNG writing to complete...");
    let window = app.main_window();
    let device = window.device();
    model
        .hi_res
        .texture_capturer
        .await_active_snapshots(&device)
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
