use motion_canvas_rs::prelude::*;
use std::time::Duration;

fn main() {
    let mut project = Project::new(800, 600).with_title("Advanced Code Node Demo");

    let code = CodeNode::new(
        Vec2::new(50.0, 50.0),
        r#"fn main() {
    println!("Hello");
}"#,
        "rust",
    )
    .with_font_size(32.0)
    .with_dim_opacity(0.1);

    project.scene.add(Box::new(code.clone()));

    project.scene.timeline.add(flows::sequence![
        Duration::from_secs(1),
        // 1. Highlight line 2 (println) - using 1-based index string
        code.highlight_lines("2", Duration::from_millis(300)),
        // 2. Highlight range 1-2
        code.highlight_lines("1-2", Duration::from_millis(300)),
        // 3. Append a comment
        code.append("\n// Done!", Duration::from_millis(300)),
        // 3. Reset highlights
        code.highlight(vec![], Duration::from_millis(300)),
        // 4. Prepend a header (Now natively lazy, no wrapper needed!)
        code.prepend("// My Script\n", Duration::from_millis(300)),
    ]);

    project.show().expect("Failed to render");
}
