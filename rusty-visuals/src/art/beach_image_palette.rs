use image::io::Reader as ImageReader;
use image::{DynamicImage, ImageResult, RgbaImage};
use nannou::color::*;
use nannou::prelude::*;
use rusty_visuals::*;
use std::path::Path;

const GRID_WIDTH: usize = 80;
const GRID_HEIGHT: usize = 148;

fn main() {
    nannou::app(model).run();
}

struct Model {
    downsampled_color_map: [[Hsva; GRID_WIDTH]; GRID_HEIGHT],
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(652, 1200)
        .event(event)
        .view(view)
        .build()
        .unwrap();

    // let dynamic_image = read_image(Path::new(
    //     "src/art/resources/aron-yigin-v_DbI6EcAlo-unsplash.jpg",
    // ))
    let dynamic_image = read_image(Path::new(
        "src/art/resources/lalo-hernandez-Amo081zdJsI-unsplash.jpg",
    ))
    .unwrap()
    .into_rgba8();
    let chunk_width = dynamic_image.width() / GRID_WIDTH as u32;
    let chunk_height = dynamic_image.height() / GRID_HEIGHT as u32;
    let mut downsampled_color_map: [[Hsva; GRID_WIDTH]; GRID_HEIGHT] =
        [[Hsva::default(); GRID_WIDTH]; GRID_HEIGHT];
    for row in 0..downsampled_color_map.len() {
        for col in 0..downsampled_color_map[row].len() {
            let averaged_hsv = get_average_rgb(row, col, chunk_width, chunk_height, &dynamic_image);
            downsampled_color_map[row][col] = averaged_hsv.saturate(0.3);
            // downsampled_color_map[row][col] = averaged_hsv;
        }
    }
    Model {
        downsampled_color_map,
    }
}

fn get_average_rgb(
    downsampled_row: usize,
    downsampled_col: usize,
    chunk_width: u32,
    chunk_height: u32,
    image: &RgbaImage,
) -> Hsva {
    let mut sums: [u32; 4] = [0; 4];
    let mut count: u32 = 0;

    // Translate the smaller downsampled index into
    // the index in the larger image by multiplying the index
    // by width or height. This is because we are grabbing
    // matrices of size (chunk_height, chunk_width) in a
    // non-overlapping manner.
    let image_row_start = downsampled_row as u32 * chunk_height;
    let image_col_start = downsampled_col as u32 * chunk_width;
    for row in 0..chunk_height {
        for col in 0..chunk_width {
            // Make sure we're accessing a correct pixel
            if (image_row_start + row < image.height()) && (image_col_start + col < image.width()) {
                let pixel = image
                    .get_pixel(image_col_start + col, image_row_start + row)
                    .0;
                sums[0] += pixel[0] as u32;
                sums[1] += pixel[1] as u32;
                sums[2] += pixel[2] as u32;
                sums[3] += pixel[3] as u32;
                count += 1;
            }
        }
    }
    // println!("found {} number of points to average", count);
    let color = LinSrgba::new(
        (sums[0] as f32 / count as f32) / 255.0, // The constructor takes in floats as a % of 255 to represent intensity
        (sums[1] as f32 / count as f32) / 255.0,
        (sums[2] as f32 / count as f32) / 255.0,
        (sums[3] as f32 / count as f32) / 255.0,
    );
    let new_color = Hsva::convert_from(color);
    new_color
}

fn view(app: &App, m: &Model, frame: Frame) {
    let draw = app.draw();
    let rect = app.window_rect();

    if app.elapsed_frames() != 1 {
        return;
    }
    draw.background().color(BLACK);
    let total_num_cells = grid::CellIndex {
        row: GRID_HEIGHT,
        col: GRID_WIDTH,
    };
    let grid = grid::Grid::new(&rect, &total_num_cells);
    for cell in grid.row_major_iter() {
        let color = m.downsampled_color_map[cell.index.row][cell.index.col];

        // EXPERIMENT 1: Randomly change cells on the grid to shifted pixels in the
        // original grid
        // EXPERIMENT 2: Use ellipses instead of rectangles to render
        //
        // if random_f32() < 0.3 {
        //     let shifted_color = m.downsampled_color_map
        //         [(cell.index.row + 3) % m.downsampled_color_map.len()]
        //         [(cell.index.col + 3) % m.downsampled_color_map[0].len()];
        //     draw.rect().xy(cell.xy).wh(cell.wh).color(shifted_color);
        // } else {
        draw.ellipse().wh(cell.wh).xy(cell.xy).color(color);
        // draw.rect().xy(cell.xy).wh(cell.wh).color(color);
        // }
    }

    draw.to_frame(app, &frame).unwrap();
}

fn event(app: &App, _m: &mut Model, event: WindowEvent) {
    match event {
        KeyPressed(Key::P) => {
            println!("printing out because P was pressed");
            file_utils::capture_frame_to_path(app);
        }
        _other => (),
    }
}

fn read_image(file_path: &Path) -> ImageResult<DynamicImage> {
    ImageReader::open(file_path)?.decode()
}
