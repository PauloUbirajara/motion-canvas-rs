//! Animation lifecycle and reactive property system.
//!
//! This module provides the core traits for nodes and animations, the signal-based
//! property system, and control flow primitives for complex animations.

pub mod base;
pub mod binding;
pub mod enum_defs;
pub mod flow;
pub mod tween;

pub use base::*;
pub use binding::*;
pub use enum_defs::*;
pub use flow::*;
pub use tween::*;
