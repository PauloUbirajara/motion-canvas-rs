use motion_canvas_rs::prelude::*;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    let mut project = Project::new(1920, 1080);

    let code = CodeNode::new(
        Vec2::new(50.0, 400.0), 
        "fn main() {\n    println!(\"Hello\");\n}", 
        "rust"
    );

    let math = MathNode::new(
        Vec2::new(50.0, 200.0), 
        "e^{i\\pi} + 1 = 0", 
        60.0, 
        Color::WHITE
    );

    project.scene.add(Box::new(code));
    project.scene.add(Box::new(math.clone()));

    project.scene.timeline.add(
        math.font_size.to(80.0, Duration::from_secs(1))
    );
    
    project.show()
}
