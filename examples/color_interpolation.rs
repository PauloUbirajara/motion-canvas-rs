use glam::Vec2;
use motion_canvas_rs::engine::nodes::{Circle, Rect};
use motion_canvas_rs::engine::project::Project;
use motion_canvas_rs::flows;
use motion_canvas_rs::render::Color;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    let mut project = Project::new(400, 400)
        .with_fps(60)
        .with_title("Color Interpolation")
        .with_cache(true); // Ensure cache is active for test

    let start_y = 150.0;
    let spacing = 180.0;
    let start_x = 100.0;

    let circle = Circle::new(
        Vec2::new(start_x, start_y),
        50.0,
        Color::rgb8(0xe1, 0x32, 0x38),
    ); // Red
    let rect = Rect::new(
        Vec2::new(start_x + spacing - 50.0, start_y - 50.0),
        Vec2::new(100.0, 100.0),
        Color::rgb8(0xff, 0xc6, 0x6d), // Orange
    );

    project.scene.add(Box::new(circle.clone()));
    project.scene.add(Box::new(rect.clone()));

    let target_color = Color::rgb8(0xf2, 0xf2, 0xf2); // White-ish
    let duration = Duration::from_secs(1);

    project.scene.timeline.add(flows::all![
        circle.color.to(target_color, duration),
        rect.color.to(target_color, duration),
    ]);

    // Show
    project.show()
}
