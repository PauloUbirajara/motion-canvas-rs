use motion_canvas_rs::prelude::*;
use std::{ops::Add, time::Duration};

/// Country landmark data: (name, x, y) on a 1010x666 map (SVG)
struct Landmark {
    name: &'static str,
    x: f32,
    y: f32,
}

/// Map native dimensions (from SVG viewBox: 1009.7 x 666)
const MAP_W: f32 = 800.0;
const MAP_H: f32 = 600.0;

/// Coordinates on the 800x600 map grid.
const LANDMARKS: [Landmark; 22] = [
    Landmark {
        name: "Iceland",
        x: 337.0,
        y: 215.0,
    },
    Landmark {
        name: "Norway",
        x: 397.0,
        y: 235.0,
    },
    Landmark {
        name: "Sweden",
        x: 412.0,
        y: 225.0,
    },
    Landmark {
        name: "Finland",
        x: 437.0,
        y: 220.0,
    },
    Landmark {
        name: "Denmark",
        x: 399.0,
        y: 254.0,
    },
    Landmark {
        name: "Netherlands",
        x: 390.0,
        y: 268.0,
    },
    Landmark {
        name: "Luxembourg",
        x: 392.0,
        y: 278.0,
    },
    Landmark {
        name: "Germany",
        x: 400.0,
        y: 270.0,
    },
    Landmark {
        name: "Switzerland",
        x: 396.0,
        y: 286.0,
    },
    Landmark {
        name: "Egypt",
        x: 442.0,
        y: 340.0,
    },
    Landmark {
        name: "Russia",
        x: 542.0,
        y: 220.0,
    },
    Landmark {
        name: "China",
        x: 602.0,
        y: 320.0,
    },
    Landmark {
        name: "Singapore",
        x: 600.0,
        y: 395.0,
    },
    Landmark {
        name: "Japan",
        x: 677.0,
        y: 315.0,
    },
    Landmark {
        name: "Australia",
        x: 665.0,
        y: 455.0,
    },
    Landmark {
        name: "New Zealand",
        x: 749.0,
        y: 499.0,
    },
    Landmark {
        name: "Brazil",
        x: 272.0,
        y: 425.0,
    },
    Landmark {
        name: "Paraguay",
        x: 254.0,
        y: 452.0,
    },
    Landmark {
        name: "El Salvador",
        x: 189.0,
        y: 371.0,
    },
    Landmark {
        name: "Mexico",
        x: 157.0,
        y: 345.0,
    },
    Landmark {
        name: "USA",
        x: 177.0,
        y: 310.0,
    },
    Landmark {
        name: "Canada",
        x: 167.0,
        y: 240.0,
    },
];

/// Helper: duration that starts slow and speeds up
fn tour_duration(index: usize) -> Duration {
    // First leg: 4.5s, last leg: 1.2s. Linear interpolation.
    let t = index as f32 / (LANDMARKS.len() - 1) as f32;
    let secs = 4.0 * (1.1 - t) + 1.2 * t;
    Duration::from_secs_f32(secs)
}

/// Sky blue background
const SKY_BG: Color = Color::rgb8(0xcf, 0xe5, 0xe8);

/// Center of map in world coords
const MAP_CX: f32 = MAP_W / 2.0;
const MAP_CY: f32 = MAP_H / 2.0;

/// Cloud definition helper
struct CloudDef {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    img: &'static str,
    flip_x: bool,
    opacity: f32,
    /// Exit direction (x, y) when clearing
    exit_x: f32,
    exit_y: f32,
}

