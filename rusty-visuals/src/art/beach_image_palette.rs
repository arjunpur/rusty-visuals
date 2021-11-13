use image::io::Reader as ImageReader;
use image::{DynamicImage, ImageResult, Primitive, RgbaImage};
use itertools::Itertools;
use nannou::color::*;
use nannou::prelude::*;
use rusty_visuals::*;
use std::path::Path;

const GRID_WIDTH: usize = 200;
const GRID_HEIGHT: usize = 200;

fn main() {
    nannou::app(model).run();
}

struct Model {
    downsampled_color_map: [[Hsv; GRID_WIDTH]; GRID_HEIGHT],
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(1200, 1200)
        .event(event)
        .view(view)
        .build()
        .unwrap();

    let window_rect = app.window_rect();
    let dynamic_image = read_image(Path::new(
        "src/art/resources/lalo-hernandez-Amo081zdJsI-unsplash.jpg",
    ))
    .unwrap()
    .into_rgba8();
    println!("image width: {}", dynamic_image.width());
    println!("image height: {}", dynamic_image.height());
    let chunk_width = dynamic_image.width() / GRID_WIDTH as u32;
    let chunk_height = dynamic_image.height() / GRID_HEIGHT as u32;
    println!("chunk width: {}", chunk_width);
    println!("image height: {}", chunk_height);
    let mut downsampled_color_map: [[Hsv; GRID_WIDTH]; GRID_HEIGHT] =
        [[Hsv::default(); GRID_WIDTH]; GRID_HEIGHT];
    for row in 0..downsampled_color_map.len() {
        for col in 0..downsampled_color_map[row].len() {
            println!("processing row, col: ({}, {})", row, col);
            let averaged_hsv = get_average_rgb(row, col, chunk_width, chunk_height, &dynamic_image);
            downsampled_color_map[col][row] = averaged_hsv;
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
) -> Hsv {
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
    println!("found {} number of points to average", count);
    let color = rgba(
        (sums[0] / count) as f32,
        (sums[1] / count) as f32,
        (sums[2] / count) as f32,
        (sums[3] / count) as f32,
    )
    .into_linear();
    println!(
        "color components: ({}, {}, {}, {})",
        color.red, color.green, color.blue, color.alpha
    );
    let hsv: Hsv = color.into();
    hsv
}

fn view(app: &App, m: &Model, frame: Frame) {
    let draw = app.draw();
    let rect = app.window_rect();

    draw.background().color(WHITE);
    let total_num_cells = grid::CellIndex {
        row: GRID_HEIGHT,
        col: GRID_WIDTH,
    };
    let grid = grid::Grid::new(&rect, &total_num_cells);
    for cell in grid.row_major_iter() {
        draw.rect()
            .xy(cell.xy)
            .wh(cell.wh)
            .color(m.downsampled_color_map[cell.index.row][cell.index.col]);
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
