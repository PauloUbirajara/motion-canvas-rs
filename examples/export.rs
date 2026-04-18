use motion_canvas_rs::prelude::*;
use std::time::Duration;

fn main() {
    // 1. Initialize for Export
    let mut project = Project::default()
        .with_fps(30)
        .with_ffmpeg(true)
        .with_title("Export")
        .with_output_path("output")
        .close_on_finish();

    // 2. Setup Nodes
    let circle = Circle::default()
        .with_position(Vec2::new(400.0, 300.0))
        .with_radius(50.0)
        .with_color(Color::rgb8(0x68, 0xab, 0xdf)); // Blue

    let text = TextNode::default()
        .with_position(Vec2::new(200.0, 450.0))
        .with_text("Export Demo")
        .with_font_size(40.0)
        .with_color(Color::rgb8(0xf2, 0xf2, 0xf2)); // White

    project.scene.add(Box::new(circle.clone()));
    project.scene.add(Box::new(text.clone()));

    // 3. Define Animations (Color and Font Size)
    project.scene.video_timeline.add(flows::all![
        // Circle color and size
        circle
            .color
            .to(Color::rgb8(0xf2, 0xf2, 0xf2), Duration::from_secs(2))
            .ease(easings::quad_in_out),
        circle
            .radius
            .to(150.0, Duration::from_secs(2))
            .ease(easings::elastic_out),
        // Text font size
        text.font_size
            .to(80.0, Duration::from_secs(2))
            .ease(easings::cubic_out),
    ]);

    // 4. Export (Renders frames and combines them into out.mkv)
    println!("Starting export to {}...", project.output_path.display());
    project.export().expect("Failed to export");
}
