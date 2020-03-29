use nannou::app::Draw;
use nannou::prelude::*;

enum BeatKind {
    Sin,
    IntermittentSin,
}

pub struct Beat {
    interval: f32,
    scale: f32,
    kind: BeatKind,
    ph: f32,
}

impl Beat {
    pub fn get(&mut self, t: i32) -> f32 {
        let s_t = t as f32 / self.interval;
        match self.kind {
            BeatKind::Sin => self.sin(s_t),
            BeatKind::IntermittentSin=> {
                if (s_t.floor() as i32 % 3 == 0) {
                    self.ph += 1.0 / self.interval;
                    return self.sin(self.ph * 2.0 * std::f32::consts::PI)
                } else {
                    self.ph = 0.25;
                    return self.scale
                }
            }
        }
    }

    fn sin(&self, t :f32) -> f32 {
        self.scale *
        (t.sin() + 1.0)/2.0 // 0 - 1 cycle
        + 0.1 // don't approach 0.
    }
}

pub struct Heart {
    points: Vec<(f32, f32)>,
    pub scale: f32,
    pub beat: Beat,
}

impl Heart {
    pub fn new() -> Heart {
        let points = (0..=360)
            .map(|t_| {
                let t: f32 = deg_to_rad(t_ as f32);
                let x: f32 = 16.0 * t.sin().powi(3);
                let y: f32 = (13.0 * t.cos())
                    - (5.0 * (2.0 * t).cos())
                    - (2.0 * (3.0 * t).cos())
                    - (4.0 * t).cos();
                (x, y)
            })
            .collect();
        Heart {
            points: points,
            scale: 2.5,
            beat: Beat {
                interval: 30.0,
                scale: 5.0,
                kind: BeatKind::IntermittentSin,
                ph: 0.0,
            },
        }
    }

    pub fn scaled_points(&self) -> Vec<Point2> {
        self.points
            .iter()
            .map(|(x, y)| pt2(self.scale * x, self.scale * y))
            .collect()
    }

    pub fn display(&self, draw: &Draw) {
        let points = self.scaled_points();
        draw.polygon().color(BLACK).points(points);
    }
}
