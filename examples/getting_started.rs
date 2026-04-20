use motion_canvas_rs::prelude::*;
use std::time::Duration;

fn main() {
    // 1. Initialize the Project
    let mut project = Project::default()
        .with_fps(60)
        .with_cache(true)
        .with_title("Getting Started")
        .close_on_finish();

    // 2. Define Nodes
    let circle = Circle::default()
        .with_position(Vec2::new(400.0, 300.0))
        .with_radius(50.0)
        .with_fill(Color::rgb8(0xe1, 0x32, 0x38)); // Red

    let text = TextNode::default()
        .with_position(Vec2::new(400.0, 450.0))
        .with_text("Hello Rust")
        .with_font_size(40.0)
        .with_fill(Color::rgb8(0xf2, 0xf2, 0xf2)); // White-ish

    // 3. Add Nodes to the Scene
    project.scene.add(Box::new(circle.clone()));
    project.scene.add(Box::new(text.clone()));

    // 4. Add Animations to the Timeline
    project.scene.video_timeline.add(all![
        circle.radius.to(100.0, Duration::from_secs(1)),
        text.position
            .to(Vec2::new(400.0, 500.0), Duration::from_secs(1)),
    ]);

    // 5. Show
    project.show().expect("Failed to render");
}
