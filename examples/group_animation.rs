use motion_canvas_rs::prelude::*;
use std::time::Duration;

fn main() {
    let mut project = Project::default()
        .with_title("Group Animation")
        .close_on_finish();

    // Create some child nodes
    let rect = Rect::default()
        .with_position(Vec2::new(-50.0, -50.0))
        .with_size(Vec2::new(100.0, 100.0))
        .with_fill(Color::rgba8(100, 100, 255, 255))
        .with_radius(10.0);

    let circle1 = Circle::default()
        .with_position(Vec2::new(-40.0, -40.0))
        .with_radius(20.0)
        .with_fill(Color::rgba8(255, 100, 100, 255));

    let circle2 = Circle::default()
        .with_position(Vec2::new(40.0, 40.0))
        .with_radius(20.0)
        .with_fill(Color::rgba8(100, 255, 100, 255));

    // Create a GroupNode holding them
    let group = GroupNode::default()
        .with_nodes(vec![
            Box::new(rect.clone()),
            Box::new(circle1.clone()),
            Box::new(circle2.clone()),
        ])
        .with_position(Vec2::new(400.0, 300.0));

    // We must add the group to the scene's nodes to render it
    project.scene.add(Box::new(group.clone()));

    // Define animations and add them to the timeline
    project.scene.video_timeline.add(chain![
        // 1. Move the whole group
        group
            .position
            .to(Vec2::new(200.0, 150.0), Duration::from_secs(2)),
        // 2. Rotate the group
        group
            .rotation
            .to(std::f32::consts::PI, Duration::from_secs(2)),
        // 3. Complex transform (move + scale)
        all![
            group
                .position
                .to(Vec2::new(400.0, 450.0), Duration::from_secs(2)),
            group.scale.to(Vec2::splat(2.0), Duration::from_secs(2)),
            group.opacity.to(0.3, Duration::from_secs(2)),
        ],
        // 4. Reset (at the center)
        all![
            group
                .position
                .to(Vec2::new(400.0, 300.0), Duration::from_secs(1)),
            group.scale.to(Vec2::ONE, Duration::from_secs(1)),
            group.rotation.to(0.0, Duration::from_secs(1)),
            group.opacity.to(1.0, Duration::from_secs(1)),
        ],
    ]);

    project.show().expect("Failed to render");
}
