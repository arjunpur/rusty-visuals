use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run()
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    draw.rect()
        .x_y(50.0, 50.0)
        .w_h(100.0, 100.0)
        .z_degrees(45.0)
        .color(PLUM);
    draw.background().color(STEELBLUE);
    draw.to_frame(app, &frame).unwrap(); 
}
