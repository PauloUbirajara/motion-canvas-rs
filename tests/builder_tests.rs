use glam::Vec2;
use motion_canvas_rs::prelude::*;
use peniko::Color;

#[test]
fn test_project_builder() {
    let project = Project::default()
        .with_dimensions(1280, 720)
        .with_fps(30)
        .with_title("Test Project")
        .with_background(Color::BLUE)
        .close_on_finish();

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
        .with_fill(Color::BLUE);

    // Check initial values via signals
    let position = circle.position.get();
    assert_eq!(position.x, 100.0);
    assert_eq!(position.y, 200.0);
    assert_eq!(circle.radius.get(), 75.0);
    assert_eq!(circle.fill_paint.get(), Paint::Solid(Color::BLUE));
}

#[test]
fn test_rect_builder() {
    let rect = Rect::default()
        .with_position(Vec2::new(10.0, 10.0))
        .with_size(Vec2::new(200.0, 300.0))
        .with_radius(10.0);

    assert_eq!(rect.position.get(), Vec2::new(10.0, 10.0));
    assert_eq!(rect.size.get(), Vec2::new(200.0, 300.0));
    assert_eq!(rect.radius.get(), 10.0);
}

#[test]
fn test_line_builder() {
    let line = Line::default()
        .with_start(Vec2::new(0.0, 0.0))
        .with_end(Vec2::new(100.0, 100.0))
        .with_stroke(Color::GREEN, 5.0);

    assert_eq!(line.start.get(), Vec2::new(0.0, 0.0));
    assert_eq!(line.end.get(), Vec2::new(100.0, 100.0));
    assert_eq!(line.stroke_width.get(), 5.0);
    assert_eq!(line.stroke_paint.get(), Paint::Solid(Color::GREEN));
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
    assert_eq!(polygon.fill_paint.get(), Paint::Solid(Color::RED));
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
        .with_language("rust")
        .with_code("let mut engine = MotionCanvas::new();\nengine.render();");

    assert_eq!(code.language, "rust");

    use motion_canvas_rs::assets::code_tokenizer::tokenize_code;

    println!("--- TOKEN POSITIONS & WIDTHS ---");
    let tokens = tokenize_code(
        "let mut engine = MotionCanvas::new();\nengine.render();",
        24.0,
        "rust",
        "base16-ocean.dark",
        "Fira Code",
        &["Courier New", "monospace"],
    );
    for t in &tokens {
        println!(
            "Token {:?}: pos={:?}, width={}, glyphs={}",
            t.text,
            t.pos,
            t.width,
            t.glyphs.len()
        );
        if t.text.contains('.') || t.text.contains(':') {
            for (idx, (transform, pb)) in t.glyphs.iter().enumerate() {
                println!(
                    "  Glyph #{}: transform={:?}, elements={:?}",
                    idx,
                    transform,
                    pb.elements()
                );
            }
        }
    }
    println!("---------------------------------");
}

#[test]
fn test_group_builder() {
    let group = GroupNode::default()
        .with_position(Vec2::new(50.0, 50.0))
        .with_opacity(0.5);

    assert_eq!(group.position.get(), Vec2::new(50.0, 50.0));
    assert_eq!(group.opacity.get(), 0.5);
}

#[test]
#[cfg(feature = "image")]
fn test_image_builder() {
    let image = ImageNode::default()
        .with_path("nonexistent.png") // This won't load anything but we can check size
        .with_size(Vec2::new(100.0, 100.0));

    assert_eq!(image.size.get(), Vec2::new(100.0, 100.0));
}

#[test]
fn test_project_frame_naming() {
    let project = Project::default().with_title(" My Project (Demo) ");

    let name = project.get_frame_name(42);
    assert_eq!(name, "my_project_demo_0042.png");

    // Test default behavior with complex title
    let project_default = Project::default()
        .with_title("New  Project")
        .close_on_finish();
    let name_default = project_default.get_frame_name(0);
    assert_eq!(name_default, "new_project_0000.png");
}

#[test]
#[cfg(feature = "physics")]
fn test_physics_body_builders() {
    use motion_canvas_rs::prelude::{PhysicsShape, RigidBodyNode, StaticBodyNode};

    let circle = Circle::default();
    let rigid = RigidBodyNode::new(Box::new(circle))
        .with_position(Vec2::new(100.0, 100.0))
        .with_rotation(0.5)
        .with_shape(PhysicsShape::Ball(25.0))
        .with_bounciness(0.7)
        .with_friction(0.3)
        .with_initial_velocity(Vec2::new(10.0, 20.0))
        .with_initial_angular_velocity(2.0);

    assert_eq!(rigid.position.get(), Vec2::new(100.0, 100.0));
    assert_eq!(rigid.rotation.get(), 0.5);
    assert_eq!(rigid.bounciness, 0.7);
    assert_eq!(rigid.friction, 0.3);
    assert_eq!(rigid.initial_velocity, Vec2::new(10.0, 20.0));
    assert_eq!(rigid.initial_angular_velocity, 2.0);

    let rect = Rect::default();
    let static_body = StaticBodyNode::new(Box::new(rect))
        .with_position(Vec2::new(200.0, 200.0))
        .with_rotation(0.1)
        .with_shape(PhysicsShape::Cuboid(Vec2::new(50.0, 50.0)))
        .with_bounciness(0.4)
        .with_friction(0.8);

    assert_eq!(static_body.position.get(), Vec2::new(200.0, 200.0));
    assert_eq!(static_body.rotation.get(), 0.1);
    assert_eq!(static_body.bounciness, 0.4);
    assert_eq!(static_body.friction, 0.8);

    // Test PhysicsShape::Custom
    let custom_shape = PhysicsShape::Custom(std::sync::Arc::new(|| {
        rapier2d::prelude::ColliderBuilder::capsule_y(10.0, 5.0)
    }));
    let rigid_custom = RigidBodyNode::new(Box::new(Rect::default())).with_shape(custom_shape);

    assert!(matches!(rigid_custom.shape, PhysicsShape::Custom(_)));
    assert_eq!(format!("{:?}", rigid_custom.shape), "Custom");
}

