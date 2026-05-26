#![doc = include_str!("../README.md")]

pub mod assets;
pub mod core;
pub mod elements;
pub mod project;
#[cfg(feature = "runtime")]
pub mod runtime;

// --- RE-EXPORTS ---

/// High-level configuration and project management
pub use project::Project;

pub use crate::core::animation::paint::Paint;
/// Common mathematical types
pub use glam::Vec2;
pub use peniko::Color;
pub use peniko::{ColorStop, ColorStops, Extend, Gradient, GradientKind};

/// Custom Result type for the library
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Re-export nodes for easier access
pub mod nodes {
    pub use crate::elements::*;
}

/// Animation flow controls and macros (all!, chain!, wait, etc.)
pub mod flows {
    pub use crate::core::animation::flow::*;
    // Re-export macros at the module level as well
    pub use crate::{all, any, chain, delay, loop_anim, sequence, wait, with_easing};
    #[cfg(feature = "audio")]
    pub use crate::{audio_wait, play};
}

/// Easing functions (cubic_in, elastic_out, etc.)
pub mod easings {
    pub use crate::core::easings::*;
}

/// Common types for a quick start
pub mod prelude {
    pub use crate::project::Project;

    // Core Traits
    pub use crate::core::animation::base::Animation;
    pub use crate::core::animation::base::Node;
    pub use crate::core::animation::tween::Signal;
    pub use crate::core::animation::tween::Tweenable;
    pub use crate::core::filters::{Blur, DEFAULT_BLUR};
    pub use crate::core::{AnyAnimation, AnyNode};

    // Export the modules themselves for namespaced access
    pub use crate::easings;
    pub use crate::flows;
    pub use crate::nodes;

    #[cfg(feature = "runtime")]
    pub use crate::runtime::ProjectRuntimeExt;

    // Glob-export for direct access (e.g. Circle, all!, quad_in)
    pub use crate::core::animation::flow::*;
    pub use crate::core::easings::*;

    pub use crate::core::masks::MaskMode;
    pub use crate::elements::container::*;
    pub use crate::elements::masks::{MaskNode, MaskNode as Mask};
    pub use crate::elements::media::*;
    pub use crate::elements::shapes::*;

    #[cfg(feature = "physics")]
    pub use crate::elements::physics::{
        PhysicsMode, PhysicsNode, PhysicsShape, RigidBodyNode, StaticBodyNode,
    };

    pub use crate::{
        all, any, chain, delay, linear_gradient, loop_anim, radial_gradient, sequence, wait,
        with_easing,
    };
    #[cfg(feature = "audio")]
    pub use crate::{audio_wait, play};

    pub use crate::assets::font_manager::FontManager;
    pub use crate::assets::palette::Palette;
    pub use crate::Result;

    pub use crate::core::animation::paint::Paint;
    pub use glam::Vec2;
    pub use kurbo::{Affine, BezPath, Point};
    pub use peniko::Color;
    pub use peniko::{ColorStop, ColorStops, Extend, Gradient, GradientKind};

    #[cfg(feature = "runtime")]
    pub use vello::Scene;
}
