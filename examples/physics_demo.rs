use motion_canvas_rs::prelude::*;
use std::time::Duration;

// ─── Palette ──────────────────────────────────────────────
const BG: Color = Color::rgb8(0x0f, 0x0f, 0x14);
const FLOOR_COLOR: Color = Color::rgb8(0x2a, 0x2d, 0x3a);
const ACCENT_RED: Color = Color::rgb8(0xe1, 0x32, 0x38);
const ACCENT_BLUE: Color = Color::rgb8(0x68, 0xab, 0xdf);
const ACCENT_YELLOW: Color = Color::rgb8(0xe6, 0xa7, 0x00);
const ACCENT_TEAL: Color = Color::rgb8(0x20, 0xb2, 0xaa);
const ACCENT_PURPLE: Color = Color::rgb8(0x9b, 0x59, 0xb6);
const ACCENT_EMERALD: Color = Color::rgb8(0x25, 0xc2, 0x81);
const TEXT_DIM: Color = Color::rgb8(0x88, 0x88, 0x99);
const TEXT_BRIGHT: Color = Color::rgb8(0xe8, 0xe8, 0xf0);
const WALL_COLOR: Color = Color::rgb8(0x3a, 0x3d, 0x4a);

const W: u32 = 960;
const H: u32 = 540;
const CX: f32 = W as f32 / 2.0;
const FADE: Duration = Duration::from_millis(300);

// ─── Helpers ──────────────────────────────────────────────

fn make_floor(y: f32, width: f32, color: Color) -> StaticBodyNode {
    StaticBodyNode::new(Box::new(
        Rect::default()
            .with_size(Vec2::new(width, 30.0))
            .with_fill(color)
            .with_radius(4.0),
    ))
    .with_position(Vec2::new(CX, y))
    .with_shape(PhysicsShape::Cuboid(Vec2::new(width / 2.0, 15.0)))
}

fn make_wall(x: f32, y: f32, w: f32, h: f32) -> StaticBodyNode {
    StaticBodyNode::new(Box::new(
        Rect::default()
            .with_size(Vec2::new(w, h))
            .with_fill(WALL_COLOR)
            .with_radius(2.0),
    ))
    .with_position(Vec2::new(x, y))
    .with_shape(PhysicsShape::Cuboid(Vec2::new(w / 2.0, h / 2.0)))
}

fn make_title(text: &str) -> TextNode {
    TextNode::default()
        .with_text(text)
        .with_position(Vec2::new(CX, 40.0))
        .with_anchor(Vec2::ZERO)
        .with_font_size(32.0)
        .with_fill(TEXT_BRIGHT)
        .with_font("JetBrains Mono")
        .with_opacity(0.0)
}

fn make_subtitle(text: &str) -> TextNode {
    TextNode::default()
        .with_text(text)
        .with_position(Vec2::new(CX, 75.0))
        .with_anchor(Vec2::ZERO)
        .with_font_size(16.0)
        .with_fill(TEXT_DIM)
        .with_font("JetBrains Mono")
        .with_opacity(0.0)
}

fn make_label(text: &str, x: f32, y: f32, color: Color) -> TextNode {
    TextNode::default()
        .with_text(text)
        .with_position(Vec2::new(x, y))
        .with_anchor(Vec2::ZERO)
        .with_font_size(14.0)
        .with_fill(color)
        .with_font("JetBrains Mono")
        .with_opacity(0.0)
}

// ═══════════════════════════════════════════════════════════
// SCENE BUILDERS — return (PhysicsNode, title, subtitle, labels)
//
// Every PhysicsNode starts at opacity 0.
// Simulation ONLY runs when opacity > 0.
// ═══════════════════════════════════════════════════════════

fn build_scene1() -> (PhysicsNode, TextNode, TextNode) {
    let title = make_title("Gravity");
    let subtitle = make_subtitle("A single box falls under gravity");

    let mut p = PhysicsNode::new().with_opacity(0.0);

    p.add_static(make_floor(460.0, 600.0, FLOOR_COLOR));
    p.add_dynamic(
        RigidBodyNode::new(Box::new(
            Rect::default()
                .with_size(Vec2::new(60.0, 60.0))
                .with_fill(ACCENT_RED)
                .with_radius(6.0),
        ))
        .with_position(Vec2::new(CX, 120.0))
        .with_shape(PhysicsShape::Cuboid(Vec2::new(30.0, 30.0)))
        .with_bounciness(0.3),
    );

    (p, title, subtitle)
}

