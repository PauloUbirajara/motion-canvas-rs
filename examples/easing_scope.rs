use motion_canvas_rs::prelude::*;
use std::time::Duration;

const CANVAS_WIDTH: u32 = 800;
const CANVAS_HEIGHT: u32 = 800;
const START_X: f32 = 150.0;
const END_X: f32 = 750.0;
const START_Y: f32 = 100.0;
const SPACING_Y: f32 = 80.0;
const ANIM_DURATION_GO: Duration = Duration::from_secs(2);
const ANIM_DURATION_RETURN: Duration = Duration::from_secs(2);
const WAIT_DURATION: Duration = Duration::from_secs(1);

fn main() {
    let mut project = Project::default()
        .with_dimensions(CANVAS_WIDTH, CANVAS_HEIGHT)
        .close_on_finish();

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
        let y = START_Y + (i as f32 * SPACING_Y);
        let ball = Circle::default()
            .with_position(Vec2::new(START_X, y))
            .with_radius(20.0)
            .with_color(colors[i % colors.len()]);
        let label = TextNode::default()
            .with_position(Vec2::new(START_X - 130.0, y))
            .with_text(name)
            .with_font_size(18.0)
            .with_color(Color::rgb8(0xcc, 0xcc, 0xcc));

        project.scene.add(Box::new(ball.clone()));
        project.scene.add(Box::new(label.clone()));

        balls.push(ball);
    }

    let balls_clone = balls.clone();
    let configs_clone = easing_configs.clone();

    // Loop animation factory
    project.scene.video_timeline.add(flows::loop_anim(
        move || {
            let balls = &balls_clone;
            let configs = &configs_clone;

            // 1. Move to right using individual scoped easings
            let right_anims = flows::all(
                configs
                    .iter()
                    .enumerate()
                    .map(|(i, (_, easing))| {
                        let end_pos = Vec2::new(END_X, START_Y + i as f32 * SPACING_Y);
                        flows::with_easing(
                            *easing,
                            vec![balls[i]
                                .position
                                .to(end_pos, ANIM_DURATION_GO)
                                .into()],
                        )
                    })
                    .collect(),
            );

            // 2. Move back to left using the SAME scoped easings
            let left_anims = flows::all(
                configs
                    .iter()
                    .enumerate()
                    .map(|(i, (_, easing))| {
                        let start_pos = Vec2::new(START_X, START_Y + i as f32 * SPACING_Y);
                        flows::with_easing(
                            *easing,
                            vec![balls[i]
                                .position
                                .to(start_pos, ANIM_DURATION_RETURN)
                                .into()],
                        )
                    })
                    .collect(),
            );

            flows::chain(vec![
                right_anims,
                flows::wait(WAIT_DURATION),
                left_anims,
                flows::wait(WAIT_DURATION),
            ])
        },
        None,
    ));

    project.show().expect("Failed to render");
}
