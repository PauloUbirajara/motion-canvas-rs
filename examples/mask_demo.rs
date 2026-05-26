use motion_canvas_rs::prelude::*;
use std::time::Duration;

const SLIDE: Duration = Duration::from_millis(2500);
const BEAT: Duration = Duration::from_millis(800);

/// Reusable source content: gradient rect, ring pair, label
fn make_source(w: f32, h: f32, font: f32) -> GroupNode {
    GroupNode::new(vec![
        Box::new(
            Rect::default()
                .with_size(Vec2::new(w, h))
                .with_radius(w * 0.04)
                .with_fill(linear_gradient!(
                    Color::rgb8(0xf4, 0x3f, 0x5e),
                    Color::rgb8(0x8b, 0x5c, 0xf6),
                    Color::rgb8(0x06, 0xb6, 0xd4)
                )),
        ),
        Box::new(
            Circle::default()
                .with_position(Vec2::new(-w * 0.3, 0.0))
                .with_radius(w * 0.12)
                .with_stroke(Color::rgba8(0xff, 0xff, 0xff, 0x88), 3.0),
        ),
        Box::new(
            Circle::default()
                .with_position(Vec2::new(w * 0.3, 0.0))
                .with_radius(w * 0.12)
                .with_stroke(Color::rgba8(0xff, 0xff, 0xff, 0x88), 3.0),
        ),
        Box::new(
            TextNode::default()
                .with_text("MASKED")
                .with_font_size(font)
                .with_fill(Color::WHITE),
        ),
    ])
}

/// Small mask panel: spot circle + MaskNode + label, initially hidden
fn make_panel(
    project: &mut Project,
    x: f32,
    y: f32,
    mode: MaskMode,
    name: &str,
    color: Color,
) -> (Circle, MaskNode, TextNode) {
    let spot = Circle::default()
        .with_position(Vec2::new(-110.0, 0.0))
        .with_radius(50.0)
        .with_fill(Color::WHITE);
    let mask = MaskNode::new(
        Box::new(spot.clone()),
        Box::new(make_source(220.0, 130.0, 18.0)),
    )
    .with_position(Vec2::new(x, y))
    .with_mode(mode)
    .with_opacity(0.0);
    let lbl = TextNode::default()
        .with_position(Vec2::new(x, y + 90.0))
        .with_text(name)
        .with_font_size(15.0)
        .with_fill(color)
        .with_opacity(0.0);
    project.scene.add(&mask);
    project.scene.add(&lbl);
    (spot, mask, lbl)
}

