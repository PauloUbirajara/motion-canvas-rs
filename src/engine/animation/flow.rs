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
    fn update(&mut self, dt: Duration) -> bool {
        let mut all_finished = true;
        for i in 0..self.animations.len() {
            if self.finished[i] {
                continue;
            }
            if self.animations[i].update(dt) {
                self.finished[i] = true;
            } else {
                all_finished = false;
            }
        }
        all_finished
    }

    fn duration(&self) -> Duration {
        self.animations
            .iter()
            .map(|a| a.duration())
            .max()
            .unwrap_or(Duration::ZERO)
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
    fn update(&mut self, dt: Duration) -> bool {
        let mut any_finished = false;
        for anim in &mut self.animations {
            if anim.update(dt) {
                any_finished = true;
            }
        }
        any_finished
    }

    fn duration(&self) -> Duration {
        self.animations
            .iter()
            .map(|a| a.duration())
            .min()
            .unwrap_or(Duration::ZERO)
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
    fn update(&mut self, dt: Duration) -> bool {
        if self.index >= self.animations.len() {
            return true;
        }

        if self.animations[self.index].update(dt) {
            self.index += 1;
        }

        self.index >= self.animations.len()
    }

    fn duration(&self) -> Duration {
        self.animations
            .iter()
            .map(|a| a.duration())
            .fold(Duration::ZERO, |acc, d| acc + d)
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
    fn update(&mut self, dt: Duration) -> bool {
        if self.elapsed < self.duration {
            self.elapsed += dt;
            false
        } else {
            self.inner.update(dt)
        }
    }

    fn duration(&self) -> Duration {
        self.duration + self.inner.duration()
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
    fn update(&mut self, dt: Duration) -> bool {
        self.elapsed += dt;
        let mut all_finished = true;
        for i in 0..self.items.len() {
            if self.finished[i] {
                continue;
            }
            let (start_time, anim) = &mut self.items[i];
            if self.elapsed >= *start_time {
                if anim.update(dt) {
                    self.finished[i] = true;
                } else {
                    all_finished = false;
                }
            } else {
                all_finished = false;
            }
        }
        all_finished
    }

    fn duration(&self) -> Duration {
        self.items
            .iter()
            .map(|(start, anim)| *start + anim.duration())
            .max()
            .unwrap_or(Duration::ZERO)
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
    fn update(&mut self, dt: Duration) -> bool {
        if self.current.update(dt) {
            self.finished_count += 1;

            if let Some(max) = self.repeat_count {
                if self.finished_count >= max {
                    return true;
                }
            }

            // Restart
            self.current = (self.factory)();
        }
        false
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
    fn update(&mut self, dt: Duration) -> bool {
        self.elapsed += dt;
        self.elapsed >= self.duration
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
