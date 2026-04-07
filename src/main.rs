use std::time::Duration;
use vello::kurbo::BezPath;
use vello::peniko::Color;

mod engine;
mod render;

use crate::engine::animation::wait;
use crate::engine::node::{Circle, CodeNode, Line, MathNode, PathNode, Rect, TextNode};
use crate::engine::project::Project;
use crate::engine::{easings, Vec2};

fn main() -> anyhow::Result<()> {
    // 1. Initialize Project with full API coverage
    let mut project = Project::new(800, 600)
        .with_fps(120)
        .with_cache(true)
        .with_ffmpeg(true)
        .with_output_path("output")
        .with_frame_template("frame_{:04}.png")
        .with_title("Motion Canvas Rust - Demo");

    // 2. Setup Nodes
    let mut path = BezPath::new();
    path.move_to((100.0, 300.0));
    path.curve_to((250.0, 100.0), (550.0, 500.0), (700.0, 300.0));
    let path_node = PathNode::new(path, Color::rgb8(60, 60, 60), 2.0);

    let follower = Circle::new(Vec2::new(100.0, 300.0), 20.0, Color::rgb8(255, 100, 100));

    // Showcase: Rect and Line
    let background_rect = Rect::new(
        Vec2::new(400.0, 300.0),
        Vec2::new(760.0, 560.0),
        Color::rgba8(40, 40, 40, 150),
    )
    .with_radius(20.0);
    let divider_line = Line::new(
        Vec2::new(0.0, 300.0),
        Vec2::new(0.0, 300.0),
        Color::rgb8(80, 80, 80),
        1.0,
    );
    let title_text = TextNode::new(
        Vec2::new(50.0, 50.0),
        "Motion Canvas in Rust",
        40.0,
        Color::rgb8(200, 200, 255),
    )
    .with_font("Inter");

    let code_block = CodeNode::new(
        Vec2::new(50.0, 400.0),
        "fn main() {\n    let mut engine = MotionCanvas::new();\n    engine.render();\n}",
        "rust",
    );

    let math_eq = MathNode::new(
        Vec2::new(50.0, 200.0),
        "e^{i\\pi} + 1 = 0",
        30.0,
        Color::rgb8(255, 255, 100),
    );

    // Clone signals for use in closures to avoid partial moves
    let bg_size = background_rect.size.clone();
    let bg_pos = background_rect.position.clone();
    let text_pos = title_text.position.clone();

    // 3. Define the Animation "Super Sequence"
    project.scene.timeline.add(all![
        // Background loop pulse
        loop_anim![
            bg_size
                .to(Vec2::new(780.0, 580.0), Duration::from_secs(2))
                .ease(easings::quad_in_out),
            Some(3)
        ],
        // Main sequence
        chain![
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
                    .follow(&path_node, Duration::from_secs(4))
                    .ease(easings::cubic_in_out),
                // Race: if this 'wait' finishes first, the follow is done
                wait(Duration::from_secs(5)),
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
                delay![
                    Duration::from_millis(200),
                    bg_pos
                        .to(Vec2::new(400.0, 280.0), Duration::from_secs(1))
                        .ease(easings::elastic_in)
                ],
                text_pos
                    .to(Vec2::new(100.0, 50.0), Duration::from_secs(1))
                    .ease(easings::cubic_out),
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

    // 5. Run (Choose show() for interactive or export() for PNGs)
    // project.show()
    project.export()
}
