use motion_canvas_rs::prelude::*;
use std::time::Duration;

fn main() {
    let mut project = Project::default()
        .with_dimensions(400, 400)
        .with_fps(60)
        .with_title("Color Interpolation")
        .with_cache(true)
        .close_on_finish(); // Ensure cache is active for test

    let start_y = 150.0;
    let spacing = 180.0;
    let start_x = 100.0;

    let circle = Circle::default()
        .with_position(Vec2::new(start_x, start_y))
        .with_radius(50.0)
        .with_fill(Color::rgb8(0xe1, 0x32, 0x38)); // Red

    let rect = Rect::default()
        .with_position(Vec2::new(start_x + spacing - 50.0, start_y - 50.0))
        .with_size(Vec2::new(100.0, 100.0))
        .with_fill(Color::rgb8(0xff, 0xc6, 0x6d)); // Orange

    project.scene.add(Box::new(circle.clone()));
    project.scene.add(Box::new(rect.clone()));

    let target_color = Color::rgb8(0xf2, 0xf2, 0xf2); // White-ish
    let duration = Duration::from_secs(1);

    project.scene.video_timeline.add(flows::all![
        circle.fill_color.to(target_color, duration),
        rect.fill_color.to(target_color, duration),
    ]);

    // Show
    project.show().expect("Failed to render");
}
