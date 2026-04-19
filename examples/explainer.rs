use motion_canvas_rs::prelude::*;
use std::time::Duration;

// ── Design Tokens ──────────────────────────────────────────────────────────
const FONT: &str = "JetBrains Mono";
const BG: Color = Color::rgb8(0x0e, 0x0e, 0x12);
const WHITE: Color = Color::rgb8(0xf0, 0xf0, 0xf0);
const DIM: Color = Color::rgb8(0x55, 0x55, 0x66);
const ACCENT: Color = Color::rgb8(0x68, 0xab, 0xdf);
const RED: Color = Color::rgb8(0xe1, 0x32, 0x38);
const YELLOW: Color = Color::rgb8(0xe6, 0xa7, 0x00);
const GREEN: Color = Color::rgb8(0x25, 0xc2, 0x81);
const TEAL: Color = Color::rgb8(0x20, 0xb2, 0xaa);

const CANVAS_W: u32 = 1200;
const CANVAS_H: u32 = 700;
const LEFT: f32 = 40.0;

fn ms(n: u64) -> Duration {
    Duration::from_millis(n)
}
fn secs(n: u64) -> Duration {
    Duration::from_secs(n)
}

// ── Text Helpers ───────────────────────────────────────────────────────────
fn title(text: &str, y: f32) -> TextNode {
    TextNode::default()
        .with_position(Vec2::new(LEFT, y))
        .with_text(text)
        .with_font_size(36.0)
        .with_fill(ACCENT)
        .with_font(FONT)
        .with_opacity(0.0)
}
fn h2(text: &str, y: f32) -> TextNode {
    TextNode::default()
        .with_position(Vec2::new(LEFT, y))
        .with_text(text)
        .with_font_size(22.0)
        .with_fill(WHITE)
        .with_font(FONT)
        .with_opacity(0.0)
}
fn body(text: &str, y: f32) -> TextNode {
    TextNode::default()
        .with_position(Vec2::new(LEFT, y))
        .with_text(text)
        .with_font_size(17.0)
        .with_fill(Color::rgb8(0xcc, 0xcc, 0xdd))
        .with_font(FONT)
        .with_opacity(0.0)
}
fn dim(text: &str, x: f32, y: f32) -> TextNode {
    TextNode::default()
        .with_position(Vec2::new(x, y))
        .with_text(text)
        .with_font_size(13.0)
        .with_fill(DIM)
        .with_font(FONT)
        .with_opacity(0.0)
}
fn note(text: &str, y: f32) -> TextNode {
    TextNode::default()
        .with_position(Vec2::new(LEFT + 20.0, y))
        .with_text(text)
        .with_font_size(14.0)
        .with_fill(YELLOW)
        .with_font(FONT)
        .with_opacity(0.0)
}
fn code_block(code: &str, y: f32) -> CodeNode {
    CodeNode::default()
        .with_position(Vec2::new(LEFT + 20.0, y))
        .with_code(code)
        .with_language("rust")
        .with_font(FONT)
        .with_font_size(14.0)
        .with_opacity(0.0)
}
fn hline(y: f32) -> Line {
    Line::default()
        .with_start(Vec2::new(LEFT, y))
        .with_end(Vec2::new(LEFT, y))
        .with_stroke(Color::rgba8(255, 255, 255, 25), 1.0)
}
// Shorthand for show/hide
fn show(n: &impl HasOpacity, d: Duration) -> Box<dyn Animation> {
    n.opacity_signal().to(1.0, d).ease(easings::cubic_out).into()
}
fn hide(n: &impl HasOpacity, d: Duration) -> Box<dyn Animation> {
    n.opacity_signal().to(0.0, d).ease(easings::cubic_in).into()
}

// Trait to unify opacity access across different node types
trait HasOpacity {
    fn opacity_signal(&self) -> Signal<f32>;
}
impl HasOpacity for TextNode {
    fn opacity_signal(&self) -> Signal<f32> {
        self.opacity.clone()
    }
}
impl HasOpacity for Circle {
    fn opacity_signal(&self) -> Signal<f32> {
        self.opacity.clone()
    }
}
impl HasOpacity for Rect {
    fn opacity_signal(&self) -> Signal<f32> {
        self.opacity.clone()
    }
}
impl HasOpacity for Line {
    fn opacity_signal(&self) -> Signal<f32> {
        self.opacity.clone()
    }
}
impl HasOpacity for Polygon {
    fn opacity_signal(&self) -> Signal<f32> {
        self.opacity.clone()
    }
}
impl HasOpacity for CodeNode {
    fn opacity_signal(&self) -> Signal<f32> {
        self.opacity.clone()
    }
}
impl HasOpacity for GroupNode {
    fn opacity_signal(&self) -> Signal<f32> {
        self.opacity.clone()
    }
}
impl HasOpacity for ImageNode {
    fn opacity_signal(&self) -> Signal<f32> {
        self.opacity.clone()
    }
}

