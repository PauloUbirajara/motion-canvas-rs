use motion_canvas_rs::prelude::*;
use std::time::Duration;

fn main() {
    let mut project = Project::default()
        .with_title("Camera Demo")
        .with_background(Color::rgb8(0x1a, 0x1a, 0x1a))
        .close_on_finish();

    // Create a grid of circles to move the camera around
    let mut nodes: Vec<Box<dyn Node>> = Vec::new();

    for x in -2..=2 {
        for y in -2..=2 {
            let color = if (x + y) % 2 == 0 {
                Color::rgb8(0x34, 0x98, 0xdb) // Blue
            } else {
                Color::rgb8(0xe7, 0x4c, 0x3c) // Red
            };

            nodes.push(Box::new(
                Circle::default()
                    .with_position(Vec2::new(x as f32 * 200.0, y as f32 * 200.0))
                    .with_radius(40.0)
                    .with_fill(color),
            ));
        }
    }

    // Create the camera and add our nodes to it
    let camera = CameraNode::new(nodes)
        .with_position(Vec2::new(0.0, 0.0))
        .with_zoom(1.0);

    project.scene.add(Box::new(camera.clone()));

    // Add HUD text (NOT in camera, so it stays fixed)
    project.scene.add(Box::new(
        TextNode::default()
            .with_position(Vec2::new(400.0, 50.0))
            .with_text("Camera Control")
            .with_font_size(60.0)
            .with_fill(Color::WHITE)
            .with_anchor(Vec2::ZERO),
    ));

    // Animation: Pan, Zoom, and Rotate the camera
    project.scene.video_timeline.add(loop_anim!(
        chain![
            wait(Duration::from_secs(1)),
            // 1. Pan to the right
            camera
                .position
                .to(Vec2::new(200.0, 0.0), Duration::from_secs(1)),
            // 2. Zoom in
            camera.zoom.to(2.0, Duration::from_secs(1)),
            // 3. Pan down and rotate
            all![
                camera
                    .position
                    .to(Vec2::new(200.0, 200.0), Duration::from_secs(1)),
                camera
                    .rotation
                    .to(std::f32::consts::PI / 4.0, Duration::from_secs(1)),
            ],
            // 4. Zoom out and reset
            all![
                camera.position.to(Vec2::ZERO, Duration::from_secs(1)),
                camera.zoom.to(0.5, Duration::from_secs(1)),
                camera.rotation.to(0.0, Duration::from_secs(1)),
            ],
            // 5. Back to normal
            camera.zoom.to(1.0, Duration::from_secs(1)),
        ],
        None,
    ));

    project.show().expect("Failed to render");
}
