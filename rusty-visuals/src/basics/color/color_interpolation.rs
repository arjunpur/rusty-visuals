use nannou::color::*;
use nannou::prelude::*;
use rusty_visuals::colorer::{GridColorer, GridParams, InterpolatedColorer};
use rusty_visuals::grid::CellIndex;
use rusty_visuals::*;
use std::collections::VecDeque;

fn main() {
    nannou::app(model).run()
}

struct Model {
    colorer: Box<dyn GridColorer>,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(1200, 1200)
        .event(event)
        .view(view)
        .build()
        .unwrap();
    let rect = app.window_rect();
    let sun_and_sky_colorer = SunAndSky::new(
        InterpolatedColorer::new((Hsv::new(0.0, 1.0, 1.0), Hsv::new(60.0, 1.0, 1.0))),
        rect,
    );

    let colorers: Vec<Box<dyn GridColorer>> = vec![
        Box::new(InterpolatedColorer::new((
            Hsv::new(60.0, 1.0, 1.0),
            Hsv::new(180.0, 1.0, 1.0),
        ))),
        Box::new(sun_and_sky_colorer),
    ];
    let colorers_vec_deque = VecDeque::from(colorers);
    let colorer = colorer::RotatingColorer::new(colorers_vec_deque);

    Model {
        colorer: Box::new(colorer),
    }
}

fn view(app: &App, m: &Model, frame: Frame) {
    let draw = app.draw();
    let rect = app.window_rect();
    draw.background().color(WHITE);

    let num_cells = CellIndex { row: 200, col: 200 };
    let grid = grid::Grid::new(&rect, &num_cells);
    for cell in grid.row_major_iter() {
        draw.rect()
            .xy(cell.xy)
            .wh(cell.wh)
            .color(m.colorer.color(GridParams {
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
        KeyPressed(Key::D) => {
            println!("Mouse Position: {}, {}", app.mouse.y, app.mouse.x);
        }
        KeyPressed(Key::C) => {
            m.colorer.update();
        }
        _other => (),
    }
}

struct SunAndSky {
    interpolated_colorer: InterpolatedColorer,
    grid_rect: Rect,
}

impl GridColorer for SunAndSky {
    fn color(&self, params: GridParams) -> Hsv {
        let radius = self.grid_rect.w();
        // Colors in a circle
        if (params.cell.left() + (radius / 2.0)).pow(2.0)
            + (params.cell.bottom() + (radius / 2.0)).pow(2.0)
            <= radius.pow(2.0)
        {
            return self.interpolated_colorer.color(params);
        } else if random_f32() < 0.70 {
            return Hsv::new(200.0, 0.9, 1.0);
        }
        Hsv::new(0.0, 0.0, 1.0)
    }

    fn update(&mut self) {}
}

impl SunAndSky {
    pub fn new(interpolated_colorer: InterpolatedColorer, grid_rect: Rect) -> Self {
        SunAndSky {
            interpolated_colorer,
            grid_rect,
        }
    }
}
