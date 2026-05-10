pub mod audio;
pub mod code_tokenizer;
#[cfg(feature = "export")]
pub mod export;
pub mod font_manager;
pub mod hash;
#[cfg(any(feature = "image", feature = "svg"))]
pub mod image_manager;
pub mod palette;

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
