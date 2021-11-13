use nannou::prelude::*;
use rusty_visuals::*;

fn main() {
    nannou::app(model).run();
}

struct Model {
    colorer: Box<dyn colorer::Colorer>,
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
    let colorer = colorer::NoiseColorer::new(Hsv::new(36.0, 0.53, 0.63), vec2(29.0, 42.0));

    Model {
        colorer: Box::new(colorer),
    }
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

    draw_sand(&draw, &heights, rect, m);
    draw_water(&draw, &heights, rect);
    draw_sky(&draw, &heights, rect);

    draw.to_frame(app, &frame).unwrap();
}

fn draw_sand(draw: &Draw, heights: &Heights, rect: Rect, model: &Model) {
    let _yellow = Hsv::new(36.0, 0.53, 0.63);
    let positioning_rect = Rect::from_wh(vec2(rect.w(), heights.sand_height))
        .align_left_of(rect)
        .align_bottom_of(rect);

    let total_num_cells = &grid::CellIndex { row: 10, col: 10 };
    let grid = grid::Grid::new(&positioning_rect, total_num_cells);
    grid.row_major_iter().for_each(|cell| {
        draw.rect()
            .xy(cell.xy)
            .wh(cell.wh)
            .color(model.colorer.color(colorer::ColorerParams {
                cell: &cell,
                total_num_cells,
            }));
    })
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