fn build_scene2() -> (PhysicsNode, TextNode, TextNode, Vec<TextNode>) {
    let title = make_title("Bounciness");
    let subtitle = make_subtitle("Same ball, three values: 0.0 · 0.5 · 0.95");

    let mut p = PhysicsNode::new().with_opacity(0.0);

    p.add_static(make_floor(460.0, 800.0, FLOOR_COLOR));

    let configs: [(f32, f32, Color, &str); 3] = [
        (CX - 200.0, 0.0, ACCENT_RED, "0.0 (dead)"),
        (CX, 0.5, ACCENT_YELLOW, "0.5 (rubber)"),
        (CX + 200.0, 0.95, ACCENT_EMERALD, "0.95 (super ball)"),
    ];

    let mut labels = Vec::new();
    for (x, bounce, color, label_text) in &configs {
        p.add_dynamic(
            RigidBodyNode::new(Box::new(
                Circle::default().with_radius(25.0).with_fill(*color),
            ))
            .with_position(Vec2::new(*x, 100.0))
            .with_shape(PhysicsShape::Ball(25.0))
            .with_bounciness(*bounce),
        );
        labels.push(make_label(label_text, *x, 500.0, *color));
    }

    (p, title, subtitle, labels)
}

fn build_scene3() -> (PhysicsNode, TextNode, TextNode, Vec<TextNode>) {
    let title = make_title("Shapes");
    let subtitle = make_subtitle("Circles roll, rectangles tumble — shape matters");

    let mut p = PhysicsNode::new().with_opacity(0.0);

    p.add_static(make_floor(460.0, 800.0, FLOOR_COLOR));

    // Ramps rotated by 0.25 rad (~14 degrees) down to the right
    p.add_static(
        StaticBodyNode::new(Box::new(
            Rect::default()
                .with_size(Vec2::new(300.0, 16.0))
                .with_fill(WALL_COLOR)
                .with_radius(2.0),
        ))
        .with_position(Vec2::new(300.0, 320.0))
        .with_rotation(0.25)
        .with_shape(PhysicsShape::Cuboid(Vec2::new(150.0, 8.0))),
    );
    p.add_static(
        StaticBodyNode::new(Box::new(
            Rect::default()
                .with_size(Vec2::new(300.0, 16.0))
                .with_fill(WALL_COLOR)
                .with_radius(2.0),
        ))
        .with_position(Vec2::new(660.0, 320.0))
        .with_rotation(0.25)
        .with_shape(PhysicsShape::Cuboid(Vec2::new(150.0, 8.0))),
    );

    // Circle rolls
    p.add_dynamic(
        RigidBodyNode::new(Box::new(
            Circle::default().with_radius(25.0).with_fill(ACCENT_BLUE),
        ))
        .with_position(Vec2::new(220.0, 150.0))
        .with_shape(PhysicsShape::Ball(25.0))
        .with_bounciness(0.2),
    );

    // Rect tumbles/slides (or gets stuck due to shape friction)
    p.add_dynamic(
        RigidBodyNode::new(Box::new(
            Rect::default()
                .with_size(Vec2::new(50.0, 50.0))
                .with_fill(ACCENT_PURPLE)
                .with_radius(4.0),
        ))
        .with_position(Vec2::new(580.0, 150.0))
        .with_shape(PhysicsShape::Cuboid(Vec2::new(25.0, 25.0)))
        .with_bounciness(0.2),
    );

    let labels = vec![
        make_label("Circle (rolls)", 250.0, 500.0, ACCENT_BLUE),
        make_label("Rect (tumbles)", 610.0, 500.0, ACCENT_PURPLE),
    ];

    (p, title, subtitle, labels)
}

