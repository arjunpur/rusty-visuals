use nannou::color::*;
use nannou::prelude::*;
use rusty_visuals::grid::{Colorer, ColorerParams, InterpolatedColorer};
use rusty_visuals::*;
use std::collections::VecDeque;

fn main() {
    nannou::app(model).run()
}

struct Model {
    colored_grid: grid::ColoredGrid<grid::RotatingColorer>,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(1200, 1200)
        .event(event)
        .view(view)
        .build()
        .unwrap();
    let sun_and_sky_colorer = SunAndSky::new(InterpolatedColorer::new((
        Hsv::new(0.0, 1.0, 1.0),
        Hsv::new(60.0, 1.0, 1.0),
    )));
    let colorers: Vec<Box<dyn Colorer>> = vec![
        Box::new(InterpolatedColorer::new((
            Hsv::new(60.0, 1.0, 1.0),
            Hsv::new(180.0, 1.0, 1.0),
        ))),
        Box::new(sun_and_sky_colorer),
    ];
    let colorers_vec_deque = VecDeque::from(colorers);
    let colorer = grid::RotatingColorer::new(colorers_vec_deque);
    let colored_grid = grid::ColoredGrid::new(colorer);
    Model { colored_grid }
}

fn view(app: &App, m: &Model, frame: Frame) {
    let draw = app.draw();
    let rect = app.window_rect();

    let num_boxes = pt2(200, 200);
    let _grid = m.colored_grid.draw(&draw, &rect, num_boxes);

    draw.background().color(WHITE);
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
            m.colored_grid.update();
        }
        _other => (),
    }
}

struct SunAndSky {
    interpolated_colorer: InterpolatedColorer,
}

impl Colorer for SunAndSky {
    fn color(&self, params: ColorerParams) -> Hsv {
        let radius = params.grid_rect.w();
        // Colors in a circle
        if (params.current_box_rect.left() + (radius / 2.0)).pow(2.0)
            + (params.current_box_rect.bottom() + (radius / 2.0)).pow(2.0)
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
    pub fn new(interpolated_colorer: InterpolatedColorer) -> Self {
        SunAndSky {
            interpolated_colorer,
        }
    }
}
