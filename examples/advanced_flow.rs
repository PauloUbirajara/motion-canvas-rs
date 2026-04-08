use motion_canvas_rs::engine::project::Project;
use motion_canvas_rs::engine::nodes::{Circle, Rect, Line, PathNode, TextNode, CodeNode, MathNode, ImageNode};
use motion_canvas_rs::flows;
use motion_canvas_rs::engine::easings;
use motion_canvas_rs::render::Color;
use glam::Vec2;
use std::time::Duration;
use vello::kurbo::BezPath;

fn main() -> anyhow::Result<()> {
    // 1. Initialize Project with full API coverage
    let mut project = Project::new(800, 600)
        .with_fps(120)
        .with_gpu(true)
        .with_cache(true)
        .with_ffmpeg(true)
        .with_output_path("output")
        .with_frame_template("frame_{:04}.png")
        .with_title("Motion Canvas Rust - Demo");

    // 2. Setup Nodes
    let mut path = BezPath::new();
    path.move_to((100.0, 300.0));
    path.curve_to((250.0, 100.0), (550.0, 500.0), (700.0, 300.0));
    let path_node = PathNode::new(Vec2::ZERO, path, Color::rgb8(0x44, 0x44, 0x44), 2.0); // Subtle path
    let follower = Circle::new(Vec2::new(100.0, 300.0), 20.0, Color::rgb8(0xe1, 0x32, 0x38)); // Red

    // Showcase: Rect and Line
    let background_rect = Rect::new(
        Vec2::new(400.0, 300.0),
        Vec2::new(760.0, 560.0),
        Color::rgba8(0x33, 0x33, 0x33, 150),
    )
    .with_radius(20.0);
    let divider_line = Line::new(
        Vec2::new(0.0, 300.0),
        Vec2::new(0.0, 300.0),
        Color::rgb8(0x44, 0x44, 0x44),
        1.0,
    );
    let title_text = TextNode::new(
        Vec2::new(50.0, 50.0),
        "Motion Canvas in Rust",
        40.0,
        Color::rgb8(0x68, 0xab, 0xdf), // Blue
    )
    .with_font("Inter");

    let code_block = CodeNode::new(
        Vec2::new(50.0, 400.0),
        "fn main() {\n    let mut engine = MotionCanvas::new();\n    engine.render();\n}",
        "rust",
    );

    let math_eq = MathNode::new(
        Vec2::new(50.0, 200.0),
        "e^(i pi) + 1 = 0",
        30.0,
        Color::rgb8(0xe6, 0xa7, 0x00), // Yellow
    );

    let logo = ImageNode::new(
        Vec2::new(600.0, 50.0),
        "./examples/images/motion-canvas-logo.png",
    )
    .with_size(Vec2::new(150.0, 150.0));

    // Clone signals for use in closures to avoid partial moves
    let bg_size = background_rect.size.clone();
    let bg_pos = background_rect.position.clone();
    let text_pos = title_text.position.clone();

    // 3. Define the Animation "Super Sequence"
    project.scene.timeline.add(flows::all![
        // Background loop pulse
        flows::loop_anim![
            bg_size
                .to(Vec2::new(780.0, 580.0), Duration::from_secs(2))
                .ease(easings::quad_in_out),
            Some(3)
        ],
        // Main sequence
        flows::chain![
            // Staggered appearance of nodes
            flows::sequence![
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
            flows::any![
                follower
                    .position
                    .follow(&path_node, Duration::from_secs(4))
                    .ease(easings::cubic_in_out),
                // Race: if this 'wait' finishes first, the follow is done
                flows::wait(Duration::from_secs(5)),
            ],
            // Final flourishes using different easings
            flows::all![
                follower
                    .radius
                    .to(10.0, Duration::from_secs(1))
                    .ease(easings::quad_out),
                divider_line
                    .start
                    .to(Vec2::new(400.0, 300.0), Duration::from_secs(1))
                    .ease(easings::cubic_in),
                flows::delay![
                    Duration::from_millis(200),
                    bg_pos
                        .to(Vec2::new(400.0, 280.0), Duration::from_secs(1))
                        .ease(easings::elastic_in)
                ],
                text_pos
                    .to(Vec2::new(100.0, 50.0), Duration::from_secs(1))
                    .ease(easings::cubic_out),
                logo.position
                    .to(Vec2::new(600.0, 80.0), Duration::from_secs(1))
                    .ease(easings::elastic_out),
            ]
        ]
    ]);

    // 4. Build Scene
    project.scene.add(Box::new(background_rect));
    project.scene.add(Box::new(divider_line));
    project.scene.add(Box::new(path_node));
    project.scene.add(Box::new(follower));
    project.scene.add(Box::new(title_text));
    project.scene.add(Box::new(code_block));
    project.scene.add(Box::new(math_eq));
    project.scene.add(Box::new(logo));

    // 5. Run (Choose show() for interactive or export() for PNGs)
    project.show()
    // project.export()
}
