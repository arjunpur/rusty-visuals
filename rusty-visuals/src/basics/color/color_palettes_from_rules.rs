use nannou::color::*;
use nannou::prelude::*;
use rusty_visuals::colorer::{
    AlternatingColorer, GridColorer, GridParams, PaletteColorer, RotatingColorer,
};
use rusty_visuals::file_utils;
use rusty_visuals::grid::{CellIndex, Grid};
use std::collections::VecDeque;

fn main() {
    nannou::app(model).run()
}

struct Model {
    colorer: Box<dyn GridColorer>,
    to_update_on_frames: u64,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(1200, 1200)
        .event(event)
        .view(view)
        .build()
        .unwrap();
    let pastels = PaletteColorer::new(
        (0..360).map(|n| n as f32 / 360.0).collect(),
        vec![0.6],
        vec![1.0],
    );
    let greens = PaletteColorer::new(
        (90..180).map(|n| n as f32 / 360.0).collect(),
        (60..70).map(|n| n as f32 * 0.01).collect(),
        vec![0.8],
    );
    let alternating_colorer_2 =
        AlternatingColorer::new(vec![hsv(0.2, 0.5, 1.0), hsv(0.7, 0.5, 1.0)]);
    let colorers: Vec<Box<dyn GridColorer>> = vec![
        Box::new(pastels),
        Box::new(alternating_colorer_2),
        Box::new(greens),
    ];
    let colorers_vec_deque = VecDeque::from(colorers);
    let colorer = RotatingColorer::new(colorers_vec_deque);

    Model {
        colorer: Box::new(colorer),
        to_update_on_frames: 1,
    }
}

fn view(app: &App, m: &Model, frame: Frame) {
    let draw = app.draw();
    let rect = app.window_rect();

    if app.elapsed_frames() != m.to_update_on_frames {
        return;
    }

    draw.background().color(hsv(0.0, 0.0, 0.1));

    draw_grid_with_transperancy(&draw, &rect, m);
    // draw_basic_grid(&draw, &rect, m);

    draw.to_frame(app, &frame).unwrap();
}

fn draw_grid_with_transperancy(draw: &Draw, rect: &Rect, m: &Model) {
    let num_cells = CellIndex { row: 20, col: 20 };
    let grid = Grid::new(&rect, &num_cells);
    for cell in grid.row_major_iter() {
        if random_f32() < 0.4 {
            continue;
        }
        let color = m.colorer.color(GridParams {
            cell: &cell,
            total_num_cells: &num_cells,
        });
        let transperant_color = hsva(
            color.hue.to_positive_degrees() / 360.0,
            color.saturation,
            color.value,
            0.7,
        );
        let jitter = vec2(
            (cell.xy.x.sin().abs() + 0.1) * cell.wh.x,
            (cell.xy.y.cos().abs() + 0.1) * cell.wh.y,
        );
        draw.rect()
            .xy(vec2(
                10.0 * cell.xy.x.cos() + cell.xy.x,
                10.0 * cell.xy.y.sin() + cell.xy.y,
            ))
            .wh(vec2(cell.wh.x + jitter.x, cell.wh.y + jitter.y))
            .color(transperant_color);
    }
}

fn draw_basic_grid(draw: &Draw, rect: &Rect, m: &Model) {
    let num_cells = CellIndex { row: 30, col: 10 };
    let grid = Grid::new(&rect, &num_cells);
    for cell in grid.row_major_iter() {
        draw.rect()
            .xy(cell.xy)
            .wh(cell.wh)
            .color(m.colorer.color(GridParams {
                cell: &cell,
                total_num_cells: &num_cells,
            }));
    }
}

fn event(app: &App, m: &mut Model, event: WindowEvent) {
    match event {
        KeyPressed(Key::P) => {
            println!("printing out because P was pressed");
            file_utils::capture_frame_to_path(app);
        }
        KeyPressed(Key::C) => {
            m.colorer.update();
            m.to_update_on_frames = app.elapsed_frames() + 1;
        }
        _other => (),
    }
}
