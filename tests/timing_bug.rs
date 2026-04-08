use motion_canvas_rs::prelude::*;
use std::time::Duration;

#[test]
fn test_timing() {
    let mut project = Project::new(800, 600);
    let circle = Circle::new(Vec2::new(0.0, 0.0), 10.0, Color::RED);
    
    // 1s + all![1s, 1s] should be 2s
    let anim = chain![
        circle.radius.to(20.0, Duration::from_secs(1)),
        all![
            circle.transform.to(Affine::translate((100.0, 100.0)), Duration::from_secs(1)),
            circle.radius.to(30.0, Duration::from_secs(1)),
        ]
    ];
    
    assert_eq!(anim.duration(), Duration::from_secs(2));

    // Nested all![all![1s], 1s] should be 1s
    let anim2 = all![
        all![
            circle.radius.to(20.0, Duration::from_secs(1)),
        ],
        circle.transform.to(Affine::translate((100.0, 100.0)), Duration::from_secs(1)),
    ];
    
    assert_eq!(anim2.duration(), Duration::from_secs(1));
}
