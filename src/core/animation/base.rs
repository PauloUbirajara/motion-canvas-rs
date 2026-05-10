use std::time::Duration;
#[cfg(feature = "runtime")]
use vello::Scene;
#[cfg(feature = "runtime")]
use kurbo::Affine;

/// Represents an audio playback event on the project timeline.
#[derive(Clone, Debug)]
pub struct AudioEvent {
    /// Filesystem path to the audio file.
    pub path: String,
    /// Volume multiplier.
    pub volume: f32,
    /// Duration to skip from the start of the source file.
    pub start_crop: Duration,
    /// Duration to ignore at the end of the source file.
    pub end_crop: Duration,
    /// The timestamp on the project timeline when playback should begin.
    pub start_time: Duration,
}

/// The core trait for all time-based animations in the engine.
///
/// Types implementing `Animation` represent a specific duration of time during 
/// which state changes or side-effects (like audio) occur.
pub trait Animation: Send + Sync + 'static {
    /// Advances the animation by the given delta time.
    ///
    /// Returns a tuple containing:
    /// 1. A boolean indicating if the animation has finished.
    /// 2. The amount of "leftover" delta time if it finished mid-update.
    fn update(&mut self, dt: Duration) -> (bool, Duration);

    /// Returns the total predicted duration of the animation.
    fn duration(&self) -> Duration;

    /// Optionally applies an easing function to the animation progress.
    fn set_easing(&mut self, _easing: fn(f32) -> f32) {}

    /// Recursively collects all audio events scheduled by this animation.
    fn collect_audio_events(&mut self, _current_time: Duration, _events: &mut Vec<AudioEvent>) {}

    /// Resets the animation to its initial state.
    fn reset(&mut self);
}

impl<T: ?Sized + Animation> Animation for Box<T> {
    fn update(&mut self, dt: Duration) -> (bool, Duration) {
        (**self).update(dt)
    }
    fn duration(&self) -> Duration {
        (**self).duration()
    }
    fn set_easing(&mut self, easing: fn(f32) -> f32) {
        (**self).set_easing(easing)
    }
    fn collect_audio_events(&mut self, current_time: Duration, events: &mut Vec<AudioEvent>) {
        (**self).collect_audio_events(current_time, events)
    }
    fn reset(&mut self) {
        (**self).reset()
    }
}

/// The core trait for visual elements that can be rendered to a scene.
///
/// A `Node` represents a visual object with its own state and rendering logic.
/// Nodes are typically stored in `Scene2D`.
pub trait Node: Send + Sync + 'static {
    /// Renders the node into the provided Vello scene.
    #[cfg(feature = "runtime")]
    fn render(
        &self,
        vello_scene: &mut Scene,
        parent_transform: Affine,
        parent_opacity: f32,
    );

    /// Updates the node's internal state (e.g., for procedural animations).
    fn update(&mut self, dt: Duration);

    /// Returns a hash representing the current visual state.
    /// Used by the renderer to determine if the scene needs to be re-recorded.
    fn state_hash(&self) -> u64;

    /// Creates a boxed clone of the node.
    fn clone_node(&self) -> Box<dyn Node>;

    /// Resets the node's signals to their initial values.
    fn reset(&mut self);
}
