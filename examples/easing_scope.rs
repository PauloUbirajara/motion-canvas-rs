use motion_canvas_rs::prelude::*;
use std::time::Duration;

fn main() {
    let mut project = Project::new(800, 800);

    let easing_configs = vec![
        ("Linear", easings::linear as fn(f32) -> f32),
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
    }

    let balls_clone = balls.clone();
    let configs_clone = easing_configs.clone();

    // Loop animation factory
    project.scene.timeline.add(flows::loop_anim(move || {
        let balls = &balls_clone;
        let configs = &configs_clone;
        
        // 1. Move to right using individual scoped easings
        let right_anims = flows::all(configs.iter().enumerate().map(|(i, (_, easing))| {
            let end_pos = Vec2::new(end_x, start_y + i as f32 * spacing_y);
            flows::with_easing(*easing, vec![balls[i].position.to(end_pos, Duration::from_secs(2)).into()])
        }).collect());

        // 2. Move back to left using the SAME scoped easings
        let left_anims = flows::all(configs.iter().enumerate().map(|(i, (_, easing))| {
            let start_pos = Vec2::new(start_x, start_y + i as f32 * spacing_y);
            flows::with_easing(*easing, vec![balls[i].position.to(start_pos, Duration::from_secs(2)).into()])
        }).collect());

        flows::chain(vec![
            right_anims,
            flows::wait(Duration::from_secs(1)),
            left_anims,
            flows::wait(Duration::from_secs(1)),
        ])
    }, None));

    project.show().expect("Failed to render");
}
