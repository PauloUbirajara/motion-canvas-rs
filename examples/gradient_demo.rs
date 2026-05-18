use motion_canvas_rs::prelude::*;
use std::time::Duration;

fn linear_grad(c1: Color, c2: Color, size: f64) -> Gradient {
    Gradient {
        kind: GradientKind::Linear {
            start: Point::new(-size, 0.0),
            end: Point::new(size, 0.0),
        },
        extend: Extend::Pad,
        stops: ColorStops::from(vec![
            ColorStop {
                offset: 0.0,
                color: c1,
            },
            ColorStop {
                offset: 1.0,
                color: c2,
            },
        ]),
    }
}

fn tri_grad(c1: Color, c2: Color, c3: Color, size: f64) -> Gradient {
    Gradient {
        kind: GradientKind::Linear {
            start: Point::new(-size, -size),
            end: Point::new(size, size),
        },
        extend: Extend::Pad,
        stops: ColorStops::from(vec![
            ColorStop {
                offset: 0.0,
                color: c1,
            },
            ColorStop {
                offset: 0.5,
                color: c2,
            },
            ColorStop {
                offset: 1.0,
                color: c3,
            },
        ]),
    }
}

fn radial_grad(inner: Color, outer: Color, radius: f64) -> Gradient {
    Gradient {
        kind: GradientKind::Radial {
            start_center: Point::new(0.0, 0.0),
            start_radius: 0.0,
            end_center: Point::new(0.0, 0.0),
            end_radius: radius as f32,
        },
        extend: Extend::Pad,
        stops: ColorStops::from(vec![
            ColorStop {
                offset: 0.0,
                color: inner,
            },
            ColorStop {
                offset: 1.0,
                color: outer,
            },
        ]),
    }
}

/// Build a gradient with explicit direction points and arbitrary stops.
fn directed_grad(start: Point, end: Point, stops: Vec<ColorStop>) -> Gradient {
    Gradient {
        kind: GradientKind::Linear { start, end },
        extend: Extend::Pad,
        stops: ColorStops::from(stops),
    }
}

// Palette
const CYAN: Color = Color::rgb8(0x00, 0xf2, 0xfe);
const INDIGO: Color = Color::rgb8(0x4f, 0x46, 0xe5);
const CORAL: Color = Color::rgb8(0xff, 0x5e, 0x3a);
const PINK: Color = Color::rgb8(0xff, 0x2a, 0x68);
const VIOLET: Color = Color::rgb8(0x92, 0x3c, 0xff);
const FUCHSIA: Color = Color::rgb8(0xff, 0x00, 0x7f);
const BG: Color = Color::rgb8(0x0a, 0x0a, 0x12);
const GRID_COLOR: Color = Color::rgba8(0x20, 0x20, 0x38, 60);
const LABEL: Color = Color::rgb8(0x99, 0x99, 0xbb);

// Timing
const FADE_IN: Duration = Duration::from_millis(500);
const FADE_OUT: Duration = Duration::from_millis(300);
const MORPH: Duration = Duration::from_millis(1800);

