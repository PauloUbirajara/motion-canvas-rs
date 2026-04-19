use motion_canvas_rs::prelude::*;
use std::time::Duration;

fn main() {
    let mut project = Project::default()
        .with_dimensions(800, 800)
        .with_title("Code Animation")
        .close_on_finish();

    let code = CodeNode::default()
        .with_position(Vec2::new(50.0, 50.0))
        .with_code("")
        .with_language("rust")
        .with_theme("Solarized (dark)")
        .with_font("JetBrains Mono");

    project.scene.add(Box::new(code.clone()));

    let interval = Duration::from_secs(4);
    let code_duration = Duration::from_secs(1);
    project.scene.video_timeline.add(sequence!(
        interval,
        code.edit(
            r#"use motion_canvas_rs::prelude::*;       
use std::time::Duration;"#,
            code_duration,
        ),
        code.edit(
            r#"use motion_canvas_rs::prelude::*;       
use std::time::Duration;

fn main() {
    // 1. Initialize the Project
    let mut project = Project::default()
        .with_dimensions(800, 600)
        .with_fps(60);
}"#,
            code_duration,
        ),
        code.edit(
            r#"use motion_canvas_rs::prelude::*;       
use std::time::Duration;

fn main() {
    // 1. Initialize the Project
    // 2. Define Nodes
    // Red
    let circle = Circle::default()
        .with_position(Vec2::new(400.0, 300.0))
        .with_radius(50.0)
        .with_fill(Color::rgb8(0xe1, 0x32, 0x38));

    // White-ish
    let text = TextNode::default()
        .with_position(Vec2::new(400.0, 450.0))
        .with_text("Hello Rust")
        .with_font_size(40.0)
        .with_fill(Color::rgb8(0xf2, 0xf2, 0xf2));
}"#,
            code_duration,
        ),
        code.edit(
            r#"use motion_canvas_rs::prelude::*;       
use std::time::Duration;

fn main() {
    // 1. Initialize the Project
    // 2. Define Nodes
    // 3. Add Nodes to the Scene
    project.scene.add(Box::new(circle.clone()));
    project.scene.add(Box::new(text.clone()));
}"#,
            code_duration,
        ),
        code.edit(
            r#"use motion_canvas_rs::prelude::*;       
use std::time::Duration;

fn main() {
    // 1. Initialize the Project
    // 2. Define Nodes
    // 3. Add Nodes to the Scene
    // 4. Add Animations to the Timeline
    project.scene.video_timeline.add(flows::all![
        circle.radius.to(
            100.0,
            Duration::from_secs(1)
        ),
        text.position.to(
            Vec2::new(400.0, 500.0),
            Duration::from_secs(1)
        ),
    ]);
}"#,
            code_duration,
        ),
        code.edit(
            r#"use motion_canvas_rs::prelude::*;       
use std::time::Duration;

fn main() {
    // 1. Initialize the Project
    // 2. Define Nodes
    // 3. Add Nodes to the Scene
    // 4. Add Animations to the Timeline
    // 5. Show
    project.show().expect("Failed to render");
}"#,
            code_duration,
        ),
    ));

    project.show().expect("Failed to render");
}
