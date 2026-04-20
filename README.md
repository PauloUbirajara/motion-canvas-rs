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

| Node | Description | Transform Properties |
|:---|:---|:---|
| `AudioNode` | Independent audio clip playback. | `volume`, `crop` |
| `Circle` | Basic circle primitive. | `position`, `rotation`, `scale`, `radius` |
| `CodeNode` | Syntax-highlighted code with transitions. | `position`, `rotation`, `scale`, `code` |
| `GroupNode` | Hierarchical grouping of any nodes. | `position`, `rotation`, `scale`, `children` |
| `ImageNode` | Bitmap and SVG image display. | `position`, `rotation`, `scale`, `size` |
| `Line` | Simple line between two points. | `position`, `rotation`, `scale`, `start`, `end` |
| `MathNode` | Typst-powered mathematical formulas. | `position`, `rotation`, `scale`, `equation` |
| `PathNode` | Complex path sampling and animation. | `position`, `rotation`, `scale`, `arc-length` |
| `Polygon` | Regular and custom polygon shapes. | `position`, `rotation`, `scale`, `points` |
| `Rect` | Rectangle with optional corner radius. | `position`, `rotation`, `scale`, `size`, `radius` |
| `TextNode` | High-quality text rendering (skrifa). | `position`, `rotation`, `scale`, `text` |

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
        text.position.to(Vec2::new(200.0, 400.0), Duration::from_secs(1)),
    ]);

    project.show().expect("Failed to render");
}
```

## Running Examples

The project includes 18 examples that can be found in the [examples directory](./examples).

<details>
<summary><b>Advanced Flow</b> - Complex staggered and sequential animations.</summary>

```sh
cargo run --example advanced_flow --features=full
```

https://github.com/user-attachments/assets/0e89eafa-7075-4381-b676-7eb25f45d127

<img width="800" height="600" alt="Advanced Flow" src="https://github.com/user-attachments/assets/251f877e-7993-457e-a2d0-6b9a56397a3e" />

</details>

<details>
<summary><b>Anchors</b> - Reactive transformation origins for precise positioning.</summary>

```sh
cargo run --example anchors
```

</details>

<details>
<summary><b>Audio Demo</b> - Independent audio and video timelines with cropping.</summary>

```sh
cargo run --example audio_demo --features audio
```

https://github.com/user-attachments/assets/022ef076-452a-47b9-9d5e-2d2edf555397

<img width="800" height="450" alt="Audio Demo" src="https://github.com/user-attachments/assets/f547ba7c-ec5b-4b26-8e4e-227cf11d0328" />

</details>

<details>
<summary><b>Advanced Code</b> - Fine-grained selection and content manipulation.</summary>

```sh
cargo run --example code_advanced --features code
```

https://github.com/user-attachments/assets/960fbfd0-ddc9-49d9-bcdd-7b29ca4ffe93

<img width="800" height="600" alt="Code Advanced" src="https://github.com/user-attachments/assets/ed2bd8db-7112-4c46-b49c-ecf2971e4237" />

</details>

<details>
<summary><b>Code Animation</b> - "Magic Move" token-based code transitions.</summary>

```sh
cargo run --example code_animation --features code
```

https://github.com/user-attachments/assets/0579e41a-9cf0-42ce-9dba-d9c087be53d9

<img width="800" height="800" alt="Code Animation" src="https://github.com/user-attachments/assets/3002c235-c4a3-486c-b47a-640f29d5dba6" />

</details>

<details>
<summary><b>Color Interpolation</b> - Smooth transitions between color spaces.</summary>

```sh
cargo run --example color_interpolation
```

https://github.com/user-attachments/assets/366308f8-2903-48dc-8609-79a661bba712

<img width="400" height="400" alt="Color Interpolation" src="https://github.com/user-attachments/assets/7e8d1cd4-c0f4-44ca-849e-85c53d79be35" />

</details>

<details>
<summary><b>Easing Scope</b> - 100% parity easing library visualizer.</summary>

```sh
cargo run --example easing_scope
```

https://github.com/user-attachments/assets/9b25225e-72d8-4c0e-9c62-6acd58d1e99d

<img width="800" height="800" alt="Easing Scope" src="https://github.com/user-attachments/assets/817fb722-2262-4894-8b20-86315a5ff3b1" />

</details>

<details>
<summary><b>Explainer</b> - Showcasing the library and some of its features.</summary>

```sh
cargo run --example explainer --release --features full
```

https://www.youtube.com/watch?v=v4W1Y_TrWew

<img width="1200" height="700" alt="motion_canvas_rs_deep_dive_0404" src="https://github.com/user-attachments/assets/d5e10b0b-cc80-4a99-8347-c5d4c7354727" />

</details>

<details>
<summary><b>Export</b> - Video export with color and font-size animations.</summary>

```sh
cargo run --example export --features export
```

https://github.com/user-attachments/assets/002e57f7-8d62-4dec-aea6-9107030352be

<img width="800" height="600" alt="Export" src="https://github.com/user-attachments/assets/2ea09740-e70c-416d-8dc6-9e5c6d1c226a" />

</details>

<details>
<summary><b>Getting Started</b> - Basic node creation and animation.</summary>

```sh
cargo run --example getting_started
```

https://github.com/user-attachments/assets/e36ae34c-a45c-4e99-8aec-f37a2c289639

<img width="800" height="600" alt="Getting Started" src="https://github.com/user-attachments/assets/0d4e5a59-0b55-4a02-b73f-eea714f7e7fc" />

</details>

<details>
<summary><b>Group Animation</b> - Hierarchical transformations and inherited opacity.</summary>

```sh
cargo run --example group_animation
```

https://github.com/user-attachments/assets/2041e546-5af2-4ffd-9f73-2878ba87bd24

<img width="800" height="600" alt="Group Animation" src="https://github.com/user-attachments/assets/c0866038-5232-4b56-a2bd-5c8e04e6f8f5" />

</details>

<details>
<summary><b>Images</b> - Bitmap image support and transformations.</summary>

```sh
cargo run --example images --features image,svg
```

https://github.com/user-attachments/assets/467d48cf-f358-48cb-b07f-bfaaf8cd5f36

<img width="600" height="600" alt="Images" src="https://github.com/user-attachments/assets/4b5cf1d7-e106-4b93-8936-b113713c182e" />

</details>

<details>
<summary><b>Math Animation</b> - Advanced mathematical transitions.</summary>

```sh
cargo run --example math_animation --features math
```

https://github.com/user-attachments/assets/ef4e3573-5518-42bf-8359-3aed2cbcca59

<img width="800" height="600" alt="Math Animation" src="https://github.com/user-attachments/assets/343c88dc-9889-468a-9a75-6373bb9d5615" />

</details>

<details>
<summary><b>Math & Code</b> - Typst LaTeX and Syntax Highlighting.</summary>

```sh
cargo run --example math_code --features math,code
```

https://github.com/user-attachments/assets/f51268ee-87cf-4f6d-b261-a4aa1f2d4e07

<img width="800" height="600" alt="Math & Code" src="https://github.com/user-attachments/assets/c0ce9ede-273d-498e-94ce-54f2d94ff25a" />

</details>

<details>
<summary><b>News Feed</b> - A simple architectural visualization of a news feed system.</summary>

```sh
cargo run --example news_feed
```

</details>

<details>
<summary><b>Polygon</b> - Regular and custom polygon primitives.</summary>

```sh
cargo run --example polygon
```

https://github.com/user-attachments/assets/0dead3d2-60d4-41f9-86d1-b5eafcc70c4b

<img width="800" height="600" alt="Polygon" src="https://github.com/user-attachments/assets/86254bf1-25a8-47b0-8b4c-9ef68fb87c1c" />

</details>

<details>
<summary><b>Shapes</b> - Circle, Rect, and Line primitives.</summary>

```sh
cargo run --example shapes
```

https://github.com/user-attachments/assets/132f310e-cbbb-4a11-8525-0c87a7017fbe

<img width="648" height="193" alt="Shapes" src="https://github.com/user-attachments/assets/0d828a93-b14d-4c00-ad94-28f015178cc7" />

</details>

<details>
<summary><b>Signals</b> - Reactive signal linking and independent property animation.</summary>

```sh
cargo run --example signals
```

https://github.com/user-attachments/assets/ec9568e3-5b66-42a2-85b2-70c59176b17d

<img width="800" height="600" alt="Signals" src="https://github.com/user-attachments/assets/bd788c63-50e1-4ef4-8ad1-1f8a91c707e2" />

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
