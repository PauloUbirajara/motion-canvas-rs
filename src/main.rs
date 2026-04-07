use std::time::Duration;
use vello::peniko::Color;

mod engine;
mod render;

use crate::engine::*;

fn main() -> anyhow::Result<()> {
    // 1. Initialize Project with dimensions and extra settings
    let mut project = Project::new(800, 600)
        .with_title("Dynamic Export Demo")
        .with_output_path("renders/final")
        .with_fps(30) // Lower FPS for faster demo export
        .with_frame_template("img_{:04}.png") // Custom naming
        .with_export(true); // Enable export via code

    // 2. Create nodes (like Motion Canvas)
    let circle = Box::new(Circle {
        position: Signal::new(glam::vec2(100.0, 100.0)),
        radius: Signal::new(50.0),
        fill: Color::rgb8(32, 178, 170),
    });

    let rect = Box::new(Rect {
        position: Signal::new(glam::vec2(600.0, 100.0)),
        size: Signal::new(glam::vec2(100.0, 100.0)),
        fill: Color::rgb8(255, 100, 100),
        radius: 10.0,
    });

    // 3. Define animations
    project.scene.timeline.add(all(vec![
        circle
            .radius
            .to_with_easing(150.0, Duration::from_secs(2), easing::elastic_out),
        circle.position.to_with_easing(
            glam::vec2(400.0, 300.0),
            Duration::from_secs(2),
            easing::quad_in_out,
        ),
    ]));

    project.scene.timeline.add(rect.position.to_with_easing(
        glam::vec2(200.0, 400.0),
        Duration::from_secs(2),
        easing::cubic_in_out,
    ));

    // 4. Add nodes to scene
    project.scene.add(circle);
    project.scene.add(rect);

    // 5. Run (automatic detection of --export OR code-enabled export)
    project.run()
}
