use nannou::prelude::*;
use rusty_visuals::*;

const GRID_SIDE_LENGTH: f32 = 5.0;

fn main() {
    nannou::app(model).run();
}

struct Model {
    colored_grid: grid::Grid<grid::NoiseColorer>,
}

struct Heights {
    sand_height: f32,
    water_height: f32,
    sky_height: f32,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(1200, 1200)
        .event(event)
        .view(view)
        .build()
        .unwrap();
    let colorer = grid::NoiseColorer::new(Hsv::new(36.0, 0.53, 0.63), vec2(29.0, 42.0));
    let colored_grid = grid::Grid::new(colorer);

    Model { colored_grid }
}

fn view(app: &App, m: &Model, frame: Frame) {
    let draw = app.draw();
    let rect = app.window_rect();

    draw.background().color(WHITE);

    let heights = Heights {
        sand_height: rect.h() / 5.0,
        water_height: 2.0 * (rect.h() / 5.0),
        sky_height: 2.0 * (rect.h() / 5.0),
    };

    draw_sand(&draw, &heights, rect, &m.colored_grid);
    draw_water(&draw, &heights, rect);
    draw_sky(&draw, &heights, rect);

    draw.to_frame(app, &frame).unwrap();
}

fn draw_sand(
    draw: &Draw,
    heights: &Heights,
    rect: Rect,
    colored_grid: &grid::Grid<grid::NoiseColorer>,
) {
    let _yellow = Hsv::new(36.0, 0.53, 0.63);
    let positioning_rect = Rect::from_wh(vec2(rect.w(), heights.sand_height))
        .align_left_of(rect)
        .align_bottom_of(rect);

    let num_boxes = pt2(240, 240);
    colored_grid.draw(draw, &positioning_rect, num_boxes);
}

fn draw_water(draw: &Draw, heights: &Heights, rect: Rect) {
    let positioning_rect = Rect::from_wh(vec2(rect.w(), heights.water_height))
        .align_left_of(rect)
        .align_bottom_of(rect)
        .shift_y(heights.sand_height);

    draw.rect()
        .h(heights.water_height)
        .w(rect.w())
        .xy(positioning_rect.xy())
        .color(DARKBLUE);
}

fn draw_sky(draw: &Draw, heights: &Heights, rect: Rect) {
    let positioning_rect = Rect::from_wh(vec2(rect.w(), heights.sky_height))
        .align_left_of(rect)
        .align_bottom_of(rect)
        .shift_y(heights.sand_height + heights.water_height);

    draw.rect()
        .h(heights.sky_height)
        .w(rect.w())
        .xy(positioning_rect.xy())
        .color(SKYBLUE);
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