fn main() {
    let cx = 960.0_f32;
    let cy = 480.0_f32;

    let mut project = Project::default()
        .with_dimensions(1920, 1080)
        .with_cache(true)
        .with_title("Gradient Demo")
        .close_on_finish();

    // Background
    let bg_rect = Rect::default()
        .with_position(Vec2::new(cx, 540.0))
        .with_size(Vec2::new(1920.0, 1080.0))
        .with_fill(BG);

    let grid = GridNode::default()
        .with_position(Vec2::new(cx, 540.0))
        .with_columns(32.0)
        .with_rows(18.0)
        .with_spacing_all(60.0)
        .with_stroke(GRID_COLOR, 1.0)
        .with_opacity(0.0);

    // Gradients
    let sunset = tri_grad(CORAL, PINK, VIOLET, 120.0);
    let ocean = linear_grad(CYAN, INDIGO, 120.0);
    let glow = radial_grad(FUCHSIA, Color::rgba8(0x4f, 0x46, 0xe5, 0), 130.0);
    let glow_alt = radial_grad(CYAN, Color::rgba8(0x00, 0xf2, 0xfe, 0), 180.0);

    // Sunset in different directions (same colors, different orientation)
    let sunset_stops = vec![
        ColorStop {
            offset: 0.0,
            color: CORAL,
        },
        ColorStop {
            offset: 0.5,
            color: PINK,
        },
        ColorStop {
            offset: 1.0,
            color: VIOLET,
        },
    ];
    let sunset_horiz = directed_grad(
        Point::new(-120.0, 0.0),
        Point::new(120.0, 0.0),
        sunset_stops.clone(),
    );
    let sunset_vert = directed_grad(
        Point::new(0.0, -120.0),
        Point::new(0.0, 120.0),
        sunset_stops.clone(),
    );
    let sunset_diag = directed_grad(
        Point::new(-120.0, -120.0),
        Point::new(120.0, 120.0),
        sunset_stops.clone(),
    );

    // Ocean in different directions
    let ocean_stops = vec![
        ColorStop {
            offset: 0.0,
            color: CYAN,
        },
        ColorStop {
            offset: 1.0,
            color: INDIGO,
        },
    ];
    let ocean_vert = directed_grad(
        Point::new(0.0, -120.0),
        Point::new(0.0, 120.0),
        ocean_stops.clone(),
    );

    // Persistent title
    let title = TextNode::default()
        .with_position(Vec2::new(cx, 100.0))
        .with_text("Gradient Paint Demo")
        .with_font_size(48.0)
        .with_fill(Color::rgb8(0xdd, 0xdd, 0xee))
        .with_opacity(0.0);

    // Per-scene label (reused position, swapped text via separate nodes)
    let label_y = 200.0;

    // ─── SCENE 1: Circle with linear gradient fill ───
    let s1_lbl = TextNode::default()
        .with_position(Vec2::new(cx, label_y))
        .with_text("Linear Gradient Fill")
        .with_font_size(28.0)
        .with_fill(LABEL)
        .with_opacity(0.0);

    let s1 = Circle::default()
        .with_position(Vec2::new(cx, cy))
        .with_radius(130.0)
        .with_fill(sunset.clone())
        .with_stroke(Color::rgba8(0xff, 0xff, 0xff, 20), 2.0)
        .with_scale(0.95)
        .with_opacity(0.0);

    // Guides
    let s1_g_start = Circle::default()
        .with_position(Vec2::new(cx - 120.0, cy - 120.0))
        .with_radius(8.0)
        .with_fill(Color::WHITE)
        .with_stroke(CORAL, 2.5)
        .with_opacity(0.0);

    let s1_g_end = Circle::default()
        .with_position(Vec2::new(cx + 120.0, cy + 120.0))
        .with_radius(8.0)
        .with_fill(Color::WHITE)
        .with_stroke(VIOLET, 2.5)
        .with_opacity(0.0);

    let s1_g_line = Line::default()
        .with_position(Vec2::new(cx, cy))
        .with_start(Vec2::new(-120.0, -120.0))
        .with_end(Vec2::new(120.0, 120.0))
        .with_stroke(Color::rgba8(0xff, 0xff, 0xff, 80), 1.5)
        .with_opacity(0.0);

    // ─── SCENE 2: Rect with radial gradient fill ───
    let s2_lbl = TextNode::default()
        .with_position(Vec2::new(cx, label_y))
        .with_text("Radial Gradient Fill")
        .with_font_size(28.0)
        .with_fill(LABEL)
        .with_opacity(0.0);

    let s2 = Rect::default()
        .with_position(Vec2::new(cx, cy))
        .with_size(Vec2::new(260.0, 260.0))
        .with_radius(24.0)
        .with_fill(glow.clone())
        .with_stroke(Color::rgba8(0xff, 0xff, 0xff, 20), 2.0)
        .with_scale(0.95)
        .with_opacity(0.0);

    // Guides
    let s2_g_center = Circle::default()
        .with_position(Vec2::new(cx, cy))
        .with_radius(8.0)
        .with_fill(Color::WHITE)
        .with_stroke(FUCHSIA, 2.5)
        .with_opacity(0.0);

    let s2_g_radius = Circle::default()
        .with_position(Vec2::new(cx, cy))
        .with_radius(130.0)
        .with_fill(Color::TRANSPARENT)
        .with_stroke(Color::rgba8(0xff, 0x00, 0x7f, 80), 1.5)
        .with_opacity(0.0);

    // ─── SCENE 3: Hexagon with gradient stroke ───
    let s3_lbl = TextNode::default()
        .with_position(Vec2::new(cx, label_y))
        .with_text("Gradient Stroke")
        .with_font_size(28.0)
        .with_fill(LABEL)
        .with_opacity(0.0);

    let s3 = Polygon::regular(6, 130.0)
        .with_position(Vec2::new(cx, cy))
        .with_fill(Color::rgba8(0x12, 0x12, 0x20, 255))
        .with_stroke(ocean.clone(), 5.0)
        .with_scale(0.95)
        .with_opacity(0.0);

    // Guides
    let s3_g_start = Circle::default()
        .with_position(Vec2::new(cx - 120.0, cy))
        .with_radius(8.0)
        .with_fill(Color::WHITE)
        .with_stroke(CYAN, 2.5)
        .with_opacity(0.0);

    let s3_g_end = Circle::default()
        .with_position(Vec2::new(cx + 120.0, cy))
        .with_radius(8.0)
        .with_fill(Color::WHITE)
        .with_stroke(INDIGO, 2.5)
        .with_opacity(0.0);

    let s3_g_line = Line::default()
        .with_position(Vec2::new(cx, cy))
        .with_start(Vec2::new(-120.0, 0.0))
        .with_end(Vec2::new(120.0, 0.0))
        .with_stroke(Color::rgba8(0xff, 0xff, 0xff, 80), 1.5)
        .with_opacity(0.0);

    // ─── SCENE 4: Line with gradient stroke ───
    let s4_lbl = TextNode::default()
        .with_position(Vec2::new(cx, label_y))
        .with_text("Gradient Line")
        .with_font_size(28.0)
        .with_fill(LABEL)
        .with_opacity(0.0);

    let s4 = Line::default()
        .with_position(Vec2::new(cx, cy))
        .with_start(Vec2::new(-160.0, 100.0))
        .with_end(Vec2::new(160.0, -100.0))
        .with_stroke(sunset.clone(), 8.0)
        .with_opacity(0.0);

    // Guides
    let s4_g_start = Circle::default()
        .with_position(Vec2::new(cx - 120.0, cy - 120.0))
        .with_radius(8.0)
        .with_fill(Color::WHITE)
        .with_stroke(CORAL, 2.5)
        .with_opacity(0.0);

    let s4_g_end = Circle::default()
        .with_position(Vec2::new(cx + 120.0, cy + 120.0))
        .with_radius(8.0)
        .with_fill(Color::WHITE)
        .with_stroke(VIOLET, 2.5)
        .with_opacity(0.0);

    let s4_g_line = Line::default()
        .with_position(Vec2::new(cx, cy))
        .with_start(Vec2::new(-120.0, -120.0))
        .with_end(Vec2::new(120.0, 120.0))
        .with_stroke(Color::rgba8(0xff, 0xff, 0xff, 80), 1.5)
        .with_opacity(0.0);

    // ─── SCENE 5: Path with gradient stroke ───
    let s5_lbl = TextNode::default()
        .with_position(Vec2::new(cx, label_y))
        .with_text("Gradient Path")
        .with_font_size(28.0)
        .with_fill(LABEL)
        .with_opacity(0.0);

    let mut tri_path = BezPath::new();
    tri_path.move_to(kurbo::Point::new(0.0, -120.0));
    tri_path.line_to(kurbo::Point::new(104.0, 60.0));
    tri_path.line_to(kurbo::Point::new(-104.0, 60.0));
    tri_path.close_path();

    let s5 = PathNode::new(Vec2::new(cx, cy), tri_path, CYAN, 6.0)
        .with_stroke(ocean.clone(), 6.0)
        .with_opacity(0.0);

    // Guides
    let s5_g_start = Circle::default()
        .with_position(Vec2::new(cx - 120.0, cy))
        .with_radius(8.0)
        .with_fill(Color::WHITE)
        .with_stroke(CYAN, 2.5)
        .with_opacity(0.0);

    let s5_g_end = Circle::default()
        .with_position(Vec2::new(cx + 120.0, cy))
        .with_radius(8.0)
        .with_fill(Color::WHITE)
        .with_stroke(INDIGO, 2.5)
        .with_opacity(0.0);

    let s5_g_line = Line::default()
        .with_position(Vec2::new(cx, cy))
        .with_start(Vec2::new(-120.0, 0.0))
        .with_end(Vec2::new(120.0, 0.0))
        .with_stroke(Color::rgba8(0xff, 0xff, 0xff, 80), 1.5)
        .with_opacity(0.0);

    // ─── SCENE 6: Grid with gradient stroke ───
    let s6_lbl = TextNode::default()
        .with_position(Vec2::new(cx, label_y))
        .with_text("Gradient Grid")
        .with_font_size(28.0)
        .with_fill(LABEL)
        .with_opacity(0.0);

    let s6_grid = GridNode::default()
        .with_position(Vec2::new(cx, cy))
        .with_columns(8.0)
        .with_rows(6.0)
        .with_spacing_all(50.0)
        .with_stroke(ocean.clone(), 2.0)
        .with_opacity(0.0);

    // Guides
    let s6_g_start = Circle::default()
        .with_position(Vec2::new(cx - 120.0, cy))
        .with_radius(8.0)
        .with_fill(Color::WHITE)
        .with_stroke(CYAN, 2.5)
        .with_opacity(0.0);

    let s6_g_end = Circle::default()
        .with_position(Vec2::new(cx + 120.0, cy))
        .with_radius(8.0)
        .with_fill(Color::WHITE)
        .with_stroke(INDIGO, 2.5)
        .with_opacity(0.0);

    let s6_g_line = Line::default()
        .with_position(Vec2::new(cx, cy))
        .with_start(Vec2::new(-120.0, 0.0))
        .with_end(Vec2::new(120.0, 0.0))
        .with_stroke(Color::rgba8(0xff, 0xff, 0xff, 80), 1.5)
        .with_opacity(0.0);

    // ─── SCENE 7: Text with gradient fill ───
    let s7_lbl = TextNode::default()
        .with_position(Vec2::new(cx, label_y))
        .with_text("Gradient Text")
        .with_font_size(28.0)
        .with_fill(LABEL)
        .with_opacity(0.0);

    let s7 = TextNode::default()
        .with_position(Vec2::new(cx, cy))
        .with_text("HELLO GRADIENTS")
        .with_font_size(80.0)
        .with_fill(ocean.clone())
        .with_scale(0.95)
        .with_opacity(0.0);

    // Guides
    let s7_g_start = Circle::default()
        .with_position(Vec2::new(cx - 120.0, cy))
        .with_radius(8.0)
        .with_fill(Color::WHITE)
        .with_stroke(CYAN, 2.5)
        .with_opacity(0.0);

    let s7_g_end = Circle::default()
        .with_position(Vec2::new(cx + 120.0, cy))
        .with_radius(8.0)
        .with_fill(Color::WHITE)
        .with_stroke(INDIGO, 2.5)
        .with_opacity(0.0);

    let s7_g_line = Line::default()
        .with_position(Vec2::new(cx, cy))
        .with_start(Vec2::new(-120.0, 0.0))
        .with_end(Vec2::new(120.0, 0.0))
        .with_stroke(Color::rgba8(0xff, 0xff, 0xff, 80), 1.5)
        .with_opacity(0.0);

    // ─── SCENE 8: Math with gradient fill ───
    let s8_lbl = TextNode::default()
        .with_position(Vec2::new(cx, label_y))
        .with_text("Gradient Math")
        .with_font_size(28.0)
        .with_fill(LABEL)
        .with_opacity(0.0);

    let s8 = MathNode::default()
        .with_position(Vec2::new(cx, cy))
        .with_equation("E = m c^2")
        .with_font_size(72.0)
        .with_fill(sunset.clone())
        .with_scale(0.95)
        .with_opacity(0.0);

    // Guides
    let s8_g_start = Circle::default()
        .with_position(Vec2::new(cx - 120.0, cy - 120.0))
        .with_radius(8.0)
        .with_fill(Color::WHITE)
        .with_stroke(CORAL, 2.5)
        .with_opacity(0.0);

    let s8_g_end = Circle::default()
        .with_position(Vec2::new(cx + 120.0, cy + 120.0))
        .with_radius(8.0)
        .with_fill(Color::WHITE)
        .with_stroke(VIOLET, 2.5)
        .with_opacity(0.0);

    let s8_g_line = Line::default()
        .with_position(Vec2::new(cx, cy))
        .with_start(Vec2::new(-120.0, -120.0))
        .with_end(Vec2::new(120.0, 120.0))
        .with_stroke(Color::rgba8(0xff, 0xff, 0xff, 80), 1.5)
        .with_opacity(0.0);

    // ─── SCENE 9: Solid ↔ Gradient ───
    let s9_lbl = TextNode::default()
        .with_position(Vec2::new(cx, label_y))
        .with_text("Solid to Gradient")
        .with_font_size(28.0)
        .with_fill(LABEL)
        .with_opacity(0.0);

    let s9 = Circle::default()
        .with_position(Vec2::new(cx, cy))
        .with_radius(130.0)
        .with_fill(Color::WHITE)
        .with_stroke(Color::rgba8(0xff, 0xff, 0xff, 20), 2.0)
        .with_scale(0.95)
        .with_opacity(0.0);

    // Guides (start with opacity 0, fade in when Sunset is active, fade out on Solid)
    let s9_g_start = Circle::default()
        .with_position(Vec2::new(cx - 120.0, cy - 120.0))
        .with_radius(8.0)
        .with_fill(Color::WHITE)
        .with_stroke(CORAL, 2.5)
        .with_opacity(0.0);

    let s9_g_end = Circle::default()
        .with_position(Vec2::new(cx + 120.0, cy + 120.0))
        .with_radius(8.0)
        .with_fill(Color::WHITE)
        .with_stroke(VIOLET, 2.5)
        .with_opacity(0.0);

    let s9_g_line = Line::default()
        .with_position(Vec2::new(cx, cy))
        .with_start(Vec2::new(-120.0, -120.0))
        .with_end(Vec2::new(120.0, 120.0))
        .with_stroke(Color::rgba8(0xff, 0xff, 0xff, 80), 1.5)
        .with_opacity(0.0);

    // ─── SCENE 10: Radial ↔ Linear Interpolation ───
    let s10_lbl = TextNode::default()
        .with_position(Vec2::new(cx, label_y))
        .with_text("Radial ↔ Linear Morph")
        .with_font_size(28.0)
        .with_fill(LABEL)
        .with_opacity(0.0);

    let s10 = Circle::default()
        .with_position(Vec2::new(cx, cy))
        .with_radius(130.0)
        .with_fill(glow.clone()) // Start as Radial Gradient
        .with_stroke(Color::rgba8(0xff, 0xff, 0xff, 20), 2.0)
        .with_scale(0.95)
        .with_opacity(0.0);

    // Visual guides for Scene 10 (cross-fading between radial and linear handles)
    let s10_g_rad_center = Circle::default()
        .with_position(Vec2::new(cx, cy))
        .with_radius(8.0)
        .with_fill(Color::WHITE)
        .with_stroke(FUCHSIA, 2.5)
        .with_opacity(0.0);

    let s10_g_rad_radius = Circle::default()
        .with_position(Vec2::new(cx, cy))
        .with_radius(130.0)
        .with_fill(Color::TRANSPARENT)
        .with_stroke(Color::rgba8(0xff, 0x00, 0x7f, 80), 1.5)
        .with_opacity(0.0);

    let s10_g_lin_start = Circle::default()
        .with_position(Vec2::new(cx - 120.0, cy - 120.0))
        .with_radius(8.0)
        .with_fill(Color::WHITE)
        .with_stroke(CORAL, 2.5)
        .with_opacity(0.0);

    let s10_g_lin_end = Circle::default()
        .with_position(Vec2::new(cx + 120.0, cy + 120.0))
        .with_radius(8.0)
        .with_fill(Color::WHITE)
        .with_stroke(VIOLET, 2.5)
        .with_opacity(0.0);

    let s10_g_lin_line = Line::default()
        .with_position(Vec2::new(cx, cy))
        .with_start(Vec2::new(-120.0, -120.0))
        .with_end(Vec2::new(120.0, 120.0))
        .with_stroke(Color::rgba8(0xff, 0xff, 0xff, 80), 1.5)
        .with_opacity(0.0);

    // Add all to scene
    project.scene.add(&bg_rect);
    project.scene.add(&grid);
    project.scene.add(&title);

    // Scene 1
    project.scene.add(&s1_lbl);
    project.scene.add(&s1);
    project.scene.add(&s1_g_line);
    project.scene.add(&s1_g_start);
    project.scene.add(&s1_g_end);

    // Scene 2
    project.scene.add(&s2_lbl);
    project.scene.add(&s2);
    project.scene.add(&s2_g_radius);
    project.scene.add(&s2_g_center);

    // Scene 3
    project.scene.add(&s3_lbl);
    project.scene.add(&s3);
    project.scene.add(&s3_g_line);
    project.scene.add(&s3_g_start);
    project.scene.add(&s3_g_end);

    // Scene 4
    project.scene.add(&s4_lbl);
    project.scene.add(&s4);
    project.scene.add(&s4_g_line);
    project.scene.add(&s4_g_start);
    project.scene.add(&s4_g_end);

    // Scene 5
    project.scene.add(&s5_lbl);
    project.scene.add(&s5);
    project.scene.add(&s5_g_line);
    project.scene.add(&s5_g_start);
    project.scene.add(&s5_g_end);

    // Scene 6
    project.scene.add(&s6_lbl);
    project.scene.add(&s6_grid);
    project.scene.add(&s6_g_line);
    project.scene.add(&s6_g_start);
    project.scene.add(&s6_g_end);

    // Scene 7
    project.scene.add(&s7_lbl);
    project.scene.add(&s7);
    project.scene.add(&s7_g_line);
    project.scene.add(&s7_g_start);
    project.scene.add(&s7_g_end);

    // Scene 8
    project.scene.add(&s8_lbl);
    project.scene.add(&s8);
    project.scene.add(&s8_g_line);
    project.scene.add(&s8_g_start);
    project.scene.add(&s8_g_end);

    // Scene 9
    project.scene.add(&s9_lbl);
    project.scene.add(&s9);
    project.scene.add(&s9_g_line);
    project.scene.add(&s9_g_start);
    project.scene.add(&s9_g_end);

    // Scene 10
    project.scene.add(&s10_lbl);
    project.scene.add(&s10);
    project.scene.add(&s10_g_rad_radius);
    project.scene.add(&s10_g_rad_center);
    project.scene.add(&s10_g_lin_line);
    project.scene.add(&s10_g_lin_start);
    project.scene.add(&s10_g_lin_end);

    // Timeline — one scene at a time, each with its own gradient animation
    project.scene.video_timeline.add(chain![
        // Intro
        all![
            grid.opacity.to(1.0, Duration::from_millis(800)),
            title.opacity.to(1.0, Duration::from_millis(800)),
        ],
        wait!(1),
        // ─── 1: Circle linear fill — morph colors, then direction ───
        all![
            s1_lbl.opacity.to(1.0, FADE_IN),
            s1.opacity.to(1.0, FADE_IN),
            s1.scale.to(Vec2::ONE, FADE_IN),
            s1_g_start.opacity.to(1.0, FADE_IN),
            s1_g_end.opacity.to(1.0, FADE_IN),
            s1_g_line.opacity.to(1.0, FADE_IN),
        ],
        wait!(1),
        // Color morph: sunset → ocean
        all![
            s1.fill_paint
                .to(Some(Paint::Gradient(ocean.clone())), MORPH),
            s1_g_start.position.to(Vec2::new(cx - 120.0, cy), MORPH),
            s1_g_end.position.to(Vec2::new(cx + 120.0, cy), MORPH),
            s1_g_line.start.to(Vec2::new(-120.0, 0.0), MORPH),
            s1_g_line.end.to(Vec2::new(120.0, 0.0), MORPH),
            s1_g_start.stroke_color.to(CYAN, MORPH),
            s1_g_end.stroke_color.to(INDIGO, MORPH),
        ],
        wait!(1),
        // Color morph: ocean → sunset
        all![
            s1.fill_paint
                .to(Some(Paint::Gradient(sunset.clone())), MORPH),
            s1_g_start
                .position
                .to(Vec2::new(cx - 120.0, cy - 120.0), MORPH),
            s1_g_end
                .position
                .to(Vec2::new(cx + 120.0, cy + 120.0), MORPH),
            s1_g_line.start.to(Vec2::new(-120.0, -120.0), MORPH),
            s1_g_line.end.to(Vec2::new(120.0, 120.0), MORPH),
            s1_g_start.stroke_color.to(CORAL, MORPH),
            s1_g_end.stroke_color.to(VIOLET, MORPH),
        ],
        wait!(1),
        // Direction morph: diagonal → horizontal sunset
        all![
            s1.fill_paint
                .to(Some(Paint::Gradient(sunset_horiz.clone())), MORPH),
            s1_g_start.position.to(Vec2::new(cx - 120.0, cy), MORPH),
            s1_g_end.position.to(Vec2::new(cx + 120.0, cy), MORPH),
            s1_g_line.start.to(Vec2::new(-120.0, 0.0), MORPH),
            s1_g_line.end.to(Vec2::new(120.0, 0.0), MORPH),
        ],
        wait!(1),
        // Direction morph: horizontal → vertical sunset
        all![
            s1.fill_paint
                .to(Some(Paint::Gradient(sunset_vert.clone())), MORPH),
            s1_g_start.position.to(Vec2::new(cx, cy - 120.0), MORPH),
            s1_g_end.position.to(Vec2::new(cx, cy + 120.0), MORPH),
            s1_g_line.start.to(Vec2::new(0.0, -120.0), MORPH),
            s1_g_line.end.to(Vec2::new(0.0, 120.0), MORPH),
        ],
        wait!(1),
        // Direction morph: vertical → diagonal sunset
        all![
            s1.fill_paint
                .to(Some(Paint::Gradient(sunset_diag.clone())), MORPH),
            s1_g_start
                .position
                .to(Vec2::new(cx - 120.0, cy - 120.0), MORPH),
            s1_g_end
                .position
                .to(Vec2::new(cx + 120.0, cy + 120.0), MORPH),
            s1_g_line.start.to(Vec2::new(-120.0, -120.0), MORPH),
            s1_g_line.end.to(Vec2::new(120.0, 120.0), MORPH),
        ],
        wait!(1),
        all![
            s1_lbl.opacity.to(0.0, FADE_OUT),
            s1.opacity.to(0.0, FADE_OUT),
            s1.scale.to(Vec2::splat(0.95), FADE_OUT),
            s1_g_start.opacity.to(0.0, FADE_OUT),
            s1_g_end.opacity.to(0.0, FADE_OUT),
            s1_g_line.opacity.to(0.0, FADE_OUT),
        ],
        // ─── 2: Rect radial fill — morph to another radial gradient and back ───
        all![
            s2_lbl.opacity.to(1.0, FADE_IN),
            s2.opacity.to(1.0, FADE_IN),
            s2.scale.to(Vec2::ONE, FADE_IN),
            s2_g_center.opacity.to(1.0, FADE_IN),
            s2_g_radius.opacity.to(1.0, FADE_IN),
        ],
        wait!(1),
        // Radial color and radius morph: glow (Fuchsia 130px) -> glow_alt (Cyan 180px)
        all![
            s2.fill_paint
                .to(Some(Paint::Gradient(glow_alt.clone())), MORPH),
            s2_g_radius.radius.to(180.0, MORPH),
            s2_g_center.stroke_color.to(CYAN, MORPH),
            s2_g_radius
                .stroke_color
                .to(Color::rgba8(0x00, 0xf2, 0xfe, 80), MORPH),
        ],
        wait!(1),
        // Morph back to glow
        all![
            s2.fill_paint.to(Some(Paint::Gradient(glow.clone())), MORPH),
            s2_g_radius.radius.to(130.0, MORPH),
            s2_g_center.stroke_color.to(FUCHSIA, MORPH),
            s2_g_radius
                .stroke_color
                .to(Color::rgba8(0xff, 0x00, 0x7f, 80), MORPH),
        ],
        wait!(1),
        all![
            s2_lbl.opacity.to(0.0, FADE_OUT),
            s2.opacity.to(0.0, FADE_OUT),
            s2.scale.to(Vec2::splat(0.95), FADE_OUT),
            s2_g_center.opacity.to(0.0, FADE_OUT),
            s2_g_radius.opacity.to(0.0, FADE_OUT),
        ],
        // ─── 3: Hex gradient stroke — morph colors, then direction ───
        all![
            s3_lbl.opacity.to(1.0, FADE_IN),
            s3.opacity.to(1.0, FADE_IN),
            s3.scale.to(Vec2::ONE, FADE_IN),
            s3_g_start.opacity.to(1.0, FADE_IN),
            s3_g_end.opacity.to(1.0, FADE_IN),
            s3_g_line.opacity.to(1.0, FADE_IN),
        ],
        wait!(1),
        // Morph: ocean (horizontal) -> sunset (diagonal)
        all![
            s3.stroke_paint
                .to(Some(Paint::Gradient(sunset.clone())), MORPH),
            s3_g_start
                .position
                .to(Vec2::new(cx - 120.0, cy - 120.0), MORPH),
            s3_g_end
                .position
                .to(Vec2::new(cx + 120.0, cy + 120.0), MORPH),
            s3_g_line.start.to(Vec2::new(-120.0, -120.0), MORPH),
            s3_g_line.end.to(Vec2::new(120.0, 120.0), MORPH),
            s3_g_start.stroke_color.to(CORAL, MORPH),
            s3_g_end.stroke_color.to(VIOLET, MORPH),
        ],
        wait!(1),
        // Morph: sunset (diagonal) -> ocean (horizontal)
        all![
            s3.stroke_paint
                .to(Some(Paint::Gradient(ocean.clone())), MORPH),
            s3_g_start.position.to(Vec2::new(cx - 120.0, cy), MORPH),
            s3_g_end.position.to(Vec2::new(cx + 120.0, cy), MORPH),
            s3_g_line.start.to(Vec2::new(-120.0, 0.0), MORPH),
            s3_g_line.end.to(Vec2::new(120.0, 0.0), MORPH),
            s3_g_start.stroke_color.to(CYAN, MORPH),
            s3_g_end.stroke_color.to(INDIGO, MORPH),
        ],
        wait!(1),
        // Direction morph: horizontal ocean -> vertical ocean
        all![
            s3.stroke_paint
                .to(Some(Paint::Gradient(ocean_vert.clone())), MORPH),
            s3_g_start.position.to(Vec2::new(cx, cy - 120.0), MORPH),
            s3_g_end.position.to(Vec2::new(cx, cy + 120.0), MORPH),
            s3_g_line.start.to(Vec2::new(0.0, -120.0), MORPH),
            s3_g_line.end.to(Vec2::new(0.0, 120.0), MORPH),
        ],
        wait!(1),
        // Direction morph: vertical ocean -> horizontal ocean
        all![
            s3.stroke_paint
                .to(Some(Paint::Gradient(ocean.clone())), MORPH),
            s3_g_start.position.to(Vec2::new(cx - 120.0, cy), MORPH),
            s3_g_end.position.to(Vec2::new(cx + 120.0, cy), MORPH),
            s3_g_line.start.to(Vec2::new(-120.0, 0.0), MORPH),
            s3_g_line.end.to(Vec2::new(120.0, 0.0), MORPH),
        ],
        wait!(1),
        all![
            s3_lbl.opacity.to(0.0, FADE_OUT),
            s3.opacity.to(0.0, FADE_OUT),
            s3.scale.to(Vec2::splat(0.95), FADE_OUT),
            s3_g_start.opacity.to(0.0, FADE_OUT),
            s3_g_end.opacity.to(0.0, FADE_OUT),
            s3_g_line.opacity.to(0.0, FADE_OUT),
        ],
        // ─── 4: Line gradient stroke ───
        all![
            s4_lbl.opacity.to(1.0, FADE_IN),
            s4.opacity.to(1.0, FADE_IN),
            s4_g_start.opacity.to(1.0, FADE_IN),
            s4_g_end.opacity.to(1.0, FADE_IN),
            s4_g_line.opacity.to(1.0, FADE_IN),
        ],
        wait!(1),
        // Morph: sunset (diagonal) -> ocean (horizontal)
        all![
            s4.stroke_paint
                .to(Some(Paint::Gradient(ocean.clone())), MORPH),
            s4_g_start.position.to(Vec2::new(cx - 120.0, cy), MORPH),
            s4_g_end.position.to(Vec2::new(cx + 120.0, cy), MORPH),
            s4_g_line.start.to(Vec2::new(-120.0, 0.0), MORPH),
            s4_g_line.end.to(Vec2::new(120.0, 0.0), MORPH),
            s4_g_start.stroke_color.to(CYAN, MORPH),
            s4_g_end.stroke_color.to(INDIGO, MORPH),
        ],
        wait!(1),
        // Morph: ocean (horizontal) -> sunset (diagonal)
        all![
            s4.stroke_paint
                .to(Some(Paint::Gradient(sunset.clone())), MORPH),
            s4_g_start
                .position
                .to(Vec2::new(cx - 120.0, cy - 120.0), MORPH),
            s4_g_end
                .position
                .to(Vec2::new(cx + 120.0, cy + 120.0), MORPH),
            s4_g_line.start.to(Vec2::new(-120.0, -120.0), MORPH),
            s4_g_line.end.to(Vec2::new(120.0, 120.0), MORPH),
            s4_g_start.stroke_color.to(CORAL, MORPH),
            s4_g_end.stroke_color.to(VIOLET, MORPH),
        ],
        wait!(1),
        all![
            s4_lbl.opacity.to(0.0, FADE_OUT),
            s4.opacity.to(0.0, FADE_OUT),
            s4_g_start.opacity.to(0.0, FADE_OUT),
            s4_g_end.opacity.to(0.0, FADE_OUT),
            s4_g_line.opacity.to(0.0, FADE_OUT),
        ],
        // ─── 5: Path gradient stroke ───
        all![
            s5_lbl.opacity.to(1.0, FADE_IN),
            s5.opacity.to(1.0, FADE_IN),
            s5_g_start.opacity.to(1.0, FADE_IN),
            s5_g_end.opacity.to(1.0, FADE_IN),
            s5_g_line.opacity.to(1.0, FADE_IN),
        ],
        wait!(1),
        // Morph: ocean (horizontal) -> sunset (diagonal)
        all![
            s5.stroke_paint
                .to(Some(Paint::Gradient(sunset.clone())), MORPH),
            s5_g_start
                .position
                .to(Vec2::new(cx - 120.0, cy - 120.0), MORPH),
            s5_g_end
                .position
                .to(Vec2::new(cx + 120.0, cy + 120.0), MORPH),
            s5_g_line.start.to(Vec2::new(-120.0, -120.0), MORPH),
            s5_g_line.end.to(Vec2::new(120.0, 120.0), MORPH),
            s5_g_start.stroke_color.to(CORAL, MORPH),
            s5_g_end.stroke_color.to(VIOLET, MORPH),
        ],
        wait!(1),
        // Morph: sunset (diagonal) -> ocean (horizontal)
        all![
            s5.stroke_paint
                .to(Some(Paint::Gradient(ocean.clone())), MORPH),
            s5_g_start.position.to(Vec2::new(cx - 120.0, cy), MORPH),
            s5_g_end.position.to(Vec2::new(cx + 120.0, cy), MORPH),
            s5_g_line.start.to(Vec2::new(-120.0, 0.0), MORPH),
            s5_g_line.end.to(Vec2::new(120.0, 0.0), MORPH),
            s5_g_start.stroke_color.to(CYAN, MORPH),
            s5_g_end.stroke_color.to(INDIGO, MORPH),
        ],
        wait!(1),
        all![
            s5_lbl.opacity.to(0.0, FADE_OUT),
            s5.opacity.to(0.0, FADE_OUT),
            s5_g_start.opacity.to(0.0, FADE_OUT),
            s5_g_end.opacity.to(0.0, FADE_OUT),
            s5_g_line.opacity.to(0.0, FADE_OUT),
        ],
        // ─── 6: Grid gradient stroke ───
        all![
            s6_lbl.opacity.to(1.0, FADE_IN),
            s6_grid.opacity.to(1.0, FADE_IN),
            s6_g_start.opacity.to(1.0, FADE_IN),
            s6_g_end.opacity.to(1.0, FADE_IN),
            s6_g_line.opacity.to(1.0, FADE_IN),
        ],
        wait!(1),
        // Morph: ocean (horizontal) -> sunset (diagonal)
        all![
            s6_grid
                .stroke_paint
                .to(Some(Paint::Gradient(sunset.clone())), MORPH),
            s6_g_start
                .position
                .to(Vec2::new(cx - 120.0, cy - 120.0), MORPH),
            s6_g_end
                .position
                .to(Vec2::new(cx + 120.0, cy + 120.0), MORPH),
            s6_g_line.start.to(Vec2::new(-120.0, -120.0), MORPH),
            s6_g_line.end.to(Vec2::new(120.0, 120.0), MORPH),
            s6_g_start.stroke_color.to(CORAL, MORPH),
            s6_g_end.stroke_color.to(VIOLET, MORPH),
        ],
        wait!(1),
        // Morph: sunset (diagonal) -> ocean (horizontal)
        all![
            s6_grid
                .stroke_paint
                .to(Some(Paint::Gradient(ocean.clone())), MORPH),
            s6_g_start.position.to(Vec2::new(cx - 120.0, cy), MORPH),
            s6_g_end.position.to(Vec2::new(cx + 120.0, cy), MORPH),
            s6_g_line.start.to(Vec2::new(-120.0, 0.0), MORPH),
            s6_g_line.end.to(Vec2::new(120.0, 0.0), MORPH),
            s6_g_start.stroke_color.to(CYAN, MORPH),
            s6_g_end.stroke_color.to(INDIGO, MORPH),
        ],
        wait!(1),
        all![
            s6_lbl.opacity.to(0.0, FADE_OUT),
            s6_grid.opacity.to(0.0, FADE_OUT),
            s6_g_start.opacity.to(0.0, FADE_OUT),
            s6_g_end.opacity.to(0.0, FADE_OUT),
            s6_g_line.opacity.to(0.0, FADE_OUT),
        ],
        // ─── 7: Text — morph colors, then direction ───
        all![
            s7_lbl.opacity.to(1.0, FADE_IN),
            s7.opacity.to(1.0, FADE_IN),
            s7.scale.to(Vec2::ONE, FADE_IN),
            s7_g_start.opacity.to(1.0, FADE_IN),
            s7_g_end.opacity.to(1.0, FADE_IN),
            s7_g_line.opacity.to(1.0, FADE_IN),
        ],
        wait!(1),
        // Morph: ocean (horizontal) -> sunset (diagonal)
        all![
            s7.fill_paint
                .to(Some(Paint::Gradient(sunset.clone())), MORPH),
            s7_g_start
                .position
                .to(Vec2::new(cx - 120.0, cy - 120.0), MORPH),
            s7_g_end
                .position
                .to(Vec2::new(cx + 120.0, cy + 120.0), MORPH),
            s7_g_line.start.to(Vec2::new(-120.0, -120.0), MORPH),
            s7_g_line.end.to(Vec2::new(120.0, 120.0), MORPH),
            s7_g_start.stroke_color.to(CORAL, MORPH),
            s7_g_end.stroke_color.to(VIOLET, MORPH),
        ],
        wait!(1),
        // Morph: sunset (diagonal) -> ocean (horizontal)
        all![
            s7.fill_paint
                .to(Some(Paint::Gradient(ocean.clone())), MORPH),
            s7_g_start.position.to(Vec2::new(cx - 120.0, cy), MORPH),
            s7_g_end.position.to(Vec2::new(cx + 120.0, cy), MORPH),
            s7_g_line.start.to(Vec2::new(-120.0, 0.0), MORPH),
            s7_g_line.end.to(Vec2::new(120.0, 0.0), MORPH),
            s7_g_start.stroke_color.to(CYAN, MORPH),
            s7_g_end.stroke_color.to(INDIGO, MORPH),
        ],
        wait!(1),
        // Morph: horizontal ocean -> vertical ocean
        all![
            s7.fill_paint
                .to(Some(Paint::Gradient(ocean_vert.clone())), MORPH),
            s7_g_start.position.to(Vec2::new(cx, cy - 120.0), MORPH),
            s7_g_end.position.to(Vec2::new(cx, cy + 120.0), MORPH),
            s7_g_line.start.to(Vec2::new(0.0, -120.0), MORPH),
            s7_g_line.end.to(Vec2::new(0.0, 120.0), MORPH),
        ],
        wait!(1),
        // Morph: vertical ocean -> horizontal ocean
        all![
            s7.fill_paint
                .to(Some(Paint::Gradient(ocean.clone())), MORPH),
            s7_g_start.position.to(Vec2::new(cx - 120.0, cy), MORPH),
            s7_g_end.position.to(Vec2::new(cx + 120.0, cy), MORPH),
            s7_g_line.start.to(Vec2::new(-120.0, 0.0), MORPH),
            s7_g_line.end.to(Vec2::new(120.0, 0.0), MORPH),
        ],
        wait!(1),
        all![
            s7_lbl.opacity.to(0.0, FADE_OUT),
            s7.opacity.to(0.0, FADE_OUT),
            s7.scale.to(Vec2::splat(0.95), FADE_OUT),
            s7_g_start.opacity.to(0.0, FADE_OUT),
            s7_g_end.opacity.to(0.0, FADE_OUT),
            s7_g_line.opacity.to(0.0, FADE_OUT),
        ],
        // ─── 8: Math ───
        all![
            s8_lbl.opacity.to(1.0, FADE_IN),
            s8.opacity.to(1.0, FADE_IN),
            s8.scale.to(Vec2::ONE, FADE_IN),
            s8_g_start.opacity.to(1.0, FADE_IN),
            s8_g_end.opacity.to(1.0, FADE_IN),
            s8_g_line.opacity.to(1.0, FADE_IN),
        ],
        wait!(1),
        // Morph: sunset (diagonal) -> ocean (horizontal)
        all![
            s8.fill_paint
                .to(Some(Paint::Gradient(ocean.clone())), MORPH),
            s8_g_start.position.to(Vec2::new(cx - 120.0, cy), MORPH),
            s8_g_end.position.to(Vec2::new(cx + 120.0, cy), MORPH),
            s8_g_line.start.to(Vec2::new(-120.0, 0.0), MORPH),
            s8_g_line.end.to(Vec2::new(120.0, 0.0), MORPH),
            s8_g_start.stroke_color.to(CYAN, MORPH),
            s8_g_end.stroke_color.to(INDIGO, MORPH),
        ],
        wait!(1),
        // Morph: ocean (horizontal) -> sunset (diagonal)
        all![
            s8.fill_paint
                .to(Some(Paint::Gradient(sunset.clone())), MORPH),
            s8_g_start
                .position
                .to(Vec2::new(cx - 120.0, cy - 120.0), MORPH),
            s8_g_end
                .position
                .to(Vec2::new(cx + 120.0, cy + 120.0), MORPH),
            s8_g_line.start.to(Vec2::new(-120.0, -120.0), MORPH),
            s8_g_line.end.to(Vec2::new(120.0, 120.0), MORPH),
            s8_g_start.stroke_color.to(CORAL, MORPH),
            s8_g_end.stroke_color.to(VIOLET, MORPH),
        ],
        wait!(1),
        all![
            s8_lbl.opacity.to(0.0, FADE_OUT),
            s8.opacity.to(0.0, FADE_OUT),
            s8.scale.to(Vec2::splat(0.95), FADE_OUT),
            s8_g_start.opacity.to(0.0, FADE_OUT),
            s8_g_end.opacity.to(0.0, FADE_OUT),
            s8_g_line.opacity.to(0.0, FADE_OUT),
        ],
        // ─── 9: Solid ↔ Gradient ───
        all![
            s9_lbl.opacity.to(1.0, FADE_IN),
            s9.opacity.to(1.0, FADE_IN),
            s9.scale.to(Vec2::ONE, FADE_IN),
        ],
        wait!(1),
        // Morph: solid white -> sunset (diagonal gradient) + fade in guides
        all![
            s9.fill_paint
                .to(Some(Paint::Gradient(sunset.clone())), MORPH),
            s9_g_start.opacity.to(1.0, MORPH),
            s9_g_end.opacity.to(1.0, MORPH),
            s9_g_line.opacity.to(1.0, MORPH),
        ],
        wait!(1),
        // Morph: sunset -> solid white + fade out guides
        all![
            s9.fill_paint.to(Some(Paint::Solid(Color::WHITE)), MORPH),
            s9_g_start.opacity.to(0.0, MORPH),
            s9_g_end.opacity.to(0.0, MORPH),
            s9_g_line.opacity.to(0.0, MORPH),
        ],
        wait!(1),
        all![
            s9_lbl.opacity.to(0.0, FADE_OUT),
            s9.opacity.to(0.0, FADE_OUT),
            s9.scale.to(Vec2::splat(0.95), FADE_OUT),
        ],
        // ─── 10: Radial ↔ Linear Morph ───
        all![
            s10_lbl.opacity.to(1.0, FADE_IN),
            s10.opacity.to(1.0, FADE_IN),
            s10.scale.to(Vec2::ONE, FADE_IN),
            s10_g_rad_center.opacity.to(1.0, FADE_IN),
            s10_g_rad_radius.opacity.to(1.0, FADE_IN),
        ],
        wait!(1),
        // Morph Radial (glow) to Linear (sunset) + cross-fade the visual guides
        all![
            s10.fill_paint
                .to(Some(Paint::Gradient(sunset.clone())), MORPH),
            s10_g_rad_center.opacity.to(0.0, MORPH),
            s10_g_rad_radius.opacity.to(0.0, MORPH),
            s10_g_lin_start.opacity.to(1.0, MORPH),
            s10_g_lin_end.opacity.to(1.0, MORPH),
            s10_g_lin_line.opacity.to(1.0, MORPH),
        ],
        wait!(1),
        // Morph back from Linear (sunset) to Radial (glow)
        all![
            s10.fill_paint
                .to(Some(Paint::Gradient(glow.clone())), MORPH),
            s10_g_rad_center.opacity.to(1.0, MORPH),
            s10_g_rad_radius.opacity.to(1.0, MORPH),
            s10_g_lin_start.opacity.to(0.0, MORPH),
            s10_g_lin_end.opacity.to(0.0, MORPH),
            s10_g_lin_line.opacity.to(0.0, MORPH),
        ],
        wait!(1),
        // Fade everything out
        all![
            s10_lbl.opacity.to(0.0, FADE_OUT),
            s10.opacity.to(0.0, FADE_OUT),
            s10_g_rad_center.opacity.to(0.0, FADE_OUT),
            s10_g_rad_radius.opacity.to(0.0, FADE_OUT),
            title.opacity.to(0.0, FADE_OUT),
            grid.opacity.to(0.0, FADE_OUT),
        ],
    ]);

    project.show().expect("Failed to render animation");
}
