use crate::engine::animation::base::Node;
use crate::engine::animation::tween::{Signal, Tweenable};
use std::time::Duration;
use vello::kurbo::Affine;
use vello::Scene;

/// A node that synchronizes two signals using a mapping function.
pub struct BindingNode<T, S> {
    source: Signal<S>,
    target: Signal<T>,
    mapper: Box<dyn Fn(S) -> T + Send + Sync>,
}

impl<T: Tweenable + PartialEq, S: Tweenable + PartialEq> BindingNode<T, S> {
    pub fn new<F>(source: Signal<S>, target: Signal<T>, mapper: F) -> Self
    where
        F: Fn(S) -> T + Send + Sync + 'static,
    {
        // Initial sync
        target.set(mapper(source.get()));

        Self {
            source,
            target,
            mapper: Box::new(mapper),
        }
    }
}

impl<T: Tweenable + PartialEq, S: Tweenable + PartialEq> Node for BindingNode<T, S> {
    fn render(&self, _scene: &mut Scene, _parent_transform: Affine, _parent_opacity: f32) {
        // Redraw sync
        let current = self.source.get();
        let desired = (self.mapper)(current);
        if self.target.get() != desired {
            self.target.set(desired);
        }
    }

    fn update(&mut self, _dt: Duration) {
        let current_source = self.source.get();
        let desired_target = (self.mapper)(current_source);
        if self.target.get() != desired_target {
            self.target.set(desired_target);
        }
    }

    fn state_hash(&self) -> u64 {
        self.source.state_hash()
    }

    fn clone_node(&self) -> Box<dyn Node> {
        // For simplicity, we don't clone the mapper closure here easily.
        // In a real scenario, we might need a more robust approach if nodes are cloned often.
        panic!("BindingNode cloning not supported");
    }

    fn reset(&mut self) {
        self.target.set((self.mapper)(self.source.get()));
    }
}
