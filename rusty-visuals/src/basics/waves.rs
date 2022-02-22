use nannou::prelude::*;

fn main() {
    // nannou::sketch(moving_circle_view).run()
    nannou::sketch(_sine_wave).run()
}

fn _basic_circle_and_rect(app: &App, frame: Frame) {
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

fn _sine_wave(app: &App, frame: Frame) {
    let draw = app.draw();
    let r = app.window_rect();
    let num_points = 600;
    let num_cycles = 5.0;
    let t = app.time;
    // Define how many seconds a single wave should take
    let period = 1.0;
    let f = TAU / period;
    let num_points_one_period = num_points as f32 / num_cycles;
    // Split the x-axis into num_points, and for each point determine the
    // value that it's y value should take on the sine wave.
    let points = (0..num_points).map(|i| {
        // Map the number of points to the entire width of the screen
        let x = map_range(i as f32, 0.0, num_points as f32, r.left(), r.right());

        // Map the point to some factor of 2Pi
        let theta = map_range(i as f32, 0.0, num_points_one_period, 0.0, TAU);

        // Construct the value of the sine wave
        let y = r.h() * 0.08 * (theta + (t * f)).sin();

        // If you want to see a square wave, uncomment this line:
        // signum returns the sign of the sine wave at that point.
        // let y = r.h() * 0.08 * y.signum()
        pt2(x, y)
    });

    draw.polyline().join_round().weight(3.0).points(points);
    draw.background().color(WHITE);
    draw.to_frame(app, &frame).unwrap();
}

fn square_wave(app: &App, frame: Frame) {
    let draw = app.draw();
    let r = app.window_rect();
    let num_points = 600;
    let num_cycles = 3.0;
    let t = app.time;
    // Define how many seconds a single wave should take
    let period = 3.0;
    let f = TAU / period;
    let num_points_one_period = num_points as f32 / num_cycles;
    // Split the x-axis into num_points, and for each point determine the
    // value that it's y value should take on the sine wave.
    let points = (0..num_points).map(|i| {
        // Map the number of points to the entire width of the screen
        let x = map_range(i as f32, 0.0, num_points as f32, r.left(), r.right());
        let theta = map_range(i as f32, 0.0, num_points_one_period, 0.0, TAU);
        let y = r.h() * 0.08 * (theta + (t * f)).sin();

        // signum returns the sign of the sine wave at that point.
        let y = r.h() * 0.08 * y.signum();
        pt2(x, y)
    });

    draw.polyline().join_round().weight(3.0).points(points);
    draw.background().color(WHITE);
    draw.to_frame(app, &frame).unwrap();
}

fn _moving_circle_view(app: &App, frame: Frame) {
    let rect = app.window_rect().pad(50.0);
    let total = (rect.wh().x + rect.wh().y) * 2.0;

    // Approach: Use the modulus of the time and place it
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

    // Map the time modulus to a value between 0.0 and 1.0 to
    // represent linearly mapped hues (0 -> 360)
    let hue: f32 = map_range(time_mod, 0.0, total, 0.0, 1.0);
    let color = hsl(hue, 1.0, 0.6);
    let background = hsl((app.time * 0.5).sin(), 0.6, 0.8);
    let draw = app.draw();
    draw.ellipse()
        .xy(location)
        .wh(vec2(50.0, 50.0))
        .color(color);
    draw.background().color(background);
    draw.to_frame(app, &frame).unwrap();
}
