use motion_canvas_rs::prelude::*;
use std::time::Duration;

fn main() {
    let mut project = Project::default()
        .with_fps(120)
        .with_title("Math Code")
        .close_on_finish();

    let triangle = Polygon::default()
        .with_position(Vec2::ZERO)
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
        .with_position(Vec2::new(300.0, 200.0))
        .with_scale(0.0);

    let pytagorean_theorem = MathNode::default()
        .with_position(Vec2::new(50.0, 200.0))
        .with_equation("a^2 + b^2 = c^2")
        .with_font_size(10.0)
        .with_fill(Color::GRAY)
        .with_opacity(0.0);

    project.scene.add(Box::new(triangle_line_group.clone()));
    project.scene.add(Box::new(pytagorean_theorem.clone()));

    project.scene.video_timeline.add(all![
        all![
            triangle_line_group
                .position
                .to(Vec2::new(50.0, 200.0), Duration::from_secs(1)),
            triangle_line_group
                .scale
                .to(Vec2::ONE, Duration::from_secs(1)),
        ],
        all![
            pytagorean_theorem.opacity.to(1.0, Duration::from_secs(1)),
            pytagorean_theorem
                .font_size
                .to(32.0, Duration::from_secs(1)),
            pytagorean_theorem
                .position
                .to(Vec2::new(150.0, 200.0), Duration::from_secs(1)),
        ]
    ]);

    project.show().expect("Failed to render");
}
