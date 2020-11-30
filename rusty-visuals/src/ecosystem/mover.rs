use nannou::prelude::*;

pub struct Mover {
    position: geom::Point2,
    velocity: Vector2,
    acceleration: Vector2,
    top_speed: f32,
}

impl Mover {
    pub fn new(rect: geom::Rect) -> Self {
        let rand_x = random_range(rect.left(), rect.right());
        let rand_y = random_range(rect.top(), rect.bottom());
        let position = pt2(rand_x, rand_y);
        let velocity = vec2(0.0, 0.0);
        let acceleration = vec2(0.01, 0.01);
        let top_speed = 2.0;
        Mover {
            position,
            velocity,
            acceleration,
            top_speed,
        }
    }

    pub fn update(&mut self, rect: geom::Rect) {
        self.velocity += self.acceleration;
        self.velocity = vec2(
            self.velocity.x.min(self.top_speed),
            self.velocity.y.min(self.top_speed),
        );
        self.position += self.velocity;
        self.check_edges(rect);
    }
    fn check_edges(&mut self, rect: geom::Rect) {
        if self.position.x > rect.right() {
            self.position.x = rect.left();
        } else if self.position.x < rect.left() {
            self.position.x = rect.right();
        } else if self.position.y > rect.top() {
            self.position.y = rect.bottom();
        } else if self.position.y < rect.bottom() {
            self.position.y = rect.top();
        }
    }

    pub fn display(&self, draw: &Draw) {
        // Display circle at x position
        draw.ellipse()
            .xy(self.position)
            .w_h(48.0, 48.0)
            .gray(0.5)
            .stroke(BLUE)
            .stroke_weight(2.0);
    }
}
