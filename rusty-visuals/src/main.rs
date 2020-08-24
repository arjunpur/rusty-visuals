use nannou::prelude::*;

fn main() {
    nannou::sketch(moving_circle_view).run()
}

fn _view(app: &App, frame: Frame) {
    // Get the current window and pad it.
    let pad = 25.0;
    let win = app.window_rect();
    let win = win.pad(pad);
    app.draw().rect();

    // Construct a rectangle of the suplied width / height and
    // place it in the top left corner of the window
    let r = Rect::from_w_h(100.0, 100.0).top_left_of(win);

    // Draw a copy of the window
    let draw = app.draw();
    draw.rect()
        .xy(win.xy())
        .wh(win.wh())
        .color(rgba(0.3, 0.2, 0.9, 0.1));
    // Place a new rectangle in the padded corner of the padded window
    draw.rect().xy(r.xy()).wh(r.wh()).color(PLUM);

    let circle = r.below(r).shift_y(-pad);
    draw.ellipse().xy(circle.xy()).wh(circle.wh()).color(SALMON);
    draw.background().color(STEELBLUE);
    draw.scale(15.0);
    draw.to_frame(app, &frame).unwrap();
}

fn moving_circle_view(app: &App, frame: Frame) {
    let rect = app.window_rect().pad(50.0);
    let total = rect.wh().sum() * 2.0;

    // Approach 1: Generate all points along the rectangle first
    let top = rect.top().ceil() as i32;
    let bottom = rect.bottom().floor() as i32;
    let left = rect.left().ceil() as i32;
    let right = rect.right().floor() as i32;
    let vertical = |start, stop, fix| {
        (start..stop)
            .step_by(1.0 as usize)
            .map(|v| vec2(fix, v as f32))
    };
    let left_vertical = vertical(bottom, top, rect.left()).rev();
    let right_vertical = vertical(bottom, top, rect.right());
    let bottom_horizontal = vertical(left, right, rect.bottom());
    let top_horizontal = vertical(left, right, rect.top()).rev();

    // TODO: Simplify this logic
    let combined = left_vertical
        .chain(bottom_horizontal)
        .chain(right_vertical)
        .chain(top_horizontal)
        .into_iter();
    let length = combined.collect::<Vec<Vector2>>().len();
    let location: Vector2 = combined.nth(app.time as usize % length).unwrap();

    // Approach 2: Use the modulus of the time and place it
    // in one of 4 buckets: [left vert, bottom horz, right vert, top horz]
    let time_mod = (app.time * 100.0) % total;
    let mut location = vec2(0.0, 0.0);
    if time_mod <= rect.h() {
        location.x = rect.left();
        location.y = rect.top() - time_mod;
    } else if time_mod <= rect.h() + rect.w() {
        location.x = rect.left() + (time_mod - rect.h());
        location.y = rect.bottom();
    } else if time_mod <= rect.h() + rect.w() + rect.h() {
        location.x = rect.right();
        location.y = rect.bottom() + (time_mod - (rect.w() + rect.h()));
    } else {
        location.x = rect.right() - (time_mod - (rect.w() + rect.h() + rect.h()));
        location.y = rect.top();
    }
    let draw = app.draw();
    draw.ellipse()
        .xy(location)
        .wh(vec2(50.0, 50.0))
        .color(SALMON);
    draw.background().color(STEELBLUE);
    draw.to_frame(app, &frame).unwrap();
}