fn main() {
    let mut project = Project::default()
        .with_dimensions(CANVAS_W, CANVAS_H)
        .with_fps(60)
        .with_title("Explainer")
        .with_background(BG)
        .close_on_finish();

    // =====================================================================
    //  S1: TITLE CARD
    // =====================================================================
    let s1_line = hline(100.0);
    let s1_title = TextNode::default()
        .with_position(Vec2::new(LEFT, 120.0))
        .with_text("motion-canvas-rs")
        .with_font_size(52.0)
        .with_fill(ACCENT)
        .with_font(FONT)
        .with_opacity(0.0);
    let s1_sub = h2("A GPU-Accelerated Vector Animation Engine", 185.0);
    let s1_built = body(
        "Built on Vello + Typst  —  Inspired by Motion Canvas",
        220.0,
    );
    let s1_desc = body(
        "This animation will teach you how the library works,",
        280.0,
    );
    let s1_desc2 = body(
        "from struct definitions to the GPU rendering pipeline.",
        305.0,
    );

    let s1_logo = ImageNode::default()
        .with_position(Vec2::new(950.0, 420.0))
        .with_path("examples/images/motion-canvas-rs.svg")
        .with_scale(0.3)
        .with_opacity(0.0);

    for n in [&s1_title, &s1_sub, &s1_built, &s1_desc, &s1_desc2] {
        project.scene.add(Box::new(n.clone()));
    }
    project.scene.add(Box::new(s1_line.clone()));
    project.scene.add(Box::new(s1_logo.clone()));

    // =====================================================================
    //  S2: THE 5 STEPS
    // =====================================================================
    let s2_h = title("Every program follows 5 steps", 50.0);
    let steps = [
        "1. Create a Project      — your canvas settings",
        "2. Create Nodes           — shapes, text, images",
        "3. Add Nodes to Scene     — what gets drawn",
        "4. Animate the Timeline   — how things move",
        "5. Show or Export         — live window or video",
    ];
    let s2_texts: Vec<TextNode> = steps
        .iter()
        .enumerate()
        .map(|(i, s)| body(s, 120.0 + i as f32 * 35.0))
        .collect();

    project.scene.add(Box::new(s2_h.clone()));
    for t in &s2_texts {
        project.scene.add(Box::new(t.clone()));
    }

    // =====================================================================
    //  S3: WHAT IS A STRUCT?  +  The Project struct
    // =====================================================================
    let s3_h = title("What is a 'struct'?", 50.0);
    let s3_explain = h2(
        "A struct is a container that groups related data together.",
        95.0,
    );
    let s3_analogy = body(
        "Think of it like a class in Python/JS, but it only holds data.",
        130.0,
    );

    let s3_code = code_block(
        "pub struct Project {
    pub width: u32,          // Canvas width in pixels
    pub height: u32,         // Canvas height in pixels
    pub fps: u32,            // Frames per second
    pub title: String,       // Window title
    pub scene: BaseScene,    // Holds nodes + timelines
    pub background_color: Color,
    pub close_on_finish: bool,
}",
        175.0,
    );

    let s3_note = note(
        "^ This is the actual Project struct from the library.",
        420.0,
    );
    let s3_note2 = body(
        "'pub' means public — anyone can read/write these fields.",
        455.0,
    );
    let s3_note3 = body(
        "u32 = unsigned 32-bit integer,  String = text,  bool = true/false",
        485.0,
    );

    for n in [
        &s3_h,
        &s3_explain,
        &s3_analogy,
        &s3_note,
        &s3_note2,
        &s3_note3,
    ] {
        project.scene.add(Box::new(n.clone()));
    }
    project.scene.add(Box::new(s3_code.clone()));

    // =====================================================================
    //  S4: WHAT IS impl?  +  Builder pattern
    // =====================================================================
    let s4_h = title("What is 'impl'?", 50.0);
    let s4_explain = h2("impl adds methods (functions) to a struct.", 95.0);
    let s4_analogy = body(
        "Like adding methods to a class. Separated from the data.",
        130.0,
    );

    let s4_code = code_block(
        "impl Project {
    pub fn with_fps(mut self, fps: u32) -> Self {
        self.fps = fps;   // set the value
        self             // return yourself (builder pattern)
    }
    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }
}",
        175.0,
    );

    let s4_usage = body("Usage — chain calls to configure:", 430.0);
    let s4_usage_code = code_block(
        "let project = Project::default()
    .with_fps(60)
    .with_title(\"My Animation\")
    .with_dimensions(800, 600)
    .close_on_finish();",
        460.0,
    );

    let s4_note = note(
        "Each .with_*() returns 'self', so you can chain them.",
        590.0,
    );

    for n in [&s4_h, &s4_explain, &s4_analogy, &s4_usage, &s4_note] {
        project.scene.add(Box::new(n.clone()));
    }
    project.scene.add(Box::new(s4_code.clone()));
    project.scene.add(Box::new(s4_usage_code.clone()));

    // =====================================================================
    //  S5: WHAT IS A TRAIT?  +  The Node trait
    // =====================================================================
    let s5_h = title("What is a 'trait'?", 50.0);
    let s5_explain = h2(
        "A trait is a contract — like an interface in Java/TypeScript.",
        95.0,
    );
    let s5_analogy = body(
        "Any type that implements a trait promises to provide those methods.",
        130.0,
    );

    let s5_code = code_block(
        "pub trait Node: Send + Sync + 'static {
    fn render(&self, scene: &mut Scene,
              parent_transform: Affine,
              parent_opacity: f32);
    fn update(&mut self, dt: Duration);
    fn state_hash(&self) -> u64;
    fn clone_node(&self) -> Box<dyn Node>;
}",
        175.0,
    );

    let s5_r = note(
        "render()     — draw yourself using current signal values",
        410.0,
    );
    let s5_u = note(
        "update(dt)   — called every frame (for per-frame logic)",
        435.0,
    );
    let s5_s = note(
        "state_hash() — returns a number that changes when you change",
        460.0,
    );
    let s5_c = note("clone_node() — make a deep copy of yourself", 485.0);
    let s5_every = body(
        "Circle, Rect, Line, TextNode, Polygon all implement Node.",
        530.0,
    );

    for n in [
        &s5_h,
        &s5_explain,
        &s5_analogy,
        &s5_r,
        &s5_u,
        &s5_s,
        &s5_c,
        &s5_every,
    ] {
        project.scene.add(Box::new(n.clone()));
    }
    project.scene.add(Box::new(s5_code.clone()));

    // =====================================================================
    //  S6: NODE GALLERY  (visual demo)
    // =====================================================================
    let s6_h = title("The Built-in Nodes", 50.0);
    let s6_sub = body(
        "Each node uses the builder pattern and stores properties as Signals.",
        90.0,
    );

    let demo_c = Circle::default()
        .with_position(Vec2::new(120.0, 230.0))
        .with_radius(40.0)
        .with_fill(RED)
        .with_opacity(0.0);
    let demo_r = Rect::default()
        .with_position(Vec2::new(260.0, 190.0))
        .with_size(Vec2::new(80.0, 80.0))
        .with_fill(ACCENT)
        .with_radius(8.0)
        .with_opacity(0.0);
    let demo_l = Line::default()
        .with_start(Vec2::new(420.0, 200.0))
        .with_end(Vec2::new(510.0, 270.0))
        .with_stroke(WHITE, 3.0)
        .with_opacity(0.0);
    let demo_p = Polygon::regular(5, 40.0)
        .with_position(Vec2::new(610.0, 230.0))
        .with_fill(YELLOW)
        .with_opacity(0.0);
    let demo_t = TextNode::default()
        .with_position(Vec2::new(740.0, 215.0))
        .with_text("Abc")
        .with_font_size(36.0)
        .with_fill(GREEN)
        .with_font(FONT)
        .with_opacity(0.0);

    let lc = dim("Circle", 95.0, 285.0);
    let lr = dim("Rect", 280.0, 285.0);
    let ll = dim("Line", 445.0, 285.0);
    let lp = dim("Polygon", 580.0, 285.0);
    let lt = dim("TextNode", 735.0, 285.0);

    let s6_box_h = h2("Why Box<dyn Node>?", 340.0);
    let s6_box1 = body("The scene stores different node types in one list:", 375.0);
    let s6_box_code = code_block(
        "pub struct BaseScene {
    pub nodes: Vec<Box<dyn Node>>,  // a list of \"any Node\"
}
// 'Box' = heap-allocated,  'dyn Node' = any type implementing Node
// Like List<INode> in Java or Array<Node> in TypeScript
project.scene.add(Box::new(circle));  // wrap + add",
        405.0,
    );

    for n in [&s6_h, &s6_sub, &lc, &lr, &ll, &lp, &lt, &s6_box_h, &s6_box1] {
        project.scene.add(Box::new(n.clone()));
    }
    project.scene.add(Box::new(demo_c.clone()));
    project.scene.add(Box::new(demo_r.clone()));
    project.scene.add(Box::new(demo_l.clone()));
    project.scene.add(Box::new(demo_p.clone()));
    project.scene.add(Box::new(demo_t.clone()));
    project.scene.add(Box::new(s6_box_code.clone()));

    // =====================================================================
    //  S7: SIGNALS — The Reactive Core
    // =====================================================================
    let s7_h = title("Signals — The Reactive Core", 50.0);
    let s7_sub = h2("Every animatable property is a Signal<T>.", 95.0);

    let s7_code = code_block(
        "pub struct Signal<T> {
    pub data: Arc<Mutex<SignalData<T>>>,
}
pub struct SignalData<T> {
    pub value: T,  // the actual value (f32, Vec2, Color...)
}",
        140.0,
    );

    let s7_arc = note("Arc = shared pointer. Multiple owners, same data.", 310.0);
    let s7_mutex = note(
        "Mutex = lock. Only one thread reads/writes at a time.",
        335.0,
    );
    let s7_why = body(
        "Why? A node and its animation both need the same property:",
        380.0,
    );

    let s7_diagram_code = code_block(
        "let circle = Circle::default().with_radius(50.0);
// circle.radius is a Signal<f32>

circle.radius.to(100.0, Duration::from_secs(1));
// Creates a SignalTween with a CLONE of circle.radius
// Both point to the SAME underlying value (via Arc)

// The animation WRITES new values each frame
// The node READS them when rendering",
        420.0,
    );

    // Live demo circle
    let sig_demo = Circle::default()
        .with_position(Vec2::new(1000.0, 400.0))
        .with_radius(50.0)
        .with_fill(RED)
        .with_stroke(Color::rgba8(255, 255, 255, 50), 2.0)
        .with_opacity(0.0);
    let sig_lbl = dim("Live Signal demo", 900.0, 150.0);

    for n in [&s7_h, &s7_sub, &s7_arc, &s7_mutex, &s7_why, &sig_lbl] {
        project.scene.add(Box::new(n.clone()));
    }
    project.scene.add(Box::new(s7_code.clone()));
    project.scene.add(Box::new(s7_diagram_code.clone()));
    project.scene.add(Box::new(sig_demo.clone()));

    // =====================================================================
    //  S8: SIGNAL TWEEN — How animations work per-frame
    // =====================================================================
    let s8_h = title("SignalTween — The Animation Engine", 50.0);
    let s8_sub = body(
        ".to() creates a SignalTween that interpolates over time:",
        90.0,
    );

    let s8_code = code_block(
        "pub struct SignalTween<T> {
    data: Arc<Mutex<SignalData<T>>>,  // shared ref to signal
    start_value: Option<T>,  // captured on FIRST update (lazy!)
    target_value: Option<T>, // where we're going
    duration: Duration,      // how long
    elapsed: Duration,       // how much time passed
    easing: fn(f32) -> f32,  // curve function
}",
        125.0,
    );

    let s8_how = h2("Each frame update:", 340.0);
    let s8_steps = [
        "1. elapsed += dt",
        "2. t_linear = elapsed / duration        (0.0 to 1.0)",
        "3. t_eased  = easing(t_linear)          (curved)",
        "4. value    = lerp(start, target, t)     (interpolate)",
        "5. Write value into Signal               (node sees it)",
        "6. If elapsed >= duration: finished!      (return leftover dt)",
    ];
    let s8_step_texts: Vec<TextNode> = s8_steps
        .iter()
        .enumerate()
        .map(|(i, s)| body(s, 370.0 + i as f32 * 28.0))
        .collect();

    let s8_lazy = note(
        "start_value is captured lazily — so chained tweens read the",
        570.0,
    );
    let s8_lazy2 = note(
        "correct value at their actual start time, not creation time.",
        590.0,
    );

    // Progress bar
    let prog_bg = Rect::default()
        .with_position(Vec2::new(700.0, 150.0))
        .with_size(Vec2::new(400.0, 16.0))
        .with_fill(Color::rgba8(255, 255, 255, 15))
        .with_radius(8.0)
        .with_opacity(0.0);
    let prog_fill = Rect::default()
        .with_position(Vec2::new(700.0, 150.0))
        .with_size(Vec2::new(0.0, 16.0))
        .with_fill(ACCENT)
        .with_radius(8.0)
        .with_opacity(0.0);
    let plbl0 = dim("t=0", 700.0, 172.0);
    let plbl1 = dim("t=1", 1070.0, 172.0);
    let tween_ball = Circle::default()
        .with_position(Vec2::new(900.0, 430.0))
        .with_radius(30.0)
        .with_fill(RED)
        .with_opacity(0.0);
    let tween_lbl = dim("radius animating: 30 -> 80", 760.0, 220.0);

    for n in [
        &s8_h, &s8_sub, &s8_how, &s8_lazy, &s8_lazy2, &plbl0, &plbl1, &tween_lbl,
    ] {
        project.scene.add(Box::new(n.clone()));
    }
    project.scene.add(Box::new(s8_code.clone()));
    for t in &s8_step_texts {
        project.scene.add(Box::new(t.clone()));
    }
    project.scene.add(Box::new(prog_bg.clone()));
    project.scene.add(Box::new(prog_fill.clone()));
    project.scene.add(Box::new(tween_ball.clone()));

    // =====================================================================
    //  S9: TWEENABLE + EASINGS
    // =====================================================================
    let s9_h = title("Tweenable — What Can Be Animated", 50.0);
    let s9_code = code_block(
        "pub trait Tweenable: Clone + Send + Sync {
    fn interpolate(a: &Self, b: &Self, t: f32) -> Self;
    fn state_hash(&self) -> u64;
}
// Implemented for: f32, Vec2, Color, String, Affine, Vec<Vec2>
//   f32:    lerp(a, b, t) = a + (b-a)*t
//   Vec2:   lerp x and y independently
//   Color:  lerp R,G,B,A channels independently
//   String: snap — returns 'a' until t>=1, then 'b'",
        95.0,
    );

    let s9_easing_h = h2("Easing functions curve the linear t:", 310.0);
    let s9_easing_desc = body("Same distance, same duration — different feel.", 340.0);

    let enames = [
        "linear",
        "cubic_in_out",
        "elastic_out",
        "bounce_out",
        "back_out",
    ];
    let ecolors = [WHITE, ACCENT, RED, YELLOW, GREEN];
    let mut eballs: Vec<Circle> = Vec::new();
    let mut elabels: Vec<TextNode> = Vec::new();
    for (i, name) in enames.iter().enumerate() {
        let y = 390.0 + i as f32 * 55.0;
        let b = Circle::default()
            .with_position(Vec2::new(250.0, y))
            .with_radius(12.0)
            .with_fill(ecolors[i])
            .with_opacity(0.0);
        let l = dim(name, LEFT, y - 5.0);
        project.scene.add(Box::new(b.clone()));
        project.scene.add(Box::new(l.clone()));
        eballs.push(b);
        elabels.push(l);
    }
    project.scene.add(Box::new(s9_h.clone()));
    project.scene.add(Box::new(s9_code.clone()));
    project.scene.add(Box::new(s9_easing_h.clone()));
    project.scene.add(Box::new(s9_easing_desc.clone()));

    // =====================================================================
    //  S10: FLOW CONTROLS
    // =====================================================================
    let s10_h = title("Flow Controls — Composing Animations", 50.0);
    let s10_sub = body(
        "Individual tweens are simple. Power comes from composing them.",
        90.0,
    );

    // chain
    let s10_chain_h = h2("chain![ ] — one after another", 140.0);
    let chain_d: Vec<Circle> = (0..3)
        .map(|i| {
            Circle::default()
                .with_position(Vec2::new(LEFT + 30.0 + i as f32 * 60.0, 200.0))
                .with_radius(18.0)
                .with_fill([RED, ACCENT, YELLOW][i])
                .with_opacity(0.0)
        })
        .collect();

    // all
    let s10_all_h = h2("all![ ] — all at the same time", 270.0);
    let all_d: Vec<Circle> = (0..3)
        .map(|i| {
            Circle::default()
                .with_position(Vec2::new(LEFT + 30.0 + i as f32 * 60.0, 330.0))
                .with_radius(18.0)
                .with_fill([RED, ACCENT, YELLOW][i])
                .with_opacity(0.0)
        })
        .collect();

    // sequence
    let s10_seq_h = h2("sequence![ ] — staggered starts", 400.0);
    let seq_d: Vec<Circle> = (0..3)
        .map(|i| {
            Circle::default()
                .with_position(Vec2::new(LEFT + 30.0 + i as f32 * 60.0, 460.0))
                .with_radius(18.0)
                .with_fill([RED, ACCENT, YELLOW][i])
                .with_opacity(0.0)
        })
        .collect();

    let s10_code = code_block(
        "chain![ a, b, c ]     // a then b then c
all![ a, b, c ]       // a + b + c together
sequence![ 200ms, a, b, c ] // staggered
delay![ 500ms, a ]    // wait then play
wait(1s)              // pause
any![ a, b ]          // race: first wins
loop_anim![ a, 3 ]   // repeat 3 times",
        510.0,
    );

    project.scene.add(Box::new(s10_h.clone()));
    project.scene.add(Box::new(s10_sub.clone()));
    project.scene.add(Box::new(s10_chain_h.clone()));
    project.scene.add(Box::new(s10_all_h.clone()));
    project.scene.add(Box::new(s10_seq_h.clone()));
    project.scene.add(Box::new(s10_code.clone()));
    for d in &chain_d {
        project.scene.add(Box::new(d.clone()));
    }
    for d in &all_d {
        project.scene.add(Box::new(d.clone()));
    }
    for d in &seq_d {
        project.scene.add(Box::new(d.clone()));
    }

    // =====================================================================
    //  S11: TIMELINE + RENDERING
    // =====================================================================
    let s11_h = title("The Timeline — Animation Queue", 50.0);
    let s11_code = code_block(
        "pub struct Timeline {
    pub animations: Vec<Box<dyn Animation>>,
}
impl Timeline {
    fn update(&mut self, mut dt: Duration) {
        while !self.animations.is_empty() {
            let (finished, leftover) = self.animations[0].update(dt);
            if finished {
                self.animations.remove(0); // pop front
                dt = leftover;  // pass leftover to next!
            } else { break; }
        }
    }
}",
        95.0,
    );

    let s11_leftover = note(
        "leftover propagation: if A finishes mid-frame, the remaining",
        390.0,
    );
    let s11_leftover2 = note(
        "dt is immediately given to B. No 'lost frames' at transitions.",
        415.0,
    );

    let s11_render_h = h2("Rendering Pipeline (per frame):", 470.0);
    let s11_steps = [
        "1. Timeline.update(dt)  =>  SignalTween writes to Signals",
        "2. Node.render()        =>  reads signals, draws shapes",
        "3. Vello GPU            =>  compiles scene => wgpu => pixels",
        "4. state_hash()         =>  XOR of all values, skip if unchanged",
    ];
    let s11_render_texts: Vec<TextNode> = s11_steps
        .iter()
        .enumerate()
        .map(|(i, s)| body(s, 505.0 + i as f32 * 28.0))
        .collect();

    project.scene.add(Box::new(s11_h.clone()));
    project.scene.add(Box::new(s11_code.clone()));
    for n in [&s11_leftover, &s11_leftover2, &s11_render_h] {
        project.scene.add(Box::new(n.clone()));
    }
    for t in &s11_render_texts {
        project.scene.add(Box::new(t.clone()));
    }

    // =====================================================================
    //  S12: EVENT LOOP — Why an infinite loop?
    // =====================================================================
    let s12_h = title("Why an Infinite Loop? — The Event Loop", 50.0);
    let s12_sub = body(
        "GPU rendering requires a persistent event loop (winit + wgpu).",
        90.0,
    );
    let s12_code = code_block(
        "event_loop.run(|event, elwt| {
    match event {
        Resumed => {           // GPU surface ready
            renderer.resume(&window);
        }
        AboutToWait => {       // run every frame
            scene.update(dt);  // advance animations
            let hash = scene.state_hash();
            if hash != last_hash {    // dirty?
                window.request_redraw();
            }
        }
        RedrawRequested => {   // GPU draw call
            renderer.render(&scene, w, h);
        }
    }
});",
        130.0,
    );
    let s12_why = note(
        "The window stays open because the GPU surface is tied to",
        490.0,
    );
    let s12_why2 = note(
        "the OS event loop. Without it, the surface is immediately dropped.",
        510.0,
    );
    let s12_hash = body(
        "state_hash() skips re-rendering unchanged frames (dirty-checking).",
        550.0,
    );

    for n in [&s12_h, &s12_sub, &s12_why, &s12_why2, &s12_hash] {
        project.scene.add(Box::new(n.clone()));
    }
    project.scene.add(Box::new(s12_code.clone()));

    // =====================================================================
    //  S13: HEADLESS EXPORT — GPU without a window
    // =====================================================================
    let s13_h = title("Headless Export: GPU -> PNG -> FFmpeg", 50.0);
    let s13_sub = body(
        "Same GPU rendering, but without a window — output to files.",
        90.0,
    );
    let s13_code = code_block(
        "pub struct Exporter {
    texture: wgpu::Texture,       // GPU-side image
    output_buffer: wgpu::Buffer,  // CPU-readable copy
    renderer: Renderer,           // Vello
}
fn export_frame(&mut self, scene) -> Vec<u8> {
    scene.render(&mut self.scene);          // 1. build shapes
    renderer.render_to_texture(..);          // 2. GPU draws
    encoder.copy_texture_to_buffer(..);      // 3. GPU -> CPU
    output_buffer.map_async(Read, ..);       // 4. read pixels
    return pixels;                           // 5. raw RGBA
}",
        130.0,
    );
    let s13_cache = note(
        "Cache: state_hash per frame. If unchanged, skip GPU entirely.",
        420.0,
    );
    let s13_ffmpeg = note(
        "FFmpeg: raw pixels piped to stdin -> libx264 -> .mkv video.",
        445.0,
    );
    let s13_parallel = body(
        "PNG saving runs on a background thread. Export is pipelined.",
        485.0,
    );

    for n in [&s13_h, &s13_sub, &s13_cache, &s13_ffmpeg, &s13_parallel] {
        project.scene.add(Box::new(n.clone()));
    }
    project.scene.add(Box::new(s13_code.clone()));

    // =====================================================================
    //  S14: ENGINE UTILITIES
    // =====================================================================
    let s14_h = title("Under the Hood: Utility Modules", 50.0);
    let s14_sub = body(
        "Helper systems that power the engine behind the scenes.",
        90.0,
    );
    let s14_code = code_block(
        "// src/engine/util/
font_manager.rs    // Lazy-loads system fonts via Typst
                   // Global HashMap cache with lazy_static

image_manager.rs   // Loads PNG + SVG (via resvg)
                   // Caches decoded images as Arc<Image>

code_tokenizer.rs  // Syntax highlighting via Syntect
                   // Parses code -> colored spans for CodeNode

export.rs          // FFmpeg pipe: rawvideo -> libx264
                   // Audio merging with filter_complex
                   // Title sanitization for filenames",
        130.0,
    );
    let s14_lazy = note(
        "lazy_static + Mutex = global singleton, created once, cached forever.",
        460.0,
    );
    let s14_arc = body(
        "Arc<Image> lets multiple nodes share one decoded image without copies.",
        495.0,
    );

    for n in [&s14_h, &s14_sub, &s14_lazy, &s14_arc] {
        project.scene.add(Box::new(n.clone()));
    }
    project.scene.add(Box::new(s14_code.clone()));

    // =====================================================================
    //  S15: FINALE
    // =====================================================================
    let fin = TextNode::default()
        .with_position(Vec2::new(LEFT, 200.0))
        .with_text("That's how it works!")
        .with_font_size(48.0)
        .with_fill(ACCENT)
        .with_font(FONT)
        .with_opacity(0.0);
    let fin_steps = [
        "1.  struct         — data container",
        "2.  impl           — methods / builder pattern",
        "3.  trait Node      — interface contract",
        "4.  Box<dyn Node>   — type-erased heap allocation",
        "5.  Signal<T>       — Arc<Mutex> shared reactive state",
        "6.  SignalTween     — per-frame lerp interpolation",
        "7.  Timeline        — sequential queue + leftover dt",
        "8.  Event Loop      — winit + wgpu infinite loop",
        "9.  Exporter        — headless GPU -> PNG/FFmpeg",
        "10. Utilities       — font/image cache, syntax highlight",
    ];
    let fin_texts: Vec<TextNode> = fin_steps
        .iter()
        .enumerate()
        .map(|(i, s)| body(s, 270.0 + i as f32 * 28.0))
        .collect();
    let fin_hint = dim("cargo run --example getting_started", LEFT, 570.0);

    project.scene.add(Box::new(fin.clone()));
    for t in &fin_texts {
        project.scene.add(Box::new(t.clone()));
    }
    project.scene.add(Box::new(fin_hint.clone()));

    // =====================================================================
    //  ANIMATION TIMELINE
    // =====================================================================
    // Helper: hide_all takes a vec of opacity signals and fades them out
    let hide_dur = ms(200);

    project.scene.video_timeline.add(chain![
        // ── S1: TITLE ──
        s1_line
            .end
            .to(Vec2::new(500.0, 100.0), ms(500))
            .ease(easings::cubic_out),
        sequence![
            ms(120),
            show(&s1_title, ms(500)),
            show(&s1_sub, ms(500)),
            show(&s1_built, ms(500)),
            show(&s1_logo, ms(600)),
            show(&s1_desc, ms(500)),
            show(&s1_desc2, ms(500)),
        ],
        wait(secs(5)),
        all![
            hide(&s1_title, hide_dur),
            hide(&s1_sub, hide_dur),
            hide(&s1_built, hide_dur),
            hide(&s1_desc, hide_dur),
            hide(&s1_desc2, hide_dur),
            hide(&s1_logo, hide_dur),
            s1_line.end.to(Vec2::new(LEFT, 100.0), hide_dur)
        ],
        wait(ms(150)),
        // ── S2: FIVE STEPS ──
        show(&s2_h, ms(500)),
        wait(ms(400)),
        sequence![
            ms(250),
            show(&s2_texts[0], ms(400)),
            show(&s2_texts[1], ms(400)),
            show(&s2_texts[2], ms(400)),
            show(&s2_texts[3], ms(400)),
            show(&s2_texts[4], ms(400)),
        ],
        wait(secs(8)),
        all![
            hide(&s2_h, hide_dur),
            hide(&s2_texts[0], hide_dur),
            hide(&s2_texts[1], hide_dur),
            hide(&s2_texts[2], hide_dur),
            hide(&s2_texts[3], hide_dur),
            hide(&s2_texts[4], hide_dur)
        ],
        wait(ms(150)),
        // ── S3: STRUCT ──
        sequence![
            ms(120),
            show(&s3_h, ms(500)),
            show(&s3_explain, ms(400)),
            show(&s3_analogy, ms(400))
        ],
        wait(ms(500)),
        show(&s3_code, ms(500)),
        wait(secs(6)),
        sequence![
            ms(300),
            show(&s3_note, ms(400)),
            show(&s3_note2, ms(400)),
            show(&s3_note3, ms(400))
        ],
        wait(secs(6)),
        all![
            hide(&s3_h, hide_dur),
            hide(&s3_explain, hide_dur),
            hide(&s3_analogy, hide_dur),
            hide(&s3_code, hide_dur),
            hide(&s3_note, hide_dur),
            hide(&s3_note2, hide_dur),
            hide(&s3_note3, hide_dur)
        ],
        wait(ms(150)),
        // ── S4: IMPL / BUILDER ──
        sequence![
            ms(120),
            show(&s4_h, ms(500)),
            show(&s4_explain, ms(400)),
            show(&s4_analogy, ms(400))
        ],
        wait(ms(500)),
        show(&s4_code, ms(500)),
        wait(secs(7)),
        show(&s4_usage, ms(400)),
        show(&s4_usage_code, ms(500)),
        wait(secs(2)),
        show(&s4_note, ms(400)),
        wait(secs(5)),
        all![
            hide(&s4_h, hide_dur),
            hide(&s4_explain, hide_dur),
            hide(&s4_analogy, hide_dur),
            hide(&s4_code, hide_dur),
            hide(&s4_usage, hide_dur),
            hide(&s4_usage_code, hide_dur),
            hide(&s4_note, hide_dur)
        ],
        wait(ms(150)),
        // ── S5: TRAIT / NODE ──
        sequence![
            ms(120),
            show(&s5_h, ms(500)),
            show(&s5_explain, ms(400)),
            show(&s5_analogy, ms(400))
        ],
        wait(ms(500)),
        show(&s5_code, ms(500)),
        wait(secs(6)),
        sequence![
            ms(300),
            show(&s5_r, ms(400)),
            show(&s5_u, ms(400)),
            show(&s5_s, ms(400)),
            show(&s5_c, ms(400))
        ],
        wait(secs(2)),
        show(&s5_every, ms(400)),
        wait(secs(4)),
        all![
            hide(&s5_h, hide_dur),
            hide(&s5_explain, hide_dur),
            hide(&s5_analogy, hide_dur),
            hide(&s5_code, hide_dur),
            hide(&s5_r, hide_dur),
            hide(&s5_u, hide_dur),
            hide(&s5_s, hide_dur),
            hide(&s5_c, hide_dur),
            hide(&s5_every, hide_dur)
        ],
        wait(ms(150)),
        // ── S6: NODE GALLERY ──
        sequence![ms(120), show(&s6_h, ms(500)), show(&s6_sub, ms(400))],
        wait(ms(400)),
        sequence![
            ms(200),
            all![show(&demo_c, ms(400)), show(&lc, ms(400))],
            all![show(&demo_r, ms(400)), show(&lr, ms(400))],
            all![show(&demo_l, ms(400)), show(&ll, ms(400))],
            all![show(&demo_p, ms(400)), show(&lp, ms(400))],
            all![show(&demo_t, ms(400)), show(&lt, ms(400))],
        ],
        wait(secs(2)),
        sequence![
            ms(200),
            show(&s6_box_h, ms(400)),
            show(&s6_box1, ms(400)),
            show(&s6_box_code, ms(500))
        ],
        wait(secs(7)),
        all![
            hide(&s6_h, hide_dur),
            hide(&s6_sub, hide_dur),
            hide(&demo_c, hide_dur),
            hide(&demo_r, hide_dur),
            hide(&demo_l, hide_dur),
            hide(&demo_p, hide_dur),
            hide(&demo_t, hide_dur),
            hide(&lc, hide_dur),
            hide(&lr, hide_dur),
            hide(&ll, hide_dur),
            hide(&lp, hide_dur),
            hide(&lt, hide_dur),
            hide(&s6_box_h, hide_dur),
            hide(&s6_box1, hide_dur),
            hide(&s6_box_code, hide_dur)
        ],
        wait(ms(150)),
        // ── S7: SIGNALS ──
        sequence![ms(120), show(&s7_h, ms(500)), show(&s7_sub, ms(400))],
        wait(ms(400)),
        show(&s7_code, ms(500)),
        wait(secs(5)),
        sequence![ms(300), show(&s7_arc, ms(400)), show(&s7_mutex, ms(400))],
        wait(secs(4)),
        show(&s7_why, ms(400)),
        show(&s7_diagram_code, ms(500)),
        wait(secs(6)),
        // Live demo
        all![show(&sig_demo, ms(300)), show(&sig_lbl, ms(300))],
        chain![
            sig_demo.radius.to(80.0, ms(700)).ease(easings::elastic_out),
            sig_demo.fill_color.to(TEAL, ms(500)),
            sig_demo
                .position
                .to(Vec2::new(950.0, 350.0), ms(500))
                .ease(easings::cubic_out),
            wait(ms(300)),
            all![
                sig_demo.radius.to(50.0, ms(400)),
                sig_demo.fill_color.to(RED, ms(400)),
                sig_demo.position.to(Vec2::new(900.0, 300.0), ms(400))
            ],
        ],
        wait(secs(3)),
        all![
            hide(&s7_h, hide_dur),
            hide(&s7_sub, hide_dur),
            hide(&s7_code, hide_dur),
            hide(&s7_arc, hide_dur),
            hide(&s7_mutex, hide_dur),
            hide(&s7_why, hide_dur),
            hide(&s7_diagram_code, hide_dur),
            hide(&sig_demo, hide_dur),
            hide(&sig_lbl, hide_dur)
        ],
        wait(ms(150)),
        // ── S8: SIGNAL TWEEN ──
        sequence![ms(120), show(&s8_h, ms(500)), show(&s8_sub, ms(400))],
        wait(ms(400)),
        show(&s8_code, ms(500)),
        wait(secs(6)),
        show(&s8_how, ms(300)),
        sequence![
            ms(100),
            show(&s8_step_texts[0], ms(250)),
            show(&s8_step_texts[1], ms(250)),
            show(&s8_step_texts[2], ms(250)),
            show(&s8_step_texts[3], ms(250)),
            show(&s8_step_texts[4], ms(250)),
            show(&s8_step_texts[5], ms(250)),
        ],
        wait(ms(500)),
        sequence![ms(100), show(&s8_lazy, ms(300)), show(&s8_lazy2, ms(300))],
        wait(ms(500)),
        // Progress bar demo
        all![
            show(&prog_bg, ms(200)),
            show(&prog_fill, ms(200)),
            show(&plbl0, ms(200)),
            show(&plbl1, ms(200)),
            show(&tween_ball, ms(200)),
            show(&tween_lbl, ms(200))
        ],
        all![
            prog_fill
                .size
                .to(Vec2::new(400.0, 16.0), secs(2))
                .ease(easings::cubic_in_out),
            tween_ball
                .radius
                .to(80.0, secs(2))
                .ease(easings::cubic_in_out),
        ],
        wait(secs(3)),
        all![
            hide(&s8_h, hide_dur),
            hide(&s8_sub, hide_dur),
            hide(&s8_code, hide_dur),
            hide(&s8_how, hide_dur),
            hide(&s8_lazy, hide_dur),
            hide(&s8_lazy2, hide_dur),
            hide(&prog_bg, hide_dur),
            hide(&prog_fill, hide_dur),
            hide(&plbl0, hide_dur),
            hide(&plbl1, hide_dur),
            hide(&tween_ball, hide_dur),
            hide(&tween_lbl, hide_dur),
            hide(&s8_step_texts[0], hide_dur),
            hide(&s8_step_texts[1], hide_dur),
            hide(&s8_step_texts[2], hide_dur),
            hide(&s8_step_texts[3], hide_dur),
            hide(&s8_step_texts[4], hide_dur),
            hide(&s8_step_texts[5], hide_dur)
        ],
        wait(ms(150)),
        // ── S9: TWEENABLE + EASINGS ──
        show(&s9_h, ms(500)),
        show(&s9_code, ms(500)),
        wait(secs(4)),
        sequence![
            ms(60),
            show(&s9_easing_h, ms(300)),
            show(&s9_easing_desc, ms(300))
        ],
        sequence![
            ms(50),
            all![show(&eballs[0], ms(200)), show(&elabels[0], ms(200))],
            all![show(&eballs[1], ms(200)), show(&elabels[1], ms(200))],
            all![show(&eballs[2], ms(200)), show(&elabels[2], ms(200))],
            all![show(&eballs[3], ms(200)), show(&elabels[3], ms(200))],
            all![show(&eballs[4], ms(200)), show(&elabels[4], ms(200))],
        ],
        wait(ms(300)),
        // Race!
        all![
            eballs[0]
                .position
                .to(Vec2::new(1050.0, 390.0), secs(2))
                .ease(easings::linear),
            eballs[1]
                .position
                .to(Vec2::new(1050.0, 445.0), secs(2))
                .ease(easings::cubic_in_out),
            eballs[2]
                .position
                .to(Vec2::new(1050.0, 500.0), secs(2))
                .ease(easings::elastic_out),
            eballs[3]
                .position
                .to(Vec2::new(1050.0, 555.0), secs(2))
                .ease(easings::bounce_out),
            eballs[4]
                .position
                .to(Vec2::new(1050.0, 610.0), secs(2))
                .ease(easings::back_out),
        ],
        wait(ms(500)),
        all![
            eballs[0]
                .position
                .to(Vec2::new(250.0, 390.0), secs(2))
                .ease(easings::linear),
            eballs[1]
                .position
                .to(Vec2::new(250.0, 445.0), secs(2))
                .ease(easings::cubic_in_out),
            eballs[2]
                .position
                .to(Vec2::new(250.0, 500.0), secs(2))
                .ease(easings::elastic_out),
            eballs[3]
                .position
                .to(Vec2::new(250.0, 555.0), secs(2))
                .ease(easings::bounce_out),
            eballs[4]
                .position
                .to(Vec2::new(250.0, 610.0), secs(2))
                .ease(easings::back_out),
        ],
        wait(ms(500)),
        all![
            hide(&s9_h, hide_dur),
            hide(&s9_code, hide_dur),
            hide(&s9_easing_h, hide_dur),
            hide(&s9_easing_desc, hide_dur),
            hide(&eballs[0], hide_dur),
            hide(&eballs[1], hide_dur),
            hide(&eballs[2], hide_dur),
            hide(&eballs[3], hide_dur),
            hide(&eballs[4], hide_dur),
            hide(&elabels[0], hide_dur),
            hide(&elabels[1], hide_dur),
            hide(&elabels[2], hide_dur),
            hide(&elabels[3], hide_dur),
            hide(&elabels[4], hide_dur)
        ],
        wait(ms(150)),
        // ── S10: FLOW CONTROLS ──
        sequence![ms(120), show(&s10_h, ms(500)), show(&s10_sub, ms(400))],
        wait(ms(400)),
        // chain demo
        show(&s10_chain_h, ms(400)),
        all![
            show(&chain_d[0], ms(300)),
            show(&chain_d[1], ms(300)),
            show(&chain_d[2], ms(300))
        ],
        wait(ms(300)),
        chain![
            chain_d[0]
                .position
                .to(Vec2::new(700.0, 200.0), ms(500))
                .ease(easings::cubic_out),
            chain_d[1]
                .position
                .to(Vec2::new(800.0, 200.0), ms(500))
                .ease(easings::cubic_out),
            chain_d[2]
                .position
                .to(Vec2::new(900.0, 200.0), ms(500))
                .ease(easings::cubic_out),
        ],
        wait(secs(1)),
        // all demo
        show(&s10_all_h, ms(400)),
        all![
            show(&all_d[0], ms(300)),
            show(&all_d[1], ms(300)),
            show(&all_d[2], ms(300))
        ],
        wait(ms(300)),
        all![
            all_d[0]
                .position
                .to(Vec2::new(700.0, 330.0), ms(500))
                .ease(easings::cubic_out),
            all_d[1]
                .position
                .to(Vec2::new(800.0, 330.0), ms(500))
                .ease(easings::cubic_out),
            all_d[2]
                .position
                .to(Vec2::new(900.0, 330.0), ms(500))
                .ease(easings::cubic_out),
        ],
        wait(secs(1)),
        // sequence demo
        show(&s10_seq_h, ms(400)),
        all![
            show(&seq_d[0], ms(300)),
            show(&seq_d[1], ms(300)),
            show(&seq_d[2], ms(300))
        ],
        wait(ms(300)),
        sequence![
            ms(250),
            seq_d[0]
                .position
                .to(Vec2::new(700.0, 460.0), ms(500))
                .ease(easings::cubic_out),
            seq_d[1]
                .position
                .to(Vec2::new(800.0, 460.0), ms(500))
                .ease(easings::cubic_out),
            seq_d[2]
                .position
                .to(Vec2::new(900.0, 460.0), ms(500))
                .ease(easings::cubic_out),
        ],
        wait(secs(1)),
        show(&s10_code, ms(500)),
        wait(secs(8)),
        all![
            hide(&s10_h, hide_dur),
            hide(&s10_sub, hide_dur),
            hide(&s10_chain_h, hide_dur),
            hide(&s10_all_h, hide_dur),
            hide(&s10_seq_h, hide_dur),
            hide(&s10_code, hide_dur),
            hide(&chain_d[0], hide_dur),
            hide(&chain_d[1], hide_dur),
            hide(&chain_d[2], hide_dur),
            hide(&all_d[0], hide_dur),
            hide(&all_d[1], hide_dur),
            hide(&all_d[2], hide_dur),
            hide(&seq_d[0], hide_dur),
            hide(&seq_d[1], hide_dur),
            hide(&seq_d[2], hide_dur)
        ],
        wait(ms(150)),
        // ── S11: TIMELINE + RENDERING ──
        show(&s11_h, ms(500)),
        wait(ms(400)),
        show(&s11_code, ms(500)),
        wait(secs(7)),
        sequence![
            ms(200),
            show(&s11_leftover, ms(400)),
            show(&s11_leftover2, ms(400))
        ],
        wait(secs(4)),
        show(&s11_render_h, ms(400)),
        sequence![
            ms(200),
            show(&s11_render_texts[0], ms(350)),
            show(&s11_render_texts[1], ms(350)),
            show(&s11_render_texts[2], ms(350)),
            show(&s11_render_texts[3], ms(350)),
        ],
        wait(secs(7)),
        all![
            hide(&s11_h, hide_dur),
            hide(&s11_code, hide_dur),
            hide(&s11_leftover, hide_dur),
            hide(&s11_leftover2, hide_dur),
            hide(&s11_render_h, hide_dur),
            hide(&s11_render_texts[0], hide_dur),
            hide(&s11_render_texts[1], hide_dur),
            hide(&s11_render_texts[2], hide_dur),
            hide(&s11_render_texts[3], hide_dur)
        ],
        wait(ms(300)),
        // ── S12: EVENT LOOP ──
        sequence![ms(120), show(&s12_h, ms(500)), show(&s12_sub, ms(400))],
        wait(ms(500)),
        show(&s12_code, ms(500)),
        wait(secs(8)),
        sequence![
            ms(200),
            show(&s12_why, ms(400)),
            show(&s12_why2, ms(400)),
            show(&s12_hash, ms(400))
        ],
        wait(secs(5)),
        all![
            hide(&s12_h, hide_dur),
            hide(&s12_sub, hide_dur),
            hide(&s12_code, hide_dur),
            hide(&s12_why, hide_dur),
            hide(&s12_why2, hide_dur),
            hide(&s12_hash, hide_dur)
        ],
        wait(ms(150)),
        // ── S13: HEADLESS EXPORT ──
        sequence![ms(120), show(&s13_h, ms(500)), show(&s13_sub, ms(400))],
        wait(ms(500)),
        show(&s13_code, ms(500)),
        wait(secs(8)),
        sequence![
            ms(200),
            show(&s13_cache, ms(400)),
            show(&s13_ffmpeg, ms(400)),
            show(&s13_parallel, ms(400))
        ],
        wait(secs(5)),
        all![
            hide(&s13_h, hide_dur),
            hide(&s13_sub, hide_dur),
            hide(&s13_code, hide_dur),
            hide(&s13_cache, hide_dur),
            hide(&s13_ffmpeg, hide_dur),
            hide(&s13_parallel, hide_dur)
        ],
        wait(ms(150)),
        // ── S14: UTILITIES ──
        sequence![ms(120), show(&s14_h, ms(500)), show(&s14_sub, ms(400))],
        wait(ms(500)),
        show(&s14_code, ms(500)),
        wait(secs(8)),
        sequence![ms(200), show(&s14_lazy, ms(400)), show(&s14_arc, ms(400))],
        wait(secs(5)),
        all![
            hide(&s14_h, hide_dur),
            hide(&s14_sub, hide_dur),
            hide(&s14_code, hide_dur),
            hide(&s14_lazy, hide_dur),
            hide(&s14_arc, hide_dur)
        ],
        wait(ms(300)),
        // ── S15: FINALE ──
        show(&fin, ms(700)),
        wait(ms(500)),
        sequence![
            ms(150),
            show(&fin_texts[0], ms(350)),
            show(&fin_texts[1], ms(350)),
            show(&fin_texts[2], ms(350)),
            show(&fin_texts[3], ms(350)),
            show(&fin_texts[4], ms(350)),
            show(&fin_texts[5], ms(350)),
            show(&fin_texts[6], ms(350)),
            show(&fin_texts[7], ms(350)),
            show(&fin_texts[8], ms(350)),
            show(&fin_texts[9], ms(350)),
        ],
        wait(secs(2)),
        show(&fin_hint, ms(400)),
        wait(secs(6)),
    ]);

    #[cfg(feature = "audio")]
    project.scene.audio_timeline.add(
        play!(AudioNode::new("./background.mp3").with_volume(0.3))
    );

    project
        .with_ffmpeg(true)
        .export()
        .expect("Failed to render");
}
