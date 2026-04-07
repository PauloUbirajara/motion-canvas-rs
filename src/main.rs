use std::time::{Duration, Instant};
use vello::peniko::Color;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

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

    // Shared static reference to satisfy 'static bound
    let window: &'static winit::window::Window = Box::leak(Box::new(window));

    let mut renderer = VelloRenderer::new();
    pollster::block_on(renderer.resume(window));

    let mut scene = BaseScene::new();

    let mut circle = Box::new(Circle {
        position: Signal::new(glam::vec2(400.0, 300.0)),
        radius: Signal::new(50.0),
        fill: Color::rgb8(32, 178, 170),
    });

    circle.radius.to(150.0, Duration::from_secs(2));
    scene.add(circle);

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
