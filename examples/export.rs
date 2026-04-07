use motion_canvas_rs::prelude::*;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    // 1. Initialize for Export
    let mut project = Project::new(800, 600)
        .with_fps(30)
        .with_ffmpeg(true)
        .with_output_path("output");

    // 2. Setup Nodes
    let circle = Circle::new(Vec2::new(400.0, 300.0), 50.0, Color::BLUE);
    let text = TextNode::new(Vec2::new(400.0, 450.0), "Export Demo", 40.0, Color::WHITE);
    let math = MathNode::new(Vec2::new(400.0, 200.0), "E = mc^2", 40.0, Color::YELLOW);

    project.scene.add(Box::new(circle.clone()));
    project.scene.add(Box::new(text.clone()));
    project.scene.add(Box::new(math.clone()));

    // 3. Define Animations (Color and Font Size)
    project.scene.timeline.add(all![
        // Circle color and size
        circle.color.to(Color::WHITE, Duration::from_secs(2)).ease(easings::quad_in_out),
        circle.radius.to(150.0, Duration::from_secs(2)).ease(easings::elastic_out),
        
        // Text font size
        text.font_size.to(80.0, Duration::from_secs(2)).ease(easings::cubic_out),
        
        // Math color
        math.color.to(Color::RED, Duration::from_secs(2)).ease(easings::cubic_in),
    ]);

    // 4. Export (Renders frames and combines them into out.mkv)
    println!("Starting export to {}...", project.output_path.display());
    project.export()
}
