use motion_canvas_rs::prelude::*;
use glam::Vec2;
use vello::peniko::Color;

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
    let math = MathNode::default()
        .with_equation("a^2 + b^2 = c^2");
    
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
