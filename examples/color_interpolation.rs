use motion_canvas_rs::prelude::*;
use std::time::Duration;

fn main() {
    let mut project = Project::default()
        .with_dimensions(300, 300)
        .with_fps(60)
        .with_title("Color Interpolation")
        .with_cache(true)
        .close_on_finish();

    let circle = Circle::default()
        .with_position(Vec2::new(150.0, 150.0))
        .with_radius(50.0)
        .with_fill(Color::RED); // Red

    project.scene.add(Box::new(circle.clone()));

    let duration = Duration::from_secs(1);

    project.scene.video_timeline.add(loop_anim(
        move || {
            chain![
                circle.fill_color.to(Color::YELLOW, duration),
                circle.fill_color.to(Color::GREEN, duration),
                circle.fill_color.to(Color::BLUE, duration),
                circle.fill_color.to(Color::RED, duration),
            ]
        },
        None,
    ));

    // Show
    project.show().expect("Failed to render");
}
