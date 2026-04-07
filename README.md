# Motion Canvas in Rust

A high-performance vector animation engine inspired by Motion Canvas, built on Vello and Typst.

## Features

- High-performance vector rendering using Vello.
- Typst-powered LaTeX math rendering.
- Syntax-highlighted code blocks via Syntect.
- System font discovery and path-based text rendering.
- Parallel PNG encoding and direct FFmpeg streaming.
- Arc-length sampled path animations.

## Usage Examples

<details>
<summary>Complete Getting Started Example</summary>

```rust
use motion_canvas_rs::engine::animation::{all, chain, wait};
use motion_canvas_rs::engine::node::{Circle, TextNode};
use motion_canvas_rs::engine::Project;
use glam::Vec2;
use vello::peniko::Color;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    // 1. Initialize the Project
    let mut project = Project::new(800, 600)
        .with_fps(60)
        .with_ffmpeg(true)
        .with_cache(true);

    // 2. Define Nodes
    let circle = Circle::new(Vec2::new(400.0, 300.0), 50.0, Color::RED);
    let text = TextNode::new(Vec2::new(400.0, 450.0), "Hello Rust", 40.0, Color::WHITE);

    // 3. Add Animations to the Scene
    project.add(all![
        circle.radius.to(100.0, Duration::from_secs(1)),
        text.position.to(Vec2::new(400.0, 500.0), Duration::from_secs(1)),
    ]);

    // 4. Export the Animation to out.mkv
    project.export()
}
```
</details>

<details>
<summary>Shapes and Layout</summary>

```rust
use motion_canvas_rs::engine::node::{Circle, Rect, Line};
use motion_canvas_rs::engine::Project;
use glam::Vec2;
use vello::peniko::Color;

fn main() -> anyhow::Result<()> {
    // Initialization
    let mut project = Project::new(800, 600);

    // Shapes
    let circle = Circle::new(Vec2::new(400.0, 300.0), 50.0, Color::RED);
    let rect = Rect::new(Vec2::new(100.0, 100.0), Vec2::new(200.0, 100.0), Color::BLUE)
        .with_radius(10.0);
    let line = Line::new(Vec2::new(0.0, 0.0), Vec2::new(100.0, 100.0), Color::WHITE, 2.0);

    // Add to project
    project.add(circle.radius.to(60.0, std::time::Duration::from_secs(0))); // Static add
    
    // Show Interactive Window
    project.show()
}
```
</details>

<details>
<summary>Math and Code Rendering</summary>

```rust
use motion_canvas_rs::engine::node::{CodeNode, MathNode};
use motion_canvas_rs::engine::Project;
use glam::Vec2;
use vello::peniko::Color;

fn main() -> anyhow::Result<()> {
    let mut project = Project::new(1920, 1080);

    // Syntax-highlighted Code
    let code = CodeNode::new(
        Vec2::new(50.0, 400.0), 
        "fn main() {\n    println!(\"Hello\");\n}", 
        "rs"
    );

    // Typst LaTeX Math
    let math = MathNode::new(
        Vec2::new(50.0, 200.0), 
        "e^{i\\pi} + 1 = 0", 
        60.0, 
        Color::WHITE
    );

    project.add(math.font_size.to(80.0, std::time::Duration::from_secs(1)));
    
    project.export()
}
```
</details>

<details>
<summary>Advanced Animation Flows</summary>

```rust
use motion_canvas_rs::engine::animation::{all, chain, delay, wait, sequence};
use motion_canvas_rs::engine::Project;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    let mut project = Project::new(800, 600);
    // ... nodes initialization ...

    // Complex Flow Logic
    project.add(chain![
        // Start with parallel expansion
        all![
            circle.radius.to(100.0, Duration::from_secs(1)),
            rect.size.to(Vec2::new(300.0, 150.0), Duration::from_secs(1)),
        ],
        // Wait for impact
        wait(Duration::from_secs(0.5)),
        // Sequential movement
        sequence![
            circle.position.to(Vec2::new(0.0, 0.0), Duration::from_secs(1)),
            rect.position.to(Vec2::new(100.0, 100.0), Duration::from_secs(1)),
        ],
        // Delayed conclusion
        delay(Duration::from_secs(0.2), text.color.to(Color::YELLOW, Duration::from_secs(0.5))),
    ]);

    project.export()
}
```
</details>

## Requirements

- Rust 1.75+
- FFmpeg (optional, for direct video streaming)
- System fonts (Inter, Fira Code, etc. for specific examples)
