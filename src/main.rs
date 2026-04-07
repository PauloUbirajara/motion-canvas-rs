use std::time::Duration;
use vello::kurbo::BezPath;
use vello::peniko::Color;

mod engine;
mod render;

use crate::engine::*;

fn main() -> anyhow::Result<()> {
    // 1. Initialize Project
    let mut project = Project::new(800, 600)
        .with_fps(60)
        .with_cache(false)
        .with_title("Path Follow Demo");

    // 2. Create a Path
    let mut path = BezPath::new();
    path.move_to((100.0, 300.0));
    path.curve_to((250.0, 100.0), (550.0, 500.0), (700.0, 300.0));

    let path_node = PathNode::new(path, Color::rgb8(100, 100, 100), 2.0);

    // 3. Create a Follower
    let circle = Circle {
        position: Signal::new(glam::vec2(100.0, 300.0)),
        radius: Signal::new(20.0),
        fill: Color::rgb8(255, 165, 0),
    };

    // 4. Define Animation
    // Make the circle follow the path, then come back!
    project.scene.timeline.add(chain![
        circle
            .position
            .follow(&path_node, Duration::from_secs(3))
            .ease(easings::cubic_in_out),
        wait(Duration::from_millis(500)),
        circle
            .radius
            .to(40.0, Duration::from_millis(500))
            .ease(easings::elastic_out),
        circle
            .radius
            .to(20.0, Duration::from_millis(500))
            .ease(easings::quad_in)
    ]);

    // 5. Add nodes to scene
    project.scene.add(Box::new(path_node));
    project.scene.add(Box::new(circle));

    // 6. Run
    // project.show();
    project.export()
}
