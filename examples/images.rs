use motion_canvas_rs::prelude::*;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    let mut project = Project::new(800, 600);

    // Using the sample logo path from the project
    let logo = ImageNode::new(
        Vec2::new(400.0, 300.0), 
        "./examples/images/motion-canvas-logo.png"
    ).with_size(Vec2::new(200.0, 200.0));

    project.scene.add(Box::new(logo.clone()));

    project.scene.timeline.add(
        logo.position.to(Vec2::new(400.0, 100.0), Duration::from_secs(1))
    );
    
    project.show()
}
