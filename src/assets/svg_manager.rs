#![cfg(feature = "svg")]
use std::collections::HashMap;
use std::sync::LazyLock;
use std::sync::{Arc, Mutex};
use usvg::Tree;

#[cfg(feature = "runtime")]
use vello::Scene;

static SVG_CACHE: LazyLock<Mutex<HashMap<String, (Arc<Tree>, Arc<Scene>)>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Global manager for loading and caching SVG assets.
pub struct SvgManager;

impl SvgManager {
    /// Loads an SVG from the specified file path and returns a parsed tree and pre-rendered scene.
    pub fn get_svg(path: &str) -> Option<(Arc<Tree>, Arc<Scene>)> {
        let mut cache = SVG_CACHE.lock().unwrap();
        if let Some((tree, scene)) = cache.get(path) {
            return Some((tree.clone(), scene.clone()));
        }

        let svg_data = std::fs::read(path).ok()?;
        let opt = usvg::Options::default();
        let tree = Tree::from_data(&svg_data, &opt).ok()?;

        let scene = vello_svg::render_tree(&tree);

        let arc_tree = Arc::new(tree);
        let arc_scene = Arc::new(scene);

        cache.insert(path.to_string(), (arc_tree.clone(), arc_scene.clone()));
        Some((arc_tree, arc_scene))
    }
}
