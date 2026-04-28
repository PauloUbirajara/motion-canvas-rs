use motion_canvas_rs::prelude::*;
use std::time::Duration;

fn main() {
    let mut project = Project::default()
        .with_title("Nested Cameras Demo")
        .with_background(Color::rgb8(0x1a, 0x1a, 0x1a))
        .close_on_finish();

    // Elements to view
    let rect = Rect::default()
        .with_size(Vec2::new(200.0, 200.0))
        .with_fill(Color::BLUE);
    
    let circle = Circle::default()
        .with_position(Vec2::new(100.0, 100.0))
        .with_radius(30.0)
        .with_fill(Color::WHITE);

    // Inner Camera: Zooms into the action
    // We disable 'centered' here so it doesn't double-shift the viewport
    let inner_camera = CameraNode::new(vec![
        Box::new(rect.clone()),
        Box::new(circle.clone()),
    ])
    .with_centered(false);

    // Outer Camera: Handles global movement (centered)
    let outer_camera = CameraNode::new(vec![
        Box::new(inner_camera.clone()),
    ])
    .with_centered(true);

    // HUD Indicators (Outside cameras to show state)
    let outer_status = TextNode::default()
        .with_text("Outer Camera: Panning")
        .with_position(Vec2::new(20.0, 20.0))
        .with_anchor(Vec2::new(-1.0, -1.0))
        .with_font_size(24.0)
        .with_fill(Color::rgb8(0x2e, 0xcc, 0x71)) // Green
        .with_opacity(0.0);

    let inner_status = TextNode::default()
        .with_text("Inner Camera: Zooming")
        .with_position(Vec2::new(20.0, 60.0))
        .with_anchor(Vec2::new(-1.0, -1.0))
        .with_font_size(24.0)
        .with_fill(Color::rgb8(0xf1, 0xc4, 0x0f)) // Yellow
        .with_opacity(0.0);

    project.scene.add(Box::new(outer_status.clone()));
    project.scene.add(Box::new(inner_status.clone()));

    project.scene.add(Box::new(outer_camera.clone()));

    // Animation Flow
    project.scene.video_timeline.add(chain![
        // 1. Outer camera moves left
        all![
            outer_status.opacity.to(1.0, Duration::from_millis(300)),
            outer_camera.position.to(Vec2::new(-200.0, 0.0), Duration::from_secs(1)),
        ],
        outer_status.opacity.to(0.0, Duration::from_millis(300)),

        // 2. Inner camera zooms in
        all![
            inner_status.opacity.to(1.0, Duration::from_millis(300)),
            inner_camera.zoom.to(3.0, Duration::from_secs(1)),
        ],
        inner_status.opacity.to(0.0, Duration::from_millis(300)),

        // 3. Reset both
        all![
            outer_status.opacity.to(0.5, Duration::from_millis(300)),
            inner_status.opacity.to(0.5, Duration::from_millis(300)),
            outer_camera.position.to(Vec2::new(0.0, 0.0), Duration::from_secs(1)),
            inner_camera.zoom.to(1.0, Duration::from_secs(1)),
        ],
        all![
            outer_status.opacity.to(0.0, Duration::from_millis(300)),
            inner_status.opacity.to(0.0, Duration::from_millis(300)),
        ],
    ]);

    project.show().expect("Failed to render");
}
