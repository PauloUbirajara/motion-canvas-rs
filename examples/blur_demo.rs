use motion_canvas_rs::prelude::*;
use std::time::Duration;

const FAST: Duration = Duration::from_millis(1200);
const SLOW: Duration = Duration::from_millis(2400);
const BEAT: Duration = Duration::from_millis(600);

fn main() {
    let mut project = Project::default()
        .with_title("Blur Demo")
        .with_background(Color::rgb8(0x08, 0x08, 0x10))
        .close_on_finish();

    // Dot Grid Backdrop
    for x in 1..8 {
        for y in 1..6 {
            project.scene.add(
                Circle::default()
                    .with_position(Vec2::new(x as f32 * 100.0, y as f32 * 100.0))
                    .with_radius(1.5)
                    .with_fill(Color::rgb8(0x1a, 0x1a, 0x2e)),
            );
        }
    }

    // Title
    let title = TextNode::default()
        .with_position(Vec2::new(400.0, 60.0))
        .with_text("Universal Blur")
        .with_font_size(42.0)
        .with_fill(Color::rgb8(0xe2, 0xe8, 0xf0))
        .with_blur(0.0);
    project.scene.add(&title);

    // Shape
    let orb = Circle::default()
        .with_position(Vec2::new(180.0, 220.0))
        .with_radius(55.0)
        .with_fill(Color::rgb8(0xa7, 0x6b, 0xf1))
        .with_opacity(0.9)
        .with_blur(20.0);
    project.scene.add(&orb);

    // SVG
    let logo = SvgNode::default()
        .with_position(Vec2::new(620.0, 220.0))
        .with_path("./examples/images/motion-canvas-rs.svg")
        .with_size(Vec2::new(100.0, 100.0))
        .with_blur(0.0);
    project.scene.add(&logo);

    // Math
    let eq = MathNode::default()
        .with_position(Vec2::new(200.0, 400.0))
        .with_equation(r#"E = m c^2"#)
        .with_font_size(38.0)
        .with_fill(Color::rgb8(0xfb, 0xd3, 0x8d))
        .with_blur(0.0);
    project.scene.add(&eq);

    // Code
    let snippet = CodeNode::default()
        .with_position(Vec2::new(400.0, 400.0))
        .with_language("rs")
        .with_code("let blur = 20.0;\ncircle.with_blur(blur);")
        .with_font_size(18.0)
        .with_blur(0.0);
    project.scene.add(&snippet);

    // Group
    let card = GroupNode::new(vec![
        Box::new(
            Rect::default()
                .with_size(Vec2::new(200.0, 90.0))
                .with_radius(12.0)
                .with_fill(Color::rgba8(0x38, 0xa1, 0xdb, 0xbb)),
        ),
        Box::new(
            TextNode::default()
                .with_text("Group Blur")
                .with_font_size(22.0)
                .with_fill(Color::WHITE),
        ),
    ])
    .with_position(Vec2::new(400.0, 300.0))
    .with_blur(0.0);
    project.scene.add(&card);

    // Subtitle
    let sub = TextNode::default()
        .with_position(Vec2::new(400.0, 550.0))
        .with_text("Shape · SVG · Math · Code · Group")
        .with_font_size(16.0)
        .with_fill(Color::rgb8(0x64, 0x6e, 0x7a))
        .with_blur(0.0);
    project.scene.add(&sub);

    // Animation: staggered blur waves
    // Phase 1 — blur everything except the orb (which sharpens)
    // Phase 2 — restore, then blur the card and equation
    // Phase 3 — return to initial state
    project.scene.video_timeline.add(loop_anim!(
        chain![
            wait(BEAT),
            // Phase 1: orb sharpens, everything else softens
            all![
                orb.blur.to(0.0, FAST),
                logo.blur.to(18.0, FAST),
                eq.blur.to(14.0, FAST),
                snippet.blur.to(14.0, FAST),
                card.blur.to(12.0, FAST),
                title.blur.to(6.0, FAST),
                sub.blur.to(4.0, FAST),
            ],
            wait(BEAT),
            // Phase 2: card + equation sharp, rest blurred
            all![
                orb.blur.to(24.0, SLOW),
                logo.blur.to(0.0, SLOW),
                eq.blur.to(0.0, SLOW),
                snippet.blur.to(18.0, SLOW),
                card.blur.to(0.0, SLOW),
                title.blur.to(0.0, SLOW),
                sub.blur.to(0.0, SLOW),
            ],
            wait(BEAT),
            // Phase 3: return to opening state
            all![
                orb.blur.to(20.0, FAST),
                logo.blur.to(0.0, FAST),
                eq.blur.to(0.0, FAST),
                snippet.blur.to(0.0, FAST),
                card.blur.to(0.0, FAST),
                title.blur.to(0.0, FAST),
                sub.blur.to(0.0, FAST),
            ],
        ],
        None,
    ));

    project.show().expect("Failed to show window");
}
