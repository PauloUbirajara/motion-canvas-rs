use motion_canvas_rs::prelude::*;

fn main() {
    let mut project = Project::default()
        .with_dimensions(800, 300)
        .with_title("Shapes")
        .close_on_finish();

    // Configuration
    let y_shapes = 150.0;
    let y_labels = 50.0;
    let spacing = 180.0;
    let x_start = 130.0;

    // Circle
    let circle_text = TextNode::default()
        .with_text("Circle")
        .with_position(Vec2::new(x_start, y_labels))
        .with_anchor(Vec2::ZERO)
        .with_font_size(24.0)
        .with_fill(Color::DIM_GRAY);
    let circle = Circle::default()
        .with_position(Vec2::new(x_start, y_shapes))
        .with_radius(50.0)
        .with_fill(Color::rgb8(0xe1, 0x32, 0x38));

    // Rectangle
    let x_rect = x_start + spacing;
    let rect_text = TextNode::default()
        .with_text("Rect")
        .with_position(Vec2::new(x_rect, y_labels))
        .with_anchor(Vec2::ZERO)
        .with_font_size(24.0)
        .with_fill(Color::DIM_GRAY);
    let rect = Rect::default()
        .with_position(Vec2::new(x_rect, y_shapes))
        .with_size(Vec2::new(100.0, 100.0))
        .with_radius(8.0)
        .with_fill(Color::rgb8(0x68, 0xab, 0xdf));

    // Line
    let x_line = x_rect + spacing;
    let line_text = TextNode::default()
        .with_text("Line")
        .with_position(Vec2::new(x_line, y_labels))
        .with_anchor(Vec2::ZERO)
        .with_font_size(24.0)
        .with_fill(Color::DIM_GRAY);
    let line = Line::default()
        .with_position(Vec2::new(x_line, y_shapes))
        .with_start(Vec2::new(-50.0, 50.0))
        .with_end(Vec2::new(50.0, -50.0))
        .with_stroke(Color::WHITE, 4.0);

    // Polygon
    let x_poly = x_line + spacing;
    let poly_text = TextNode::default()
        .with_text("Poly")
        .with_position(Vec2::new(x_poly, y_labels))
        .with_anchor(Vec2::ZERO)
        .with_font_size(24.0)
        .with_fill(Color::DIM_GRAY);
    let poly = Polygon::regular(5, 50.0)
        .with_position(Vec2::new(x_poly, y_shapes))
        .with_fill(Color::rgb8(0xe6, 0xa7, 0x00));

    // Add all nodes to the scene
    project.scene.add(Box::new(circle));
    project.scene.add(Box::new(circle_text));
    project.scene.add(Box::new(rect));
    project.scene.add(Box::new(rect_text));
    project.scene.add(Box::new(line));
    project.scene.add(Box::new(line_text));
    project.scene.add(Box::new(poly));
    project.scene.add(Box::new(poly_text));

    project.show().expect("Failed to render");
}
