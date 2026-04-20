use motion_canvas_rs::prelude::*;
use std::time::Duration;

fn main() {
    let mut project = Project::default()
        .with_title("Anchor Test")
        .with_dimensions(1200, 800)
        .close_on_finish();

    // 1. TOP-LEFT ANCHOR (-1, -1)
    let rect_tl = Rect::default()
        .with_size(Vec2::new(100.0, 100.0))
        .with_fill(Color::rgb8(0x32, 0xe1, 0x38)) // Green
        .with_position(Vec2::new(300.0, 200.0))
        .with_anchor(Vec2::new(-1.0, -1.0));

    let label_tl = TextNode::default()
        .with_font("JetBrains Mono")
        .with_text("Top-Left Anchor")
        .with_font_size(16.0)
        .with_position(Vec2::new(300.0, 150.0))
        .with_fill(Color::WHITE);

    // 2. CENTER ANCHOR (0, 0)
    let rect_c = Rect::default()
        .with_size(Vec2::new(100.0, 100.0))
        .with_fill(Color::rgb8(0xe1, 0x32, 0x38)) // Red
        .with_position(Vec2::new(600.0, 200.0))
        .with_anchor(Vec2::new(0.0, 0.0));

    let label_c = TextNode::default()
        .with_font("JetBrains Mono")
        .with_text("Center Anchor")
        .with_font_size(16.0)
        .with_position(Vec2::new(600.0, 150.0))
        .with_fill(Color::WHITE);

    // 3. BOTTOM-RIGHT ANCHOR (1, 1)
    let rect_br = Rect::default()
        .with_size(Vec2::new(100.0, 100.0))
        .with_fill(Color::rgb8(0x32, 0x38, 0xe1)) // Blue
        .with_position(Vec2::new(900.0, 200.0))
        .with_anchor(Vec2::new(1.0, 1.0));

    let label_br = TextNode::default()
        .with_font("JetBrains Mono")
        .with_text("Bottom-Right Anchor")
        .with_font_size(16.0)
        .with_position(Vec2::new(900.0, 150.0))
        .with_fill(Color::WHITE);

    // 4. TEXT ANCHOR TEST
    let text_anchored = TextNode::default()
        .with_font("JetBrains Mono")
        .with_text("ANCHORED")
        .with_font_size(40.0)
        .with_fill(Color::YELLOW)
        .with_position(Vec2::new(600.0, 500.0))
        .with_anchor(Vec2::new(-1.0, 0.0)); // Left-center

    let text_marker = Circle::default()
        .with_radius(5.0)
        .with_fill(Color::WHITE)
        .with_position(Vec2::new(600.0, 500.0));

    // Show markers for each rect position
    let marker_tl = Circle::default()
        .with_radius(3.0)
        .with_fill(Color::WHITE)
        .with_position(Vec2::new(300.0, 200.0));
    let marker_c = Circle::default()
        .with_radius(3.0)
        .with_fill(Color::WHITE)
        .with_position(Vec2::new(600.0, 200.0));
    let marker_br = Circle::default()
        .with_radius(3.0)
        .with_fill(Color::WHITE)
        .with_position(Vec2::new(900.0, 200.0));

    project.scene.add(Box::new(rect_tl.clone()));
    project.scene.add(Box::new(label_tl));
    project.scene.add(Box::new(marker_tl));

    project.scene.add(Box::new(rect_c.clone()));
    project.scene.add(Box::new(label_c));
    project.scene.add(Box::new(marker_c));

    project.scene.add(Box::new(rect_br.clone()));
    project.scene.add(Box::new(label_br));
    project.scene.add(Box::new(marker_br));

    project.scene.add(Box::new(text_anchored.clone()));
    project.scene.add(Box::new(text_marker));

    // Animate rotation for all rects
    project.scene.video_timeline.add(loop_anim(
        move || {
            chain![
                // Test rotation
                all![
                    rect_tl
                        .rotation
                        .to(std::f32::consts::PI * 2.0, Duration::from_secs(2)),
                    rect_c
                        .rotation
                        .to(std::f32::consts::PI * 2.0, Duration::from_secs(2)),
                    rect_br
                        .rotation
                        .to(std::f32::consts::PI * 2.0, Duration::from_secs(2)),
                    text_anchored
                        .rotation
                        .to(std::f32::consts::PI * 2.0, Duration::from_secs(2)),
                ],
                // Test scaling
                all![
                    rect_tl.scale.to(Vec2::splat(1.5), Duration::from_secs(2)),
                    rect_c.scale.to(Vec2::splat(1.5), Duration::from_secs(2)),
                    rect_br.scale.to(Vec2::splat(1.5), Duration::from_secs(2)),
                    text_anchored
                        .scale
                        .to(Vec2::splat(1.5), Duration::from_secs(2)),
                ],
                // Test moving anchor during animation
                all![
                    text_anchored
                        .rotation
                        .to(std::f32::consts::PI * 3.0, Duration::from_secs(3)),
                    text_anchored
                        .anchor
                        .to(Vec2::new(1.0, 0.0), Duration::from_secs(3))
                        .ease(linear),
                ],
                // Reset
                all![
                    rect_tl.scale.to(Vec2::splat(1.0), Duration::from_secs(2)),
                    rect_c.scale.to(Vec2::splat(1.0), Duration::from_secs(2)),
                    rect_br.scale.to(Vec2::splat(1.0), Duration::from_secs(2)),
                    text_anchored
                        .scale
                        .to(Vec2::splat(1.0), Duration::from_secs(2)),
                ],
                all![
                    rect_tl.rotation.to(0.0, Duration::from_secs(2)),
                    rect_c.rotation.to(0.0, Duration::from_secs(2)),
                    rect_br.rotation.to(0.0, Duration::from_secs(2)),
                    text_anchored.rotation.to(0.0, Duration::from_secs(2)),
                    text_anchored
                        .anchor
                        .to(Vec2::new(-1.0, 0.0), Duration::from_secs(2)),
                ],
            ]
        },
        None,
    ));

    project.show().expect("Failed to render");
}
