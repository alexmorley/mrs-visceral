pub mod heart;
pub mod particle;

extern crate winit;

use heart::Heart;
use nannou::prelude::*;
use nannou::winit::window::WindowBuilder;
use particle::ParticleSystem;
use std::collections::VecDeque;

const GLOBAL_FADE: f32 = 0.02;
const NEW_PARTICLES: f32 = 3.0;

const ITERATIONS: i32 = 3000;

struct Model {
    states: VecDeque<ModelState>,
    current_state: Option<ModelState>
}

impl Model {
    fn run(&mut self, iter: i32) {
        for i in 0..iter {
            println!("iter: {}", i);
            let next_model_state = self.states.back().cloned();
            if let Some(mut model) = next_model_state{
                model.step();
                self.states.push_back(model);
            }
        }
    }

    fn next(&mut self) -> Option<ModelState> {
        self.states.pop_front()
    }
}


#[derive(Clone)]
struct ModelState {
    t: i32,
    heart: Heart,
    particle_system: ParticleSystem,
    container: Rect,
}

impl ModelState {
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
    nannou::app(initialize_model).update(update).run();
}

fn initialize_model(_app: &App) -> Model {
    // Start a maximized window
    _app.new_window()
        .window(WindowBuilder::new().with_maximized(false))
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

    let initial_model = ModelState {
        t: 0,
        heart: Heart::new(),
        particle_system: particle_system,
        container: window_container,
    };
    let mut model = Model {
        states: VecDeque::from(vec![initial_model]),
        current_state: None,
    };
    model.run(ITERATIONS);
    model
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.current_state = model.next();
}

fn view(_app: &App, model: &Model, _frame: Frame) {
    let draw = _app.draw();
    
    // Fade out everything a tiny bit.
    let (w, h) = _app.main_window().inner_size_points();
    draw.rect()
        .rgba(1.0, 1.0, 1.0, GLOBAL_FADE)
        .w(w)
        .h(h);

    if let Some(m) = &model.current_state {
        // Draw particles
        for particle in &m.particle_system.particles {
            particle.display(&draw);
        }

        // Draw a black heart with default size and position.
        m.heart.display(&draw);
    }

    // Write to the window frame.
    draw.to_frame(_app, &_frame).unwrap();
    
    if false {
        let mut file_path = _app.project_path()
            .expect("  ");
        file_path.push("output/img");
        file_path = file_path.join(format!("bleeding_{:06}", _frame.nth()))
            .with_extension("png");
        _app.main_window().capture_frame(file_path);
    }
}
