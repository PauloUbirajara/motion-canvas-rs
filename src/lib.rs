pub mod engine;
pub mod render;

// --- RE-EXPORTS ---

/// Core project and scene management
pub use engine::project::Project;

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
    
    // Export the modules themselves for namespaced access (e.g. easings::quad_in)
    pub use crate::nodes;
    pub use crate::flows;
    pub use crate::easings;
    
    // Glob-export for direct access (e.g. Circle, all!, quad_in)
    pub use crate::engine::nodes::*;
    pub use crate::engine::animation::flow::*;
    pub use crate::engine::easings::*;
    pub use crate::{all, any, chain, delay, loop_anim, sequence};
    
    pub use glam::Vec2;
    pub use vello::peniko::Color;
}
