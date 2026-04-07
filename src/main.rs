use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use std::time::{Duration, Instant};
use vello::peniko::Color;

mod engine;
mod render;

use crate::engine::*;
use crate::render::VelloRenderer;

fn main() -> anyhow::Result<()> {
    let event_loop = EventLoop::new()?;
    let window = WindowBuilder::new()
        .with_title("Motion Canvas RS")
        .with_inner_size(winit::dpi::LogicalSize::new(800, 600))
        .build(&event_loop)?;

    let window: &'static winit::window::Window = Box::leak(Box::new(window));

    let mut renderer = VelloRenderer::new();
    pollster::block_on(renderer.resume(window));

    let mut scene = BaseScene::new();
    
    let circle = Box::new(Circle {
        position: Signal::new(glam::vec2(100.0, 100.0)),
        radius: Signal::new(50.0),
        fill: Color::rgb8(32, 178, 170),
    });

    let rect = Box::new(Rect {
        position: Signal::new(glam::vec2(600.0, 100.0)),
        size: Signal::new(glam::vec2(100.0, 100.0)),
        fill: Color::rgb8(255, 100, 100),
        radius: 10.0,
    });

    // Animate circle with spring-like effect
    scene.timeline.add(all(vec![
        circle.radius.to_with_easing(150.0, Duration::from_secs(2), easing::elastic_out),
        circle.position.to_with_easing(glam::vec2(400.0, 300.0), Duration::from_secs(2), easing::quad_in_out),
    ]));

    // Animate rect
    scene.timeline.add(
        rect.position.to_with_easing(glam::vec2(200.0, 400.0), Duration::from_secs(2), easing::cubic_in_out)
    );

    scene.add(circle);
    scene.add(rect);

    let mut last_update = Instant::now();

    event_loop.run(move |event, elwt| {
        elwt.set_control_flow(ControlFlow::Poll);

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => elwt.exit(),
                WindowEvent::RedrawRequested => {
                    let size = window.inner_size();
                    renderer.render(&scene, size.width, size.height);
                }
                _ => {}
            },
            Event::AboutToWait => {
                let now = Instant::now();
                let dt = now - last_update;
                last_update = now;

                scene.update(dt);
                window.request_redraw();
            }
            _ => {}
        }
    })?;

    Ok(())
}
