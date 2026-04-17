use crate::engine::animation::{Node, Signal};
use glam::Vec2;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use vello::kurbo::{Affine, BezPath};
use vello::peniko::{Brush, Color, Fill};
use vello::Scene;

lazy_static::lazy_static! {
    static ref GLOBAL_MATH_CACHE: Mutex<HashMap<MathCacheKey, Arc<Vec<(Affine, BezPath)>>>> = Mutex::new(HashMap::new());
}

const DEFAULT_FONT_SIZE: f32 = 32.0;
const DEFAULT_COLOR: Color = Color::WHITE;
const DEFAULT_OPACITY: f32 = 1.0;

const TYPST_MATH_TEMPLATE: &str = r#"
#set text(size: {size}pt)
#show math.equation: set text(font: "{font}")
$ {equation} $
"#;

#[derive(Hash, Eq, PartialEq)]
struct MathCacheKey {
    equation: String,
    font_size_bits: u32,
}

pub struct MathNode {
    pub position: Signal<Vec2>,
    pub rotation: Signal<f32>,
    pub scale: Signal<Vec2>,
    pub equation: Signal<String>,
    pub font_size: Signal<f32>,
    pub color: Signal<Color>,
    pub opacity: Signal<f32>,
    pub transition_progress: Signal<f32>,
    cache: Arc<Mutex<Option<Arc<Vec<(Affine, BezPath)>>>>>,
    prev_cache: Arc<Mutex<Option<Arc<Vec<(Affine, BezPath)>>>>>,
}

impl Default for MathNode {
    fn default() -> Self {
        Self {
            position: Signal::new(Vec2::ZERO),
            rotation: Signal::new(0.0),
            scale: Signal::new(Vec2::ONE),
            equation: Signal::new("".to_string()),
            font_size: Signal::new(DEFAULT_FONT_SIZE),
            color: Signal::new(DEFAULT_COLOR),
            opacity: Signal::new(DEFAULT_OPACITY),
            transition_progress: Signal::new(1.0),
            cache: Arc::new(Mutex::new(None)),
            prev_cache: Arc::new(Mutex::new(None)),
        }
    }
}

impl Clone for MathNode {
    fn clone(&self) -> Self {
        Self {
            position: self.position.clone(),
            rotation: self.rotation.clone(),
            scale: self.scale.clone(),
            equation: self.equation.clone(),
            font_size: self.font_size.clone(),
            color: self.color.clone(),
            opacity: self.opacity.clone(),
            transition_progress: self.transition_progress.clone(),
            cache: self.cache.clone(),
            prev_cache: self.prev_cache.clone(),
        }
    }
}

impl MathNode {
    pub fn new(pos: Vec2, equation: &str, size: f32, color: Color) -> Self {
        Self::default()
            .with_position(pos)
            .with_equation(equation)
            .with_font_size(size)
            .with_color(color)
    }

    pub fn with_position(mut self, position: Vec2) -> Self {
        self.position = Signal::new(position);
        self
    }

    pub fn with_rotation(mut self, angle: f32) -> Self {
        self.rotation = Signal::new(angle);
        self
    }

    pub fn with_scale(mut self, scale: f32) -> Self {
        self.scale = Signal::new(Vec2::splat(scale));
        self
    }

