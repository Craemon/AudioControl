use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct Mixer {
    last_value: HashMap<usize, f32>,
    last_time: HashMap<usize, Instant>,
    interval: Duration,
    smooth_factor: f32,
}

impl Mixer {
    pub fn new(interval_ms: u64, smooth_factor: f32) -> Self {
        Self {
            last_value: HashMap::new(),
            last_time: HashMap::new(),
            interval: Duration::from_millis(interval_ms),
            smooth_factor,
        }
    }

    pub fn update(&mut self, id: usize, raw: f32) -> Option<f32> {
        let now = Instant::now();

        let allow = match self.last_time.get(&id) {
            Some(t) => t.elapsed() >= self.interval,
            None => true,
        };

        if !allow {
            return None;
        }

        let value = match self.last_value.get(&id) {
            Some(prev) => smooth(*prev, raw, self.smooth_factor),
            None => raw,
        }.clamp(0.0, 1.0);

        self.last_value.insert(id, value);
        self.last_time.insert(id, now);

        Some(value)
    }
}
fn smooth(prev: f32, next: f32, factor: f32) -> f32 {
    prev + (next - prev) * factor
}