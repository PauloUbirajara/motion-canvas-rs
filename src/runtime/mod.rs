pub mod renderer;
pub mod window;

#[cfg(feature = "export")]
pub mod export;

use crate::Project;

/// Extension trait to provide ergonomic runtime methods to Project.
/// This keeps the core Project struct clean of side-effects while 
/// allowing for a simple .show() or .export() API when the runtime is available.
pub trait ProjectRuntimeExt {
    /// Opens the playback window for the project.
    fn show(self) -> crate::Result<()>;

    /// Starts an export session for the project.
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
