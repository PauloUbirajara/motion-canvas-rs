use std::time::Duration;
use vello::Scene;

pub trait Animation: Send + Sync + 'static {
    /// Returns (finished, leftover_dt)
    fn update(&mut self, dt: Duration) -> (bool, Duration);
    fn duration(&self) -> Duration;
    fn set_easing(&mut self, _easing: fn(f32) -> f32) {}
}

pub trait Node: Send + Sync + 'static {
    fn render(
        &self,
        vello_scene: &mut Scene,
        parent_transform: vello::kurbo::Affine,
        parent_opacity: f32,
    );
    fn update(&mut self, dt: Duration);
    fn state_hash(&self) -> u64;
    fn clone_node(&self) -> Box<dyn Node>;
}
