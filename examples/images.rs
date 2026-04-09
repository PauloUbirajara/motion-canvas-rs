use motion_canvas_rs::prelude::*;
use std::time::Duration;

fn main() {
    let mut project = Project::default()
        .with_dimensions(600, 600)
        .close_on_finish();

    // Using the sample logo path from the project
    let png = ImageNode::default()
        .with_position(Vec2::new(350.0, 350.0))
        .with_path("./examples/images/motion-canvas-logo.png")
        .with_size(Vec2::new(200.0, 200.0));

    let svg = ImageNode::default()
        .with_position(Vec2::new(50.0, 50.0))
        .with_path("./examples/images/motion-canvas-rs.svg")
        .with_size(Vec2::new(200.0, 200.0));

    project.scene.video_timeline.add(chain!(
        all!(
            png.transform
                .to(Affine::translate((50.0, 350.0)), Duration::from_secs(1)),
            svg.transform
                .to(Affine::translate((350.0, 50.0)), Duration::from_secs(1)),
        ),
        all!(
            png.transform
                .to(Affine::translate((50.0, 50.0)), Duration::from_secs(1)),
            svg.transform
                .to(Affine::translate((350.0, 350.0)), Duration::from_secs(1)),
        ),
        all!(
            png.transform
                .to(Affine::translate((350.0, 50.0)), Duration::from_secs(1)),
            svg.transform
                .to(Affine::translate((50.0, 350.0)), Duration::from_secs(1)),
        ),
        all!(
            png.transform
                .to(Affine::translate((350.0, 350.0)), Duration::from_secs(1)),
            svg.transform
                .to(Affine::translate((50.0, 50.0)), Duration::from_secs(1)),
        ),
    ));

    project.scene.add(Box::new(png.clone()));
    project.scene.add(Box::new(svg.clone()));

    project.show().expect("Failed to render");
}
