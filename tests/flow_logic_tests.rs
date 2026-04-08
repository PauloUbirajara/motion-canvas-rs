use motion_canvas_rs::prelude::*;
use std::time::Duration;

#[test]
fn test_all_macro_duration() {
    let circle = Circle::new(Vec2::new(0.0, 0.0), 10.0, Color::RED);
    
    // (1s, all!(1s, 2s)) should take 3 seconds
    // This implies a chain of 1s and an all![1s, 2s]
    let anim = chain![
        circle.radius.to(20.0, Duration::from_secs(1)),
        all![
            circle.radius.to(30.0, Duration::from_secs(1)),
            circle.radius.to(40.0, Duration::from_secs(2)),
        ]
    ];
    
    assert_eq!(anim.duration(), Duration::from_secs(3));
}

#[test]
fn test_any_macro_duration() {
    let circle = Circle::new(Vec2::new(0.0, 0.0), 10.0, Color::RED);
    
    // (1s, any!(1s, 2s, 0.5s)) should take 1.5 seconds
    let anim = chain![
        circle.radius.to(20.0, Duration::from_secs(1)),
        any![
            circle.radius.to(30.0, Duration::from_secs(1)),
            circle.radius.to(40.0, Duration::from_secs(2)),
            circle.radius.to(50.0, Duration::from_secs_f32(0.5)),
        ]
    ];
    
    assert_eq!(anim.duration(), Duration::from_secs_f32(1.5));
}

#[test]
fn test_timeline_sequential_behavior() {
    let mut project = Project::new(800, 600);
    let circle = Circle::new(Vec2::new(0.0, 0.0), 10.0, Color::RED);
    
    // Adding to timeline without macro should be sequential
    project.scene.timeline.add(circle.radius.to(20.0, Duration::from_secs(1)));
    project.scene.timeline.add(circle.radius.to(30.0, Duration::from_secs(1)));
    project.scene.timeline.add(circle.radius.to(40.0, Duration::from_secs(1)));
    
    // Expected total duration: 3s
    assert_eq!(project.scene.timeline.duration(), Duration::from_secs(3));
    
    // After 1.5s, only the second animation should be halfway done, or the first should be finished
    // If it's sequential:
    // 0-1s: anim 1
    // 1-2s: anim 2
    // 2-3s: anim 3
    
    project.scene.timeline.update(Duration::from_millis(1500));
    
    // First anim should be gone, second anim should have had 0.5s of progress
    // circle.radius started at 10.0
    // anim 1: 10.0 -> 20.0 (finished)
    // anim 2: 20.0 -> 30.0 (halfway) -> 25.0
    assert_eq!(circle.radius.get(), 25.0);
}
