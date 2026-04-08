use crate::engine::animation::base::Animation;
use std::time::Duration;

// --- All (Parallel) ---
pub struct All {
    animations: Vec<Box<dyn Animation>>,
    finished: Vec<bool>,
}

impl All {
    pub fn new(animations: Vec<Box<dyn Animation>>) -> Self {
        let len = animations.len();
        Self {
            animations,
            finished: vec![false; len],
        }
    }
}

impl Animation for All {
    fn update(&mut self, dt: Duration) -> (bool, Duration) {
        let mut all_finished = true;
        let mut min_leftover = dt;
        for i in 0..self.animations.len() {
            if self.finished[i] {
                continue;
            }
            let (finished, leftover) = self.animations[i].update(dt);
            if finished {
                self.finished[i] = true;
                if leftover < min_leftover {
                    min_leftover = leftover;
                }
            } else {
                all_finished = false;
            }
        }
        (all_finished, if all_finished { min_leftover } else { Duration::ZERO })
    }

    fn duration(&self) -> Duration {
        self.animations
            .iter()
            .map(|a| a.duration())
            .max()
            .unwrap_or(Duration::ZERO)
    }

    fn set_easing(&mut self, easing: fn(f32) -> f32) {
        for anim in &mut self.animations {
            anim.set_easing(easing);
        }
    }
}

// --- Any (Race) ---
pub struct Any {
    animations: Vec<Box<dyn Animation>>,
}

impl Any {
    pub fn new(animations: Vec<Box<dyn Animation>>) -> Self {
        Self { animations }
    }
}

impl Animation for Any {
    fn update(&mut self, dt: Duration) -> (bool, Duration) {
        let mut any_finished = false;
        let mut max_leftover = Duration::ZERO;
        for anim in &mut self.animations {
            let (finished, leftover) = anim.update(dt);
            if finished {
                any_finished = true;
                if leftover > max_leftover {
                    max_leftover = leftover;
                }
            }
        }
        (any_finished, if any_finished { max_leftover } else { Duration::ZERO })
    }

    fn duration(&self) -> Duration {
        self.animations
            .iter()
            .map(|a| a.duration())
            .min()
            .unwrap_or(Duration::ZERO)
    }

    fn set_easing(&mut self, easing: fn(f32) -> f32) {
        for anim in &mut self.animations {
            anim.set_easing(easing);
        }
    }
}

// --- Chain (Sequential) ---
pub struct Chain {
    animations: Vec<Box<dyn Animation>>,
    index: usize,
}

impl Chain {
    pub fn new(animations: Vec<Box<dyn Animation>>) -> Self {
        Self {
            animations,
            index: 0,
        }
    }
}

impl Animation for Chain {
    fn update(&mut self, mut dt: Duration) -> (bool, Duration) {
        while self.index < self.animations.len() {
            let (finished, leftover) = self.animations[self.index].update(dt);
            if finished {
                self.index += 1;
                dt = leftover;
                if dt == Duration::ZERO && self.index < self.animations.len() {
                    // Even if dt is zero, we might want to move to the next one if it's zero-duration
                    // but usually we just stop here for the frame.
                    return (false, Duration::ZERO);
                }
            } else {
                return (false, Duration::ZERO);
            }
        }
        (true, dt)
    }

    fn duration(&self) -> Duration {
        self.animations
            .iter()
            .map(|a| a.duration())
            .fold(Duration::ZERO, |acc, d| acc + d)
    }

    fn set_easing(&mut self, easing: fn(f32) -> f32) {
        for anim in &mut self.animations {
            anim.set_easing(easing);
        }
    }
}

// --- Delay ---
pub struct Delay {
    duration: Duration,
    elapsed: Duration,
    inner: Box<dyn Animation>,
}

impl Delay {
    pub fn new(duration: Duration, inner: Box<dyn Animation>) -> Self {
        Self {
            duration,
            elapsed: Duration::ZERO,
            inner,
        }
    }
}

impl Animation for Delay {
    fn update(&mut self, dt: Duration) -> (bool, Duration) {
        if self.elapsed < self.duration {
            self.elapsed += dt;
            if self.elapsed >= self.duration {
                let leftover = self.elapsed - self.duration;
                self.inner.update(leftover)
            } else {
                (false, Duration::ZERO)
            }
        } else {
            self.inner.update(dt)
        }
    }

    fn duration(&self) -> Duration {
        self.duration + self.inner.duration()
    }

    fn set_easing(&mut self, easing: fn(f32) -> f32) {
        self.inner.set_easing(easing);
    }
}

// --- Sequence (Staggered Parallel) ---
pub struct Sequence {
    items: Vec<(Duration, Box<dyn Animation>)>,
    finished: Vec<bool>,
    elapsed: Duration,
}

impl Sequence {
    pub fn new(stagger: Duration, animations: Vec<Box<dyn Animation>>) -> Self {
        let len = animations.len();
        let items = animations
            .into_iter()
            .enumerate()
            .map(|(i, anim)| (stagger * i as u32, anim))
            .collect();
        Self {
            items,
            finished: vec![false; len],
            elapsed: Duration::ZERO,
        }
    }
}

