use crate::core::animation::Node;
use kurbo::Affine;
#[cfg(feature = "runtime")]
use vello::Scene;

/// A trait for objects that can manage a 2D visual scene.
pub trait Scene2D {
    /// Renders all elements of the scene into a Vello scene.
    #[cfg(feature = "runtime")]
    fn render(&self, scene: &mut Scene);
    /// Advances the state of the scene by the given delta time.
    fn update(&mut self, dt: std::time::Duration);
    /// Returns a hash representing the current visual state of the entire scene.
    fn state_hash(&self) -> u64;
    /// Check if scene is dirty (has visual updates since last frame).
    fn is_dirty(&self) -> bool {
        true
    }
    /// Set the dirty flag.
    fn set_dirty(&mut self, _dirty: bool) {}
}

/// The standard implementation of a 2D scene, containing a collection of nodes
/// and timelines for video and audio animations.
pub struct BaseScene {
    /// The collection of visual nodes in the scene.
    pub nodes: Vec<Box<dyn Node>>,
    /// The primary timeline for video animations.
    pub video_timeline: crate::core::Timeline,
    /// The separate timeline for purely audio events.
    #[cfg(feature = "audio")]
    pub audio_timeline: crate::core::Timeline,
    // New fields:
}

impl BaseScene {
    /// Creates a new empty scene.
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            video_timeline: crate::core::Timeline::new(),
            #[cfg(feature = "audio")]
            audio_timeline: crate::core::Timeline::new(),
        }
    }

    /// Adds a node to the scene.
    pub fn add(&mut self, node: Box<dyn Node>) {
        self.nodes.push(node);
    }

    /// Resets all timelines and nodes within the scene to their initial state.
    pub fn reset(&mut self) {
        self.video_timeline.reset();
        #[cfg(feature = "audio")]
        self.audio_timeline.reset();
        for node in &mut self.nodes {
            node.reset();
        }
    }

    /// Recursively collects audio events from all timelines.
    #[cfg(feature = "audio")]
    pub fn collect_audio_events(
        &mut self,
        current_time: std::time::Duration,
        events: &mut Vec<crate::core::animation::base::AudioEvent>,
    ) {
        self.video_timeline
            .collect_audio_events(current_time, events);
        self.audio_timeline
            .collect_audio_events(current_time, events);
    }
}

impl Scene2D for BaseScene {
    #[cfg(feature = "runtime")]
    fn render(&self, scene: &mut Scene) {
        for node in &self.nodes {
            node.render(scene, Affine::IDENTITY, 1.0);
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
            .map(|(i, node)| crate::assets::hash::combine_hashes(node.state_hash(), i as u64))
            .reduce(|| 0u64, |a, b| a.wrapping_add(b))
    }
}
