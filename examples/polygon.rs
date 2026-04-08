use motion_canvas_rs::prelude::*;
use std::time::Duration;

fn main() {
    let mut project = Project::new(800, 600).with_title("Polygon Demo");

    // Create a regular pentagon
    let pentagon = Polygon::regular(
        Vec2::new(200.0, 300.0),
        5,
        100.0,
        Color::rgb8(0xe1, 0x32, 0x38), // Red
    );

    // Create a custom triangle
    let triangle = Polygon::new(
        Vec2::new(500.0, 300.0),
        vec![
            Vec2::new(0.0, -100.0),
            Vec2::new(100.0, 100.0),
            Vec2::new(-100.0, 100.0),
        ],
        Color::rgb8(0x68, 0xab, 0xdf), // Blue
    )
    .with_stroke(Color::WHITE, 4.0);

    project.scene.add(Box::new(pentagon.clone()));
    project.scene.add(Box::new(triangle.clone()));

    // Animate rotation and opacity
    project.scene.timeline.add(all![
        pentagon
            .transform
            .to(pentagon.transform.get() * Affine::rotate(std::f64::consts::PI), Duration::from_secs(2)),
        flows::chain![
            triangle.opacity.to(0.2, Duration::from_secs(1)),
            triangle.opacity.to(1.0, Duration::from_secs(1)),
        ],
    ]);

    project.show().expect("Failed to render");
}
