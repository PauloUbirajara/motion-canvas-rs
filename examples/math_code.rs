use motion_canvas_rs::prelude::*;
use std::time::Duration;

fn main() {
    let mut project = Project::default()
        .with_fps(120)
        .with_title("Math Code")
        .close_on_finish();

    let triangle = Polygon::default()
        .with_position(Vec2::ZERO)
        .with_anchor(Vec2::new(-1.0, -1.0))
        .with_points(vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(0.0, 200.0),
            Vec2::new(200.0, 200.0),
        ])
        .with_fill(Color::rgb8(0x68, 0xab, 0xdf));

    let lines = vec![
        Line::default()
            .with_start(Vec2::new(0.0, 0.0))
            .with_end(Vec2::new(0.0, 200.0))
            .with_stroke(Color::WHITE, 10.0),
        Line::default()
            .with_start(Vec2::new(0.0, 0.0))
            .with_end(Vec2::new(200.0, 200.0))
            .with_stroke(Color::WHITE, 10.0),
        Line::default()
            .with_start(Vec2::new(0.0, 200.0))
            .with_end(Vec2::new(200.0, 200.0))
            .with_stroke(Color::WHITE, 10.0),
    ];

    let triangle_line_group = GroupNode::default()
        .with_nodes(vec![
            triangle.clone_node(),
            lines[0].clone_node(),
            lines[1].clone_node(),
            lines[2].clone_node(),
        ])
        .with_position(Vec2::new(400.0, 200.0))
        .with_scale(0.0);

    let pytagorean_theorem = MathNode::default()
        .with_position(Vec2::new(150.0, 150.0))
        .with_equation("a^2 + b^2 = c^2")
        .with_font_size(10.0)
        .with_fill(Color::GRAY)
        .with_opacity(0.0);

    let text_a = TextNode::default()
        .with_text("a")
        .with_fill(Color::GRAY)
        .with_opacity(0.0)
        .with_scale(0.7)
        .with_position(Vec2::new(370.0, 300.0));

    let text_b = TextNode::default()
        .with_text("b")
        .with_fill(Color::GRAY)
        .with_opacity(0.0)
        .with_scale(0.7)
        .with_position(Vec2::new(500.0, 430.0));

    let text_c = TextNode::default()
        .with_text("c")
        .with_fill(Color::GRAY)
        .with_opacity(0.0)
        .with_scale(0.7)
        .with_position(Vec2::new(510.0, 260.0));

    project.scene.add(Box::new(triangle_line_group.clone()));
    project.scene.add(Box::new(pytagorean_theorem.clone()));
    project.scene.add(Box::new(text_a.clone()));
    project.scene.add(Box::new(text_b.clone()));
    project.scene.add(Box::new(text_c.clone()));

    project.scene.video_timeline.add(all![
        all![triangle_line_group
            .scale
            .to(Vec2::ONE, Duration::from_secs(1)),],
        all![
            pytagorean_theorem.opacity.to(1.0, Duration::from_secs(1)),
            pytagorean_theorem
                .font_size
                .to(32.0, Duration::from_secs(1)),
        ],
        sequence![
            Duration::from_millis(300),
            text_a.scale.to(Vec2::new(1.0, 1.0), Duration::from_secs(1)),
            text_b.scale.to(Vec2::new(1.0, 1.0), Duration::from_secs(1)),
            text_c.scale.to(Vec2::new(1.0, 1.0), Duration::from_secs(1)),
            text_a.opacity.to(1.0, Duration::from_secs(1)),
            text_b.opacity.to(1.0, Duration::from_secs(1)),
            text_c.opacity.to(1.0, Duration::from_secs(1))
        ]
    ]);

    project.show().expect("Failed to render");
}
