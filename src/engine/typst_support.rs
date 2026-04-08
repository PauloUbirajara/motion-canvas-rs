use typst::diag::FileResult;
use typst::foundations::{Bytes, Datetime};
use typst::syntax::{FileId, Source, VirtualPath};
use typst::text::{Font, FontBook};
use typst::Library;
use typst::World;
use typst::utils::LazyHash;
use typst::LibraryExt;
use crate::engine::font::FontManager;

pub struct TypstWorld {
    library: LazyHash<Library>,
    book: LazyHash<FontBook>,
    fonts: Vec<Font>,
    source: Source,
    main_id: FileId,
}

impl TypstWorld {
    pub fn new(text: &str) -> Self {
        let library = Library::default();
        let mut fonts = Vec::new();
        
        // 1. Always try a system math font first for MathNode compatibility
        if let Some(data) = FontManager::get_math_font() {
            if let Some(font) = Font::new(Bytes::new(data.data.clone()), 0) {
                fonts.push(font);
            }
        }

        // 2. Add some standard fallbacks for general text within Typst
        let families = ["Inter", "Noto Sans", "DejaVu Sans", "Arial", "sans-serif"];
        for family in families {
            if let Some(data) = FontManager::get_font(family) {
                if let Some(font) = Font::new(Bytes::new(data.data.clone()), 0) {
                    fonts.push(font);
                }
            }
        }
        
        if fonts.is_empty() {
            eprintln!("CRITICAL: TypstWorld has no fonts. Layout will fail.");
        }
        
        let book = FontBook::from_fonts(&fonts);
        let main_id = FileId::new(None, VirtualPath::new("/main.typ"));
        let source = Source::new(main_id, text.to_string());
        
        Self {
            library: LazyHash::new(library),
            book: LazyHash::new(book),
            fonts,
            source,
            main_id,
        }
    }
}

impl World for TypstWorld {
    fn library(&self) -> &LazyHash<Library> { &self.library }
    fn book(&self) -> &LazyHash<FontBook> { &self.book }
    fn main(&self) -> FileId { self.main_id }
    fn source(&self, _id: FileId) -> FileResult<Source> { Ok(self.source.clone()) }
    fn file(&self, _id: FileId) -> FileResult<Bytes> { Ok(Bytes::new(vec![])) }
    fn font(&self, id: usize) -> Option<Font> { self.fonts.get(id).cloned() }
    fn today(&self, _offset: Option<i64>) -> Option<Datetime> { None }
}
