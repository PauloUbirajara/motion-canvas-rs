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
    // anim 1: 10.0 -> 20.0 (finished at 1s, 0.5s leftover)
    // anim 2: 20.0 -> 30.0 (halfway) -> 25.0
    // (Timeline::update correctly passes the 0.5s leftover to anim 2)
    assert_eq!(circle.radius.get(), 25.0);
}

#[test]
fn test_sequence_macro_duration() {
    let circle = Circle::new(Vec2::new(0.0, 0.0), 10.0, Color::RED);
    
    // sequence!(0.5s, [1s, 1s])
    // anim 1 starts at 0s, finishes at 1s
    // anim 2 starts at 0.5s, finishes at 1.5s
    // Total should be 1.5s
    let anim = sequence![
        Duration::from_millis(500),
        circle.radius.to(20.0, Duration::from_secs(1)),
        circle.radius.to(30.0, Duration::from_secs(1)),
    ];
    
    assert_eq!(anim.duration(), Duration::from_millis(1500));
}

#[test]
fn test_delay_macro_duration() {
    let circle = Circle::new(Vec2::new(0.0, 0.0), 10.0, Color::RED);
    
    // delay!(1s, 1s) should be 2s
    let anim = delay![
        Duration::from_secs(1),
        circle.radius.to(20.0, Duration::from_secs(1)),
    ];
    
    assert_eq!(anim.duration(), Duration::from_secs(2));
}

#[test]
fn test_loop_anim_macro_duration() {
    let circle = Circle::new(Vec2::new(0.0, 0.0), 10.0, Color::RED);
    
    // loop_anim!(1s, 3) should be 3s
    let anim = loop_anim![
        circle.radius.to(20.0, Duration::from_secs(1)),
        Some(3)
    ];
    
    assert_eq!(anim.duration(), Duration::from_secs(3));
}

#[test]
fn test_wait_duration() {
    let anim = wait(Duration::from_secs(2));
    assert_eq!(anim.duration(), Duration::from_secs(2));
}

#[test]
fn test_with_easing_macro_duration() {
    let circle = Circle::new(Vec2::new(0.0, 0.0), 10.0, Color::RED);
    
    // with_easing!(ease, [1s, 2s]) should take 2s (behaves like all!)
    let anim = with_easing![
        easings::cubic_in_out,
        [
            circle.radius.to(20.0, Duration::from_secs(1)),
            circle.radius.to(30.0, Duration::from_secs(2)),
        ]
    ];
    
    assert_eq!(anim.duration(), Duration::from_secs(2));
}

#[test]
fn test_delay_execution() {
    let circle = Circle::new(Vec2::new(0.0, 0.0), 10.0, Color::RED);
    let mut anim = delay![
        Duration::from_secs(1),
        circle.radius.to(20.0, Duration::from_secs(1)).ease(easings::linear),
    ];
    
    // Update by 0.5s: Should still be 10.0
    anim.update(Duration::from_millis(500));
    assert_eq!(circle.radius.get(), 10.0);
    
    // Update by another 0.8s: Total 1.3s. Anim should have 0.3s progress.
    anim.update(Duration::from_millis(800));
    assert_eq!(circle.radius.get(), 13.0);
}

#[test]
fn test_loop_anim_execution() {
    let circle = Circle::new(Vec2::new(0.0, 0.0), 10.0, Color::RED);
    
    // loop anim that increments radius by 10 over 1s, 3 times
    let radius = circle.radius.clone();
    let mut anim = loop_anim!({
        let r = radius.clone();
        let start = r.get();
        r.to(start + 10.0, Duration::from_secs(1))
    }, Some(3));
    
    // Round 1
    anim.update(Duration::from_millis(500));
    assert_eq!(circle.radius.get(), 15.0);
    
    // Finish Round 1 and start Round 2 in same frame
    // Total update 1.5s (0.5 + 1.0)
    anim.update(Duration::from_secs(1));
    assert_eq!(circle.radius.get(), 25.0); 
    
    // Finish Round 2 and Round 3
    let (finished, leftover) = anim.update(Duration::from_secs(2));
    assert!(finished);
    assert_eq!(circle.radius.get(), 40.0);
    assert_eq!(leftover, Duration::from_millis(500));
}
