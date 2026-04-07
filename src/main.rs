use std::time::Duration;
use vello::peniko::Color;

mod engine;
mod render;

use crate::engine::*;

fn main() -> anyhow::Result<()> {
    // 1. Initialize Project with dimensions
    let mut project = Project::new(800, 600).with_fps(60);

    // 2. Create nodes (wrapping in Arc/Mutex is handled by Signal)
    let circle = Circle {
        position: Signal::new(glam::vec2(100.0, 100.0)),
        radius: Signal::new(50.0),
        fill: Color::rgb8(32, 178, 170),
    };

    let rect = Rect {
        position: Signal::new(glam::vec2(600.0, 100.0)),
        size: Signal::new(glam::vec2(100.0, 100.0)),
        fill: Color::rgb8(255, 100, 100),
        radius: 10.0,
    };

    // Clone signals for use in animation factory (for loops)
    let c_pos = circle.position.clone();
    let c_rad = circle.radius.clone();

    // 3. Define animations using the new advanced flow system
    
    // Example: Chain multiple animations one after another
    project.scene.timeline.add(chain![
        // First - expand and move right
        all![
            circle.radius.to(100.0, Duration::from_secs(1)).ease(easings::elastic_out),
            circle.position.to(glam::vec2(400.0, 100.0), Duration::from_secs(1))
        ],
        // Then - move down with a delay
        circle.position.to(glam::vec2(400.0, 300.0), Duration::from_secs(1))
    ]);

    // Example: Sequence (Staggered starts)
    project.scene.timeline.add(sequence![
        Duration::from_millis(500), // delay between elements
        rect.position.to(glam::vec2(200.0, 400.0), Duration::from_secs(2)).ease(easings::cubic_in_out),
        rect.size.to(glam::vec2(200.0, 150.0), Duration::from_secs(1))
    ]);

    // Example: Loop an animation 3 times
    project.scene.timeline.add(loop_anim(
        move || {
            // This factory function is called each time the loop restarts
            chain![
                c_rad.to(120.0, Duration::from_millis(500)).ease(easings::quad_out),
                c_rad.to(100.0, Duration::from_millis(500)).ease(easings::quad_in)
            ].into()
        },
        Some(3) // Loop 3 times
    ));

    // 4. Add nodes to scene (move them now)
    project.scene.add(Box::new(circle));
    project.scene.add(Box::new(rect));

    // 5. Run preview
    project.show()
}
