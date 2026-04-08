use motion_canvas_rs::prelude::*;
use std::time::Duration;

fn main() {
    let mut project = Project::new(800, 800);

    let code = CodeNode::new(Vec2::new(50.0, 50.0), r#""#, "rust")
        .with_theme("Solarized (dark)")
        .with_font("JetBrains Mono");

    project.scene.add(Box::new(code.clone()));

    let interval = Duration::from_secs(4);
    let code_duration = Duration::from_secs(1);
    project.scene.timeline.add(sequence!(
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
    let mut project = Project::new(800, 600)
        .with_fps(60)
        .with_cache(true);
}"#,
            code_duration,
        ),
        code.edit(
            r#"use motion_canvas_rs::prelude::*;       
use std::time::Duration;

fn main() {
    // 1. Initialize the Project
    // 2. Define Nodes
    let circle = Circle::new(
        Vec2::new(400.0, 300.0), // Position
        50.0, // Radius
        Color::rgb8(0xe1, 0x32, 0x38) // Color
    ); // Red
    let text = TextNode::new(
        Vec2::new(400.0, 450.0), // Position
        "Hello Rust", // Text
        40.0, // Font Size
        Color::rgb8(0xf2, 0xf2, 0xf2), // Color
    ); // White-ish
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
    project.scene.timeline.add(flows::all![
        circle.radius
            .to(
                100.0,
                Duration::from_secs(1)
            ),
        text.transform
            .to(
                Affine::translate((400.0, 500.0)),
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
