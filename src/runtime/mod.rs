//! Runtime modules for rendering and interacting with animations.
//!
//! This module provides the glue between the core animation engine and the
//! physical output devices (Window/Screen) or files (Export).

pub mod renderer;
pub mod window;

#[cfg(feature = "export")]
pub mod export;

use crate::Project;

/// Extension trait to provide ergonomic runtime methods to [`Project`].
///
/// This trait keeps the core `Project` struct clean of side-effects while
/// allowing for a simple `.show()` or `.export()` API.
pub trait ProjectRuntimeExt {
    /// Opens the playback window for the project.
    ///
    /// This will initialize a `winit` event loop and a Vello renderer to
    /// provide an interactive preview of the animation.
    ///
    /// ### Example
    /// ```no_run
    /// # use motion_canvas_rs::prelude::*;
    /// # let project = Project::default();
    /// project.show().expect("Failed to open window");
    /// ```
    fn show(self) -> crate::Result<()>;

    /// Starts an export session for the project.
    ///
    /// This will render every frame of the project at full resolution and save
    /// them to the configured output path.
    ///
    /// ### Example
    /// ```no_run
    /// # use motion_canvas_rs::prelude::*;
    /// # let mut project = Project::default();
    /// project.export().expect("Failed to export project");
    /// ```
    #[cfg(feature = "export")]
    fn export(&mut self) -> crate::Result<()>;
}

impl ProjectRuntimeExt for Project {
    fn show(self) -> crate::Result<()> {
        self::window::run_window_session(self)
    }

    #[cfg(feature = "export")]
    fn export(&mut self) -> crate::Result<()> {
        self::export::run_export_session(self)
    }
}
