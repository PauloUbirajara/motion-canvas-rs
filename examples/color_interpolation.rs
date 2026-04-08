use motion_canvas_rs::prelude::*;
use std::time::Duration;
use vello::kurbo::BezPath;

fn main() -> anyhow::Result<()> {
    let mut project = Project::new(1200, 400)
        .with_fps(60)
        .with_title("Color Interpolation Demo");

    let start_y = 150.0;
    let spacing = 180.0;
    let start_x = 100.0;

    // 1. Circle
    let circle = Circle::new(Vec2::new(start_x, start_y), 50.0, Color::RED);

    // 2. Rect
    let rect = Rect::new(
        Vec2::new(start_x + spacing - 50.0, start_y - 50.0),
        Vec2::new(100.0, 100.0),
        Color::ORANGE,
    );

    // 3. Line
    let line = Line::new(
        Vec2::new(start_x + spacing * 2.0 - 50.0, start_y - 50.0),
        Vec2::new(start_x + spacing * 2.0 + 50.0, start_y + 50.0),
        Color::YELLOW,
        5.0,
    );

    // 4. Path (a triangle)
    let mut triangle = BezPath::new();
    triangle.move_to((0.0, -50.0));
    triangle.line_to((50.0, 50.0));
    triangle.line_to((-50.0, 50.0));
    triangle.close_path();
    let path = PathNode::new(
        Vec2::new(start_x + spacing * 3.0, start_y),
        triangle,
        Color::GREEN,
        3.0,
    );

    // 5. Math
    let math = MathNode::new(
        Vec2::new(start_x + spacing * 4.0 - 40.0, start_y - 30.0),
        "\\pi",
        60.0,
        Color::BLUE,
    );

    // 6. Text
    let text = TextNode::new(
        Vec2::new(start_x + spacing * 5.0 - 40.0, start_y - 30.0),
        "ABC",
        60.0,
        Color::PURPLE,
    );

    // 7. Footer Text
    let footer = TextNode::new(
        Vec2::new(50.0, 320.0),
        "Color Interpolation Across All Shapes",
        40.0,
        Color::WHITE,
    );

    // Add all to scene
    project.scene.add(Box::new(circle.clone()));
    project.scene.add(Box::new(rect.clone()));
    project.scene.add(Box::new(line.clone()));
    project.scene.add(Box::new(path.clone()));
    project.scene.add(Box::new(math.clone()));
    project.scene.add(Box::new(text.clone()));
    project.scene.add(Box::new(footer.clone()));

    // Define color change animation
    let target_color = Color::WHITE;
    let duration = Duration::from_secs(2);

    project.scene.timeline.add(all![
        circle
            .color
            .to(target_color, duration)
            .ease(easings::quad_in_out),
        rect.color
            .to(target_color, duration)
            .ease(easings::quad_in_out),
        line.color
            .to(target_color, duration)
            .ease(easings::quad_in_out),
        path.color
            .to(target_color, duration)
            .ease(easings::quad_in_out),
        math.color
            .to(target_color, duration)
            .ease(easings::quad_in_out),
        text.color
            .to(target_color, duration)
            .ease(easings::quad_in_out),
        footer
            .color
            .to(Color::YELLOW, duration)
            .ease(easings::sine_in_out),
    ]);

    // Back to original colors
    project.scene.timeline.add(all![
        circle
            .color
            .to(Color::RED, duration)
            .ease(easings::quad_in_out),
        rect.color
            .to(Color::ORANGE, duration)
            .ease(easings::quad_in_out),
        line.color
            .to(Color::YELLOW, duration)
            .ease(easings::quad_in_out),
        path.color
            .to(Color::GREEN, duration)
            .ease(easings::quad_in_out),
        math.color
            .to(Color::BLUE, duration)
            .ease(easings::quad_in_out),
        text.color
            .to(Color::PURPLE, duration)
            .ease(easings::quad_in_out),
        footer
            .color
            .to(Color::WHITE, duration)
            .ease(easings::sine_in_out),
    ]);

    project.show()
}
