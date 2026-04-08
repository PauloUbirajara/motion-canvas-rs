use crate::engine::animation::{Signal, Node};
use vello::peniko::{Brush, Color, Fill};
use vello::Scene;
use glam::Vec2;
use vello::kurbo::{Affine, BezPath};
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref GLOBAL_MATH_CACHE: Mutex<HashMap<MathCacheKey, Arc<Vec<(Affine, BezPath)>>>> = Mutex::new(HashMap::new());
}



#[derive(Hash, Eq, PartialEq)]
struct MathCacheKey {
    equation: String,
    font_size_bits: u32,
}

pub struct MathNode {
    pub transform: Signal<Affine>,
    pub equation: Signal<String>,
    pub font_size: Signal<f32>,
    pub color: Signal<Color>,
    pub opacity: Signal<f32>,
    pub transition_progress: Signal<f32>,
    cache: Arc<Mutex<Option<Arc<Vec<(Affine, BezPath)>>>>>,
    prev_cache: Arc<Mutex<Option<Arc<Vec<(Affine, BezPath)>>>>>,
}

impl Clone for MathNode {
    fn clone(&self) -> Self {
        Self {
            transform: self.transform.clone(),
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
        Self {
            transform: Signal::new(Affine::translate((pos.x as f64, pos.y as f64))),
            equation: Signal::new(equation.to_string()),
            font_size: Signal::new(size),
            color: Signal::new(color),
            opacity: Signal::new(1.0),
            transition_progress: Signal::new(1.0),
            cache: Arc::new(Mutex::new(None)),
            prev_cache: Arc::new(Mutex::new(None)),
        }
    }

    pub fn with_transform(mut self, transform: Affine) -> Self {
        self.transform = Signal::new(transform);
        self
    }

    pub fn with_position(mut self, position: Vec2) -> Self {
        self.transform = Signal::new(Affine::translate((position.x as f64, position.y as f64)));
        self
    }

    pub fn with_rotation(mut self, angle: f32) -> Self {
        let current = self.transform.get();
        let coeffs = current.as_coeffs();
        let tx = coeffs[4];
        let ty = coeffs[5];
        self.transform = Signal::new(Affine::translate((tx, ty)) * Affine::rotate(angle as f64));
        self
    }

    pub fn with_scale(mut self, scale: f32) -> Self {
        let current = self.transform.get();
        let coeffs = current.as_coeffs();
        let tx = coeffs[4];
        let ty = coeffs[5];
        self.transform = Signal::new(Affine::translate((tx, ty)) * Affine::scale(scale as f64));
        self
    }

    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = Signal::new(opacity);
        self
    }

    pub fn tex(&self, equation: &str, duration: Duration) -> Box<dyn crate::engine::animation::Animation> {
        Box::new(MathTransition {
            node: self.clone(),
            target_eq: equation.to_string(),
            duration,
            tween: None,
        })
    }
    // ...

    pub fn start_transition(&self, new_eq: &str) {
        let prev_eq = self.equation.get();
        if prev_eq == new_eq { return; }

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
        let typst_code = format!("#set text(size: {}pt)\n#show math.equation: set text(font: \"{}\")\n$ {} $", size, font_name, eq);
        let world = crate::engine::typst_support::TypstWorld::new(&typst_code);
        let output = typst::compile::<typst::layout::PagedDocument>(&world).output;
        
        match output {
            Ok(document) => {
                for page in document.pages {
                    crate::engine::typst_support::collect_paths(&page.frame, Affine::IDENTITY, &mut paths_with_color);
                }
            }
            Err(e) => println!("Typst compilation failed: {:?}", e),
        }

        let paths: Arc<Vec<(Affine, BezPath)>> = Arc::new(paths_with_color.into_iter().map(|(a, _, p)| (a, p)).collect());
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

use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

impl Node for MathNode {
    fn render(&self, scene: &mut Scene, parent_transform: Affine, parent_opacity: f32) {
        let color = self.color.get();

        self.rebuild_if_needed();

        let cache_guard = self.cache.lock().unwrap();
        let progress = self.transition_progress.get();
        let base_opacity = self.opacity.get();
        let root_transform = parent_transform * self.transform.get();

        // 1. Draw previous equation if transitioning
        if progress < 1.0 {
            if let Some(prev) = self.prev_cache.lock().unwrap().as_ref() {
                let mut prev_color = color;
                prev_color.a = (color.a as f32 * base_opacity * (1.0 - progress) * parent_opacity).clamp(0.0, 255.0) as u8;
                for (local_transform, pb) in prev.as_ref() {
                    scene.fill(Fill::NonZero, root_transform * *local_transform, &Brush::Solid(prev_color), None, pb);
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
            current_color.a = (color.a as f32 * current_alpha * parent_opacity).clamp(0.0, 255.0) as u8;

            for (local_transform, pb) in c.as_ref() {
                scene.fill(Fill::NonZero, root_transform * *local_transform, &Brush::Solid(current_color), None, pb);
            }
        }
    }
    fn update(&mut self, _dt: Duration) {}
    fn state_hash(&self) -> u64 {
        let mut s = DefaultHasher::new();
        
        let coeffs = self.transform.get().as_coeffs();
        for c in coeffs {
            c.to_bits().hash(&mut s);
        }
        
        self.equation.get().hash(&mut s);
        self.font_size.get().to_bits().hash(&mut s);
        let color = self.color.get();
        color.r.hash(&mut s);
        color.g.hash(&mut s);
        color.b.hash(&mut s);
        color.a.hash(&mut s);
        self.opacity.get().to_bits().hash(&mut s);
        self.transition_progress.get().to_bits().hash(&mut s);
        s.finish()
    }

    fn clone_node(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}
