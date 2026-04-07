use motion_canvas_rs::prelude::*;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    let mut project = Project::new(800, 600);
    
    let circle = Circle::new(Vec2::new(200.0, 300.0), 50.0, Color::RED);
    let rect = Rect::new(Vec2::new(400.0, 300.0), Vec2::new(100.0, 100.0), Color::BLUE);
    let text = TextNode::new(Vec2::new(400.0, 500.0), "Flow Demo", 30.0, Color::WHITE);

    project.scene.add(Box::new(circle.clone()));
    project.scene.add(Box::new(rect.clone()));
    project.scene.add(Box::new(text.clone()));

    // Complex Flow Logic
    project.scene.timeline.add(chain![
        // Start with parallel expansion
        all![
            circle.radius.to(100.0, Duration::from_secs(1)),
            rect.size.to(Vec2::new(300.0, 150.0), Duration::from_secs(1)),
        ],
        // Wait for impact
        wait(Duration::from_millis(500)),
        // Sequential movement
        sequence![
            Duration::from_millis(200),
            circle.position.to(Vec2::new(100.0, 100.0), Duration::from_secs(1)),
            rect.position.to(Vec2::new(500.0, 100.0), Duration::from_secs(1)),
        ],
        // Delayed conclusion
        delay![
            Duration::from_millis(200), 
            text.color.to(Color::YELLOW, Duration::from_millis(500))
        ],
    ]);

    project.show()
}
