use typst::diag::FileResult;
use typst::foundations::{Bytes, Datetime};
use typst::syntax::{FileId, Source, VirtualPath};
use typst::text::{Font, FontBook};
use typst::Library;
use typst::World;
use typst::utils::LazyHash;
use typst::LibraryExt;
use crate::engine::font::FontManager;
use vello::kurbo::BezPath;
use vello::peniko::Color;
use vello::kurbo::Affine;
use skrifa::MetadataProvider;

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
        let (_, math_font_data) = FontManager::get_math_font();
        if let Some(data) = math_font_data {
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

pub fn collect_paths(frame: &typst::layout::Frame, transform: Affine, paths: &mut Vec<(Affine, Color, BezPath)>) {
    for (p, item) in frame.items() {
        let item_transform = transform * Affine::translate((p.x.to_pt(), p.y.to_pt()));
        match item {
            typst::layout::FrameItem::Text(text) => {
                let mut x_cursor = 0.0;
                let font_data = text.font.data();
                let font_ref = skrifa::FontRef::new(font_data).unwrap();
                let outlines = font_ref.outline_glyphs();
                for glyph in &text.glyphs {
                    let mut pb = BezPath::new();
                    if let Some(g_out) = outlines.get(skrifa::GlyphId::from(glyph.id)) {
                        let mut sink = PathSink(&mut pb);
                        let s = text.size.to_pt() as f32;
                        let _ = g_out.draw(skrifa::instance::Size::new(s), &mut sink);
                    }
                    let glyph_transform = item_transform * Affine::translate((
                        x_cursor + glyph.x_offset.at(text.size).to_pt(), 
                        glyph.y_offset.at(text.size).to_pt() + text.size.to_pt()
                    )) * Affine::scale_non_uniform(1.0, -1.0);
                    // Note: Color is passed from the node
                    paths.push((glyph_transform, Color::WHITE, pb));
                    x_cursor += glyph.x_advance.at(text.size).to_pt();
                }
            }
            typst::layout::FrameItem::Group(group) => {
                collect_paths(&group.frame, item_transform * Affine::scale_non_uniform(1.0, 1.0), paths);
            }
            _ => {}
        }
    }
}

struct PathSink<'a>(&'a mut BezPath);

impl<'a> skrifa::outline::OutlinePen for PathSink<'a> {
    fn move_to(&mut self, x: f32, y: f32) { self.0.move_to((x as f64, y as f64)); }
    fn line_to(&mut self, x: f32, y: f32) { self.0.line_to((x as f64, y as f64)); }
    fn quad_to(&mut self, cx0: f32, cy0: f32, x: f32, y: f32) { self.0.quad_to((cx0 as f64, cy0 as f64), (x as f64, y as f64)); }
    fn curve_to(&mut self, cx0: f32, cy0: f32, cx1: f32, cy1: f32, x: f32, y: f32) { self.0.curve_to((cx0 as f64, cy0 as f64), (cx1 as f64, cy1 as f64), (x as f64, y as f64)); }
    fn close(&mut self) { self.0.close_path(); }
}
