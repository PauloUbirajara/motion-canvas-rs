use motion_canvas_rs::prelude::*;
use std::time::Duration;

fn main() {
    let mut project = Project::new(800, 600).with_title("Advanced Code Node Demo");

    let code = CodeNode::default()
        .with_position(Vec2::new(50.0, 50.0))
        .with_code(r#"fn main() {
    println!("Hello");
}"#)
        .with_language("rust")
        .with_font_size(32.0)
        .with_dim_opacity(0.1);

    project.scene.add(Box::new(code.clone()));

    project.scene.timeline.add(flows::sequence![
        Duration::from_secs(1),
        // 1. Select line 2 (println) - using 1-based index string
        code.select_string("2", Duration::from_millis(300)),
        // 2. Select range 1-2
        code.select_string("1-2", Duration::from_millis(300)),
        // 3. Append a comment
        code.append("\n// Done!", Duration::from_millis(300)),
        // 3. Reset selection
        code.select_lines(vec![], Duration::from_millis(300)),
        // 4. Prepend a header (Now natively lazy, no wrapper needed!)
        code.prepend("// My Script\n", Duration::from_millis(300)),
    ]);

    project.show().expect("Failed to render");
}
