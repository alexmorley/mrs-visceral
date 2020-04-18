use nannou::draw::Draw;
use nannou::prelude::*;

const UPDATE_PRECISION: f32 = 1.0; // [0.0, 1.0]
const BOUNCE_VELOCITY_GAIN: f32 = 0.01;
const LIFETIMES: i32 = 100000;

#[derive(Clone)]
pub struct Particle {
    lifetime: i32,
    position: Point2,
    velocity: Vector2,
    acceleration: Vector2,
    mass: f32,
    _lifetime_initial: i32,
}

#[derive(Clone)]
struct Modulation {
    size: f32,
    speed: f32,
    probability: f32,
}

impl Modulation {
    pub fn new() -> Modulation {
        Modulation{size: 1.0, speed: 1.0, probability: 1.0}
    }
}

#[derive(Clone)]
pub struct Source {
    extent: Rect,
    modulation: Modulation,
}

impl Source {
    pub fn new(extent: Rect) -> Source {
        Source {
            extent: extent,
            modulation: Modulation::new()
        }
    }

    pub fn modulate(&mut self, size: f32, speed: f32, probability: f32) {
        self.modulation.size = size;
        self.modulation.speed = speed;
        self.modulation.probability = probability;
    }

    pub fn get(&self, size: f32, speed: f32) -> Option<Particle> {
        if self.modulation.probability > random_range(0.0, 1.0) {
            let speed_f = speed * self.modulation.speed;
            return Some(Particle::new(
                size * self.modulation.size,
                random_range(self.extent.left(), self.extent.right()),
                random_range(self.extent.top(), self.extent.bottom()),
                LIFETIMES,
                Some(Vector2::new(-2.0 * speed_f, 1.0 * speed_f))
            ))
        }
        return None
    }
}

#[derive(Clone)]
pub struct ParticleSystem {
    pub particles: Vec<Particle>,
    pub sources: Vec<Source>,
    extent: Rect,
}

impl ParticleSystem {
    pub fn new(extent: Rect) -> Self {
        ParticleSystem {
            particles: Vec::<Particle>::new(),
            sources: Vec::<Source>::new(),
            extent: extent,
        }
    }

    pub fn add_source(&mut self, extent: Rect) {
        self.sources.push(Source::new(extent));
    }

    pub fn step(&mut self) {
        for i in 0..self.particles.len() {
            // Interactions with other particles
            if random_range(0.0, 1.0) < UPDATE_PRECISION {
                for j in 0..self.particles.len() {
                    if i != j {
                        let force = self.particles[j].repel(&self.particles[i]);
                        self.particles[i].apply_force(force);
                    }
                }
            }
            // Gravity
            let gravity = self.particles[i].gravity();
            self.particles[i].apply_force(gravity);

            // Update and bound (no deceleration);
            self.particles[i].update();
            self.particles[i]._check_edges(self.extent);
        }

        // Reap dead particles
        if LIFETIMES >= 0 {
            self.particles.retain(|p| p.lifetime > 0);
        }

        // Get new particles from sources
        for source in &self.sources {
            for _ in 0..10 {
                if let Some(p) = source.get(1.5, 1.0) {
                    self.particles.push(p);
                }
            }
        }
    }
}

impl Particle {
    pub fn new(m: f32, x: f32, y: f32, lifetime: i32, _velocity: Option<Vector2>) -> Self {
        let mass = m;
        let position = pt2(x, y);
        let velocity = _velocity.unwrap_or(vec2(-1.0, 1.0));
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
        let alpha: f32 = 1.0; //self.lifetime as f32 / self._lifetime_initial as f32;
        draw.ellipse()
            .xy(self.position)
            .w_h(self.mass*2.0, self.mass*2.0)
            .rgba(0.6, 0.0, 0.0, alpha);
    }

    pub fn repel(&self, m: &Particle) -> Vector2 {
        let mut force = self.position - m.position; // Calculate direction of force
        let mut distance = force.magnitude(); // Distance between objects
        distance = distance.max(5.0).min(10000.0); // Limiting the distance to eliminate "extreme" results for very cose or very far object
        if distance > 10.0 {
            return Vector2::zero();
        }
        force = force.normalize(); // Normalize vector (distance doesn't matter, we just want this vector for direction)
        let g = 1.0;
        let strength = (g * self.mass * m.mass) / (distance * distance); // Calculate gravitational force magnitude
        force * (0.5 * -1.0 * strength) // Get force vector --> magnitude * direction
    }

    pub fn gravity(&self) -> Vector2 {
        let force = Vector2::new(0.0, -1.0);
        let g = 0.01;
        let strength = g * self.mass;
        force * strength
    }

    pub fn _check_edges(&mut self, rect: Rect) {
        if self.position.x < rect.left() {
            self.position.x = rect.left();
            self.velocity.x *= -1.0 * BOUNCE_VELOCITY_GAIN;
        } else if self.position.x > rect.right() {
            self.position.x = rect.right();
            self.velocity.x *= -1.0 * BOUNCE_VELOCITY_GAIN;
        }
        if self.position.y > rect.top() {
            self.position.y = rect.top();
            self.velocity.y *= -1.0 * BOUNCE_VELOCITY_GAIN;
        } else if self.position.y < rect.bottom() {
            self.position.y = rect.bottom();
            self.velocity.y *= -1.0 * BOUNCE_VELOCITY_GAIN;
        }
    }
}
