use motion_canvas_rs::engine::animation::flow::{All, Chain};
use motion_canvas_rs::engine::Animation;
use std::time::Duration;

struct MockAnim {
    finished: bool,
}

impl MockAnim {
    fn new() -> Self {
        Self { finished: false }
    }
}

impl Animation for MockAnim {
    fn update(&mut self, dt: Duration) -> (bool, Duration) {
        self.finished = true;
        (true, dt)
    }

    fn duration(&self) -> Duration {
        Duration::ZERO
    }
}

#[test]
fn test_chain_execution() {
    let anims: Vec<Box<dyn Animation>> = vec![
        Box::new(MockAnim::new()) as Box<dyn Animation>,
        Box::new(MockAnim::new()) as Box<dyn Animation>,
    ];
    let mut chain = Chain::new(anims);

    // Each call should finish one animation in this mock
    assert!(!chain.update(Duration::from_secs(1)).0); // finished first
    assert!(chain.update(Duration::from_secs(1)).0); // finished second
}

#[test]
fn test_all_execution() {
    let anims: Vec<Box<dyn Animation>> = vec![
        Box::new(MockAnim::new()) as Box<dyn Animation>,
        Box::new(MockAnim::new()) as Box<dyn Animation>,
    ];
    let mut all = All::new(anims);

    // All should finish in one tick since mocks are instant
    assert!(all.update(Duration::from_secs(1)).0);
}
