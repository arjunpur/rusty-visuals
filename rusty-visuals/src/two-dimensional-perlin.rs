use itertools::Itertools;
use nannou::geom;
use nannou::geom::path;
use nannou::geom::quad;
use nannou::mesh;
use nannou::noise;
use nannou::prelude::*;
use rand::Rng;

fn main() {
    nannou::sketch(two_dimensional_perlin).run()
}

fn two_dimensional_perlin(app: &App, frame: Frame) {
    // Begin drawing
    let draw = app.draw();
    draw.background().color(SKYBLUE);
    let rect = app.window_rect();

    // let perlin = noise::Perlin.new();

    let min_rect_w = rect.w() / 300.0;
    let min_rect_h = rect.h() / 300.0;

    let a = pt2(0.0, 0.0);
    let b = pt2(0.0, 0.0);
    let c = pt2(0.0, 0.0);
    let d = pt2(0.0, 0.0);

    let points = (1..rect.w() as usize).cartesian_product(1..rect.h() as usize);
    let tris = points.flat_map(|tup| geom::Quad([a, b, c, d]).triangles_iter());
    draw.mesh().tris(tris);
    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
