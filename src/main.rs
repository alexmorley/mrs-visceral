use std::panic;
use nannou::prelude::*;

struct Heart {
    points: Vec<(f32, f32)>, 
}

impl Heart {
    fn new() -> Heart {
        let points = (0..=360).map(|t_| {
            let t: f32 = deg_to_rad(t_ as f32);
            let x: f32 = 16.0 * t.sin().powi(3);
            let y: f32 = (13.0 * t.cos()) - (5.0 * (2.0*t).cos()) - (2.0 * (3.0*t).cos()) - (4.0*t).cos();
        (x, y)
        }).collect();
        Heart {
            points: points,
        }
    }

    fn scaled_points(&self, scale: f32) -> Vec<Point2> {
        self.points.iter().map(|(x, y)| {
            pt2(scale*x, scale*y)
        }).collect()
    }
}

struct Model {
    t: i32,
    heart: Heart,
    beat_t: f32, 
    beat_interval: f32, 
    beat_scale: f32,
}

impl Model {
    fn step(&mut self) {
        self.t += 1;
        self.beat_t = (
            self.beat_scale * 
            (((self.t as f32)/self.beat_interval).sin() + 1.0)/2.0) // 0 - 1 cycle
        + 0.1; // don't approach 0.
    }
}

fn main() {
    nannou::app(model)
        //.event(event)
        .update(update)
        .simple_window(view)
        .run();
}

fn model(_app: &App) -> Model {
    Model {
        t: 1500,
        heart: Heart::new(),
        beat_t: 0.1,
        beat_interval: 100.0,
        beat_scale: 20.0,
    }
}

//fn event(_app: &App, _model: &mut Model, _event: Event) {
//}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    _model.step()
}

fn view(_app: &App, _model: &Model, _frame: Frame) {
    // Prepare to draw.
    let draw = _app.draw();

    // Clear the background to purple.
    draw.background().color(PLUM);

    // Draw a blue ellipse with default size and position.
    let points = _model.heart.scaled_points(_model.beat_t);
    //println!("model.t {}", _model.t);
    //println!("scale {}", _model.beat_t);
    {
        draw.polygon()
        .color(STEELBLUE)
        .points(points);
    }

    
    /*{
    let points = (0..=_model.t).map(|t_| {
        let t: f32;
        let x: f32;
        let y: f32;
        t = (t_ as f32);
        x = 16.0 * t.sin().powi(3);
        y = (13.0 * t.cos()) - (5.0 * (2.0*t).cos()) - (2.0 * (3.0*t).cos()) - (4.0*t).cos();
        (pt2(scale*x, scale*y), PLUM)
    });
    draw.polyline()
        .weight(0.1)
        .colored_points(points);
    }*/

    // Write to the window frame.
    draw.to_frame(_app, &_frame).unwrap();
}
