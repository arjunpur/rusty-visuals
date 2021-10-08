use nannou::color::*;
use nannou::prelude::*;
use rusty_visuals::grid::{Colorer, InterpolatedColorer};
use rusty_visuals::*;
use std::collections::VecDeque;
use std::iter::FromIterator;

fn main() {
    nannou::app(model).run();
}

struct Model {
    colorers: VecDeque<Box<dyn Colorer>>,
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
            Hsv::new(0.0, 1.0, 1.0),
            Hsv::new(60.0, 1.0, 1.0),
        ))),
        Box::new(sun_and_sky_colorer),
    ];
    let colorers_vec_deque = VecDeque::from(colorers);
    Model {
        colorers: colorers_vec_deque,
    }
}

fn view(app: &App, m: &Model, frame: Frame) {
    let draw = app.draw();
    let rect = app.window_rect();

    let num_boxes = pt2(200, 200);
    let colorer_clone = m.colorers.front().unwrap().clone();
    let colorer = colorer_clone.as_mut();
    let _grid = grid::ColoredGrid::draw(&draw, &rect, num_boxes, colorer);

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
            let front = m.colorers.pop_front().unwrap();
            m.colorers.push_back(front);
        }
        _other => (),
    }
}

struct SunAndSky {
    interpolated_colorer: InterpolatedColorer,
}

impl Colorer for SunAndSky {
    fn color(&mut self, i_x: i32, i_y: i32, t_x: i32, t_y: i32) -> Hsv {
        return self.interpolated_colorer.color(i_x, i_y, t_x, t_y);
    }
}

impl SunAndSky {
    pub fn new(interpolated_colorer: InterpolatedColorer) -> Self {
        SunAndSky {
            interpolated_colorer,
        }
    }
}
