use font_kit::family_name::FamilyName;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;
use lazy_static::lazy_static;
use skrifa::FontRef;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct FontData {
    pub name: String,
    pub data: Vec<u8>,
}

lazy_static! {
    static ref FONT_CACHE: Mutex<HashMap<String, Arc<FontData>>> = Mutex::new(HashMap::new());
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
        if let Ok(handle) =
            source.select_best_match(&[FamilyName::Title(family.to_string())], &Properties::new())
        {
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

    pub fn get_font_ref(data: &Arc<FontData>) -> FontRef<'_> {
        FontRef::new(&data.data).unwrap()
    }
}
