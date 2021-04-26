use nannou::prelude::*;

pub struct Mover {
    pub position: geom::Point2,
    velocity: Vector2,
    top_speed: f32,
    min_speed: f32,
    // inherent_force is some force that is exerted on the object
    // as a result of the inherent properties of the object.
    // Example: Helium's buoyancy
    mass: f32,
    inherent_force: Vector2,
    current_force: Vector2,
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
            velocity,
            top_speed,
            min_speed,
            mass,
            inherent_force,
            current_force,
        }
    }

    pub fn new_with_inherent_force(rect: geom::Rect, inherent_force: Vector2) -> Self {
        let mut mover = Mover::new(rect);
        mover.inherent_force = inherent_force;
        mover
    }

    pub fn apply_force(&mut self, force: Vector2) {
        self.current_force += force / self.mass;
    }

    pub fn update(&mut self, rect: geom::Rect) {
        self.current_force += self.inherent_force;
        if self.check_edges(rect) {
            // Simulate bounce
            self.current_force *= -5.5;
        }
        self.velocity += self.current_force;
        self.velocity = vec2(
            self.velocity.x.min(self.top_speed).max(self.min_speed),
            self.velocity.y.min(self.top_speed).max(self.min_speed),
        );
        self.position += self.velocity;
        self.current_force = self.inherent_force;
    }

    fn check_edges(&mut self, rect: geom::Rect) -> bool {
        if self.position.x > rect.right() {
            self.position.x = rect.right();
            return true;
        // self.position.x = rect.left();
        } else if self.position.x < rect.left() {
            self.position.x = rect.left();
            return true;
        // self.position.x = rect.right();
        } else if self.position.y > rect.top() {
            self.position.y = rect.top();
            return true;
        // self.apply_force(self.current_force * -2.0);
        // self.position.y = rect.bottom();
        } else if self.position.y < rect.bottom() {
            self.position.y = rect.bottom();
            return true;
            // self.position.y = rect.top();
        }
        return false;
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