    pub fn with_scale_xy(mut self, scale: Vec2) -> Self {
        self.scale = Signal::new(scale);
        self
    }

    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = Signal::new(opacity);
        self
    }

    pub fn tex(
        &self,
        equation: &str,
        duration: Duration,
    ) -> Box<dyn crate::engine::animation::Animation> {
        Box::new(MathTransition {
            node: self.clone(),
            target_eq: equation.to_string(),
            duration,
            tween: None,
        })
    }

    pub fn with_equation(mut self, equation: &str) -> Self {
        self.equation = Signal::new(equation.to_string());
        self
    }

    pub fn with_font_size(mut self, size: f32) -> Self {
        self.font_size = Signal::new(size);
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = Signal::new(color);
        self
    }
    // ...

    pub fn start_transition(&self, new_eq: &str) {
        let prev_eq = self.equation.get();
        if prev_eq == new_eq {
            return;
        }

        self.rebuild_if_needed();

        // 1. Move current cache to prev_cache
        {
            let cache = self.cache.lock().unwrap();
            let mut prev = self.prev_cache.lock().unwrap();
            *prev = cache.clone();
        }

        // 2. Start transition
        self.transition_progress.set(0.0);
        self.equation.set(new_eq.to_string());

        self.rebuild_if_needed();
    }

    fn rebuild_if_needed(&self) {
        let eq = self.equation.get();
        let size = self.font_size.get();

        let key = MathCacheKey {
            equation: eq.clone(),
            font_size_bits: size.to_bits(),
        };

        // 1. Check global cache
        let mut global = GLOBAL_MATH_CACHE.lock().unwrap();
        if let Some(paths) = global.get(&key) {
            let mut local = self.cache.lock().unwrap();
            *local = Some(paths.clone());
            return;
        }

        // 3. Compile
        let mut paths_with_color = Vec::new();
        let (font_name, _) = crate::engine::font::FontManager::get_math_font();
        let typst_code = TYPST_MATH_TEMPLATE
            .replace("{size}", &size.to_string())
            .replace("{font}", &font_name)
            .replace("{equation}", &eq);
        let world = crate::engine::typst_support::TypstWorld::new(&typst_code);
        let output = typst::compile::<typst::layout::PagedDocument>(&world).output;

        match output {
            Ok(document) => {
                for page in document.pages {
                    crate::engine::typst_support::collect_paths(
                        &page.frame,
                        Affine::IDENTITY,
                        &mut paths_with_color,
                    );
                }
            }
            Err(e) => println!("Typst compilation failed: {:?}", e),
        }

        let paths: Arc<Vec<(Affine, BezPath)>> = Arc::new(
            paths_with_color
                .into_iter()
                .map(|(a, _, p)| (a, p))
                .collect(),
        );
        global.insert(key, paths.clone());

        let mut local = self.cache.lock().unwrap();
        *local = Some(paths);
    }
}

struct MathTransition {
    node: MathNode,
    target_eq: String,
    duration: Duration,
    tween: Option<crate::engine::animation::SignalTween<f32>>,
}

impl crate::engine::animation::Animation for MathTransition {
    fn update(&mut self, dt: Duration) -> (bool, Duration) {
        if self.tween.is_none() {
            self.node.start_transition(&self.target_eq);
            self.tween = Some(self.node.transition_progress.to(1.0, self.duration));
        }
        self.tween.as_mut().unwrap().update(dt)
    }
    fn duration(&self) -> Duration {
        self.duration
    }
}


impl Node for MathNode {
    fn render(&self, scene: &mut Scene, parent_transform: Affine, parent_opacity: f32) {
        let color = self.color.get();

        self.rebuild_if_needed();

        let cache_guard = self.cache.lock().unwrap();
        let progress = self.transition_progress.get();
        let base_opacity = self.opacity.get();

        let pos = self.position.get();
        let rot = self.rotation.get();
        let sc = self.scale.get();

        let local_transform = Affine::translate((pos.x as f64, pos.y as f64))
            * Affine::rotate(rot as f64)
            * Affine::scale_non_uniform(sc.x as f64, sc.y as f64);

        let root_transform = parent_transform * local_transform;

        // 1. Draw previous equation if transitioning
        if progress < 1.0 {
            let prev_cache = self.prev_cache.lock().unwrap();
            if let Some(prev) = prev_cache.as_ref() {
                let mut prev_color = color;
                prev_color.a = (color.a as f32 * base_opacity * (1.0 - progress) * parent_opacity)
                    .clamp(0.0, 255.0) as u8;
                for (local_transform, pb) in prev.as_ref() {
                    scene.fill(
                        Fill::NonZero,
                        root_transform * *local_transform,
                        &Brush::Solid(prev_color),
                        None,
                        pb,
                    );
                }
            }
        }

        if let Some(c) = cache_guard.as_ref() {
            let current_alpha = if progress < 1.0 {
                base_opacity * progress
            } else {
                base_opacity
            };
            let mut current_color = color;
            current_color.a =
                (color.a as f32 * current_alpha * parent_opacity).clamp(0.0, 255.0) as u8;

            for (local_transform, pb) in c.as_ref() {
                scene.fill(
                    Fill::NonZero,
                    root_transform * *local_transform,
                    &Brush::Solid(current_color),
                    None,
                    pb,
                );
            }
        }
    }
    fn update(&mut self, _dt: Duration) {}
    fn state_hash(&self) -> u64 {
        self.position.state_hash()
            ^ self.rotation.state_hash()
            ^ self.scale.state_hash()
            ^ self.equation.state_hash()
            ^ self.font_size.state_hash()
            ^ self.color.state_hash()
            ^ self.opacity.state_hash()
            ^ self.transition_progress.state_hash()
    }

    fn clone_node(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}
