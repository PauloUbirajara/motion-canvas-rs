use motion_canvas_rs::engine::Animation;
use motion_canvas_rs::engine::animation::flow::{Chain, All};
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
    fn update(&mut self, _dt: Duration) -> bool {
        self.finished = true;
        true
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
    assert!(!chain.update(Duration::from_secs(1))); // finished first
    assert!(chain.update(Duration::from_secs(1)));  // finished second
}

#[test]
fn test_all_execution() {
    let anims: Vec<Box<dyn Animation>> = vec![
        Box::new(MockAnim::new()) as Box<dyn Animation>,
        Box::new(MockAnim::new()) as Box<dyn Animation>,
    ];
    let mut all = All::new(anims);

    // All should finish in one tick since mocks are instant
    assert!(all.update(Duration::from_secs(1)));
}
