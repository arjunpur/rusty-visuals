use nannou::geom::{pt2, Point2};
use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(600, 600)
        .run();
}

struct Model {
    v1: Point2,
    v2: Point2,
    mouse: Point2,
}

impl Model {
    fn add(&self) -> Vec2 {
        self.v1 + self.v2
    }

    fn magnitudes(&self) -> (f32, f32) {
        let m1 = ((self.v1.x - self.mouse.x).pow(2.0) + (self.v1.y - self.mouse.y).pow(2.0)).sqrt();
        let m2 = ((self.v2.x - self.mouse.x).pow(2.0) + (self.v2.y - self.mouse.y).pow(2.0)).sqrt();
        (m1, m2)
    }
}

fn model(app: &App) -> Model {
    let v1 = pt2(50.0, 50.0);
    let v2 = pt2(100.0, 200.0);
    let mouse = vec2(app.mouse.x, app.mouse.y);
    Model { v1, v2, mouse }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let mouse = vec2(app.mouse.x, app.mouse.y);
    model.mouse = mouse;
    model.add();
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();
    let window_rect = app.window_rect();
    draw.background().color(WHITE);

    let (v1_magnitude, v2_magnitude) = model.magnitudes();

    // Draw two lines connceting v1, v2 and mouse
    draw.line()
        .points(model.v1, model.mouse)
        .color(BLUE)
        .weight(2.0);
    draw.line()
        .points(model.v2, model.mouse)
        .color(BLUE)
        .weight(2.0);

    // Draw two rectangles that represent the magnitude of the vector
    // created between v1 and the mouse, and v2 and the mouse.
    draw.rect()
        .wh(vec2(20.0, v1_magnitude))
        .xy(pt2(window_rect.left(), window_rect.bottom()))
        .color(PURPLE);

    draw.rect()
        .wh(vec2(20.0, v2_magnitude))
        .xy(pt2(window_rect.right(), window_rect.bottom()))
        .color(GREEN);

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