fn build_scene4() -> (PhysicsNode, TextNode, TextNode) {
    let title = make_title("Stacking");
    let subtitle = make_subtitle("Bodies interact — stacking, settling, and toppling");

    let mut p = PhysicsNode::new().with_opacity(0.0);

    p.add_static(make_floor(460.0, 500.0, FLOOR_COLOR));

    let colors = [
        ACCENT_RED,
        ACCENT_BLUE,
        ACCENT_YELLOW,
        ACCENT_TEAL,
        ACCENT_PURPLE,
        ACCENT_EMERALD,
    ];
    let sizes = [
        Vec2::new(80.0, 30.0),
        Vec2::new(60.0, 40.0),
        Vec2::new(70.0, 35.0),
        Vec2::new(50.0, 50.0),
        Vec2::new(65.0, 25.0),
        Vec2::new(45.0, 45.0),
    ];

    for (i, (size, color)) in sizes.iter().zip(colors.iter()).enumerate() {
        let x_offset = if i % 2 == 0 { 3.0 } else { -5.0 };
        p.add_dynamic(
            RigidBodyNode::new(Box::new(
                Rect::default()
                    .with_size(*size)
                    .with_fill(*color)
                    .with_radius(4.0),
            ))
            .with_position(Vec2::new(CX + x_offset, 100.0 + i as f32 * 55.0))
            .with_shape(PhysicsShape::Cuboid(*size * 0.5))
            .with_bounciness(0.05),
        );
    }

    (p, title, subtitle)
}

fn build_scene5() -> (PhysicsNode, TextNode, TextNode) {
    let title = make_title("Containment");
    let subtitle = make_subtitle("Static bodies form containers — balls rain in");

    let mut p = PhysicsNode::new()
        .with_moving_container(true)
        .with_opacity(0.0);

    p.add_static(make_floor(450.0, 400.0, FLOOR_COLOR));
    p.add_static(make_wall(CX - 200.0, 350.0, 20.0, 220.0));
    p.add_static(make_wall(CX + 200.0, 350.0, 20.0, 220.0));

    let ball_colors = [
        ACCENT_RED,
        ACCENT_BLUE,
        ACCENT_YELLOW,
        ACCENT_TEAL,
        ACCENT_PURPLE,
        ACCENT_EMERALD,
    ];
    let radii = [
        12.0, 15.0, 10.0, 18.0, 13.0, 11.0, 14.0, 16.0, 9.0, 12.0, 15.0, 10.0,
    ];

    for (i, radius) in radii.iter().enumerate() {
        p.add_dynamic(
            RigidBodyNode::new(Box::new(
                Circle::default()
                    .with_radius(*radius)
                    .with_fill(ball_colors[i % ball_colors.len()]),
            ))
            .with_position(Vec2::new(
                CX - 120.0 + (i as f32 * 25.0),
                80.0 + (i as f32 * 15.0),
            ))
            .with_shape(PhysicsShape::Ball(*radius))
            .with_bounciness(0.4),
        );
    }

    (p, title, subtitle)
}

fn build_scene6() -> (PhysicsNode, TextNode, TextNode) {
    let title = make_title("Zero Gravity");
    let subtitle = make_subtitle("gravity = (0, 0) — objects float, collisions still work");

    let mut p = PhysicsNode::new()
        .with_gravity(Vec2::new(0.0, 0.0))
        .with_opacity(0.0);

    // Boundary
    p.add_static(make_wall(CX, 100.0, 800.0, 20.0));
    p.add_static(make_wall(CX, 480.0, 800.0, 20.0));
    p.add_static(make_wall(80.0, 290.0, 20.0, 400.0));
    p.add_static(make_wall(880.0, 290.0, 20.0, 400.0));

    let positions = [
        (250.0, 200.0),
        (480.0, 180.0),
        (700.0, 250.0),
        (350.0, 350.0),
        (600.0, 380.0),
        (200.0, 400.0),
    ];
    let velocities = [
        Vec2::new(150.0, 80.0),
        Vec2::new(-200.0, 100.0),
        Vec2::new(-100.0, -150.0),
        Vec2::new(180.0, -120.0),
        Vec2::new(220.0, 180.0),
        Vec2::new(-140.0, -200.0),
    ];
    let spins = [1.0, -1.5, 2.0, -0.8, 1.2, -2.5];
    let colors = [
        ACCENT_RED,
        ACCENT_BLUE,
        ACCENT_YELLOW,
        ACCENT_TEAL,
        ACCENT_PURPLE,
        ACCENT_EMERALD,
    ];

    for (i, (((x, y), vel), spin)) in positions
        .iter()
        .zip(velocities.iter())
        .zip(spins.iter())
        .enumerate()
    {
        let color = colors[i % colors.len()];
        if i % 2 == 0 {
            p.add_dynamic(
                RigidBodyNode::new(Box::new(
                    Circle::default().with_radius(22.0).with_fill(color),
                ))
                .with_position(Vec2::new(*x, *y))
                .with_shape(PhysicsShape::Ball(22.0))
                .with_bounciness(0.8)
                .with_initial_velocity(*vel)
                .with_initial_angular_velocity(*spin),
            );
        } else {
            p.add_dynamic(
                RigidBodyNode::new(Box::new(
                    Rect::default()
                        .with_size(Vec2::new(40.0, 40.0))
                        .with_fill(color)
                        .with_radius(4.0),
                ))
                .with_position(Vec2::new(*x, *y))
                .with_shape(PhysicsShape::Cuboid(Vec2::new(20.0, 20.0)))
                .with_bounciness(0.8)
                .with_initial_velocity(*vel)
                .with_initial_angular_velocity(*spin),
            );
        }
    }

    (p, title, subtitle)
}