const CLOUDS: [CloudDef; 24] = [
    // Layer 1: Large foreground clouds (dense cover)
    CloudDef {
        x: 150.0,
        y: 180.0,
        w: 400.0,
        h: 200.0,
        img: "cloud-1.png",
        flip_x: false,
        opacity: 0.95,
        exit_x: -500.0,
        exit_y: 100.0,
    },
    CloudDef {
        x: 800.0,
        y: 140.0,
        w: 380.0,
        h: 180.0,
        img: "cloud-2.png",
        flip_x: false,
        opacity: 0.92,
        exit_x: 1600.0,
        exit_y: 50.0,
    },
    CloudDef {
        x: 500.0,
        y: 480.0,
        w: 450.0,
        h: 210.0,
        img: "cloud-3.png",
        flip_x: false,
        opacity: 0.90,
        exit_x: 500.0,
        exit_y: 900.0,
    },
    CloudDef {
        x: 920.0,
        y: 350.0,
        w: 350.0,
        h: 160.0,
        img: "cloud-1.png",
        flip_x: true,
        opacity: 0.88,
        exit_x: 1500.0,
        exit_y: 450.0,
    },
    CloudDef {
        x: 400.0,
        y: 320.0,
        w: 420.0,
        h: 200.0,
        img: "cloud-2.png",
        flip_x: true,
        opacity: 0.93,
        exit_x: -500.0,
        exit_y: 400.0,
    },
    CloudDef {
        x: 700.0,
        y: 500.0,
        w: 400.0,
        h: 190.0,
        img: "cloud-3.png",
        flip_x: false,
        opacity: 0.91,
        exit_x: 1400.0,
        exit_y: 800.0,
    },
    // Layer 2: Medium clouds
    CloudDef {
        x: 50.0,
        y: 400.0,
        w: 350.0,
        h: 170.0,
        img: "cloud-2.png",
        flip_x: true,
        opacity: 0.82,
        exit_x: -450.0,
        exit_y: 500.0,
    },
    CloudDef {
        x: 350.0,
        y: 90.0,
        w: 320.0,
        h: 150.0,
        img: "cloud-3.png",
        flip_x: true,
        opacity: 0.80,
        exit_x: 350.0,
        exit_y: -300.0,
    },
    CloudDef {
        x: 700.0,
        y: 300.0,
        w: 300.0,
        h: 140.0,
        img: "cloud-1.png",
        flip_x: false,
        opacity: 0.78,
        exit_x: 1400.0,
        exit_y: 300.0,
    },
    CloudDef {
        x: 200.0,
        y: 550.0,
        w: 380.0,
        h: 180.0,
        img: "cloud-2.png",
        flip_x: false,
        opacity: 0.76,
        exit_x: -400.0,
        exit_y: 750.0,
    },
    CloudDef {
        x: 950.0,
        y: 100.0,
        w: 330.0,
        h: 155.0,
        img: "cloud-1.png",
        flip_x: true,
        opacity: 0.79,
        exit_x: 1500.0,
        exit_y: -200.0,
    },
    CloudDef {
        x: 100.0,
        y: 100.0,
        w: 340.0,
        h: 160.0,
        img: "cloud-3.png",
        flip_x: false,
        opacity: 0.77,
        exit_x: -400.0,
        exit_y: -200.0,
    },
    // Layer 3: Small wisps filling gaps
    CloudDef {
        x: 600.0,
        y: 200.0,
        w: 250.0,
        h: 120.0,
        img: "cloud-3.png",
        flip_x: false,
        opacity: 0.72,
        exit_x: 1300.0,
        exit_y: 100.0,
    },
    CloudDef {
        x: 100.0,
        y: 280.0,
        w: 280.0,
        h: 130.0,
        img: "cloud-1.png",
        flip_x: true,
        opacity: 0.70,
        exit_x: -380.0,
        exit_y: 280.0,
    },
    CloudDef {
        x: 450.0,
        y: 350.0,
        w: 260.0,
        h: 120.0,
        img: "cloud-2.png",
        flip_x: true,
        opacity: 0.68,
        exit_x: 450.0,
        exit_y: 800.0,
    },
    CloudDef {
        x: 850.0,
        y: 500.0,
        w: 300.0,
        h: 140.0,
        img: "cloud-3.png",
        flip_x: true,
        opacity: 0.65,
        exit_x: 1400.0,
        exit_y: 700.0,
    },
    CloudDef {
        x: 250.0,
        y: 450.0,
        w: 270.0,
        h: 125.0,
        img: "cloud-1.png",
        flip_x: false,
        opacity: 0.67,
        exit_x: -350.0,
        exit_y: 600.0,
    },
    CloudDef {
        x: 550.0,
        y: 100.0,
        w: 290.0,
        h: 135.0,
        img: "cloud-2.png",
        flip_x: true,
        opacity: 0.69,
        exit_x: 550.0,
        exit_y: -300.0,
    },
    // Layer 4: Tiny accent clouds (extra cover)
    CloudDef {
        x: 300.0,
        y: 150.0,
        w: 200.0,
        h: 100.0,
        img: "cloud-1.png",
        flip_x: false,
        opacity: 0.60,
        exit_x: -300.0,
        exit_y: 50.0,
    },
    CloudDef {
        x: 750.0,
        y: 450.0,
        w: 220.0,
        h: 100.0,
        img: "cloud-2.png",
        flip_x: false,
        opacity: 0.58,
        exit_x: 1300.0,
        exit_y: 550.0,
    },
    CloudDef {
        x: 550.0,
        y: 550.0,
        w: 240.0,
        h: 110.0,
        img: "cloud-3.png",
        flip_x: true,
        opacity: 0.55,
        exit_x: 550.0,
        exit_y: 850.0,
    },
    CloudDef {
        x: 950.0,
        y: 200.0,
        w: 200.0,
        h: 90.0,
        img: "cloud-1.png",
        flip_x: true,
        opacity: 0.52,
        exit_x: 1500.0,
        exit_y: 100.0,
    },
    CloudDef {
        x: 50.0,
        y: 550.0,
        w: 230.0,
        h: 105.0,
        img: "cloud-3.png",
        flip_x: false,
        opacity: 0.56,
        exit_x: -350.0,
        exit_y: 800.0,
    },
    CloudDef {
        x: 830.0,
        y: 250.0,
        w: 210.0,
        h: 95.0,
        img: "cloud-2.png",
        flip_x: true,
        opacity: 0.54,
        exit_x: 1400.0,
        exit_y: 150.0,
    },
];

