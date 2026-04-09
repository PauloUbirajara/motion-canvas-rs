use glam::Vec2;
use motion_canvas_rs::prelude::*;
use vello::peniko::Color;

#[test]
fn test_project_builder() {
    let project = Project::default()
        .with_dimensions(1280, 720)
        .with_fps(30)
        .with_title("Test Project")
        .with_background(Color::BLUE);

    assert_eq!(project.width, 1280);
    assert_eq!(project.height, 720);
    assert_eq!(project.fps, 30);
    assert_eq!(project.title, "Test Project");
    assert_eq!(project.background_color, Color::BLUE);
}

#[test]
fn test_circle_builder() {
    let circle = Circle::default()
        .with_position(Vec2::new(100.0, 200.0))
        .with_radius(75.0)
        .with_color(Color::BLUE);

    // Check initial values via signals
    let transform = circle.transform.get();
    let coeffs = transform.as_coeffs();
    assert_eq!(coeffs[4], 100.0);
    assert_eq!(coeffs[5], 200.0);
    assert_eq!(circle.radius.get(), 75.0);
    assert_eq!(circle.color.get(), Color::BLUE);
}

#[test]
fn test_rect_builder() {
    let rect = Rect::default()
        .with_position(Vec2::new(10.0, 10.0))
        .with_size(Vec2::new(200.0, 300.0))
        .with_radius(10.0);

    assert_eq!(rect.size.get(), Vec2::new(200.0, 300.0));
    assert_eq!(rect.radius.get(), 10.0);
}

#[test]
fn test_line_builder() {
    let line = Line::default()
        .with_start(Vec2::new(0.0, 0.0))
        .with_end(Vec2::new(100.0, 100.0))
        .with_width(5.0)
        .with_color(Color::GREEN);

    assert_eq!(line.start.get(), Vec2::new(0.0, 0.0));
    assert_eq!(line.end.get(), Vec2::new(100.0, 100.0));
    assert_eq!(line.width.get(), 5.0);
    assert_eq!(line.color.get(), Color::GREEN);
}

#[test]
fn test_polygon_builder() {
    let points = vec![
        Vec2::new(0.0, 0.0),
        Vec2::new(10.0, 0.0),
        Vec2::new(5.0, 10.0),
    ];
    let polygon = Polygon::default()
        .with_points(points.clone())
        .with_fill(Color::RED);

    assert_eq!(polygon.points.get(), points);
    assert_eq!(polygon.fill_color.get(), Color::RED);
}

#[test]
fn test_text_builder() {
    let text = TextNode::default()
        .with_text("Hello World")
        .with_font_size(48.0);

    assert_eq!(text.text.get(), "Hello World");
    assert_eq!(text.font_size.get(), 48.0);
}

#[test]
#[cfg(feature = "math")]
fn test_math_builder() {
    let math = MathNode::default().with_equation("a^2 + b^2 = c^2");

    assert_eq!(math.equation.get(), "a^2 + b^2 = c^2");
}

#[test]
#[cfg(feature = "code")]
fn test_code_builder() {
    let code = CodeNode::default()
        .with_language("javascript")
        .with_code("console.log('hi')");

    assert_eq!(code.language, "javascript");
    assert_eq!(code.code.get().text, "console.log('hi')");
}

#[test]
fn test_group_builder() {
    let group = GroupNode::default()
        .with_position(Vec2::new(50.0, 50.0))
        .with_opacity(0.5);

    assert_eq!(group.opacity.get(), 0.5);
}

#[test]
fn test_image_builder() {
    let image = ImageNode::default()
        .with_path("nonexistent.png") // This won't load anything but we can check size
        .with_size(Vec2::new(100.0, 100.0));

    assert_eq!(image.size.get(), Vec2::new(100.0, 100.0));
}
