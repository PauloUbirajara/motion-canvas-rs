use motion_canvas_rs::prelude::*;
use std::time::Duration;

fn main() {
    let mut project = Project::new(800, 400);

    let code = CodeNode::new(
        Vec2::new(50.0, 50.0),
        r#"fn main() {
    println!("Hello, world!");
}"#,
        "rust",
    );

    project.scene.add(Box::new(code.clone()));

    // Wait 1 second
    project.scene.timeline.add(delay!(Duration::from_secs(1), code.position.to(Vec2::new(50.0, 50.0), Duration::ZERO)));

    // Transition to new code
    let new_code = r#"fn main() {
    let message = "Magic Move!";
    println!("{}", message);
}"#;
    
    project.scene.timeline.add(code.edit(new_code, Duration::from_secs(2)));

    project.show().expect("Failed to render");
}
