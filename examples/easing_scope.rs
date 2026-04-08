use glam::Vec2;
use motion_canvas_rs::engine::easings;
use motion_canvas_rs::engine::nodes::{Circle, TextNode};
use motion_canvas_rs::engine::project::Project;
use motion_canvas_rs::{all, with_easing, chain, loop_anim, flows::wait};
use motion_canvas_rs::render::Color;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    let mut project = Project::new(800, 800);

    let easing_configs = vec![
        ("Sine InOut", easings::sine_in_out as fn(f32) -> f32),
        ("Quad InOut", easings::quad_in_out as fn(f32) -> f32),
        ("Cubic InOut", easings::cubic_in_out as fn(f32) -> f32),
        ("Quart InOut", easings::quart_in_out as fn(f32) -> f32),
        ("Expo InOut", easings::expo_in_out as fn(f32) -> f32),
        ("Back Out", easings::back_out as fn(f32) -> f32),
        ("Elastic Out", easings::elastic_out as fn(f32) -> f32),
        ("Bounce Out", easings::bounce_out as fn(f32) -> f32),
    ];

    let mut balls = Vec::new();
    let mut labels = Vec::new();

    let start_x = 150.0;
    let end_x = 750.0;
    let start_y = 100.0;
    let spacing_y = 80.0;

    let colors = vec![
        Color::rgb8(0xe1, 0x32, 0x38), // Red
        Color::rgb8(0x68, 0xab, 0xdf), // Blue
        Color::rgb8(0xe6, 0xa7, 0x00), // Yellow
        Color::rgb8(0x20, 0xb2, 0xaa), // Teal
        Color::rgb8(0x99, 0xc4, 0x7a), // Green
        Color::rgb8(0xff, 0xc6, 0x6d), // Orange
        Color::rgb8(0x25, 0xc2, 0x81), // Emerald
        Color::rgb8(0xf2, 0xf2, 0xf2), // White-ish
    ];

    for (i, (name, _)) in easing_configs.iter().enumerate() {
        let y = start_y + (i as f32 * spacing_y);
        let ball = Circle::new(Vec2::new(start_x, y), 20.0, colors[i % colors.len()]);
        let label = TextNode::new(Vec2::new(start_x - 130.0, y), name, 18.0, Color::rgb8(0xcc, 0xcc, 0xcc));
        
        project.scene.add(Box::new(ball.clone()));
        project.scene.add(Box::new(label.clone()));
        
        balls.push(ball);
        labels.push(label);
    }

    project.scene.timeline.add(loop_anim![
        chain![
            // 1. Move to right using individual scoped easings
            all![
                with_easing!(easing_configs[0].1, [balls[0].position.to(Vec2::new(end_x, start_y + 0.0 * spacing_y), Duration::from_secs(2))]),
                with_easing!(easing_configs[1].1, [balls[1].position.to(Vec2::new(end_x, start_y + 1.0 * spacing_y), Duration::from_secs(2))]),
                with_easing!(easing_configs[2].1, [balls[2].position.to(Vec2::new(end_x, start_y + 2.0 * spacing_y), Duration::from_secs(2))]),
                with_easing!(easing_configs[3].1, [balls[3].position.to(Vec2::new(end_x, start_y + 3.0 * spacing_y), Duration::from_secs(2))]),
                with_easing!(easing_configs[4].1, [balls[4].position.to(Vec2::new(end_x, start_y + 4.0 * spacing_y), Duration::from_secs(2))]),
                with_easing!(easing_configs[5].1, [balls[5].position.to(Vec2::new(end_x, start_y + 5.0 * spacing_y), Duration::from_secs(2))]),
                with_easing!(easing_configs[6].1, [balls[6].position.to(Vec2::new(end_x, start_y + 6.0 * spacing_y), Duration::from_secs(2))]),
                with_easing!(easing_configs[7].1, [balls[7].position.to(Vec2::new(end_x, start_y + 7.0 * spacing_y), Duration::from_secs(2))]),
            ],
            wait(Duration::from_secs(1)),
            // 2. Move back to left with linear easing (to show contrast)
            with_easing!(easings::linear, [
                all![
                    balls[0].position.to(Vec2::new(start_x, start_y + 0.0 * spacing_y), Duration::from_secs(1)),
                    balls[1].position.to(Vec2::new(start_x, start_y + 1.0 * spacing_y), Duration::from_secs(1)),
                    balls[2].position.to(Vec2::new(start_x, start_y + 2.0 * spacing_y), Duration::from_secs(1)),
                    balls[3].position.to(Vec2::new(start_x, start_y + 3.0 * spacing_y), Duration::from_secs(1)),
                    balls[4].position.to(Vec2::new(start_x, start_y + 4.0 * spacing_y), Duration::from_secs(1)),
                    balls[5].position.to(Vec2::new(start_x, start_y + 5.0 * spacing_y), Duration::from_secs(1)),
                    balls[6].position.to(Vec2::new(start_x, start_y + 6.0 * spacing_y), Duration::from_secs(1)),
                    balls[7].position.to(Vec2::new(start_x, start_y + 7.0 * spacing_y), Duration::from_secs(1)),
                ]
            ]),
            wait(Duration::from_secs(1)),
        ],
        None
    ]);

    project.show()
}
