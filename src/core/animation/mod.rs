//! Animation lifecycle and reactive property system.
//!
//! This module provides the core traits for nodes and animations, the signal-based
//! property system, and control flow primitives for complex animations.

pub mod base;
pub mod tween;
pub mod flow;
pub mod binding;

pub use base::*;
pub use tween::*;
pub use flow::*;
pub use binding::*;
