use nannou::prelude::*;
use rand::{thread_rng, Rng};
use rusty_visuals::*;

// Adjust this
const JITTER_FACTOR: f32 = 0.1;

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

fn view(app: &App, _: &Model, frame: Frame) {
    let draw = app.draw();
    let rect = app.window_rect();

    let total_num_cells = &grid::CellIndex { row: 20, col: 20 };
    let grid = grid::Grid::new(&rect, total_num_cells);

    // Only render in the first frame
    if app.elapsed_frames() != 1 {
        return;
    }
    draw.background().color(WHITE);

    use colorer::Colorer;
    let colorer = colorer::AlternatingColorer::new(vec![
        hsv(0.53, 1.0, 0.64),
        hsv(0.275, 1.0, 0.64),
        hsv(0.138, 0.48, 0.99),
    ]);

    for cell in grid.row_major_iter() {
        let r = Rect::from_x_y_w_h(cell.xy.x, cell.xy.y, cell.wh.x, cell.wh.y);
        let points = vec![
            pt2(r.top_left().x, r.top_left().y),
            pt2(r.top_right().x, r.top_right().y),
            pt2(r.bottom_right().x, r.bottom_right().y),
            pt2(r.bottom_left().x, r.bottom_left().y),
        ];
        let jittered_points = points
            .iter()
            .map(|pt| pt2(pt.x + jitter(cell.wh.x), pt.y + jitter(cell.wh.y)));
        // draw jittered rectangles and color with alternate colors
        draw.polygon()
            .points(jittered_points)
            .color(colorer.color(colorer::ColorerParams {
                cell: &cell,
                total_num_cells,
            }));
    }

    draw.to_frame(app, &frame).unwrap();
}

fn jitter(tile_size: f32) -> f32 {
    let mut rng = thread_rng();
    JITTER_FACTOR * tile_size * rng.gen_range(-1.0, 1.0)
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
