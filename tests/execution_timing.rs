use motion_canvas_rs::prelude::*;
use std::time::Duration;

#[test]
fn test_execution_timing() {
    let mut project = Project::new(800, 600);
    let circle = Circle::new(Vec2::new(0.0, 0.0), 10.0, Color::RED);
    
    // 1s + 1s = 2s total duration
    let mut anim = chain![
        circle.radius.to(20.0, Duration::from_secs(1)),
        circle.radius.to(30.0, Duration::from_secs(1))
    ];
    
    assert_eq!(anim.duration(), Duration::from_secs(2));
    
    // Update by 1s
    let finished_1 = anim.update(Duration::from_secs(1)).0;
    assert!(!finished_1, "Should not be finished after 1s");
    
    // Update by 1s more
    let finished_2 = anim.update(Duration::from_secs(1)).0;
    assert!(finished_2, "Should be finished after 2s");
}

#[test]
fn test_all_execution_timing() {
    let mut _project = Project::new(800, 600);
    let circle = Circle::new(Vec2::new(0.0, 0.0), 10.0, Color::RED);
    
    // all![1s, 1s] = 1s total duration
    let mut anim = all![
        circle.radius.to(20.0, Duration::from_secs(1)),
        circle.transform.to(Affine::translate((10.0, 10.0)), Duration::from_secs(1))
    ];
    
    assert_eq!(anim.duration(), Duration::from_secs(1));
    
    // Update by 0.5s
    assert!(!anim.update(Duration::from_millis(500)).0);
    assert_eq!(circle.radius.get(), 15.0); // Halfway
    
    // Update by another 0.5s
    assert!(anim.update(Duration::from_millis(500)).0);
    assert_eq!(circle.radius.get(), 20.0); // Finished
}
