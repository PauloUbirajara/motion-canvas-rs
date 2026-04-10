# Motion Canvas in Rust

![Motion Canvas Banner](https://github.com/user-attachments/assets/e951737f-36ed-4a5a-829a-9643c7c1a3d8)

A high-performance vector animation engine inspired by Motion Canvas, built on Vello and Typst.

> [!IMPORTANT]
> **Prototype Status**: This project is a functional prototype and proof-of-concept. It is **not** a 1:1 implementation of the original Motion Canvas API or features.

## Installation

Add the library to your `Cargo.toml`. To enable all features (math, code blocks, images, export), use the `full` flag:

```bash
# Enable everything
cargo add motion-canvas-rs --features full

# Or pick only what you need (e.g., just math, images, and audio)
cargo add motion-canvas-rs --features math,image,audio
```

## Features

| Feature | Description | Enables |
|:---|:---|:---|
| `math` | Typst-powered LaTeX math rendering. | `MathNode` |
| `code` | Syntax-highlighted code blocks via Syntect. | `CodeNode` |
| `image` | Bitmap (PNG, JPEG) and Vector (SVG) support. | `ImageNode` |
| `audio` | Independent audio timeline and MP3 playback. | `play!`, `AudioNode` |
| `export` | Headless frame rendering and video generation. | `project.export()` |
| `full` | Meta-feature that enables all of the above. | Everything |

### Key Capabilities
- **High-performance**: GPU-accelerated vector rendering via Vello.
- **Arc-length Sampling**: Accurate path animations and offsets.
- **Easing Library**: 30+ standardized easing functions.
- **FFmpeg Integration**: Direct streaming of animation frames or merging with audio.
- **Audio Support**: Synchronized MP3 playback and independent audio timelines.
- **Clean API**: Streamlined prelude for high-speed prototyping.
- **Node Primitives**: Built-in support for Circles, Rects, Polygons, Lines, and Groups.

## Supported Nodes

| Node | Description | Features |
|:---|:---|:---|
| `Circle` | Basic circle primitive. | `radius`, `color`, `transform` |
| `Rect` | Rectangle with optional corner radius. | `size`, `radius`, `color` |
| `Polygon` | Regular and custom polygon shapes. | `points`, `fill`, `stroke` |
| `Line` | Simple line between two points. | `start`, `end`, `width` |
| `PathNode` | Complex path sampling and animation. | `arc-length`, `sample` |
| `TextNode` | High-quality text rendering (skrifa). | `text`, `font_size`, `family` |
| `MathNode` | Typst-powered mathematical formulas. | `LaTeX`, `smooth-transitions` |
| `CodeNode` | Syntax-highlighted code with transitions. | `syntect`, `magic-move` |
| `ImageNode` | Bitmap and SVG image display. | `png`, `jpeg`, `svg` |
| `AudioNode` | Independent audio clip playback. | `mp3`, `volume`, `crop` |
| `GroupNode` | Hierarchical grouping of any nodes. | `children`, `inherited-opacity` |

## Project Structure

The engine is organized into a modular structure:

- `src/lib.rs`: Library entry point with clean module re-exports.
- `src/engine/nodes/`: Individual node implementations.
- `src/engine/animation/`: Core animation traits and flow controls.
- `src/engine/easings.rs`: Comprehensive easing function library.
- `examples/`: Ready-to-run demonstration scripts.

## Quick Start

```rust
use motion_canvas_rs::prelude::*;
use std::time::Duration;

fn main() {
    // Project::default() uses default values (800x600, 60fps)
    let mut project = Project::default()
        .with_title("Quick Start")
        .with_background(Color::rgb8(0x1a, 0x1a, 0x1a))
        .close_on_finish();

    // Nodes support a builder pattern and Default traits
    let circle = Circle::default()
        .with_position(Vec2::new(400.0, 300.0))
        .with_radius(100.0)
        .with_color(Color::RED);

    let text = TextNode::default()
        .with_position(Vec2::new(200.0, 300.0))
        .with_text("Hello Motion Canvas!")
        .with_font_size(48.0)
        .with_color(Color::WHITE);

    project.scene.add(Box::new(circle.clone()));
    project.scene.add(Box::new(text.clone()));

    project.scene.video_timeline.add(all![
        circle.radius.to(100.0, Duration::from_secs(1)),
        text.transform
            .to(Affine::translate((200.0, 400.0)), Duration::from_secs(1)),
    ]);

    project.show().expect("Failed to render");
}
```

## Running Examples

The project includes 14 examples that can be found in the [examples directory](./examples).

<details>
<summary><b>Advanced Flow</b> - Complex staggered and sequential animations.</summary>

```bash
cargo run --example advanced_flow --features=math,code,image,svg
```
https://github.com/user-attachments/assets/d283b03a-ae50-4011-9fab-77ced70a2632
</details>

<details>
<summary><b>Audio Demo</b> - Independent audio and video timelines with cropping.</summary>

```bash
cargo run --example audio_demo --features audio
```
https://github.com/user-attachments/assets/02670f39-8499-4202-8b22-c160d35f9031
</details>

<details>
<summary><b>Advanced Code</b> - Fine-grained selection and content manipulation.</summary>

```bash
cargo run --example code_advanced --features code
```
https://github.com/user-attachments/assets/23ad4662-e499-42f0-8468-3e1666e33d84
</details>

<details>
<summary><b>Code Animation</b> - "Magic Move" token-based code transitions.</summary>

```bash
cargo run --example code_animation --features code
```
https://github.com/user-attachments/assets/96135e70-b5d5-471f-9107-cc70f2b416fa
</details>

<details>
<summary><b>Color Interpolation</b> - Smooth transitions between color spaces.</summary>

```bash
cargo run --example color_interpolation
```
https://github.com/user-attachments/assets/cd002797-84ec-4bcb-af1f-0ab6e7c20433
</details>

<details>
<summary><b>Easing Scope</b> - 100% parity easing library visualizer.</summary>

```bash
cargo run --example easing_scope
```
https://github.com/user-attachments/assets/f875086e-d927-42a4-9f21-e57afbdaaaa4
</details>

<details>
<summary><b>Export</b> - Video export with color and font-size animations.</summary>

```bash
cargo run --example export --features export
```
https://github.com/user-attachments/assets/c01897a9-e744-43af-bfee-045f44549ba9
</details>

<details>
<summary><b>Getting Started</b> - Basic node creation and animation.</summary>

```bash
cargo run --example getting_started
```
https://github.com/user-attachments/assets/510d8aac-67ba-42d8-882a-b3c0ad969437
</details>

<details>
<summary><b>Group Animation</b> - Hierarchical transformations and inherited opacity.</summary>

```bash
cargo run --example group_animation
```
https://github.com/user-attachments/assets/75f078ba-51c2-4d26-8993-25e6b77372a9
</details>

<details>
<summary><b>Images</b> - Bitmap image support and transformations.</summary>

```bash
cargo run --example images --features image,svg
```
https://github.com/user-attachments/assets/25248e66-ccc7-4422-9f2f-7b9ef361d8d9
</details>

<details>
<summary><b>Math Animation</b> - Advanced mathematical transitions.</summary>

```bash
cargo run --example math_animation --features math
```
https://github.com/user-attachments/assets/f3d8e774-31f4-4e96-b7b7-9e6bda0ec16f
</details>

<details>
<summary><b>Math & Code</b> - Typst LaTeX and Syntax Highlighting.</summary>

```bash
cargo run --example math_code --features math,code
```
https://github.com/user-attachments/assets/967e0b47-a8de-4ab7-9b21-8758a2c7f508
</details>

<details>
<summary><b>Polygon</b> - Regular and custom polygon primitives.</summary>

```bash
cargo run --example polygon
```
https://github.com/user-attachments/assets/efc1e214-4297-47a2-b6e4-1eae0840b0c9
</details>

<details>
<summary><b>Shapes</b> - Circle, Rect, and Line primitives.</summary>

```bash
cargo run --example shapes
```
<img width="400" height="200" alt="shapes" src="https://github.com/user-attachments/assets/24d3c9a4-6330-4d03-a0b0-6d0fed318ab7" />
</details>

## Requirements

- Rust 1.75+
- FFmpeg (optional, for direct video streaming)
- System fonts (Inter, Fira Code, etc. for specific examples)

## Credits

This project is heavily inspired by the original [Motion Canvas](https://github.com/motion-canvas/motion-canvas) by [aarthificial](https://github.com/aarthificial).

Special thanks to:
- [easings.net](https://easings.net/) for the standardized easing function library.
- [shiki-magic-move](https://github.com/shikijs/shiki-magic-move) for the inspiration behind the token-based code transition logic.