fn main() {
    let w = 800u32;
    let h = 600u32;

    let mut project = Project::new(w, h)
        .with_fps(60)
        .with_title("World Map")
        .with_background(SKY_BG)
        .close_on_finish();

    // ── Map (rendered at native SVG size for crispness) ──
    let map = SvgNode::default()
        .with_position(Vec2::new(MAP_CX, MAP_CY))
        .with_path("./examples/images/world.svg")
        .with_size(Vec2::new(MAP_W, MAP_H))
        .with_opacity(0.0);

    // ── Clouds — 16 total from 3 source PNGs with variants ──
    let mut cloud_nodes: Vec<ImageNode> = Vec::new();
    for def in &CLOUDS {
        let mut node = ImageNode::default()
            .with_position(Vec2::new(def.x, def.y))
            .with_path(&format!("./examples/images/{}", def.img))
            .with_size(Vec2::new(def.w, def.h))
            .with_opacity(def.opacity);
        if def.flip_x {
            node = node.with_scale_xy(Vec2::new(-1.0, 1.0));
        }
        cloud_nodes.push(node);
    }

    // ── Camera ──
    let camera = CameraNode::default()
        .with_size(Vec2::new(w as f32, h as f32))
        .with_position(Vec2::new(MAP_CX, MAP_CY))
        .with_zoom(1.0)
        .with_centered(true);

    // ── Plane ──
    let plane = SvgNode::default()
        .with_position(Vec2::new(LANDMARKS[0].x, LANDMARKS[0].y))
        .with_path("./examples/images/plane.svg")
        .with_size(Vec2::new(12.0, 12.0))
        .with_anchor(Vec2::new(2.0, -2.65))
        .with_scale(0.0)
        .with_opacity(0.0);

    // ── Landmark pins + name labels ──
    let mut pins: Vec<Circle> = Vec::new();
    let mut name_labels: Vec<TextNode> = Vec::new();
    for lm in &LANDMARKS {
        let pin = Circle::default()
            .with_position(Vec2::new(lm.x, lm.y))
            .with_radius(0.0)
            .with_fill(Palette::RED)
            .with_stroke(Color::rgba8(0xff, 0xff, 0xff, 180), 2.0)
            .with_opacity(0.0);
        pins.push(pin);

        // Country name below pin
        let name_lbl = TextNode::default()
            .with_position(Vec2::new(lm.x, lm.y + 12.0))
            .with_text(lm.name)
            .with_font_size(10.0)
            .with_fill(Palette::DARK_GRAY)
            .with_opacity(0.0);
        name_labels.push(name_lbl);
    }

    // ── Route lines (straight between consecutive landmarks) ──
    let mut route_lines: Vec<Line> = Vec::new();
    for i in 0..LANDMARKS.len() - 1 {
        let from = &LANDMARKS[i];
        let line = Line::default()
            .with_start(Vec2::new(from.x, from.y))
            .with_end(Vec2::new(from.x, from.y))
            .with_stroke(Color::rgba8(0xff, 0xff, 0xff, 180), 2.0)
            .with_opacity(0.0);
        route_lines.push(line);
    }

    // ── Build Scene Tree ──
    // Order: map → route lines → plane → pins → labels (pins on top of lines)
    let mut camera_children: Vec<Box<dyn Node>> = Vec::new();
    camera_children.push(Box::new(map.clone()));
    // Route lines first (behind pins)
    for rl in &route_lines {
        camera_children.push(Box::new(rl.clone()));
    }
    // Plane between routes and pins
    camera_children.push(Box::new(plane.clone()));
    // Pins on top of route lines
    for pin in &pins {
        camera_children.push(Box::new(pin.clone()));
    }

    for nl in &name_labels {
        camera_children.push(Box::new(nl.clone()));
    }

    // Clouds on top of everything (so they cover the map initially)
    for cn in &cloud_nodes {
        camera_children.push(Box::new(cn.clone()));
    }

    let camera = camera.with_nodes(camera_children);
    project.scene.add(camera.clone());

    // ═══════════════════════════════════════════════════
    //  ANIMATION TIMELINE
    // ═══════════════════════════════════════════════════

    // Phase 1: Intro — clouds everywhere, then zoom in / scatter clouds / reveal map
    let mut phase1_anims: Vec<AnyAnimation> = Vec::new();

    // Camera zoom
    phase1_anims.push(
        camera
            .zoom
            .to(2.25, Duration::from_secs(3))
            .ease(easings::cubic_in_out)
            .into(),
    );
    // Map fade in
    phase1_anims.push(
        map.opacity
            .to(1.0, Duration::from_secs(2))
            .ease(easings::cubic_out)
            .into(),
    );

    // Scatter all clouds
    for (i, cn) in cloud_nodes.iter().enumerate() {
        let def = &CLOUDS[i];
        phase1_anims.push(
            cn.position
                .to(Vec2::new(def.exit_x, def.exit_y), Duration::from_secs(3))
                .ease(easings::cubic_in)
                .into(),
        );
        phase1_anims.push(
            cn.opacity
                .to(0.0, Duration::from_secs(2))
                .ease(easings::cubic_in)
                .into(),
        );
    }

    let phase1_intro: AnyAnimation = chain![
        // Everything happens at once: zoom, clouds scatter, map reveals, title appears
        all(phase1_anims),
        wait(Duration::from_millis(500)),
    ];

    // Phase 2: Pan camera to first landmark and show plane
    let first = &LANDMARKS[0];
    let phase2_start: AnyAnimation = chain![
        camera
            .position
            .to(Vec2::new(first.x, first.y), Duration::from_secs(2))
            .ease(easings::cubic_in_out),
        camera
            .zoom
            .to(5.0, Duration::from_secs(1))
            .ease(easings::cubic_out),
        // Show first pin + labels
        all![
            pins[0].opacity.to(1.0, Duration::from_millis(400)),
            pins[0]
                .radius
                .to(3.0, Duration::from_millis(600))
                .ease(easings::elastic_out),
            name_labels[0].opacity.to(1.0, Duration::from_millis(400)),
        ],
        // Show plane
        plane.opacity.to(1.0, Duration::from_millis(300)),
        wait(Duration::from_millis(800)),
    ];

    // Phase 3: Tour through each country (scale in -> move -> scale out -> show landmark)
    let mut tour_legs: Vec<AnyAnimation> = Vec::new();
    for i in 0..LANDMARKS.len() - 1 {
        let from = &LANDMARKS[i];
        let to = &LANDMARKS[i + 1];
        let dur = tour_duration(i);

        // Compute heading angle from straight line
        let dx = to.x - from.x;
        let dy = to.y - from.y;
        let angle = dy.atan2(dx) + (std::f32::consts::PI);

        let leg: AnyAnimation = chain![
            // 1. Scale plane in at the start of the leg
            all![
                plane
                    .rotation
                    .to(angle, Duration::from_millis(200))
                    .ease(easings::cubic_out),
                plane
                    .scale
                    .to(Vec2::splat(1.0), Duration::from_millis(300))
                    .ease(easings::cubic_out),
                // Hide current landmark name as we depart
                name_labels[i]
                    .opacity
                    .to(0.0, Duration::from_millis(200))
                    .ease(easings::cubic_out),
            ],
            // 2. Move plane to destination
            all![
                // Show route line
                route_lines[i].opacity.to(1.0, Duration::from_millis(100)),
                // Draw line to destination
                route_lines[i]
                    .end
                    .to(Vec2::new(to.x, to.y), dur)
                    .ease(easings::cubic_in_out),
                all![
                    // Move plane to destination
                    plane
                        .position
                        .to(Vec2::new(to.x, to.y), dur)
                        .ease(easings::cubic_in_out),
                    // Camera follows to destination
                    camera
                        .position
                        .to(Vec2::new(to.x, to.y), dur.add(Duration::from_millis(500)))
                        .ease(easings::cubic_in_out),
                ]
            ],
            // 3. Scale plane out at the destination
            plane
                .scale
                .to(Vec2::splat(0.0), Duration::from_millis(300))
                .ease(easings::cubic_in),
            // 4. Reveal destination pin + name
            all![
                pins[i + 1].opacity.to(1.0, Duration::from_millis(300)),
                pins[i + 1]
                    .radius
                    .to(3.0, Duration::from_millis(500))
                    .ease(easings::elastic_out),
                name_labels[i + 1]
                    .opacity
                    .to(1.0, Duration::from_millis(300)),
            ],
            // 5. Wait a bit for viewer to read
            wait(Duration::from_millis(600)),
            // 6. Name disappears before plane scales in for next leg
            name_labels[i + 1]
                .opacity
                .to(0.0, Duration::from_millis(200))
                .ease(easings::cubic_out),
        ];
        tour_legs.push(leg);
    }

    let phase3_tour = chain(tour_legs);

    // Phase 4: Zoom out to show full map
    let phase4_finale: AnyAnimation = chain![
        // Hide the last landmark's name
        name_labels[LANDMARKS.len() - 1]
            .opacity
            .to(0.0, Duration::from_millis(200)),
        wait(Duration::from_millis(300)),
        all![
            camera
                .position
                .to(Vec2::new(MAP_CX, MAP_CY), Duration::from_secs(3))
                .ease(easings::cubic_in_out),
            camera
                .zoom
                .to(1.0, Duration::from_secs(3))
                .ease(easings::sine_in_out),
        ],
        // Pulse all pins (dynamic loop)
        {
            let mut pulse_anims: Vec<AnyAnimation> = Vec::new();
            for pin in &pins {
                pulse_anims.push(
                    pin.radius
                        .to(10.0, Duration::from_millis(300))
                        .ease(easings::elastic_out)
                        .into(),
                );
            }
            sequence(Duration::from_millis(60), pulse_anims)
        },
        plane.opacity.to(0.0, Duration::from_millis(500)),
        wait(Duration::from_secs(3)),
    ];

    project.scene.video_timeline.add(chain![
        phase1_intro,
        phase2_start,
        phase3_tour,
        phase4_finale,
    ]);

    project.show().expect("Failed to render");
}
