use motion_canvas_rs::prelude::*;
use std::time::Duration;

fn main() {
    // 1. Initialize Project with full API coverage
    let mut project = Project::default()
        .with_fps(120)
        .with_gpu(true)
        .with_cache(true)
        .with_ffmpeg(true)
        .with_output_path("output")
        .with_title("Advanced Flow")
        .close_on_finish();

    // 2. Setup Nodes
    let mut path = BezPath::new();
    path.move_to((100.0, 300.0));
    path.curve_to((250.0, 100.0), (550.0, 500.0), (700.0, 300.0));
    let path_node = PathNode::default()
        .with_path(path)
        .with_stroke(Color::rgb8(0x44, 0x44, 0x44), 2.0);
    let follower = Circle::default()
        .with_position(Vec2::new(100.0, 300.0))
        .with_radius(20.0)
        .with_fill(Color::rgb8(0xe1, 0x32, 0x38)); // Red

    // Showcase: Rect and Line
    let background_rect = Rect::default()
        .with_position(Vec2::new(400.0, 300.0))
        .with_size(Vec2::new(760.0, 560.0))
        .with_fill(Color::rgba8(0x33, 0x33, 0x33, 150))
        .with_radius(20.0);

    let divider_line = Line::default()
        .with_start(Vec2::new(0.0, 300.0))
        .with_end(Vec2::new(0.0, 300.0))
        .with_stroke(Color::rgb8(0x44, 0x44, 0x44), 1.0);
    let title_text = TextNode::default()
        .with_position(Vec2::new(50.0, 50.0))
        .with_anchor(Vec2::new(-1.0, -1.0))
        .with_text("Motion Canvas in Rust")
        .with_font_size(40.0)
        .with_fill(Color::rgb8(0x68, 0xab, 0xdf)) // Blue
        .with_font("JetBrains Mono");

    let code_block = CodeNode::default()
        .with_anchor(Vec2::new(-1.0, -1.0))
        .with_position(Vec2::new(50.0, 415.0))
        .with_language("rust")
        .with_opacity(0.0);

    let math_eq = MathNode::default()
        .with_position(Vec2::new(150.0, 150.0))
        .with_equation("f(x) = sin(x)")
        .with_font_size(30.0)
        .with_fill(Color::rgb8(0xe6, 0xa7, 0x00)); // Yellow

    let logo = ImageNode::default()
        .with_position(Vec2::new(650.0, 150.0))
        .with_path("./examples/images/motion-canvas-logo.png")
        .with_size(Vec2::new(150.0, 150.0));

    project.scene.video_timeline.add(all![
        // Show code
        chain![
            code_block.opacity.to(1.0, Duration::from_secs(1)),
            code_block.append("fn main() {\n", Duration::from_secs(1)),
            code_block.append(
                "    let mut engine = MotionCanvas::new();\n",
                Duration::from_secs(1)
            ),
            code_block.append("    engine.render();\n", Duration::from_secs(1)),
            code_block.append("}", Duration::from_secs(1)),
            // Staggered appearance of nodes
            sequence![
                Duration::from_millis(200),
                divider_line
                    .end
                    .to(Vec2::new(800.0, 300.0), Duration::from_secs(1))
                    .ease(easings::cubic_out),
                follower
                    .radius
                    .to(30.0, Duration::from_millis(500))
                    .ease(easings::elastic_out),
            ],
            // The path follow combined with a "race" logic
            any![
                follower
                    .position
                    .follow(&path_node, Duration::from_secs(3))
                    .ease(easings::cubic_in_out),
                // Race: if this 'wait' finishes first, the follow is done
                wait(Duration::from_secs(4)),
            ],
            // Final flourishes using different easings
            all![
                follower
                    .radius
                    .to(10.0, Duration::from_secs(1))
                    .ease(easings::quad_out),
                divider_line
                    .start
                    .to(Vec2::new(400.0, 300.0), Duration::from_secs(1))
                    .ease(easings::cubic_in),
            ]
        ]
    ]);

    // 4. Build Scene
    project.scene.add(Box::new(background_rect.clone()));
    project.scene.add(Box::new(divider_line.clone()));
    project.scene.add(Box::new(path_node.clone()));
    project.scene.add(Box::new(follower.clone()));
    project.scene.add(Box::new(title_text.clone()));
    project.scene.add(Box::new(code_block.clone()));
    project.scene.add(Box::new(math_eq.clone()));
    project.scene.add(Box::new(logo.clone()));

    // 5. Run (Choose show() for interactive or export() for PNGs)
    project.show().expect("Failed to render");
    // project.export().expect("Failed to export");
}
