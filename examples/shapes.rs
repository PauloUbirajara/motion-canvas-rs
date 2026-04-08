use motion_canvas_rs::engine::project::Project;
use motion_canvas_rs::engine::nodes::{Circle, Rect, Line};
use motion_canvas_rs::render::Color;
use glam::Vec2;

fn main() -> anyhow::Result<()> {
    let mut project = Project::new(800, 600);

    let circle = Circle::new(Vec2::new(400.0, 300.0), 50.0, Color::rgb8(0xe1, 0x32, 0x38)); // Red
    let rect = Rect::new(Vec2::new(100.0, 100.0), Vec2::new(200.0, 100.0), Color::rgb8(0x68, 0xab, 0xdf)) // Blue
        .with_radius(10.0);
    let line = Line::new(Vec2::new(0.0, 0.0), Vec2::new(100.0, 100.0), Color::rgb8(0xf2, 0xf2, 0xf2), 2.0); // White

    project.scene.add(Box::new(circle));
    project.scene.add(Box::new(rect));
    project.scene.add(Box::new(line));
    
    project.show()
}
