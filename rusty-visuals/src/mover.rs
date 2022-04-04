use nannou::prelude::*;

const FRICTION_CONSTANT: f32 = 0.3;

pub struct Mover {
    pub position: geom::Point2,
    rect: geom::Rect,
    velocity: Vec2,
    top_speed: f32,
    min_speed: f32,
    // inherent_force is some force that is exerted on the object
    // as a result of the inherent properties of the object.
    // Example: Helium's buoyancy
    pub mass: f32,
    inherent_force: Vec2,
    current_force: Vec2,
}

impl Mover {
    pub fn new(rect: geom::Rect) -> Self {
        let rand_x = random_range(rect.left(), rect.right());
        let rand_y = random_range(rect.top(), rect.bottom());
        let position = pt2(rand_x, rand_y);
        let velocity = vec2(0.0, 0.0);
        let top_speed = 4.0;
        let min_speed = -4.0;
        let mass = random_range(1.0, 10.0);
        let inherent_force = vec2(0.0, 0.0);
        let current_force = vec2(0.0, 0.0);
        Mover {
            position,
            rect,
            velocity,
            top_speed,
            min_speed,
            mass,
            inherent_force,
            current_force,
        }
    }

    pub fn new_with_inherent_force(rect: geom::Rect, inherent_force: Vec2) -> Self {
        let mut mover = Mover::new(rect);
        mover.inherent_force = inherent_force;
        mover
    }

    pub fn apply_force(&mut self, force: Vec2) {
        self.current_force += force;
    }

    pub fn apply_friction(&mut self) {
        let opposite_force = self.current_force * -1.0 * FRICTION_CONSTANT;
        self.apply_force(opposite_force);
    }

    pub fn update(&mut self) {
        self.current_force += self.inherent_force;
        self.velocity += self.current_force;
        self.velocity = vec2(
            self.velocity.x.min(self.top_speed).max(self.min_speed),
            self.velocity.y.min(self.top_speed).max(self.min_speed),
        );
        self.position += self.velocity;
        self.apply_friction(); 
        self.check_edges(self.rect);
    }

    fn check_edges(&mut self, rect: geom::Rect) {
        if self.position.x > rect.right() {
            self.position.x = rect.right();
        } else if self.position.x < rect.left() {
            self.position.x = rect.left();
        }
        if self.position.y > rect.top() {
            self.position.y = rect.top();
        } else if self.position.y < rect.bottom() {
            self.position.y = rect.bottom();
        }
    }

    pub fn display(&self, draw: &Draw) {
        // Display circle at x position
        draw.ellipse()
            .xy(self.position)
            .radius(self.mass * 3.0)
            .gray(0.5)
            .stroke(BLUE)
            .stroke_weight(2.0);
    }
}
