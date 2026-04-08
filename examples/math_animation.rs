use motion_canvas_rs::prelude::*;
use std::time::Duration;

fn main() {
    // 1. Initialize Project
    let mut project = Project::new(800, 600)
        .with_fps(60)
        .with_title("Typst Math Animation");

    // 2. Create MathNode (Typst syntax)
    let tex = MathNode::new(
        Vec2::new(100.0, 300.0),
        "y = a x^2",
        48.0,
        Color::rgb8(0xf2, 0xf2, 0xf2),
    );
    project.scene.add(Box::new(tex.clone()));

    // 3. Define Animation Sequence
    project.scene.timeline.add(flows::chain![
        flows::wait(Duration::from_millis(500)),
        tex.tex("y = a x^2 + b x", Duration::from_secs(1)),
        flows::wait(Duration::from_millis(500)),
        // Euler's Identity in Typst: e^(i pi) + 1 = 0
        tex.tex("e^(i pi) + 1 = 0", Duration::from_secs(1)),
        flows::wait(Duration::from_millis(500)),
        tex.tex("y = a x^2", Duration::from_secs(1)),
    ]);

    // 4. Run interactive preview
    project.show().expect("Failed to render");
}
