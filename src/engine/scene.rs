use crate::engine::animation::Node;
use vello::Scene;

pub trait Scene2D {
    fn render(&self, scene: &mut Scene);
    fn update(&mut self, dt: std::time::Duration);
    fn state_hash(&self) -> u64;
}

pub struct BaseScene {
    pub nodes: Vec<Box<dyn Node>>,
    pub video_timeline: crate::engine::animation::Timeline,
    #[cfg(feature = "audio")]
    pub audio_timeline: crate::engine::animation::Timeline,
}

impl BaseScene {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            video_timeline: crate::engine::animation::Timeline::new(),
            #[cfg(feature = "audio")]
            audio_timeline: crate::engine::animation::Timeline::new(),
        }
    }

    pub fn add(&mut self, node: Box<dyn Node>) {
        self.nodes.push(node);
    }

    #[cfg(feature = "audio")]
    pub fn collect_audio_events(
        &mut self,
        current_time: std::time::Duration,
        events: &mut Vec<crate::engine::animation::base::AudioEvent>,
    ) {
        self.audio_timeline
            .collect_audio_events(current_time, events);
    }
}

impl Scene2D for BaseScene {
    fn render(&self, scene: &mut Scene) {
        for node in &self.nodes {
            node.render(scene, vello::kurbo::Affine::IDENTITY, 1.0);
        }
    }

    fn update(&mut self, dt: std::time::Duration) {
        self.video_timeline.update(dt);
        #[cfg(feature = "audio")]
        self.audio_timeline.update(dt);
        for node in &mut self.nodes {
            node.update(dt);
        }
    }

    fn state_hash(&self) -> u64 {
        use rayon::prelude::*;
        self.nodes
            .par_iter()
            .enumerate()
            .map(|(i, node)| crate::engine::util::hash::combine_hashes(node.state_hash(), i as u64))
            .reduce(|| 0u64, |a, b| a.wrapping_add(b))
    }
}
