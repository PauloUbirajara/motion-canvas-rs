pub mod engine;
pub mod render;

// --- RE-EXPORTS ---

/// Core project and scene management
pub use engine::project::Project;

/// Common mathematical types
pub use glam::Vec2;

/// Custom Result type for the library
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Individual node types (Circle, Rect, TextNode, etc.)
pub mod nodes {
    pub use crate::engine::nodes::*;
}

/// Animation flow controls and macros (all!, chain!, wait, etc.)
pub mod flows {
    pub use crate::engine::animation::flow::*;
    // Re-export macros at the module level as well
    pub use crate::{all, any, chain, delay, loop_anim, sequence};
}

/// Easing functions (cubic_in, elastic_out, etc.)
pub mod easings {
    pub use crate::engine::easings::*;
}

/// Common types for a quick start
pub mod prelude {
    pub use crate::engine::project::Project;

    // Core Traits
    pub use crate::engine::animation::base::Animation;
    pub use crate::engine::animation::base::Node;
    pub use crate::engine::animation::tween::Tweenable;

    // Export the modules themselves for namespaced access
    pub use crate::easings;
    pub use crate::flows;
    pub use crate::nodes;

    // Glob-export for direct access (e.g. Circle, all!, quad_in)
    pub use crate::engine::animation::flow::*;
    pub use crate::engine::easings::*;
    pub use crate::engine::nodes::*;
    pub use crate::{all, any, chain, delay, loop_anim, sequence, with_easing};

    pub use crate::Result;
    pub use glam::Vec2;
    pub use vello::peniko::Color;
    pub use vello::kurbo::{Affine, BezPath};
}
