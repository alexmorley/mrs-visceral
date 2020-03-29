pub mod heart;
pub mod particle;

use heart::Heart;
use nannou::prelude::*;
use particle::Particle;

const GLOBAL_FADE: f32 = 0.00025;
const LIFETIMES: i32 = 250;
const NEW_PARTICLES: bool = true;

struct Model {
    t: i32,
    heart: Heart,
    particles: Vec<Particle>,
    container: Rect,
    source: Rect,
}

impl Model {
    fn step(&mut self) {
        self.t += 1;
        self.heart.scale = self.heart.beat.get(self.t);
    }
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(_app: &App) -> Model {
    _app.new_window().event(event).view(view).build().unwrap();

    _app.main_window().set_maximized(true);

    let rect = Rect::from_w_h(640.0, 400.0);

    let particles = (0..500)
        .map(|_| {
            Particle::new(
                2.0 as f32,
                random_range(rect.left(), rect.right()),
                random_range(rect.top(), rect.bottom()),
                LIFETIMES,
            )
        })
        .collect();

    Model {
        t: 0,
        heart: Heart::new(),
        particles: particles,
        container: rect,
        source: rect,
    }
}

fn event(_app: &App, m: &mut Model, event: WindowEvent) {
    match event {
        Resized(_) => {
            let (w, h) = _app.main_window().inner_size_points();
            let rect = Rect::from_w_h(w as f32, h as f32);
            m.container = rect;
        }
        _other => (),
    }
}

fn update(_app: &App, m: &mut Model, _update: Update) {
    if m.t == 30 {
        let (w, h) = _app.main_window().inner_size_points();
        let rect = Rect::from_w_h(w as f32, h as f32);
        m.container = rect;
    }

    m.step();

    for i in 0..m.particles.len() {
        for j in 0..m.particles.len() {
            if i != j {
                let force = m.particles[j].repel(&m.particles[i]);
                m.particles[i].apply_force(force);
            }
        }
        m.particles[i].update();
        m.particles[i]._check_edges(m.container);
    }
    if LIFETIMES >= 0 {
        m.particles.retain(|p| p.lifetime > 0);
    }

    if NEW_PARTICLES {
        let new_particles: Vec<Particle> = (0..1)
            .map(|_| {
                Particle::new(
                    2.0 as f32,
                    random_range(m.source.left(), m.source.right()),
                    random_range(m.source.top(), m.source.bottom()),
                    1000,
                )
            })
            .collect();
        m.particles.extend(new_particles);
    }
}

fn view(_app: &App, m: &Model, _frame: Frame) {
    // Prepare to draw.
    let draw = _app.draw();

    // Fade out everything a tiny bit.
    draw.rect()
        .rgba(1.0, 1.0, 1.0, GLOBAL_FADE)
        .w(m.container.w())
        .h(m.container.h());

    // Draw particles
    for particle in &m.particles {
        particle.display(&draw);
    }

    // Draw a black heart with default size and position.
    m.heart.display(&draw);

    // Write to the window frame.
    draw.to_frame(_app, &_frame).unwrap();
}
