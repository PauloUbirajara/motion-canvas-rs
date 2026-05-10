//! Asset management and resource loading.
//!
//! This module handles external resources such as images, fonts, audio,
//! and specialized compilation tasks like Typst math and code syntax highlighting.

#[cfg(feature = "audio")]
pub mod audio;
#[cfg(feature = "code")]
pub mod code_tokenizer;
#[cfg(feature = "export")]
pub mod export;
pub mod font_manager;
pub mod hash;
#[cfg(any(feature = "image", feature = "svg"))]
pub mod image_manager;
pub mod palette;
#[cfg(feature = "math")]
pub mod typst_support;

/// Sanitizes a string to be safe for use as a filename.
///
/// Converts to lowercase, replaces non-alphanumeric characters with underscores,
/// and collapses multiple underscores.
pub fn sanitize_title(title: &str) -> String {
    title
        .trim()
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect::<String>()
        .split('_')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("_")
}
