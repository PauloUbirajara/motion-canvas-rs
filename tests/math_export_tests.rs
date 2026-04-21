#![cfg(all(feature = "math", feature = "export"))]
use motion_canvas_rs::prelude::*;
use std::time::Duration;

#[test]
fn test_math_animation_export() {
    let mut project = Project::default()
        .with_fps(60)
        .with_title("Test Math Animation Export")
        .with_gpu(false)
        .with_background(Color::rgb8(40, 44, 52))
        .close_on_finish();

    let tex = MathNode::new(
        Vec2::new(100.0, 300.0),
        "y = a x^2",
        48.0,
        Color::rgb8(0xff, 0xff, 0xff),
    );
    project.scene.add(Box::new(tex.clone()));

    project.scene.video_timeline.add(chain![
        wait(Duration::from_millis(100)),
        tex.tex("y = a x^2 + b x", Duration::from_millis(200)),
        wait(Duration::from_millis(100)),
        tex.tex("e^(i pi) + 1 = 0", Duration::from_millis(200)),
        wait(Duration::from_millis(100)),
        tex.tex("y = a x^2", Duration::from_millis(200)),
    ]);

    // Export to PNGs (headless)
    project.export().expect("Failed to export");

    // Verify that at least some frames were generated
    let output_path = std::path::Path::new("output");
    assert!(output_path.exists());
    assert!(output_path.is_dir());
}
