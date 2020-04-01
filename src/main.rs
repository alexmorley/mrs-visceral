pub mod heart;
pub mod particle;

extern crate winit;

use heart::Heart;
use nannou::prelude::*;
use nannou::winit::window::WindowBuilder;
use particle::ParticleSystem;

const GLOBAL_FADE: f32 = 0.02;
const NEW_PARTICLES: f32 = 3.0;

struct Model {
    t: i32,
    heart: Heart,
    particle_system: ParticleSystem,
    container: Rect,
}

impl Model {
    fn step(&mut self) {
        self.t += 1;
        self.heart.scale = 20.0 + self.heart.beat.get(self.t);

        let mut f = (0.25 - self.heart.beat.ph).abs();
        f = if f > 0.2 {1.0 - f} else {0.0};
        self.particle_system.sources[0].modulate(f+0.5, f.max(0.1), f);
        self.particle_system.step();
    }
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(_app: &App) -> Model {
    _app.new_window()
        .window(WindowBuilder::new().with_maximized(true))
        .event(event)
        .view(view)
        .build()
        .unwrap();

    let (w, h) = _app.main_window().inner_size_points();
    let window_container = Rect::from_w_h(w as f32, h as f32);

    let heart = Heart::new();

    let heart_point = heart.scaled_points()[290];
    let rect = Rect::from_xy_wh(heart_point, Vector2::new(30.0, 30.0));

    let mut particle_system = ParticleSystem::new(window_container);
    particle_system.add_source(rect);

    Model {
        t: 0,
        heart: Heart::new(),
        particle_system: particle_system,
        container: rect,
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
    m.step();

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
    for particle in &m.particle_system.particles {
        particle.display(&draw);
    }

    // Draw a black heart with default size and position.
    m.heart.display(&draw);

    // Write to the window frame.
    draw.to_frame(_app, &_frame).unwrap();
}
