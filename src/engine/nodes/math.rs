use crate::engine::animation::{Signal, Node};
use vello::peniko::{Brush, Color, Fill};
use vello::Scene;
use glam::Vec2;
use vello::kurbo::{Affine, BezPath};
use std::time::Duration;
use std::sync::Mutex;

#[derive(Clone, Default)]
struct MathCache {
    equation: String,
    font_size: f32,
    color: Color,
    paths: Vec<(Affine, Color, BezPath)>,
}

pub struct MathNode {
    pub position: Signal<Vec2>,
    pub equation: Signal<String>,
    pub font_size: Signal<f32>,
    pub color: Signal<Color>,
    pub opacity: Signal<f32>,
    pub transition_progress: Signal<f32>,
    cache: Arc<Mutex<Option<MathCache>>>,
    prev_cache: Arc<Mutex<Option<MathCache>>>,
}

use std::sync::Arc;

impl Clone for MathNode {
    fn clone(&self) -> Self {
        Self {
            position: self.position.clone(),
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
            position: Signal::new(pos),
            equation: Signal::new(equation.to_string()),
            font_size: Signal::new(size),
            color: Signal::new(color),
            opacity: Signal::new(1.0),
            transition_progress: Signal::new(1.0),
            cache: Arc::new(Mutex::new(None)),
            prev_cache: Arc::new(Mutex::new(None)),
        }
    }

    pub fn tex(&self, equation: &str, duration: Duration) -> Box<dyn crate::engine::animation::Animation> {
        Box::new(MathTransition {
            node: self.clone(),
            target_eq: equation.to_string(),
            duration,
            tween: None,
        })
    }

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
        
        // Trigger a rebuild of the NEW equation cache immediately so it's ready for the first frame of transition
        self.rebuild_if_needed();
    }

    fn rebuild_if_needed(&self) {
        let eq = self.equation.get();
        let size = self.font_size.get();
        let color = self.color.get();

        // println!("MathNode: rebuild_if_needed checking '{}'", eq);
        let cache_guard = self.cache.lock().unwrap();
        // println!("MathNode: got cache lock");
        let needs_rebuild = cache_guard.as_ref().map_or(true, |c| {
            c.equation != eq || c.font_size != size || c.color != color
        });

        if needs_rebuild {
            drop(cache_guard);
            
            let mut paths = Vec::new();
            let (font_name, _) = crate::engine::font::FontManager::get_math_font();
            let typst_code = format!("#set text(size: {}pt)\n#show math.equation: set text(font: \"{}\")\n$ {} $", size, font_name, eq);
            let world = crate::engine::typst_support::TypstWorld::new(&typst_code);
            let output = typst::compile::<typst::layout::PagedDocument>(&world).output;
            match output {
                Ok(document) => {
                    for page in document.pages {
                        crate::engine::typst_support::collect_paths(&page.frame, Affine::IDENTITY, &mut paths);
                    }
                }
                Err(e) => println!("Typst compilation failed: {:?}", e),
            }

            // println!("MathNode: re-taking cache lock");
            let mut cache_guard = self.cache.lock().unwrap();
            // println!("MathNode: got cache lock again");
            if let Some(c) = cache_guard.as_ref() {
                if c.equation == eq && c.font_size == size && c.color == color {
                    // println!("MathNode: already rebuilt by someone else");
                    return;
                }
            }

            *cache_guard = Some(MathCache {
                equation: eq.clone(),
                font_size: size,
                color,
                paths,
            });
            // println!("MathNode: cache updated");
        }
        // println!("MathNode: rebuild_if_needed done");
    }
}

struct MathTransition {
    node: MathNode,
    target_eq: String,
    duration: Duration,
    tween: Option<crate::engine::animation::SignalTween<f32>>,
}

impl crate::engine::animation::Animation for MathTransition {
    fn update(&mut self, dt: Duration) -> bool {
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
    fn render(&self, scene: &mut Scene) {
        let pos = self.position.get();
        let color = self.color.get();

        self.rebuild_if_needed();

        let cache_guard = self.cache.lock().unwrap();
        let progress = self.transition_progress.get();
        let base_opacity = self.opacity.get();
        let root_transform = Affine::translate((pos.x as f64, pos.y as f64));

        // 1. Draw previous equation if transitioning
        if progress < 1.0 {
            if let Some(prev) = self.prev_cache.lock().unwrap().as_ref() {
                let mut prev_color = color;
                prev_color.a = (color.a as f32 * base_opacity * (1.0 - progress)) as u8;
                for (local_transform, _c, pb) in &prev.paths {
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
            current_color.a = (color.a as f32 * current_alpha) as u8;

            for (local_transform, _cached_color, pb) in &c.paths {
                scene.fill(Fill::NonZero, root_transform * *local_transform, &Brush::Solid(current_color), None, pb);
            }
        }
    }
    fn update(&mut self, _dt: Duration) {}
    fn state_hash(&self) -> u64 {
        let mut s = DefaultHasher::new();
        let pos = self.position.get();
        let eq = self.equation.get();
        let size = self.font_size.get();
        let color = self.color.get();
        
        pos.x.to_bits().hash(&mut s);
        pos.y.to_bits().hash(&mut s);
        eq.hash(&mut s);
        size.to_bits().hash(&mut s);
        color.r.hash(&mut s);
        color.g.hash(&mut s);
        color.b.hash(&mut s);
        color.a.hash(&mut s);
        self.opacity.get().to_bits().hash(&mut s);
        self.transition_progress.get().to_bits().hash(&mut s);
        
        s.finish()
    }
}
