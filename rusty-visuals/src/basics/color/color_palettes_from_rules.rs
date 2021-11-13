use nannou::color::*;
use nannou::prelude::*;
use rand::{thread_rng, Rng};
use rusty_visuals::colorer::{AlternatingColorer, Colorer, ColorerParams, RotatingColorer};
use rusty_visuals::file_utils;
use rusty_visuals::grid::{CellIndex, Grid};
use std::collections::VecDeque;

fn main() {
    nannou::app(model).run()
}

struct Model {
    colorer: Box<dyn Colorer>,
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
        vec![0.4],
        vec![0.8],
    );
    let greens = PaletteColorer::new(
        (90..180).map(|n| n as f32 / 360.0).collect(),
        (60..70).map(|n| n as f32 * 0.01).collect(),
        vec![0.8],
    );
    let alternating_colorer_2 =
        AlternatingColorer::new(vec![hsv(0.2, 0.5, 1.0), hsv(0.7, 0.5, 1.0)]);
    let colorers: Vec<Box<dyn Colorer>> = vec![
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

    draw.background().color(WHITE);

    let num_cells = CellIndex { row: 30, col: 10 };
    let grid = Grid::new(&rect, &num_cells);
    for cell in grid.row_major_iter() {
        draw.rect()
            .xy(cell.xy)
            .wh(cell.wh)
            .color(m.colorer.color(ColorerParams {
                cell: &cell,
                total_num_cells: &num_cells,
            }));
    }

    draw.to_frame(app, &frame).unwrap();
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

pub struct PaletteColorer {
    hues: Vec<f32>,
    saturations: Vec<f32>,
    values: Vec<f32>,
}

impl Colorer for PaletteColorer {
    fn color(&self, _: ColorerParams) -> Hsv {
        let mut rng = thread_rng();
        let hue_idx = rng.gen_range(0, self.hues.len());
        let saturations_idx = rng.gen_range(0, self.saturations.len());
        let values_idx = rng.gen_range(0, self.values.len());
        hsv(
            self.hues[hue_idx],
            self.saturations[saturations_idx],
            self.values[values_idx],
        )
    }

    fn update(&mut self) {}
}

impl PaletteColorer {
    pub fn new(hues: Vec<f32>, saturations: Vec<f32>, values: Vec<f32>) -> Self {
        if hues.len() == 0 || saturations.len() == 0 || values.len() == 0 {
            panic!("hues or saturations or values must not be empty");
        }
        PaletteColorer {
            hues,
            saturations,
            values,
        }
    }
}
