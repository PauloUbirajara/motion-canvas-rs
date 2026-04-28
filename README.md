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
| `CameraNode` | Viewport transformation (pan, zoom, rotate). | `position`, `rotation`, `zoom`, `centered` |
| `Circle` | Basic circle primitive. | `position`, `rotation`, `scale`, `radius`, `anchor` |
| `CodeNode` | Syntax-highlighted code with transitions. | `position`, `rotation`, `scale`, `code`, `anchor` |
| `GroupNode` | Hierarchical grouping of any nodes. | `position`, `rotation`, `scale`, `children`, `anchor` |
| `ImageNode` | Bitmap and SVG image display. | `position`, `rotation`, `scale`, `size`, `anchor` |
| `Line` | Simple line between two points. | `position`, `rotation`, `scale`, `start`, `end`, `anchor` |
| `MathNode` | Typst-powered mathematical formulas. | `position`, `rotation`, `scale`, `equation`, `anchor` |
| `PathNode` | Complex path sampling and animation. | `position`, `rotation`, `scale`, `arc-length`, `anchor` |
| `Polygon` | Regular and custom polygon shapes. | `position`, `rotation`, `scale`, `points`, `anchor` |
| `Rect` | Rectangle with optional corner radius. | `position`, `rotation`, `scale`, `size`, `radius`, `anchor` |
| `TextNode` | High-quality text rendering (skrifa). | `position`, `rotation`, `scale`, `text`, `anchor` |

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
        .with_fill(Color::RED);

    let text = TextNode::default()
        .with_position(Vec2::new(400.0, 150.0))
        .with_text("Hello Motion Canvas!")
        .with_font_size(48.0)
        .with_fill(Color::WHITE);

    project.scene.add(Box::new(circle.clone()));
    project.scene.add(Box::new(text.clone()));

    project.scene.video_timeline.add(all![
        circle.radius.to(100.0, Duration::from_secs(1)),
        text.position.to(Vec2::new(400.0, 400.0), Duration::from_secs(1)),
    ]);

    project.show().expect("Failed to render");
}
```

## Running Examples

The project includes 20 examples that can be found in the [examples directory](./examples).

<details>
<summary><b>Advanced Flow</b> - Complex staggered and sequential animations.</summary>

```sh
cargo run --example advanced_flow --features=full
```

https://github.com/user-attachments/assets/66b01caf-103c-4ef6-bc5f-893a654ebed9

<img width="800" height="600" alt="Advanced Flow" src="https://github.com/user-attachments/assets/ee55d3a0-229f-42ec-b56b-d254194044bc" />

</details>

<details>
<summary><b>Anchors</b> - Reactive transformation origins for precise positioning.</summary>

```sh
cargo run --example anchors
```

https://github.com/user-attachments/assets/1ef59b41-e1d9-4c18-96ec-eb5c3bc61cfb

<img width="1200" height="800" alt="Anchors" src="https://github.com/user-attachments/assets/09a1a37f-78f9-4d2c-839c-26668edf5756" />

</details>

<details>
<summary><b>Audio Demo</b> - Independent audio and video timelines with cropping.</summary>

```sh
cargo run --example audio_demo --features audio
```

https://github.com/user-attachments/assets/f280e6e7-b718-44bc-bf90-f0d7acec507e

<img width="800" height="450" alt="Audio Demo" src="https://github.com/user-attachments/assets/5a91416d-753a-4ee5-9e0f-7438e9ced426" />

</details>

<details>
<summary><b>Advanced Code</b> - Fine-grained selection and content manipulation.</summary>

```sh
cargo run --example code_advanced --features code
```

https://github.com/user-attachments/assets/331ce753-ced8-4a9b-aec2-c01b52f8194c

<img width="800" height="600" alt="Code Advanced" src="https://github.com/user-attachments/assets/43349bbb-5a24-43da-b249-cf5046eb95c5" />

</details>

<details>
<summary><b>Camera Control</b> - Viewport-level panning, zooming, and rotation.</summary>

```sh
cargo run --example camera_demo
```

https://github.com/user-attachments/assets/62b1a691-c49e-44e6-9bb0-a8f1b46bf1b5

<img width="800" height="600" alt="Camera Control" src="https://github.com/user-attachments/assets/3952bd99-204f-44af-b539-74297fe9fc4e" />

</details>

<details>
<summary><b>Code Animation</b> - "Magic Move" token-based code transitions.</summary>

```sh
cargo run --example code_animation --features code
```

https://github.com/user-attachments/assets/15698738-738a-4aad-bee7-73dfcb64a88c

<img width="800" height="800" alt="Code Animation" src="https://github.com/user-attachments/assets/54f22334-c59b-4477-a12e-9b026b946fc9" />

</details>

<details>
<summary><b>Color Interpolation</b> - Smooth transitions between color spaces.</summary>

```sh
cargo run --example color_interpolation
```

https://github.com/user-attachments/assets/a73a9995-ad63-4aca-b797-7f13f160f923

<img width="300" height="300" alt="Color Interpolation" src="https://github.com/user-attachments/assets/2e4576fb-c612-45b9-8608-559282e080d7" />

</details>

<details>
<summary><b>Easing Scope</b> - 100% parity easing library visualizer.</summary>

```sh
cargo run --example easing_scope
```

https://github.com/user-attachments/assets/0efc6e66-94b9-4392-958d-b2e077fe9ce5

<img width="800" height="800" alt="Easing Scope" src="https://github.com/user-attachments/assets/537e87f9-bdc6-42e5-a198-ae89bf08490c" />

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

https://github.com/user-attachments/assets/9cf41b9e-f17b-407e-9817-8423e787e844

<img width="800" height="600" alt="Export" src="https://github.com/user-attachments/assets/005076d6-f616-4420-8187-2916b9634467" />

</details>

<details>
<summary><b>Getting Started</b> - Basic node creation and animation.</summary>

```sh
cargo run --example getting_started
```

https://github.com/user-attachments/assets/6bfaa089-7cbe-4b5b-936b-4d8e6d537e25

<img width="800" height="600" alt="Getting Started" src="https://github.com/user-attachments/assets/7d4b1f72-28ed-4f6b-9d55-ca3d721c6f55" />

</details>

<details>
<summary><b>Group Animation</b> - Hierarchical transformations and inherited opacity.</summary>

```sh
cargo run --example group_animation
```

https://github.com/user-attachments/assets/bd3e429a-9f8b-4126-adba-f65435d3944e

<img width="800" height="600" alt="Group Animation" src="https://github.com/user-attachments/assets/900f761d-b78f-443f-81fb-c3a7fb9c65cf" />

</details>

<details>
<summary><b>Images</b> - Bitmap image support and transformations.</summary>

```sh
cargo run --example images --features image,svg
```

https://github.com/user-attachments/assets/a46363a9-f681-4cc3-89c1-10506e59518d

<img width="600" height="600" alt="Images" src="https://github.com/user-attachments/assets/0c899a89-0935-4721-8b41-41c7acadc3e3" />

</details>

<details>
<summary><b>Math Animation</b> - Advanced mathematical transitions.</summary>

```sh
cargo run --example math_animation --features math
```

https://github.com/user-attachments/assets/b64f00db-0950-45cf-834d-1f0e74845cab

<img width="500" height="500" alt="Math Animation" src="https://github.com/user-attachments/assets/5c10b547-8487-465a-abe6-8fda5dfe64e4" />

</details>

<details>
<summary><b>Math & Code</b> - Typst LaTeX and Syntax Highlighting.</summary>

```sh
cargo run --example math_code --features math,code
```

https://github.com/user-attachments/assets/0eec990c-66a8-40f6-b816-15e2636c6fd0

<img width="800" height="600" alt="Math Code" src="https://github.com/user-attachments/assets/77c1b82f-b36b-4cc7-88f9-56519f996c86" />

</details>

<details>
<summary><b>Nested Cameras</b> - Hierarchical viewport control and coordinate shifting.</summary>

```sh
cargo run --example nested_cameras
```

https://github.com/user-attachments/assets/c362012f-4e56-450d-aa21-08015a955d29

<img width="800" height="600" alt="Nested Cameras" src="https://github.com/user-attachments/assets/8bab09a3-c113-46bc-a058-18d2451a1b6b" />

</details>

<details>
<summary><b>News Feed</b> - A simple architectural visualization of a news feed system.</summary>

> Based on the "News Feed System" architecture from **"System Design Interview: An Insider's Guide" (Second Edition)** by **Alex Xu**.

```sh
cargo run --example news_feed
```

https://github.com/user-attachments/assets/933f4cca-f3f2-4a0e-bc2d-f39fa5b1fc46

<img width="1920" height="1080" alt="News Feed" src="https://github.com/user-attachments/assets/48a6f524-31a0-45f1-a7a5-21c56faaa531" />

</details>

<details>
<summary><b>Polygon</b> - Regular and custom polygon primitives.</summary>

```sh
cargo run --example polygon
```

https://github.com/user-attachments/assets/262fbc60-ab83-4b21-85d5-ebbfb77fe22c

<img width="800" height="600" alt="Polygon" src="https://github.com/user-attachments/assets/c5d7f1d7-86e8-40df-a51e-e75f9633dc5f" />

</details>

<details>
<summary><b>Shapes</b> - Circle, Rect, and Line primitives.</summary>

```sh
cargo run --example shapes
```

<img width="800" height="300" alt="Shapes" src="https://github.com/user-attachments/assets/ee548b7b-39e3-4e23-9149-e91dd670cea1" />

</details>

<details>
<summary><b>Signals</b> - Reactive signal linking and independent property animation.</summary>

```sh
cargo run --example signals
```

https://github.com/user-attachments/assets/5a6fdd0f-4da2-418c-8615-b30e50e86b14

<img width="800" height="600" alt="Signals" src="https://github.com/user-attachments/assets/718cc644-b24a-43b3-b7a3-de04eeb292fc" />

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
- **Alex Xu** for the excellent system design diagrams in *"System Design Interview: An Insider's Guide"*, represented in the `news_feed` example.
