#![cfg(feature = "code")]
use crate::engine::font::FontManager;
use glam::Vec2;
use lazy_static::lazy_static;
use skrifa::instance::{LocationRef, Size};
use skrifa::MetadataProvider;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use vello::kurbo::{Affine, BezPath};
use vello::peniko::Color;

lazy_static! {
    pub static ref SYNTAX_SET: SyntaxSet = SyntaxSet::load_defaults_newlines();
    pub static ref THEME_SET: ThemeSet = ThemeSet::load_defaults();
    pub static ref GLOBAL_CODE_CACHE: Mutex<HashMap<CodeCacheKey, Arc<Vec<Token>>>> =
        Mutex::new(HashMap::new());
}

pub const DEFAULT_THEME: &str = "base16-ocean.dark";
pub const FONT_FALLBACKS: &[&str] = &["Fira Code", "Courier New", "monospace"];
pub const ADVANCE_FALLBACK_FACTOR: f32 = 0.6;
pub const LINE_HEIGHT_MULTIPLIER: f32 = 1.5;

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct CodeCacheKey {
    pub code: String,
    pub font_size_bits: u32,
    pub language: String,
    pub theme: String,
    pub font_family: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub text: String,
    pub color: Color,
    pub pos: Vec2,
    pub size: f32,
    pub glyphs: Vec<(Affine, BezPath)>,
    pub width: f32,
    pub line_index: usize,
}

pub fn strip_common_indent(text: &str) -> String {
    let lines: Vec<&str> = text.lines().collect();
    if lines.is_empty() {
        return text.to_string();
    }

    let min_indent = lines
        .iter()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.chars().take_while(|c| c.is_whitespace()).count())
        .min()
        .unwrap_or(0);

    if min_indent == 0 {
        return text.to_string();
    }

    lines
        .iter()
        .map(|l| {
            if l.trim().is_empty() {
                ""
            } else {
                &l[min_indent..]
            }
        })
        .collect::<Vec<&str>>()
        .join("\n")
}

pub struct PathSink<'a>(pub &'a mut BezPath);

impl<'a> skrifa::outline::OutlinePen for PathSink<'a> {
    fn move_to(&mut self, x: f32, y: f32) {
        self.0.move_to((x as f64, y as f64));
    }
    fn line_to(&mut self, x: f32, y: f32) {
        self.0.line_to((x as f64, y as f64));
    }
    fn quad_to(&mut self, cx0: f32, cy0: f32, x: f32, y: f32) {
        self.0
            .quad_to((cx0 as f64, cy0 as f64), (x as f64, y as f64));
    }
    fn curve_to(&mut self, cx0: f32, cy0: f32, cx1: f32, cy1: f32, x: f32, y: f32) {
        self.0.curve_to(
            (cx0 as f64, cy0 as f64),
            (cx1 as f64, cy1 as f64),
            (x as f64, y as f64),
        );
    }
    fn close(&mut self) {
        self.0.close_path();
    }
}

pub fn tokenize_code(
    code: &str,
    font_size: f32,
    language: &str,
    theme_name: &str,
    font_family: &str,
    font_fallbacks: &[&str],
) -> Vec<Token> {
    let key = CodeCacheKey {
        code: code.to_string(),
        font_size_bits: font_size.to_bits(),
        language: language.to_string(),
        theme: theme_name.to_string(),
        font_family: font_family.to_string(),
    };

    if let Some(cached) = GLOBAL_CODE_CACHE.lock().unwrap().get(&key) {
        return (**cached).clone();
    }

    let mut tokens = Vec::new();
    let syntax = SYNTAX_SET
        .find_syntax_by_extension(language)
        .or_else(|| SYNTAX_SET.find_syntax_by_name(language))
        .or_else(|| SYNTAX_SET.find_syntax_by_name(&language.to_lowercase()))
        .or_else(|| {
            SYNTAX_SET.find_syntax_by_name(&format!(
                "{}{}",
                (&language[..1]).to_uppercase(),
                &language[1..]
            ))
        })
        .unwrap_or_else(|| SYNTAX_SET.find_syntax_plain_text());

    let theme_name = if THEME_SET.themes.contains_key(theme_name) {
        theme_name
    } else {
        DEFAULT_THEME
    };
    let theme = &THEME_SET.themes[theme_name];
    let mut h = HighlightLines::new(syntax, theme);
    let mut y_offset = 0.0;

    let mut fallback_list = vec![font_family];
    fallback_list.extend_from_slice(font_fallbacks);

    if let Some(font_data) = FontManager::get_font_with_fallback(&fallback_list) {
        let font_ref = FontManager::get_font_ref(&font_data);
        let charmap = font_ref.charmap();
        let outlines = font_ref.outline_glyphs();

        for (line_idx, line) in code.lines().enumerate() {
            let ranges = h.highlight_line(line, &SYNTAX_SET).unwrap();
            let mut x_offset = 0.0;
            for (style, text) in ranges {
                let fg = style.foreground;
                let color = Color::rgba8(fg.r, fg.g, fg.b, fg.a);

                let mut token_text = String::new();
                let mut glyphs = Vec::new();
                let mut token_width = 0.0;

                for c in text.chars() {
                    let glyph_id = charmap.map(c).unwrap_or_default();
                    let mut pb = BezPath::new();
                    let mut advance = (font_size * ADVANCE_FALLBACK_FACTOR) as f64;

                    if let Some(glyph) = outlines.get(glyph_id) {
                        let mut sink = PathSink(&mut pb);
                        let size = Size::new(font_size);
                        let _ = glyph.draw(size, &mut sink);

                        if let Some(metrics) = font_ref
                            .glyph_metrics(size, LocationRef::default())
                            .advance_width(glyph_id)
                        {
                            advance = metrics as f64;
                        }
                    }

                    let base_transform = Affine::translate((token_width, font_size as f64))
                        * Affine::scale_non_uniform(1.0, -1.0);
                    glyphs.push((base_transform, pb));
                    token_width += advance;
                    token_text.push(c);
                }

                tokens.push(Token {
                    text: token_text,
                    color,
                    pos: Vec2::new(x_offset as f32, y_offset as f32),
                    size: font_size,
                    glyphs,
                    width: token_width as f32,
                    line_index: line_idx,
                });

                x_offset += token_width;
            }
            y_offset += (font_size * LINE_HEIGHT_MULTIPLIER) as f64;
        }
    }

    let arc_tokens: Arc<Vec<Token>> = Arc::new(tokens.clone());
    GLOBAL_CODE_CACHE.lock().unwrap().insert(key, arc_tokens);
    tokens
}
