use motion_canvas_rs::prelude::*;

fn main() -> anyhow::Result<()> {
    let mut project = Project::new(800, 600);

    let circle = Circle::new(Vec2::new(400.0, 300.0), 50.0, Color::RED);
    let rect = Rect::new(Vec2::new(100.0, 100.0), Vec2::new(200.0, 100.0), Color::BLUE)
        .with_radius(10.0);
    let line = Line::new(Vec2::new(0.0, 0.0), Vec2::new(100.0, 100.0), Color::WHITE, 2.0);

    project.scene.add(Box::new(circle));
    project.scene.add(Box::new(rect));
    project.scene.add(Box::new(line));
    
    project.show()
}
