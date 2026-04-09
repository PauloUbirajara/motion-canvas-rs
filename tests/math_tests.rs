#![cfg(feature = "math")]
use motion_canvas_rs::engine::nodes::MathNode;
use motion_canvas_rs::render::Color;
use std::time::Duration;

#[test]
fn test_math_transition_sequencing() {
    let node = MathNode::new(glam::Vec2::ZERO, "a", 12.0, Color::BLACK);
    // Note: We need to use update() directly on the animation
    let mut anim = node.tex("b", Duration::from_secs(1));

    let dt = Duration::from_millis(100);

    // 1. Initial state
    assert_eq!(node.equation.get(), "a");

    // 2. First update
    anim.update(dt);
    assert_eq!(node.equation.get(), "b");
    let p = node.transition_progress.get();
    assert!(p > 0.0 && p < 1.0);

    // 3. Middle update
    anim.update(Duration::from_millis(400));
    let p = node.transition_progress.get();
    assert!(p > 0.4 && p < 0.6);

    // 4. Final update
    anim.update(Duration::from_millis(500));
    assert_eq!(node.transition_progress.get(), 1.0);
}
