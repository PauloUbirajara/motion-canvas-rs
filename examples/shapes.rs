use motion_canvas_rs::prelude::*;

fn main() {
    let mut project = Project::default()
        .with_dimensions(400, 200)
        .close_on_finish();

    // Red
    let circle = Circle::default()
        .with_position(Vec2::new(100.0, 100.0))
        .with_radius(50.0)
        .with_color(Color::rgb8(0xe1, 0x32, 0x38));

    // Blue
    let rect = Rect::default()
        .with_position(Vec2::new(150.0, 50.0))
        .with_size(Vec2::new(100.0, 100.0))
        .with_color(Color::rgb8(0x68, 0xab, 0xdf))
        .with_radius(10.0);

    // White
    let line = Line::default()
        .with_start(Vec2::new(250.0, 100.0))
        .with_end(Vec2::new(350.0, 100.0))
        .with_color(Color::rgb8(0xf2, 0xf2, 0xf2))
        .with_width(2.0);

    project.scene.add(Box::new(circle));
    project.scene.add(Box::new(rect));
    project.scene.add(Box::new(line));

    project.show().expect("Failed to render");
}
