use font_kit::family_name::FamilyName;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;
use lazy_static::lazy_static;
use skrifa::FontRef;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

pub struct FontData {
    pub name: String,
    pub data: Vec<u8>,
}

lazy_static! {
    static ref FONT_CACHE: Mutex<HashMap<String, Arc<FontData>>> = Mutex::new(HashMap::new());
    static ref FONT_WARNINGS: Mutex<HashMap<String, bool>> = Mutex::new(HashMap::new());
}

pub struct FontManager;

impl FontManager {
    pub fn get_font(family: &str) -> Option<Arc<FontData>> {
        let mut cache = FONT_CACHE.lock().unwrap();

        if let Some(font) = cache.get(family) {
            return Some(font.clone());
        }

        // Search system fonts
        let source = SystemSource::new();
        let family_names = [FamilyName::Title(family.to_string())];
        if let Ok(handle) = source.select_best_match(&family_names, &Properties::new()) {
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
        }

        None
    }

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
        let generic_fallbacks = [
            (FamilyName::SansSerif, "Sans-Serif"),
            (FamilyName::Monospace, "Monospace"),
            (FamilyName::Serif, "Serif"),
        ];

        for (generic, name) in generic_fallbacks {
            if let Ok(handle) = source.select_best_match(&[generic], &Properties::new()) {
                if let Ok(font) = handle.load() {
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
            }
        }

        None
    }

    pub fn get_math_font() -> (String, Option<Arc<FontData>>) {
        static MATH_CACHE: OnceLock<(String, Option<Arc<FontData>>)> = OnceLock::new();
        MATH_CACHE
            .get_or_init(|| {
                // First try specific known math fonts
                let known_math = [
                    "DejaVu Math TeX Gyre",
                    "Noto Sans Math",
                    "New Computer Modern Math",
                    "STIX Two Math",
                ];
                for family in known_math {
                    if let Some(font) = Self::get_font(family) {
                        return (family.to_string(), Some(font));
                    }
                }

                // Search all system fonts for anything with "Math" in the name
                let source = SystemSource::new();
                if let Ok(fonts) = source.all_fonts() {
                    for handle in fonts {
                        if let Ok(font) = handle.load() {
                            let name = font.full_name();
                            if name.contains("Math") {
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
                        }
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

    pub fn get_font_ref(data: &Arc<FontData>) -> FontRef<'_> {
        FontRef::new(&data.data).unwrap()
    }
}
