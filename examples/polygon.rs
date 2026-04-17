use motion_canvas_rs::prelude::*;
use std::time::Duration;

fn main() {
    let mut project = Project::default()
        .with_title("Polygon Demo")
        .close_on_finish();

    // Create a regular pentagon
    let pentagon = Polygon::regular(5, 100.0)
        .with_fill(Color::rgb8(0xe1, 0x32, 0x38)) // Red
        .with_position(Vec2::new(200.0, 300.0))
        .with_scale(0.0);

    // Create a custom triangle
    let triangle = Polygon::default()
        .with_position(Vec2::new(500.0, 300.0))
        .with_points(vec![
            Vec2::new(0.0, -100.0),
            Vec2::new(100.0, 100.0),
            Vec2::new(-100.0, 100.0),
        ])
        .with_fill(Color::rgb8(0x68, 0xab, 0xdf)) // Blue
        .with_stroke(Color::WHITE, 4.0);

    project.scene.add(Box::new(pentagon.clone()));
    project.scene.add(Box::new(triangle.clone()));

    // Animate rotation and opacity
    project.scene.video_timeline.add(all![
        chain![
            pentagon
                .position
                .to(Vec2::new(200.0, 300.0), Duration::from_secs(1)),
            pentagon
                .rotation
                .to(std::f32::consts::PI, Duration::from_secs(1)),
            pentagon.scale.to(Vec2::ONE, Duration::from_secs(2))
        ],
        chain![
            triangle.opacity.to(1.0, Duration::from_secs(1)),
            triangle
                .position
                .to(Vec2::new(500.0, 300.0), Duration::from_secs(1)),
            triangle
                .rotation
                .to(360.0_f32.to_radians(), Duration::from_secs(1))
        ],
    ]);

    project.show().expect("Failed to render");
}
