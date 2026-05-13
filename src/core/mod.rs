//! Core animation engine logic.
//!
//! This module contains the fundamental traits and structures that drive the
//! animation system, including signals, tweening, timelines, and scenes.

pub mod animation;
pub mod easings;
pub mod scene;
pub mod timeline;

pub use animation::*;
pub use easings::*;
pub use scene::*;
pub use timeline::*;