fn main() {
    let mut project = Project::default()
        .with_dimensions(1280, 720)
        .with_fps(60)
        .with_title("Mask Demo")
        .with_background(Color::rgb8(0x0b, 0x0f, 0x19))
        .close_on_finish();

    // Dot grid
    for x in 1..26 {
        for y in 1..15 {
            project.scene.add(
                Circle::default()
                    .with_position(Vec2::new(x as f32 * 50.0, y as f32 * 50.0))
                    .with_radius(1.0)
                    .with_fill(Color::rgb8(0x22, 0x2e, 0x4a)),
            );
        }
    }

    // Title
    project.scene.add(
        TextNode::default()
            .with_position(Vec2::new(640.0, 50.0))
            .with_text("Masking")
            .with_font_size(36.0)
            .with_fill(Color::rgb8(0xf8, 0xfa, 0xfc)),
    );

    // Mode label (animated)
    let label = TextNode::default()
        .with_position(Vec2::new(640.0, 100.0))
        .with_text("Intersect (Source In)")
        .with_font_size(20.0)
        .with_fill(Color::rgb8(0x38, 0xbd, 0xf8));
    project.scene.add(&label);

    // ── Sequential: single full-size mask ──
    let spot = Circle::default()
        .with_position(Vec2::new(-250.0, 0.0))
        .with_radius(120.0)
        .with_fill(Color::WHITE);

    let big = MaskNode::new(
        Box::new(spot.clone()),
        Box::new(make_source(500.0, 300.0, 44.0)),
    )
    .with_position(Vec2::new(640.0, 380.0))
    .with_mode(MaskMode::Intersect);
    project.scene.add(&big);

    // ── Side-by-side: 4 small panels (initially hidden) ──
    let xs: [f32; 4] = [250.0, 510.0, 770.0, 1030.0];
    let sy = 380.0;

    let (s0, p0, t0) = make_panel(
        &mut project,
        xs[0],
        sy,
        MaskMode::Intersect,
        "Intersect",
        Color::rgb8(0x38, 0xbd, 0xf8),
    );
    let (s1, p1, t1) = make_panel(
        &mut project,
        xs[1],
        sy,
        MaskMode::Subtract,
        "Subtract",
        Color::rgb8(0xf4, 0x3f, 0x5e),
    );
    let (s2, p2, t2) = make_panel(
        &mut project,
        xs[2],
        sy,
        MaskMode::Exclude,
        "Exclude",
        Color::rgb8(0xa8, 0x55, 0xf7),
    );
    let (s3, p3, t3) = make_panel(
        &mut project,
        xs[3],
        sy,
        MaskMode::Union,
        "Union",
        Color::rgb8(0x10, 0xb9, 0x81),
    );

    // ── Timeline ──
    project.scene.video_timeline.add(loop_anim!(
        chain![
            wait(BEAT),
            // --- Intersect: slide left→right ---
            spot.position.to(Vec2::new(250.0, 0.0), SLIDE),
            wait(BEAT),
            // reset spot, switch to Subtract
            all![
                spot.position.to(Vec2::new(-250.0, 0.0), Duration::ZERO),
                label
                    .text
                    .to("Subtract (Source Out)".into(), Duration::ZERO),
                label
                    .fill_paint
                    .to(Paint::Solid(Color::rgb8(0xf4, 0x3f, 0x5e)), Duration::ZERO),
                big.mode.to(MaskMode::Subtract, Duration::ZERO),
            ],
            wait(BEAT),
            // --- Subtract: slide left→right ---
            spot.position.to(Vec2::new(250.0, 0.0), SLIDE),
            wait(BEAT),
            // reset spot, switch to Exclude
            all![
                spot.position.to(Vec2::new(-250.0, 0.0), Duration::ZERO),
                label.text.to("Exclude (XOR)".into(), Duration::ZERO),
                label
                    .fill_paint
                    .to(Paint::Solid(Color::rgb8(0xa8, 0x55, 0xf7)), Duration::ZERO),
                big.mode.to(MaskMode::Exclude, Duration::ZERO),
            ],
            wait(BEAT),
            // --- Exclude: slide left→right ---
            spot.position.to(Vec2::new(250.0, 0.0), SLIDE),
            wait(BEAT),
            // reset spot, switch to Union
            all![
                spot.position.to(Vec2::new(-250.0, 0.0), Duration::ZERO),
                label.text.to("Union (Source Over)".into(), Duration::ZERO),
                label
                    .fill_paint
                    .to(Paint::Solid(Color::rgb8(0x10, 0xb9, 0x81)), Duration::ZERO),
                big.mode.to(MaskMode::Union, Duration::ZERO),
            ],
            wait(BEAT),
            // --- Union: slide left→right ---
            spot.position.to(Vec2::new(250.0, 0.0), SLIDE),
            wait(BEAT),
            // --- Side-by-side: show all 4, hide big ---
            all![
                big.opacity.to(0.0, Duration::ZERO),
                label.text.to("All Modes".into(), Duration::ZERO),
                label
                    .fill_paint
                    .to(Paint::Solid(Color::rgb8(0xe2, 0xe8, 0xf0)), Duration::ZERO),
                p0.opacity.to(1.0, Duration::ZERO),
                p1.opacity.to(1.0, Duration::ZERO),
                p2.opacity.to(1.0, Duration::ZERO),
                p3.opacity.to(1.0, Duration::ZERO),
                t0.opacity.to(1.0, Duration::ZERO),
                t1.opacity.to(1.0, Duration::ZERO),
                t2.opacity.to(1.0, Duration::ZERO),
                t3.opacity.to(1.0, Duration::ZERO),
            ],
            wait(BEAT),
            // All 4 spots slide left→right simultaneously
            all![
                s0.position.to(Vec2::new(110.0, 0.0), SLIDE),
                s1.position.to(Vec2::new(110.0, 0.0), SLIDE),
                s2.position.to(Vec2::new(110.0, 0.0), SLIDE),
                s3.position.to(Vec2::new(110.0, 0.0), SLIDE),
            ],
            wait(BEAT),
            // --- Reset everything for loop ---
            all![
                // Hide small panels
                p0.opacity.to(0.0, Duration::ZERO),
                p1.opacity.to(0.0, Duration::ZERO),
                p2.opacity.to(0.0, Duration::ZERO),
                p3.opacity.to(0.0, Duration::ZERO),
                t0.opacity.to(0.0, Duration::ZERO),
                t1.opacity.to(0.0, Duration::ZERO),
                t2.opacity.to(0.0, Duration::ZERO),
                t3.opacity.to(0.0, Duration::ZERO),
                // Reset small spots
                s0.position.to(Vec2::new(-110.0, 0.0), Duration::ZERO),
                s1.position.to(Vec2::new(-110.0, 0.0), Duration::ZERO),
                s2.position.to(Vec2::new(-110.0, 0.0), Duration::ZERO),
                s3.position.to(Vec2::new(-110.0, 0.0), Duration::ZERO),
                // Restore big mask
                big.opacity.to(1.0, Duration::ZERO),
                big.mode.to(MaskMode::Intersect, Duration::ZERO),
                spot.position.to(Vec2::new(-250.0, 0.0), Duration::ZERO),
                label
                    .text
                    .to("Intersect (Source In)".into(), Duration::ZERO),
                label
                    .fill_paint
                    .to(Paint::Solid(Color::rgb8(0x38, 0xbd, 0xf8)), Duration::ZERO),
            ],
        ],
        None
    ));

    project.show().expect("Failed to render");
}
