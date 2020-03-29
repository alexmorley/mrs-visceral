use nannou::draw::Draw;
use nannou::prelude::*;

pub struct Particle {
    position: Point2,
    velocity: Vector2,
    acceleration: Vector2,
    mass: f32,
    pub lifetime: i32,
    _lifetime_initial: i32,
}

impl Particle {
    pub fn new(m: f32, x: f32, y: f32, lifetime: i32) -> Self {
        let mass = m;
        let position = pt2(x, y);
        let velocity = vec2(0.0, 0.0);
        let acceleration = vec2(0.0, 0.0);
        Particle {
            position,
            velocity,
            acceleration,
            mass,
            lifetime,
            _lifetime_initial: lifetime,
        }
    }

    pub fn apply_force(&mut self, force: Vector2) {
        let f = force / self.mass;
        self.acceleration += f;
    }

    pub fn update(&mut self) {
        self.lifetime -= 1;
        self.velocity += self.acceleration;
        self.position += self.velocity;
        self.acceleration *= 0.0;
    }

    pub fn display(&self, draw: &Draw) {
        let alpha: f32 = self.lifetime as f32 / self._lifetime_initial as f32;
        draw.ellipse()
            .xy(self.position)
            .w_h(self.mass, self.mass)
            .rgba(0.6, 0.6, 0.6, alpha);
    }

    pub fn repel(&self, m: &Particle) -> Vector2 {
        let mut force = self.position - m.position; // Calculate direction of force
        let mut distance = force.magnitude(); // Distance between objects
        distance = distance.max(10.0).min(10000.0); // Limiting the distance to eliminate "extreme" results for very cose or very far object
        if distance > 100.0 {
            return Vector2::zero();
        }
        force = force.normalize(); // Normalize vector (distance doesn't matter, we just want this vector for direction)
        let g = 1.0;
        let strength = (g * self.mass * m.mass) / (distance * distance); // Calculate gravitational force magnitude
        force * (-1.0 * strength) // Get force vector --> magnitude * direction
    }

    pub fn _check_edges(&mut self, rect: Rect) {
        if self.position.x < rect.left() {
            self.position.x = rect.left();
            self.velocity.x *= -1.0;
        } else if self.position.x > rect.right() {
            self.position.x = rect.right();
            self.velocity.x *= -1.0;
        }
        if self.position.y > rect.top() {
            self.position.y = rect.top();
            self.velocity.y *= -1.0;
        } else if self.position.y < rect.bottom() {
            self.position.y = rect.bottom();
            self.velocity.y *= -1.0;
        }
    }
}
