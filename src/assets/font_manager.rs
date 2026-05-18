//! Global system font loader, layout manager, and glyph resolution cache.
//!
//! This module coordinates font lookups using `fontdb` and parses them using `skrifa`.
//! It manages standard fallbacks, Math glyph faces for LaTeX nodes, and user-registered fonts.

use fontdb::{Database, Family, Query};
use skrifa::FontRef;
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, LazyLock, Mutex, OnceLock};

/// Represents loaded font raw data and its collection metadata.
pub struct FontData {
    /// The user-friendly identifier or family name of the font face.
    pub name: String,
    /// The raw byte array containing the TTF/OTF font file content.
    pub data: Vec<u8>,
    /// The face index inside a font collection (TTC/OTC). Normally 0 for standard single-face fonts.
    pub index: u32,
}

const KNOWN_MATH_FONTS: &[&str] = &[
    "DejaVu Math TeX Gyre",
    "Noto Sans Math",
    "New Computer Modern Math",
    "STIX Two Math",
];

const GENERIC_FALLBACKS: &[(Family<'static>, &str)] = &[
    (Family::SansSerif, "Sans-Serif"),
    (Family::Monospace, "Monospace"),
    (Family::Serif, "Serif"),
];

static FONT_DB: LazyLock<Mutex<Database>> = LazyLock::new(|| {
    let mut db = Database::new();
    db.load_system_fonts();
    Mutex::new(db)
});

static FONT_CACHE: LazyLock<Mutex<HashMap<String, Arc<FontData>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

static FONT_WARNINGS: LazyLock<Mutex<HashMap<String, bool>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Thread-safe global font coordinator.
///
/// It handles lazy-loading system fonts, registering custom local paths, and caching
/// parsed binary assets to prevent redundant filesystem overhead.
///
/// ### Example
/// ```rust
/// # use motion_canvas_rs::assets::font_manager::FontManager;
/// // Get standard sans-serif font
/// let font = FontManager::get_font("sans-serif");
/// assert!(font.is_some());
/// ```
pub struct FontManager;

impl FontManager {
    /// Retrieves a font by its path or registered family name.
    ///
    /// It first checks the static query cache, then checks if `family` points to an active
    /// local filepath, and finally queries the OS/system font database via `fontdb`.
    pub fn get_font(family: &str) -> Option<Arc<FontData>> {
        let mut cache = FONT_CACHE.lock().unwrap();

        if let Some(font) = cache.get(family) {
            return Some(font.clone());
        }

        // 1. Local file path check
        let path = Path::new(family);
        if path.exists() && path.is_file() {
            if let Ok(data) = std::fs::read(path) {
                let font_data = Arc::new(FontData {
                    name: path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or(family)
                        .to_string(),
                    data,
                    index: 0,
                });
                cache.insert(family.to_string(), font_data.clone());
                return Some(font_data);
            }
        }

        // 2. Database search
        let db = FONT_DB.lock().unwrap();
        let family_enum = match family.to_lowercase().as_str() {
            "monospace" => Family::Monospace,
            "sans-serif" | "sans" => Family::SansSerif,
            "serif" => Family::Serif,
            "cursive" => Family::Cursive,
            "fantasy" => Family::Fantasy,
            _ => Family::Name(family),
        };
        let query = Query {
            families: &[family_enum],
            ..Query::default()
        };

        if let Some(id) = db.query(&query) {
            if let Some(font_data) = Self::load_from_db(&db, id, family) {
                cache.insert(family.to_string(), font_data.clone());
                return Some(font_data);
            }
        }

        None
    }

    /// Internal helper method to load a matching database face into raw `FontData`.
    fn load_from_db(db: &Database, id: fontdb::ID, name: &str) -> Option<Arc<FontData>> {
        let actual_name = db
            .face(id)
            .and_then(|face| face.families.first().map(|(fam, _)| fam.clone()))
            .unwrap_or_else(|| name.to_string());
        db.with_face_data(id, |data, index| {
            Arc::new(FontData {
                name: actual_name.clone(),
                data: data.to_vec(),
                index,
            })
        })
    }

    /// Reads a font file from a local path and registers it dynamically with a custom face name.
    ///
    /// Once registered, any standard Text nodes querying this name will instantly resolve to it.
    pub fn register_font(
        name: &str,
        path: impl AsRef<Path>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let data = std::fs::read(path.as_ref())?;

        let mut db = FONT_DB.lock().unwrap();
        db.load_font_data(data.clone());

        let font_data = Arc::new(FontData {
            name: name.to_string(),
            data,
            index: 0,
        });

        FONT_CACHE
            .lock()
            .unwrap()
            .insert(name.to_string(), font_data);
        Ok(())
    }

    /// Tries to load the first available font from a prioritised list of families.
    ///
    /// If none of the requested families are found, it queries standard generic fallbacks
    /// (e.g. Sans-Serif, Serif, Monospace).
    pub fn get_font_with_fallback(families: &[&str]) -> Option<Arc<FontData>> {
        let primary = families.first()?.to_string();

        for &family in families {
            if let Some(font) = Self::get_font(family) {
                if family != primary {
                    Self::warn_fallback(&primary, family);
                }
                return Some(font);
            }
        }

        let db = FONT_DB.lock().unwrap();
        for (generic_fam, name) in GENERIC_FALLBACKS {
            let query = Query {
                families: &[generic_fam.clone()],
                ..Query::default()
            };

            if let Some(id) = db.query(&query) {
                if let Some(font) = Self::load_from_db(&db, id, name) {
                    Self::warn_fallback(&primary, name);
                    return Some(font);
                }
            }
        }
        None
    }

    /// Logs a single thread-safe console warning when a fallback font is selected.
    fn warn_fallback(primary: &str, fallback: &str) {
        let mut warnings = FONT_WARNINGS.lock().unwrap();
        if !warnings.contains_key(primary) {
            eprintln!(
                "Warning: Font '{}' not found. Falling back to '{}'.",
                primary, fallback
            );
            warnings.insert(primary.to_string(), true);
        }
    }

    /// Automatically scans the system and internal list for standard Math TeX Gyre or compatible math fonts.
    ///
    /// Used by LaTeX math rendering nodes to fetch math-compatible layouts.
    pub fn get_math_font() -> (String, Option<Arc<FontData>>) {
        static MATH_CACHE: OnceLock<(String, Option<Arc<FontData>>)> = OnceLock::new();

        let cached = MATH_CACHE.get_or_init(|| {
            for &family in KNOWN_MATH_FONTS {
                if let Some(font) = Self::get_font(family) {
                    return (family.to_string(), Some(font));
                }
            }

            let db = FONT_DB.lock().unwrap();
            for face in db.faces() {
                for (fam_name, _) in &face.families {
                    if fam_name.contains("Math") {
                        if let Some(font) = Self::load_from_db(&db, face.id, fam_name) {
                            return (fam_name.clone(), Some(font));
                        }
                    }
                }
            }

            (
                "serif".to_string(),
                Self::get_font_with_fallback(&["serif"]),
            )
        });

        cached.clone()
    }

    /// Parses `FontData` using Skrifa to extract a lightweight `FontRef` pointer for text rendering.
    pub fn get_font_ref(data: &Arc<FontData>) -> FontRef<'_> {
        FontRef::from_index(&data.data, data.index).unwrap()
    }
}