#[test]
#[cfg(feature = "audio")]
fn test_audio_event_offsets() {
    use motion_canvas_rs::core::scene::BaseScene;
    use motion_canvas_rs::prelude::*;
    use std::time::Duration;

    let mut scene = BaseScene::new();

    // Create a chain of audio animations
    let play1 = play!(AudioNode::new("a.mp3").with_volume(0.5)); // duration 1s
    let wait1 = audio_wait!(2.0); // duration 2s
    let play2 = play!(AudioNode::new("b.mp3").with_volume(1.0)); // duration 1s

    scene.audio_timeline.add(chain!(play1, wait1, play2));

    // Simulate the frame-by-frame export loop
    let mut events = Vec::new();
    let dt = Duration::from_millis(100);
    for _ in 0..40 {
        scene
            .audio_timeline
            .collect_audio_events(Duration::ZERO, &mut events);
        scene.audio_timeline.update(dt);
    }
    scene.collect_audio_events(Duration::ZERO, &mut events);

    // Filter events to find the ones we pushed
    let a_event = events.iter().find(|e| e.path == "a.mp3").unwrap();
    let b_event = events.iter().find(|e| e.path == "b.mp3").unwrap();

    // play1 starts at 0.0s
    assert_eq!(a_event.start_time, Duration::from_secs(0));
    // play2 starts at play1 duration (1s) + wait1 duration (2s) = 3s
    assert_eq!(b_event.start_time, Duration::from_secs(3));
}

#[test]
fn test_gradient_macros() {
    let grad_lin = linear_gradient!(Color::RED, Color::GREEN, Color::BLUE);
    assert!(matches!(grad_lin.kind, GradientKind::Linear { .. }));
    let stops = grad_lin.stops.as_slice();
    assert_eq!(stops.len(), 3);
    assert_eq!(stops[0].offset, 0.0);
    assert_eq!(stops[0].color, Color::RED);
    assert_eq!(stops[1].offset, 0.5);
    assert_eq!(stops[1].color, Color::GREEN);
    assert_eq!(stops[2].offset, 1.0);
    assert_eq!(stops[2].color, Color::BLUE);

    let grad_rad = radial_gradient!(Color::YELLOW, Color::CYAN);
    assert!(matches!(grad_rad.kind, GradientKind::Radial { .. }));
    let stops_rad = grad_rad.stops.as_slice();
    assert_eq!(stops_rad.len(), 2);
    assert_eq!(stops_rad[0].offset, 0.0);
    assert_eq!(stops_rad[0].color, Color::YELLOW);
    assert_eq!(stops_rad[1].offset, 1.0);
    assert_eq!(stops_rad[1].color, Color::CYAN);
}

#[test]
#[should_panic(expected = "Gradients require at least 2 colors")]
fn test_gradient_macro_less_than_two_colors() {
    let _ = linear_gradient!(Color::RED);
}

#[test]
#[should_panic(expected = "Gradients require at least 2 colors")]
fn test_radial_gradient_macro_less_than_two_colors() {
    let _ = radial_gradient!(Color::RED);
}

#[test]
#[cfg(feature = "runtime")]
fn test_peniko_mix() {
    let mut scene = vello::Scene::new();
    let bm = peniko::BlendMode {
        mix: peniko::Mix::Normal,
        compose: peniko::Compose::SrcIn,
    };
    scene.push_layer(
        bm,
        1.0,
        kurbo::Affine::IDENTITY,
        &kurbo::Rect::new(-10.0, -10.0, 10.0, 10.0),
    );
    scene.pop_layer();
}

#[test]
fn test_mask_node_builder() {
    let mask_circle = Circle::default().with_radius(50.0);
    let source_rect = Rect::default().with_size(Vec2::new(100.0, 100.0));

    let mask_node = MaskNode::new(Box::new(mask_circle), Box::new(source_rect))
        .with_position(Vec2::new(10.0, 20.0))
        .with_mode(MaskMode::Subtract);

    assert_eq!(mask_node.position.get(), Vec2::new(10.0, 20.0));
    assert_eq!(mask_node.mode.get(), MaskMode::Subtract);

    mask_node.mode.set(MaskMode::Intersect);
    assert_eq!(mask_node.mode.get(), MaskMode::Intersect);

    mask_node.mode.set(MaskMode::Union);
    assert_eq!(mask_node.mode.get(), MaskMode::Union);

    mask_node.mode.set(MaskMode::Exclude);
    assert_eq!(mask_node.mode.get(), MaskMode::Exclude);
}