fn build_scene7() -> (PhysicsNode, TextNode, TextNode) {
    let title = make_title("Final Example");
    let subtitle = make_subtitle("20 shapes, high bounciness, one container");

    let mut p = PhysicsNode::new().with_opacity(0.0);

    p.add_static(make_floor(500.0, 700.0, FLOOR_COLOR));
    p.add_static(make_wall(CX - 350.0, 350.0, 20.0, 320.0));
    p.add_static(make_wall(CX + 350.0, 350.0, 20.0, 320.0));

    let all_colors = [
        ACCENT_RED,
        ACCENT_BLUE,
        ACCENT_YELLOW,
        ACCENT_TEAL,
        ACCENT_PURPLE,
        ACCENT_EMERALD,
    ];

    for i in 0..20 {
        // Stack vertically one-by-one above the box, staggered slightly for cascading chaos
        let x = CX + (if i % 2 == 0 { 8.0 } else { -8.0 });
        let y = -100.0 - (i as f32 * 140.0);
        let color = all_colors[i % all_colors.len()];

        if i % 3 == 0 {
            // Large ball radius (from 24.0 to 48.0 px)
            let r = 24.0 + (i % 4) as f32 * 8.0;
            p.add_dynamic(
                RigidBodyNode::new(Box::new(Circle::default().with_radius(r).with_fill(color)))
                    .with_position(Vec2::new(x, y))
                    .with_shape(PhysicsShape::Ball(r))
                    .with_bounciness(0.85),
            );
        } else {
            // Large square side length (from 45.0 to 69.0 px)
            let s = 45.0 + (i % 3) as f32 * 12.0;
            p.add_dynamic(
                RigidBodyNode::new(Box::new(
                    Rect::default()
                        .with_size(Vec2::new(s, s))
                        .with_fill(color)
                        .with_radius(4.0),
                ))
                .with_position(Vec2::new(x, y))
                .with_shape(PhysicsShape::Cuboid(Vec2::new(s / 2.0, s / 2.0)))
                .with_bounciness(0.7),
            );
        }
    }

    (p, title, subtitle)
}

// ═══════════════════════════════════════════════════════════
// MAIN — Show one scene at a time.
//
// Each PhysicsNode starts at opacity 0 (simulation paused).
// We fade it in → simulation begins → watch → fade out.
// ═══════════════════════════════════════════════════════════

