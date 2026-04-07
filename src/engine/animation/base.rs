use std::time::Duration;
use vello::Scene;

pub trait Animation: Send + Sync + 'static {
    fn update(&mut self, dt: Duration) -> bool;
    fn duration(&self) -> Duration;
}

pub trait Node: Send + Sync + 'static {
    fn render(&self, vello_scene: &mut Scene);
    fn update(&mut self, dt: Duration);
    fn state_hash(&self) -> u64;
}
