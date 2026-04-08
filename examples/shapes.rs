use motion_canvas_rs::prelude::*;

fn main() {
    let mut project = Project::new(400, 200);

    // Red
    let circle = Circle::new(Vec2::new(100.0, 100.0), 50.0, Color::rgb8(0xe1, 0x32, 0x38));

    // Blue
    let rect = Rect::new(
        Vec2::new(150.0, 50.0),
        Vec2::new(100.0, 100.0),
        Color::rgb8(0x68, 0xab, 0xdf),
    )
    .with_radius(10.0);

    // White
    let line = Line::new(
        Vec2::new(250.0, 100.0),
        Vec2::new(350.0, 100.0),
        Color::rgb8(0xf2, 0xf2, 0xf2),
        2.0,
    );

    project.scene.add(Box::new(circle));
    project.scene.add(Box::new(rect));
    project.scene.add(Box::new(line));

    project.show().expect("Failed to render");
}
