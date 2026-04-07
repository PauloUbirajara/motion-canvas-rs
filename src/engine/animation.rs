use std::time::Duration;

pub struct Animation {
    pub duration: Duration,
    pub elapsed: Duration,
    pub is_finished: bool,
}

impl Animation {
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            elapsed: Duration::ZERO,
            is_finished: false,
        }
    }

    pub fn update(&mut self, dt: Duration) -> f32 {
        if self.is_finished {
            return 1.0;
        }

        self.elapsed += dt;
        if self.elapsed >= self.duration {
            self.elapsed = self.duration;
            self.is_finished = true;
        }

        self.elapsed.as_secs_f32() / self.duration.as_secs_f32()
    }
}

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

// Ease functions can be added here
pub fn ease_in_out(t: f32) -> f32 {
    if t < 0.5 {
        2.0 * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
    }
}