impl Animation for Sequence {
    fn update(&mut self, dt: Duration) -> (bool, Duration) {
        self.elapsed += dt;
        let mut all_finished = true;
        let mut min_leftover = dt;
        for i in 0..self.items.len() {
            if self.finished[i] {
                continue;
            }
            let (start_time, anim) = &mut self.items[i];
            if self.elapsed >= *start_time {
                // Calculate how much dt actually applied to this animation
                // if it just started in this frame
                let effective_dt = if self.elapsed - dt < *start_time {
                    self.elapsed - *start_time
                } else {
                    dt
                };

                let (finished, leftover) = anim.update(effective_dt);
                if finished {
                    self.finished[i] = true;
                    if leftover < min_leftover {
                        min_leftover = leftover;
                    }
                } else {
                    all_finished = false;
                }
            } else {
                all_finished = false;
            }
        }

        let total_finished = all_finished && self.elapsed >= self.duration();
        let final_leftover = if total_finished {
            let dur = self.duration();
            if self.elapsed > dur { self.elapsed - dur } else { Duration::ZERO }
        } else {
            Duration::ZERO
        };

        (total_finished, final_leftover)
    }

    fn duration(&self) -> Duration {
        self.items
            .iter()
            .map(|(start, anim)| *start + anim.duration())
            .max()
            .unwrap_or(Duration::ZERO)
    }

    fn set_easing(&mut self, easing: fn(f32) -> f32) {
        for (_, anim) in &mut self.items {
            anim.set_easing(easing);
        }
    }
}

// --- Loop ---
pub struct LoopAnim {
    factory: Box<dyn Fn() -> Box<dyn Animation> + Send + Sync>,
    current: Box<dyn Animation>,
    repeat_count: Option<usize>, // None for infinity
    finished_count: usize,
}

impl LoopAnim {
    pub fn new(
        factory: Box<dyn Fn() -> Box<dyn Animation> + Send + Sync>,
        count: Option<usize>,
    ) -> Self {
        let current = factory();
        Self {
            factory,
            current,
            repeat_count: count,
            finished_count: 0,
        }
    }
}

impl Animation for LoopAnim {
    fn update(&mut self, mut dt: Duration) -> (bool, Duration) {
        loop {
            let (finished, leftover) = self.current.update(dt);
            if finished {
                self.finished_count += 1;

                if let Some(max) = self.repeat_count {
                    if self.finished_count >= max {
                        return (true, leftover);
                    }
                }

                // Restart
                self.current = (self.factory)();
                dt = leftover;
                if dt == Duration::ZERO {
                    return (false, Duration::ZERO);
                }
            } else {
                return (false, Duration::ZERO);
            }
        }
    }

    fn duration(&self) -> Duration {
        match self.repeat_count {
            Some(count) => self.current.duration() * count as u32,
            None => Duration::from_secs(3600), // Cap infinite at 1 hour for progress bar
        }
    }
}

// --- Factory Functions ---
pub fn all(animations: Vec<Box<dyn Animation>>) -> Box<dyn Animation> {
    Box::new(All::new(animations))
}

pub fn any(animations: Vec<Box<dyn Animation>>) -> Box<dyn Animation> {
    Box::new(Any::new(animations))
}

pub fn chain(animations: Vec<Box<dyn Animation>>) -> Box<dyn Animation> {
    Box::new(Chain::new(animations))
}

pub fn delay(duration: Duration, inner: Box<dyn Animation>) -> Box<dyn Animation> {
    Box::new(Delay::new(duration, inner))
}

pub fn sequence(stagger: Duration, animations: Vec<Box<dyn Animation>>) -> Box<dyn Animation> {
    Box::new(Sequence::new(stagger, animations))
}

pub struct Wait {
    duration: Duration,
    elapsed: Duration,
}

impl Animation for Wait {
    fn update(&mut self, dt: Duration) -> (bool, Duration) {
        self.elapsed += dt;
        let finished = self.elapsed >= self.duration;
        let leftover = if finished { self.elapsed - self.duration } else { Duration::ZERO };
        (finished, leftover)
    }

    fn duration(&self) -> Duration {
        self.duration
    }
}

pub fn wait(duration: Duration) -> Box<dyn Animation> {
    Box::new(Wait {
        duration,
        elapsed: Duration::ZERO,
    })
}

pub fn loop_anim<F>(factory: F, count: Option<usize>) -> Box<dyn Animation>
where
    F: Fn() -> Box<dyn Animation> + Send + Sync + 'static,
{
    Box::new(LoopAnim::new(Box::new(factory), count))
}

pub fn with_easing(easing: fn(f32) -> f32, animations: Vec<Box<dyn Animation>>) -> Box<dyn Animation> {
    let mut all = All::new(animations);
    all.set_easing(easing);
    Box::new(all)
}
