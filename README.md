# Motion Canvas in Rust

![example](examples/images/motion-canvas-rs.svg)

A high-performance vector animation engine inspired by Motion Canvas, built on Vello and Typst.

## Features

- High-performance vector rendering using Vello.
- Typst-powered LaTeX math rendering.
- Syntax-highlighted code blocks via Syntect.
- System font discovery and path-based text rendering.
- Support for bitmap images (PNG, JPEG, etc.) and Vector graphics (SVG).
- Native SVG rasterization using `resvg` for high-fidelity vector rendering.
- Parallel PNG encoding and direct FFmpeg streaming.
- Arc-length sampled path animations.

## Project Structure

The engine is organized into a modular structure:

- `src/lib.rs`: Library entry point with clean module re-exports.
- `src/engine/nodes/`: Individual node implementations.
- `src/engine/animation/`: Core animation traits and flow controls.
- `src/engine/easings.rs`: Comprehensive easing function library.
- `examples/`: Ready-to-run demonstration scripts.

## Quick Start (Getting Started)

```rust
use motion_canvas_rs::prelude::*;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    let mut project = Project::new(800, 600);

    let circle = Circle::new(Vec2::new(400.0, 300.0), 50.0, Color::RED);
    let text = TextNode::new(Vec2::new(400.0, 450.0), "Hello Rust", 40.0, Color::WHITE);

    project.scene.add(Box::new(circle.clone()));
    project.scene.add(Box::new(text.clone()));

    project.scene.timeline.add(all![
        circle.radius.to(100.0, Duration::from_secs(1)),
        text.position.to(Vec2::new(400.0, 500.0), Duration::from_secs(1)),
    ]);

    project.show()
}
```

## Running Examples

The project includes several formal examples covering different features. You can run them using `cargo run --example <name>`:

| Example | Command | Description |
| :--- | :--- | :--- |
| **Getting Started** | `cargo run --example getting_started` | Basic node creation and animation. |
| **Shapes** | `cargo run --example shapes` | Circle, Rect, and Line primitives. |
| **Math & Code** | `cargo run --example math_code` | Typst LaTeX and Syntax Highlighting. |
| **Images** | `cargo run --example images` | Bitmap image support and transformations. |
| **Advanced Flow** | `cargo run --example advanced_flow` | Complex staggered and sequential animations. |
| **Export** | `cargo run --example export` | Video export with color and font-size animations. |

## Requirements

- Rust 1.75+
- FFmpeg (optional, for direct video streaming)
- System fonts (Inter, Fira Code, etc. for specific examples)

## Credits

This project is heavily inspired by the original [Motion Canvas](https://github.com/motion-canvas/motion-canvas) by [aarthificial](https://github.com/aarthificial). It aims to be a proof of concept of the same declarative animation feel in Rust.
