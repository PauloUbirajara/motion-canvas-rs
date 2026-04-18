use std::time::Duration;

use motion_canvas_rs::prelude::*;

fn main() {
    let mut project = Project::default()
        .with_dimensions(650, 200)
        .with_title("Shapes")
        .close_on_finish();

    // Circle
    let circle_text = TextNode::default().with_text("Circle").with_opacity(0.0);
    let circle = Circle::default().with_scale(0.0).with_radius(50.0);

    // Rectangle
    let rect_text = TextNode::default().with_text("Rectangle").with_opacity(0.0);
    let rect = Rect::default()
        .with_scale(0.0)
        .with_size(Vec2::new(100.0, 100.0));

    // Line
    let line_text = TextNode::default().with_text("Line").with_opacity(0.0);
    let line = Line::default().with_scale(0.0).with_width(1.0);

    // Polygon
    let poly_text = TextNode::default().with_text("Polygon").with_opacity(0.0);
    let poly = Polygon::regular(5, 50.0).with_scale(0.0);

    project.scene.video_timeline.add(chain![
        // Circle
        all![
            circle_text.opacity.to(1.0, Duration::from_secs(1)),
            circle_text
                .position
                .to(Vec2::new(55.0, 25.0), Duration::from_secs(1)),
        ],
        all![
            circle.scale.to(Vec2::ONE, Duration::from_secs(1)),
            circle
                .position
                .to(Vec2::new(100.0, 125.0), Duration::from_secs(1)),
        ],
        // Rectangle
        all![
            rect_text.opacity.to(1.0, Duration::from_secs(1)),
            rect_text
                .position
                .to(Vec2::new(180.0, 25.0), Duration::from_secs(1)),
        ],
        all![
            rect.scale.to(Vec2::ONE, Duration::from_secs(1)),
            rect.position
                .to(Vec2::new(200.0, 75.0), Duration::from_secs(1)),
        ],
        // Line
        all![
            line_text.opacity.to(1.0, Duration::from_secs(1)),
            line_text
                .position
                .to(Vec2::new(380.0, 25.0), Duration::from_secs(1)),
        ],
        all![
            line.scale.to(Vec2::ONE, Duration::from_secs(1)),
            line.position
                .to(Vec2::new(365.0, 125.0), Duration::from_secs(1)),
        ],
        // Polygon
        all![
            poly_text.opacity.to(1.0, Duration::from_secs(1)),
            poly_text
                .position
                .to(Vec2::new(505.0, 25.0), Duration::from_secs(1)),
        ],
        all![
            poly.scale.to(Vec2::ONE, Duration::from_secs(1)),
            poly.position
                .to(Vec2::new(567.0, 125.0), Duration::from_secs(1)),
        ],
    ]);

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
