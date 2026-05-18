# Motion Canvas in Rust

![Motion Canvas Banner](https://media.githubusercontent.com/media/PauloUbirajara/motion-canvas-rs/master/assets/cover.webp)

A high-performance vector animation engine inspired by Motion Canvas, built on Vello and Typst.

> [!IMPORTANT]
> **Prototype Status**: This project is a functional prototype and proof-of-concept. It is **not** a 1:1 implementation of the original Motion Canvas API or features.

## Links

- [https://docs.rs/motion-canvas-rs](https://docs.rs/motion-canvas-rs)
- [https://crates.io/crates/motion-canvas-rs](https://crates.io/crates/motion-canvas-rs)

## Installation

Add the library to your `Cargo.toml`. To enable all features (math, code blocks, images, export), use the `full` flag:

```bash
# Enable everything
cargo add motion-canvas-rs --features full

# Or pick only what you need (e.g., just math, SVGs, and audio)
cargo add motion-canvas-rs --features math,svg,audio
```

## Features

| Feature | Description | Enables |
|:---|:---|:---|
| `audio` | Independent audio timeline and MP3 playback. | `play!`, `AudioNode` |
| `code` | Syntax-highlighted code blocks via Syntect. | `CodeNode` |
| `export` | Headless frame rendering and video generation. | `project.export()` |
| `image` | Bitmap image support (WebP, JPEG, PNG). | `ImageNode` |
| `math` | Typst-powered LaTeX math rendering. | `MathNode` |
| `physics` | Simplified 2D physics engine integration via Rapier. | `PhysicsNode`, `RigidBodyNode`, `StaticBodyNode` |
| `svg` | Vector graphics support via Vello SVG. | `SvgNode` |
| `full` | Meta-feature that enables all of the above. | Everything |

### Key Capabilities
- **Arc-length Sampling**: Accurate path animations and offsets.
- **Audio Support**: Synchronized MP3 playback and independent audio timelines.
- **Clean API**: Streamlined prelude for high-speed prototyping.
- **Easing Library**: 30+ standardized easing functions.
- **FFmpeg Integration**: Direct streaming of animation frames or merging with audio.
- **High-performance**: GPU-accelerated vector rendering via Vello.
- **Node Primitives**: Built-in support for Circles, Rects, Polygons, Lines, and Groups.

## Supported Nodes

| Node | Description | Transform Properties |
|:---|:---|:---|
| `AudioNode` | Independent audio clip playback. | `volume`, `crop` |
| `CameraNode` | Viewport transformation (pan, zoom, rotate). | `position`, `rotation`, `zoom`, `centered` |
| `Circle` | Basic circle primitive. | `position`, `rotation`, `scale`, `radius`, `anchor` |
| `CodeNode` | Syntax-highlighted code with transitions. | `position`, `rotation`, `scale`, `code`, `anchor` |
| `GroupNode` | Hierarchical grouping of any nodes. | `position`, `rotation`, `scale`, `children`, `anchor` |
| `ImageNode` | Bitmap image display (WebP, PNG, JPG). | `position`, `rotation`, `scale`, `size`, `anchor` |
| `Line` | Simple line between two points. | `position`, `rotation`, `scale`, `start`, `end`, `anchor` |
| `MathNode` | Typst-powered mathematical formulas. | `position`, `rotation`, `scale`, `equation`, `anchor` |
| `PathNode` | Complex path sampling and animation. | `position`, `rotation`, `scale`, `arc-length`, `anchor` |
| `PhysicsNode` | Container orchestrating a 2D physics simulation world. | `opacity`, `gravity`, `is_moving_container` |
| `Polygon` | Regular and custom polygon shapes. | `position`, `rotation`, `scale`, `points`, `anchor` |
| `Rect` | Rectangle with optional corner radius. | `position`, `rotation`, `scale`, `size`, `radius`, `anchor` |
| `RigidBodyNode` | Dynamic physical body reacting to gravity, collisions, and forces. | `position`, `rotation`, `shape`, `bounciness`, `friction`, `initial_velocity`, `initial_angular_velocity` |
| `StaticBodyNode` | Immovable physical body acting as boundaries or obstacles. | `position`, `rotation`, `shape`, `bounciness`, `friction` |
| `SvgNode` | Vector SVG image display and animation. | `position`, `rotation`, `scale`, `size`, `anchor` |
| `TextNode` | High-quality text rendering (skrifa). | `position`, `rotation`, `scale`, `text`, `anchor` |

## Project Structure

The engine is organized into the following structure:

- `examples/`: Comprehensive demonstration scripts.
- `src/assets/`: Resource management (fonts, images, audio, palettes).
- `src/core/`: Pure logic layer (animations, easings, timeline, scene).
- `src/elements/`: High-level node hierarchy (shapes, media, containers).
- `src/lib.rs`: Library entry point and unified prelude.
- `src/project.rs`: Core `Project` configuration struct.
- `src/runtime/`: Side-effect-heavy runners (windowing, export, renderer).

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

    project.scene.add(circle.clone());
    project.scene.add(text.clone());

    project.scene.video_timeline.add(all![
        circle.radius.to(100.0, Duration::from_secs(1)),
        text.position.to(Vec2::new(400.0, 400.0), Duration::from_secs(1)),
    ]);

    project.show().expect("Failed to render");
}
```

## Running Examples

The project includes 24 examples that can be found in the [examples directory](./examples).

<details>
    <summary> [ <a href="./examples/advanced_flow.rs">code</a> ] <b>Advanced Flow</b> - Complex staggered and sequential animations.</summary>

```sh
cargo run --example advanced_flow --features=full
```

| Preview |
| - |
| ![Advanced Flow](https://media.githubusercontent.com/media/PauloUbirajara/motion-canvas-rs/master/assets/examples/advanced_flow.webp) |
| [Advanced Flow Video](https://media.githubusercontent.com/media/PauloUbirajara/motion-canvas-rs/master/assets/examples/advanced_flow.mp4) |

</details>

<details>
<summary> [ <a href="./examples/anchors.rs">code</a> ] <b>Anchors</b> - Reactive transformation origins for precise positioning.</summary>

```sh
cargo run --example anchors
```

| Preview |
| - |
| ![Anchors](https://media.githubusercontent.com/media/PauloUbirajara/motion-canvas-rs/master/assets/examples/anchors.webp) |
| [Anchors Video](https://media.githubusercontent.com/media/PauloUbirajara/motion-canvas-rs/master/assets/examples/anchors.mp4) |

</details>

<details>
<summary> [ <a href="./examples/audio_demo.rs">code</a> ] <b>Audio Demo</b> - Independent audio and video timelines with cropping.</summary>

```sh
cargo run --example audio_demo --features audio
```

| Preview |
| - |
| ![Audio Demo](https://media.githubusercontent.com/media/PauloUbirajara/motion-canvas-rs/master/assets/examples/audio_demo.webp) |
| [Audio Demo Video](https://media.githubusercontent.com/media/PauloUbirajara/motion-canvas-rs/master/assets/examples/audio_demo.mp4) |

</details>

<details>
<summary> [ <a href="./examples/camera_demo.rs">code</a> ] <b>Camera Control</b> - Viewport-level panning, zooming, and rotation.</summary>

```sh
cargo run --example camera_demo
```

| Preview |
| - |
| ![Camera Demo](https://media.githubusercontent.com/media/PauloUbirajara/motion-canvas-rs/master/assets/examples/camera_demo.webp) |
| [Camera Demo Video](https://media.githubusercontent.com/media/PauloUbirajara/motion-canvas-rs/master/assets/examples/camera_demo.mp4) |

</details>

<details>
<summary> [ <a href="./examples/code_advanced.rs">code</a> ] <b>Code Advanced</b> - Fine-grained selection and content manipulation.</summary>

```sh
cargo run --example code_advanced --features code
```

| Preview |
| - |
| ![Code Advanced](https://media.githubusercontent.com/media/PauloUbirajara/motion-canvas-rs/master/assets/examples/code_advanced.webp) |
| [Code Advanced Video](https://media.githubusercontent.com/media/PauloUbirajara/motion-canvas-rs/master/assets/examples/code_advanced.mp4) |

</details>

<details>
<summary> [ <a href="./examples/code_animation.rs">code</a> ] <b>Code Animation</b> - "Magic Move" token-based code transitions.</summary>

```sh
cargo run --example code_animation --features code
```

| Preview |
| - |
| ![Code Animation](https://media.githubusercontent.com/media/PauloUbirajara/motion-canvas-rs/master/assets/examples/code_animation.webp) |
| [Code Animation Video](https://media.githubusercontent.com/media/PauloUbirajara/motion-canvas-rs/master/assets/examples/code_animation.mp4) |

</details>

<details>
<summary> [ <a href="./examples/color_interpolation.rs">code</a> ] <b>Color Interpolation</b> - Smooth transitions between color spaces.</summary>

```sh
cargo run --example color_interpolation
```

| Preview |
| - |
| ![Color Interpolation](https://media.githubusercontent.com/media/PauloUbirajara/motion-canvas-rs/master/assets/examples/color_interpolation.webp) |
| [Color Interpolation Video](https://media.githubusercontent.com/media/PauloUbirajara/motion-canvas-rs/master/assets/examples/color_interpolation.mp4) |

</details>

<details>
<summary> [ <a href="./examples/easing_scope.rs">code</a> ] <b>Easing Scope</b> - 100% parity easing library visualizer.</summary>

```sh
cargo run --example easing_scope
```

| Preview |
| - |
| ![Easing Scope](https://media.githubusercontent.com/media/PauloUbirajara/motion-canvas-rs/master/assets/examples/easing_scope.webp) |
| [Easing Scope Video](https://media.githubusercontent.com/media/PauloUbirajara/motion-canvas-rs/master/assets/examples/easing_scope.mp4) |

</details>

<details>
<summary> [ <a href="./examples/explainer.rs">code</a> ] <b>Explainer</b> - Showcasing the library and some of its features.</summary>

```sh
cargo run --example explainer --release --features full
```

| Preview |
| - |
| ![Explainer](https://media.githubusercontent.com/media/PauloUbirajara/motion-canvas-rs/master/assets/examples/explainer.webp) |
| [Explainer Video](https://www.youtube.com/watch?v=v4W1Y_TrWew) |

</details>

<details>
<summary> [ <a href="./examples/export.rs">code</a> ] <b>Export</b> - Video export with color and font-size animations.</summary>

```sh
cargo run --example export --features export
```

| Preview |
| - |
| ![Export](https://media.githubusercontent.com/media/PauloUbirajara/motion-canvas-rs/master/assets/examples/export.webp) |
| [Export Video](https://media.githubusercontent.com/media/PauloUbirajara/motion-canvas-rs/master/assets/examples/export.mp4) |

</details>

<details>
<summary> [ <a href="./examples/getting_started.rs">code</a> ] <b>Getting Started</b> - Basic node creation and animation.</summary>

```sh
cargo run --example getting_started
```

| Preview |
| - |
| ![Getting Started](https://media.githubusercontent.com/media/PauloUbirajara/motion-canvas-rs/master/assets/examples/getting_started.webp) |
| [Getting Started Video](https://media.githubusercontent.com/media/PauloUbirajara/motion-canvas-rs/master/assets/examples/getting_started.mp4) |

</details>

<details>
<summary> [ <a href="./examples/gradient_demo.rs">code</a> ] <b>Gradient Demo</b> - Shapes, texts, fill and stroke gradients, and morph animations between solid and gradients using Vello.</summary>

```sh
cargo run --example gradient_demo --features math
```

| Preview |
| - |
| ![Gradient Demo](https://media.githubusercontent.com/media/PauloUbirajara/motion-canvas-rs/master/assets/examples/gradient_demo.webp) |
| [Gradient Demo Video](https://media.githubusercontent.com/media/PauloUbirajara/motion-canvas-rs/master/assets/examples/gradient_demo.mp4) |

</details>

<details>
<summary> [ <a href="./examples/grid.rs">code</a> ] <b>Grid</b> - Procedural grid generation.</summary>

```sh
cargo run --example grid
```

| Preview |
| - |
| ![Grid](https://media.githubusercontent.com/media/PauloUbirajara/motion-canvas-rs/master/assets/examples/grid.webp) |
| [Grid Video](https://media.githubusercontent.com/media/PauloUbirajara/motion-canvas-rs/master/assets/examples/grid.mp4) |

</details>

<details>
<summary> [ <a href="./examples/group_animation.rs">code</a> ] <b>Group Animation</b> - Hierarchical transformations and inherited opacity.</summary>

```sh
cargo run --example group_animation
```

| Preview |
| - |
| ![Group Animation](https://media.githubusercontent.com/media/PauloUbirajara/motion-canvas-rs/master/assets/examples/group_animation.webp) |
| [Group Animation Video](https://media.githubusercontent.com/media/PauloUbirajara/motion-canvas-rs/master/assets/examples/group_animation.mp4) |

</details>

<details>
<summary> [ <a href="./examples/images.rs">code</a> ] <b>Images</b> - Bitmap image support and transformations.</summary>

```sh
cargo run --example images --features image,svg
```

| Preview |
| - |
| ![Images](https://media.githubusercontent.com/media/PauloUbirajara/motion-canvas-rs/master/assets/examples/images.webp) |
| [Images Video](https://media.githubusercontent.com/media/PauloUbirajara/motion-canvas-rs/master/assets/examples/images.mp4) |

</details>

<details>
<summary> [ <a href="./examples/math_animation.rs">code</a> ] <b>Math Animation</b> - Advanced mathematical transitions.</summary>

```sh
cargo run --example math_animation --features math
```

| Preview |
| - |
| ![Math Animation](https://media.githubusercontent.com/media/PauloUbirajara/motion-canvas-rs/master/assets/examples/math_animation.webp) |
| [Math Animation Video](https://media.githubusercontent.com/media/PauloUbirajara/motion-canvas-rs/master/assets/examples/math_animation.mp4) |

</details>

<details>
<summary> [ <a href="./examples/math_code.rs">code</a> ] <b>Math & Code</b> - Typst LaTeX and Syntax Highlighting.</summary>

```sh
cargo run --example math_code --features math,code
```

| Preview |
| - |
| ![Math Code](https://media.githubusercontent.com/media/PauloUbirajara/motion-canvas-rs/master/assets/examples/math_code.webp) |
| [Math Code Video](https://media.githubusercontent.com/media/PauloUbirajara/motion-canvas-rs/master/assets/examples/math_code.mp4) |

</details>

<details>
<summary> [ <a href="./examples/nested_cameras.rs">code</a> ] <b>Nested Cameras</b> - Hierarchical viewport control and coordinate shifting.</summary>

```sh
cargo run --example nested_cameras
```

| Preview |
| - |
| ![Nested Cameras](https://media.githubusercontent.com/media/PauloUbirajara/motion-canvas-rs/master/assets/examples/nested_cameras.webp) |
| [Nested Cameras Video](https://media.githubusercontent.com/media/PauloUbirajara/motion-canvas-rs/master/assets/examples/nested_cameras.mp4) |

</details>

<details>
<summary> [ <a href="./examples/news_feed.rs">code</a> ] <b>News Feed</b> - A simple architectural visualization of a news feed system.</summary>

> Based on the "News Feed System" architecture from **"System Design Interview: An Insider's Guide" (Second Edition)** by **Alex Xu**.

```sh
cargo run --example news_feed
```

| Preview |
| - |
| ![News Feed](https://media.githubusercontent.com/media/PauloUbirajara/motion-canvas-rs/master/assets/examples/news_feed.webp) |
| [News Feed Video](https://media.githubusercontent.com/media/PauloUbirajara/motion-canvas-rs/master/assets/examples/news_feed.mp4) |

</details>

<details>
<summary> [ <a href="./examples/physics_demo.rs">code</a> ] <b>Physics Simulation</b> - 2D physics simulation using Rapier.</summary>

```sh
cargo run --example physics_demo --features physics
```

| Preview |
| - |
| ![Physics Simulation](https://media.githubusercontent.com/media/PauloUbirajara/motion-canvas-rs/master/assets/examples/physics_demo.webp) |
| [Physics Simulation Video](https://media.githubusercontent.com/media/PauloUbirajara/motion-canvas-rs/master/assets/examples/physics_demo.mp4) |

</details>

<details>
<summary> [ <a href="./examples/polygon.rs">code</a> ] <b>Polygon</b> - Regular and custom polygon primitives.</summary>

```sh
cargo run --example polygon
```

| Preview |
| - |
| ![Polygon](https://media.githubusercontent.com/media/PauloUbirajara/motion-canvas-rs/master/assets/examples/polygon.webp) |
| [Polygon Video](https://media.githubusercontent.com/media/PauloUbirajara/motion-canvas-rs/master/assets/examples/polygon.mp4) |

</details>

<details>
<summary> [ <a href="./examples/shapes.rs">code</a> ] <b>Shapes</b> - Circle, Rect, and Line primitives.</summary>

```sh
cargo run --example shapes
```

| Preview |
| - |
| ![Shapes](https://media.githubusercontent.com/media/PauloUbirajara/motion-canvas-rs/master/assets/examples/shapes.webp) |

</details>

<details>
<summary> [ <a href="./examples/signals.rs">code</a> ] <b>Signals</b> - Reactive signal linking and independent property animation.</summary>

```sh
cargo run --example signals
```

| Preview |
| - |
| ![Signals](https://media.githubusercontent.com/media/PauloUbirajara/motion-canvas-rs/master/assets/examples/signals.webp) |
| [Signals Video](https://media.githubusercontent.com/media/PauloUbirajara/motion-canvas-rs/master/assets/examples/signals.mp4) |

</details>

<details>
<summary> [ <a href="./examples/world_map.rs">code</a> ] <b>World Map</b> - Animated world tour with camera panning, cloud effects, and plane flight paths.</summary>

> Map SVG sourced from [vemaps.com](https://vemaps.com/world/wrld-21).

```sh
cargo run --example world_map --features image,svg
```

| Preview |
| - |
| ![World Map](https://media.githubusercontent.com/media/PauloUbirajara/motion-canvas-rs/master/assets/examples/world_map.webp) |
| [World Map Video](https://media.githubusercontent.com/media/PauloUbirajara/motion-canvas-rs/master/assets/examples/world_map.mp4) |

</details>

## Requirements

- Rust 1.75+
- FFmpeg (optional, for direct video streaming)
- System fonts (Inter, Fira Code, etc. for specific examples)

## Credits

This project is heavily inspired by the original [Motion Canvas](https://github.com/motion-canvas/motion-canvas) by [aarthificial](https://github.com/aarthificial).

Special thanks to:
- [easings.net](https://easings.net/) for the standardized easing function library.
- [rapier.rs](https://rapier.rs/) for the incredible 2D physics engine powering our physics simulation in the `physics_demo` example (`rapier2d`).
- [shiki-magic-move](https://github.com/shikijs/shiki-magic-move) for the inspiration behind the token-based code transition logic.
- [vemaps.com](https://vemaps.com/world/wrld-21) for the world map SVG used in the `world_map` example.
- **Alex Xu** for the excellent system design diagrams in *"System Design Interview: An Insider's Guide"*, represented in the `news_feed` example.