use motion_canvas_rs::prelude::*;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    let mut project = Project::new(400, 400)
        .with_fps(60)
        .with_title("Color Interpolation")
        .with_cache(true); // Ensure cache is active for test

    let start_y = 150.0;
    let spacing = 180.0;
    let start_x = 100.0;

    let circle = Circle::new(Vec2::new(start_x, start_y), 50.0, Color::RED);
    let rect = Rect::new(
        Vec2::new(start_x + spacing - 50.0, start_y - 50.0),
        Vec2::new(100.0, 100.0),
        Color::ORANGE,
    );

    project.scene.add(Box::new(circle.clone()));
    project.scene.add(Box::new(rect.clone()));

    let target_color = Color::WHITE;
    let duration = Duration::from_secs(1);

    project.scene.timeline.add(all![
        circle.color.to(target_color, duration),
        rect.color.to(target_color, duration),
    ]);

    // Show
    project.show()
}
