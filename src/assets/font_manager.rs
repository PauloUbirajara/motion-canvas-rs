use font_kit::family_name::FamilyName;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;
use lazy_static::lazy_static;
use skrifa::FontRef;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

/// Container for raw font byte data and its identifier.
pub struct FontData {
    /// The name of the font family or file.
    pub name: String,
    /// The raw TrueType/OpenType byte data.
    pub data: Vec<u8>,
}

const KNOWN_MATH_FONTS: &[&str] = &[
    "DejaVu Math TeX Gyre",
    "Noto Sans Math",
    "New Computer Modern Math",
    "STIX Two Math",
];

const GENERIC_FALLBACKS: &[(FamilyName, &str)] = &[
    (FamilyName::SansSerif, "Sans-Serif"),
    (FamilyName::Monospace, "Monospace"),
    (FamilyName::Serif, "Serif"),
];

lazy_static! {
    static ref FONT_CACHE: Mutex<HashMap<String, Arc<FontData>>> = Mutex::new(HashMap::new());
    static ref FONT_WARNINGS: Mutex<HashMap<String, bool>> = Mutex::new(HashMap::new());
}

/// Global manager for font discovery, loading, and caching.
///
/// `FontManager` provides a unified interface to load fonts from the system,
/// from local files, or from registered memory buffers. It includes robust 
/// fallback logic and specialized support for finding Math fonts required by Typst.
pub struct FontManager;

impl FontManager {
    /// Retrieves a font by its family name or file path.
    ///
    /// This method first checks an internal cache, then tries to load it as a 
    /// local file, and finally searches the system's font directories.
    pub fn get_font(family: &str) -> Option<Arc<FontData>> {
        let mut cache = FONT_CACHE.lock().unwrap();

        if let Some(font) = cache.get(family) {
            return Some(font.clone());
        }

        // Check if it's a local file path
        let path = std::path::Path::new(family);
        if path.exists() && path.is_file() {
            if let Ok(data) = std::fs::read(path) {
                let font_data = Arc::new(FontData {
                    name: path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or(family)
                        .to_string(),
                    data,
                });
                cache.insert(family.to_string(), font_data.clone());
                return Some(font_data);
            }
        }

        // Search system fonts
        let source = SystemSource::new();
        let family_names = [FamilyName::Title(family.to_string())];
        let handle = match source.select_best_match(&family_names, &Properties::new()) {
            Ok(h) => h,
            Err(_) => return None,
        };

        if let Ok(font) = handle.load() {
            if let Some(data) = font.copy_font_data() {
                let font_data = Arc::new(FontData {
                    name: family.to_string(),
                    data: (*data).clone(),
                });
                cache.insert(family.to_string(), font_data.clone());
                return Some(font_data);
            }
        }

        None
    }

    /// Explicitly registers a font from a file path under a custom name.
    pub fn register_font(
        name: &str,
        path: impl AsRef<std::path::Path>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let data = std::fs::read(path)?;
        let font_data = Arc::new(FontData {
            name: name.to_string(),
            data,
        });
        let mut cache = FONT_CACHE.lock().unwrap();
        cache.insert(name.to_string(), font_data);
        Ok(())
    }

    /// Attempts to load the first available font from a list of families.
    ///
    /// If the primary font is not found, it prints a warning and tries subsequent 
    /// fallbacks, eventually resorting to generic system fonts (Sans-Serif, etc.).
    pub fn get_font_with_fallback(families: &[&str]) -> Option<Arc<FontData>> {
        let primary = families
            .first()
            .map(|f| f.to_string())
            .unwrap_or_else(|| "Unknown".to_string());

        for &family in families {
            if let Some(font) = Self::get_font(family) {
                if family != primary {
                    let mut warnings = FONT_WARNINGS.lock().unwrap();
                    if !warnings.contains_key(&primary) {
                        eprintln!(
                            "Warning: Font '{}' not found. Falling back to '{}'.",
                            primary, family
                        );
                        warnings.insert(primary.clone(), true);
                    }
                }
                return Some(font);
            }
        }

        // Final attempt at generic sans-serif
        let source = SystemSource::new();

        for (generic, name) in GENERIC_FALLBACKS {
            let handle = match source.select_best_match(&[generic.clone()], &Properties::new()) {
                Ok(h) => h,
                Err(_) => continue,
            };

            let font = match handle.load() {
                Ok(f) => f,
                Err(_) => continue,
            };

            if let Some(data) = font.copy_font_data() {
                let mut warnings = FONT_WARNINGS.lock().unwrap();
                if !warnings.contains_key(&primary) {
                    eprintln!(
                        "Warning: Font '{}' not found. Falling back to system '{}'.",
                        primary, name
                    );
                    warnings.insert(primary.clone(), true);
                }
                return Some(Arc::new(FontData {
                    name: name.to_string(),
                    data: (*data).clone(),
                }));
            }
        }

        None
    }

    /// Discovers a suitable Math font on the system for Typst rendering.
    ///
    /// This method prioritizes well-known math fonts (like DejaVu Math) and then 
    /// falls back to any system font that contains "Math" in its metadata.
    pub fn get_math_font() -> (String, Option<Arc<FontData>>) {
        static MATH_CACHE: OnceLock<(String, Option<Arc<FontData>>)> = OnceLock::new();
        MATH_CACHE
            .get_or_init(|| {
                // First try specific known math fonts
                for &family in KNOWN_MATH_FONTS {
                    if let Some(font) = Self::get_font(family) {
                        return (family.to_string(), Some(font));
                    }
                }

                // Search all system fonts for anything with "Math" in the name
                let source = SystemSource::new();
                let fonts = match source.all_fonts() {
                    Ok(f) => f,
                    Err(_) => {
                        return (
                            "serif".to_string(),
                            Self::get_font_with_fallback(&["serif"]),
                        )
                    }
                };

                for handle in fonts {
                    let font = match handle.load() {
                        Ok(f) => f,
                        Err(_) => continue,
                    };
                    let name = font.full_name();
                    if !name.contains("Math") {
                        continue;
                    }
                    if let Some(data) = font.copy_font_data() {
                        return (
                            name.clone(),
                            Some(Arc::new(FontData {
                                name,
                                data: (*data).clone(),
                            })),
                        );
                    }
                }

                // Fallback to serif if no math font found (better than nothing for Typst)
                (
                    "serif".to_string(),
                    Self::get_font_with_fallback(&["serif"]),
                )
            })
            .clone()
    }

    /// Converts a [`FontData`] into a [`FontRef`] for use with the `skrifa` crate.
    pub fn get_font_ref(data: &Arc<FontData>) -> FontRef<'_> {
        FontRef::new(&data.data).unwrap()
    }
}