fn main() {
    let mut project = Project::default()
        .with_dimensions(W, H)
        .with_title("Physics Demo")
        .with_background(BG)
        .close_on_finish();

    // ── Build all scenes ──
    let (p1, t1, s1) = build_scene1();
    let (p2, t2, s2, labels2) = build_scene2();
    let (p3, t3, s3, labels3) = build_scene3();
    let (p4, t4, s4) = build_scene4();
    let (p5, t5, s5) = build_scene5();
    let (p6, t6, s6) = build_scene6();
    let (p7, t7, s7) = build_scene7();

    // ── Add all to scene (all invisible) ──
    project.scene.add(Box::new(p1.clone()));
    project.scene.add(Box::new(t1.clone()));
    project.scene.add(Box::new(s1.clone()));

    project.scene.add(Box::new(p2.clone()));
    project.scene.add(Box::new(t2.clone()));
    project.scene.add(Box::new(s2.clone()));
    let labels2_c: Vec<_> = labels2
        .iter()
        .map(|l| {
            project.scene.add(Box::new(l.clone()));
            l.clone()
        })
        .collect();

    project.scene.add(Box::new(p3.clone()));
    project.scene.add(Box::new(t3.clone()));
    project.scene.add(Box::new(s3.clone()));
    let labels3_c: Vec<_> = labels3
        .iter()
        .map(|l| {
            project.scene.add(Box::new(l.clone()));
            l.clone()
        })
        .collect();

    project.scene.add(Box::new(p4.clone()));
    project.scene.add(Box::new(t4.clone()));
    project.scene.add(Box::new(s4.clone()));

    project.scene.add(Box::new(p5.clone()));
    project.scene.add(Box::new(t5.clone()));
    project.scene.add(Box::new(s5.clone()));

    project.scene.add(Box::new(p6.clone()));
    project.scene.add(Box::new(t6.clone()));
    project.scene.add(Box::new(s6.clone()));

    project.scene.add(Box::new(p7.clone()));
    project.scene.add(Box::new(t7.clone()));
    project.scene.add(Box::new(s7.clone()));

    // ── Timeline: one scene at a time ──
    project.scene.video_timeline.add(chain![
        // Scene 1: Gravity
        t1.opacity.to(1.0, FADE),
        wait!(1),
        all![p1.opacity.to(1.0, FADE), s1.opacity.to(1.0, FADE),],
        wait!(4.0),
        all![
            p1.opacity.to(0.0, FADE),
            t1.opacity.to(0.0, FADE),
            s1.opacity.to(0.0, FADE),
        ],
        // Scene 2: Bounciness
        t2.opacity.to(1.0, FADE),
        wait!(1),
        all![
            p2.opacity.to(1.0, FADE),
            s2.opacity.to(1.0, FADE),
            labels2_c[0].opacity.to(1.0, FADE),
            labels2_c[1].opacity.to(1.0, FADE),
            labels2_c[2].opacity.to(1.0, FADE),
        ],
        wait!(5.0),
        all![
            p2.opacity.to(0.0, FADE),
            s2.opacity.to(0.0, FADE),
            t2.opacity.to(0.0, FADE),
            labels2_c[0].opacity.to(0.0, FADE),
            labels2_c[1].opacity.to(0.0, FADE),
            labels2_c[2].opacity.to(0.0, FADE),
        ],
        // Scene 3: Shapes
        t3.opacity.to(1.0, FADE),
        wait!(1),
        all![
            p3.opacity.to(1.0, FADE),
            s3.opacity.to(1.0, FADE),
            labels3_c[0].opacity.to(1.0, FADE),
            labels3_c[1].opacity.to(1.0, FADE),
        ],
        wait!(5.0),
        all![
            p3.opacity.to(0.0, FADE),
            t3.opacity.to(0.0, FADE),
            s3.opacity.to(0.0, FADE),
            labels3_c[0].opacity.to(0.0, FADE),
            labels3_c[1].opacity.to(0.0, FADE),
        ],
        // Scene 4: Stacking
        t4.opacity.to(1.0, FADE),
        wait!(1),
        all![p4.opacity.to(1.0, FADE), s4.opacity.to(1.0, FADE),],
        wait!(5.0),
        all![
            p4.opacity.to(0.0, FADE),
            t4.opacity.to(0.0, FADE),
            s4.opacity.to(0.0, FADE),
        ],
        // Scene 5: Containment
        t5.opacity.to(1.0, FADE),
        wait!(1),
        all![p5.opacity.to(1.0, FADE), s5.opacity.to(1.0, FADE),],
        wait!(10.0),
        all![
            p5.opacity.to(0.0, FADE),
            t5.opacity.to(0.0, FADE),
            s5.opacity.to(0.0, FADE),
        ],
        // Scene 6: Zero Gravity
        t6.opacity.to(1.0, FADE),
        wait!(1),
        all![p6.opacity.to(1.0, FADE), s6.opacity.to(1.0, FADE),],
        wait!(5.0),
        all![
            p6.opacity.to(0.0, FADE),
            t6.opacity.to(0.0, FADE),
            s6.opacity.to(0.0, FADE),
        ],
        // Scene 7: Final Example
        t7.opacity.to(1.0, FADE),
        wait!(1),
        all![p7.opacity.to(1.0, FADE), s7.opacity.to(1.0, FADE),],
        wait!(8.0),
        all![
            p7.opacity.to(0.0, FADE),
            t7.opacity.to(0.0, FADE),
            s7.opacity.to(0.0, FADE),
        ],
    ]);

    project.show().expect("Failed to render");
}
