use glam::Vec2;
use motion_canvas_rs::engine::nodes::{CodeNode, MathNode};
use motion_canvas_rs::engine::project::Project;
use motion_canvas_rs::render::Color;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    let mut project = Project::new(400, 400);

    let code = CodeNode::new(
        Vec2::new(50.0, 50.0),
        r#"fn main() {
    println!("Hello");
}"#,
        "rust",
    );

    let math = MathNode::new(
        Vec2::new(20.0, 100.0),
        "e^(i pi) + 1 = 0",
        50.0,
        Color::rgb8(0xe6, 0xa7, 0x00), // Yellow
    );

    project.scene.add(Box::new(code));
    project.scene.add(Box::new(math.clone()));

    project
        .scene
        .timeline
        .add(math.font_size.to(20.0, Duration::from_secs(1)));

    project.show()
}
