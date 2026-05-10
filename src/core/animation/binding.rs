use crate::core::animation::base::Node;
use crate::core::animation::tween::{Signal, Tweenable};
use std::time::Duration;
#[cfg(feature = "runtime")]
use vello::Scene;
#[cfg(feature = "runtime")]
use kurbo::Affine;

/// A logical node that synchronizes one signal to another using a mapping function.
///
/// `BindingNode` is a non-visual node that facilitates reactive data flow between elements.
/// For example, you can use a binding to ensure a label always displays the current 
/// position of a moving circle.
pub struct BindingNode<T, S>
where
    T: Tweenable + PartialEq,
    S: Tweenable + PartialEq,
{
    source: Signal<S>,
    target: Signal<T>,
    mapper: Box<dyn Fn(S) -> T + Send + Sync + 'static>,
}

impl<T, S> BindingNode<T, S>
where
    T: Tweenable + PartialEq,
    S: Tweenable + PartialEq,
{
    /// Creates a new binding that maps values from the `source` signal to the `target` signal.
    ///
    /// Every update cycle, the `mapper` function is called with the current value of `source`,
    /// and the result is applied to `target`.
    pub fn new(source: Signal<S>, target: Signal<T>, mapper: impl Fn(S) -> T + Send + Sync + 'static) -> Self {
        Self {
            source,
            target,
            mapper: Box::new(mapper),
        }
    }
}

impl<T, S> Node for BindingNode<T, S>
where
    T: Tweenable + PartialEq,
    S: Tweenable + PartialEq,
{
    #[cfg(feature = "runtime")]
    fn render(&self, _scene: &mut Scene, _transform: Affine, _opacity: f32) {
        // Bindings are purely logical, no rendering
    }

    fn update(&mut self, _dt: Duration) {
        let source_val = self.source.get();
        let mapped_val = (self.mapper)(source_val);
        self.target.set(mapped_val);
    }

    fn state_hash(&self) -> u64 {
        // Bindings drive other signals, their own state is just their source/target's state
        0
    }

    fn clone_node(&self) -> Box<dyn Node> {
        // This is hard to clone due to the closure, but usually we don't clone bindings.
        // If needed, we'd need an Arc for the mapper.
        panic!("BindingNode cloning is not implemented");
    }

    fn reset(&mut self) {
        // Target signal is reset by its owner node
    }
}
