use nannou::color::{self, Hsl, Shade};
use nannou::draw::Draw;
use nannou::geom::path;
use nannou::prelude::*;
use rand::seq::SliceRandom;
use rusty_visuals::*;

// This is light brown / baige
const BACKGROUND_HUE: color::DefaultScalar = 247.0;
const BACKGROUND_SATURATION: color::DefaultScalar = 0.65;
const BACKGROUND_LIGHTNESS: color::DefaultScalar = 0.1;

const FRAME_PADDING: app::DrawScalar = 50.0;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    first_line_positions: Vec<Point2>,
    second_line_positions: Vec<Point2>,
    third_line_positions: Vec<Point2>,
    fourth_line_positions: Vec<Point2>,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(700, 700)
        .event(event)
        .view(view)
        .build()
        .unwrap();
    Model {
        first_line_positions: vec![app.window_rect().pad(FRAME_PADDING).bottom_left()],
        second_line_positions: vec![app.window_rect().pad(FRAME_PADDING).bottom_right()],
        third_line_positions: vec![app.window_rect().pad(FRAME_PADDING).top_left()],
        fourth_line_positions: vec![app.window_rect().pad(FRAME_PADDING).top_right()],
        // first_line_positions: vec![app.window_rect().pad(FRAME_PADDING).bottom_left()],
        // second_line_positions: vec![
        //     app.window_rect().pad(FRAME_PADDING).bottom_left() + vec2(200.0, 0.0),
        // ],
        // third_line_positions: vec![app.window_rect().pad(FRAME_PADDING).top_right()],
        // fourth_line_positions: vec![
        //     app.window_rect().pad(FRAME_PADDING).top_right() - vec2(200.0, 0.0),
        // ],
    }
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

fn update(app: &App, m: &mut Model, _update: Update) {
    for _ in 0..1 {
        let last_position = m.first_line_positions.last().unwrap();
        let padded_rect = app.window_rect().pad(30.0);
        let directions: Vec<Vector2<app::DrawScalar>> =
            vec![vec2(5.0, 0.0), vec2(0.0, 5.0), vec2(-5.0, -5.0)];
        m.first_line_positions
            .push(get_next_position_with_directions(
                padded_rect,
                last_position,
                directions,
            ));

        let last_position = m.second_line_positions.last().unwrap();
        let directions: Vec<Vector2<app::DrawScalar>> =
            vec![vec2(-5.0, 0.0), vec2(0.0, 5.0), vec2(5.0, -5.0)];
        m.second_line_positions
            .push(get_next_position_with_directions(
                padded_rect,
                last_position,
                directions,
            ));

        let last_position = m.third_line_positions.last().unwrap();
        let directions: Vec<Vector2<app::DrawScalar>> =
            vec![vec2(5.0, 0.0), vec2(0.0, -5.0), vec2(-5.0, 5.0)];
        m.third_line_positions
            .push(get_next_position_with_directions(
                padded_rect,
                last_position,
                directions,
            ));

        let last_position = m.fourth_line_positions.last().unwrap();
        let directions: Vec<Vector2<app::DrawScalar>> =
            vec![vec2(-5.0, 0.0), vec2(0.0, -5.0), vec2(5.0, 5.0)];
        m.fourth_line_positions
            .push(get_next_position_with_directions(
                padded_rect,
                last_position,
                directions,
            ));
    }
}

fn get_next_position(rect: Rect<app::DrawScalar>, current_position: &Vector2) -> Vector2 {
    let directions: Vec<Vector2<app::DrawScalar>> = vec![
        vec2(0.0, 2.0),
        vec2(2.0, 0.0),
        vec2(0.0, -2.0),
        vec2(-2.0, 0.0),
    ];
    return get_next_position_with_directions(rect, current_position, directions);
}

fn get_next_position_with_directions(
    rect: Rect<app::DrawScalar>,
    current_position: &Vector2,
    directions: Vec<Vector2<app::DrawScalar>>,
) -> Vector2 {
    let random_direction = directions.choose(&mut rand::thread_rng()).unwrap();
    let future_position = *current_position + (*random_direction * 5.0);
    let mut adjusted_direction = *random_direction;
    if future_position.x < rect.left() || future_position.x > rect.right() {
        adjusted_direction.x *= -1.0;
    }
    if future_position.y < rect.bottom() || future_position.y > rect.top() {
        adjusted_direction.y *= -1.0;
    }
    return *current_position + (adjusted_direction * 5.0);
}

fn view(app: &App, m: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(Hsl::new(
        BACKGROUND_HUE,
        BACKGROUND_SATURATION,
        BACKGROUND_LIGHTNESS,
    ));

    let first_line_color = Hsl::new(158.0, 0.95, 0.35);
    let second_line_color = Hsl::new(197.0, 0.75, 0.45);
    let third_line_color = Hsl::new(4.0, 0.85, 0.55);
    let fourth_line_color = Hsl::new(57.0, 0.95, 0.45);

    draw_polyline(&draw, &m.first_line_positions, first_line_color);
    draw_polyline(&draw, &m.second_line_positions, second_line_color);
    draw_polyline(&draw, &m.fourth_line_positions, fourth_line_color);
    draw_polyline(&draw, &m.third_line_positions, third_line_color);

    draw.to_frame(app, &frame).unwrap();
}

fn draw_polyline(draw: &Draw, points: &Vec<Vector2<app::DrawScalar>>, color: Hsl) {
    let mut builder = path::Builder::new();
    builder = builder.move_to(points[0]);

    for i in 1..points.len() {
        builder = builder.line_to(points[i]);
    }

    let path = builder.build();

    // Set the start of the path to the original point
    draw.polyline()
        .join_round()
        .weight(2.0)
        .color(color)
        .events(path.iter());

    let offset = pt2(-3.50, -3.50);
    let mut shadow_builder = path::Builder::new();
    shadow_builder = shadow_builder.move_to(points[0] + offset);

    for i in 1..points.len() {
        shadow_builder = shadow_builder.line_to(points[i] + offset);
    }

    let path = shadow_builder.build();

    // Set the start of the path to the original point
    draw.polyline()
        .join_round()
        .weight(2.0)
        .color(color.darken(0.13))
        .events(path.iter());
}
