#![cfg(feature = "code")]
use crate::core::animation::Tweenable;
use crate::assets::font_manager::FontManager;
use glam::Vec2;
use lazy_static::lazy_static;
use similar::TextDiff;
use skrifa::instance::{LocationRef, Size};
use skrifa::MetadataProvider;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use vello::kurbo::{Affine, BezPath};
use vello::peniko::{Brush, Color};
use vello::Scene;

lazy_static! {
    /// The global set of syntax definitions for highlighting.
    pub static ref SYNTAX_SET: SyntaxSet = SyntaxSet::load_defaults_newlines();
    /// The global set of highlighting themes.
    pub static ref THEME_SET: ThemeSet = ThemeSet::load_defaults();
    /// Internal cache to avoid re-tokenizing identical code blocks.
    pub static ref GLOBAL_CODE_CACHE: Mutex<HashMap<CodeCacheKey, Arc<Vec<Token>>>> =
        Mutex::new(HashMap::new());
}

/// The default syntax highlighting theme name.
pub const DEFAULT_THEME: &str = "base16-ocean.dark";
/// List of fonts to try if the primary code font is missing.
pub const FONT_FALLBACKS: &[&str] = &["Fira Code", "Courier New", "monospace"];
/// Multiplier for character advance if font metrics are unavailable.
pub const ADVANCE_FALLBACK_FACTOR: f32 = 0.6;
/// Default line height as a multiple of font size.
pub const LINE_HEIGHT_MULTIPLIER: f32 = 1.5;
/// Default font size for code snippets.
pub const DEFAULT_FONT_SIZE: f32 = 24.0;
/// Default font family for code snippets.
pub const DEFAULT_FONT_FAMILY: &str = "Fira Code";
/// Default programming language for highlighting.
pub const DEFAULT_LANGUAGE: &str = "rust";
/// Default opacity for normal code.
pub const DEFAULT_OPACITY: f32 = 1.0;
/// Opacity for "dimmed" code (non-selected lines).
pub const DEFAULT_DIM_OPACITY: f32 = 0.2;

/// Cache key for lookups in [`GLOBAL_CODE_CACHE`].
#[derive(Hash, Eq, PartialEq, Clone)]
pub struct CodeCacheKey {
    /// The raw source code text.
    pub code: String,
    /// The font size bit representation.
    pub font_size_bits: u32,
    /// Programming language name.
    pub language: String,
    /// Highlighting theme name.
    pub theme: String,
    /// Font family name.
    pub font_family: String,
}

/// A single highlighted fragment of code.
///
/// `Token` contains the text, color, and pre-rasterized glyph paths for 
/// efficient rendering and interpolation.
#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    /// The raw text of the token.
    pub text: String,
    /// The syntax-highlighted color.
    pub color: Color,
    /// Local position relative to the code block.
    pub pos: Vec2,
    /// Font size in pixels.
    pub size: f32,
    /// Pre-computed vector paths for each glyph in the token.
    pub glyphs: Vec<(Affine, BezPath)>,
    /// Total layout width of the token.
    pub width: f32,
    /// Line index within the code block (0-indexed).
    pub line_index: usize,
}

/// Internal state for morphing between two code snippets.
#[derive(Clone, Debug, PartialEq)]
pub struct CodeTransition {
    /// Source text before the transition.
    pub from_text: String,
    /// Destination text after the transition.
    pub to_text: String,
    /// Source tokens.
    pub from_tokens: Vec<Token>,
    /// Destination tokens.
    pub to_tokens: Vec<Token>,
    /// Progress of the transition (0.0 to 1.0).
    pub progress: f32,
    /// Mapping of token indices between source and destination.
    pub matches: Vec<(usize, usize)>, // (from_idx, to_idx)
    /// Selection indices before the transition.
    pub from_selection: Vec<usize>,
    /// Selection indices after the transition.
    pub to_selection: Vec<usize>,
}

/// The value type for a [`CodeNode`](crate::elements::media::CodeNode).
///
/// `CodeValue` encapsulates the text, its highlighted tokens, and any 
/// active transition state between two code snippets.
#[derive(Clone, Debug, PartialEq)]
pub struct CodeValue {
    /// The raw source code.
    pub text: String,
    /// The list of highlighted tokens.
    pub tokens: Vec<Token>,
    /// Optional transition state during morphing.
    pub transition: Option<CodeTransition>,
    /// List of line indices that are currently highlighted/selected.
    pub selection: Vec<usize>,
}

impl CodeValue {
    /// Creates a new highlighted code value.
    pub fn new(
        text: String,
        font_size: f32,
        language: &str,
        theme: &str,
        font_family: &str,
    ) -> Self {
        let text = strip_common_indent(&text);
        let tokens = tokenize_code(
            &text,
            font_size,
            language,
            theme,
            font_family,
            FONT_FALLBACKS,
        );
        CodeValue {
            text,
            tokens,
            transition: None,
            selection: Vec::new(),
        }
    }
}

impl Default for CodeValue {
    fn default() -> Self {
        Self {
            text: String::new(),
            tokens: Vec::new(),
            transition: None,
            selection: Vec::new(),
        }
    }
}

impl Tweenable for CodeValue {
    /// Interpolates between two code snippets.
    ///
    /// This implementation uses a diffing algorithm (Patience) to identify 
    /// matching tokens between `a` and `b`, enabling smooth morphing transitions
    /// where code segments move to their new positions.
    fn interpolate(a: &Self, b: &Self, t: f32) -> Self {
        if t <= 0.0 {
            return a.clone();
        }
        if t >= 1.0 {
            return b.clone();
        }

        // If both text and highlights are identical, no need to transition
        if a.text == b.text && a.selection == b.selection {
            return b.clone();
        }

        // Find matches between a and b tokens using similar crate
        let a_toks: Vec<String> = a
            .tokens
            .iter()
            .map(|t| format!("{}{:?}", t.text, t.color))
            .collect();
        let b_toks: Vec<String> = b
            .tokens
            .iter()
            .map(|t| format!("{}{:?}", t.text, t.color))
            .collect();

        let a_tok_refs: Vec<&str> = a_toks.iter().map(|s| s.as_str()).collect();
        let b_tok_refs: Vec<&str> = b_toks.iter().map(|s| s.as_str()).collect();

        let diff = TextDiff::configure()
            .algorithm(similar::Algorithm::Patience)
            .diff_slices(&a_tok_refs, &b_tok_refs);
        let mut matches = Vec::new();

        for op in diff.ops() {
            match *op {
                similar::DiffOp::Equal {
                    old_index,
                    new_index,
                    len,
                } => {
                    for i in 0..len {
                        matches.push((old_index + i, new_index + i));
                    }
                }
                _ => {}
            }
        }

        CodeValue {
            text: b.text.clone(),
            tokens: b.tokens.clone(),
            transition: Some(CodeTransition {
                from_text: a.text.clone(),
                to_text: b.text.clone(),
                from_tokens: a.tokens.clone(),
                to_tokens: b.tokens.clone(),
                progress: t,
                matches,
                from_selection: a.selection.clone(),
                to_selection: b.selection.clone(),
            }),
            selection: b.selection.clone(),
        }
    }

    fn state_hash(&self) -> u64 {
        use crate::assets::hash::Hasher;
        let mut h = Hasher::new();
        h.update_bytes(self.text.as_bytes());
        for &line in &self.selection {
            h.update_u64(line as u64);
        }
        if let Some(trans) = &self.transition {
            h.update_bytes(trans.from_text.as_bytes());
            h.update_bytes(trans.to_text.as_bytes());
            h.update_u64(trans.progress.to_bits() as u64);
        }
        h.finish()
    }
}

/// Removes common whitespace indentation from a multi-line string.
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

/// Tokenizes and highlights source code into a list of [`Token`]s.
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

/// Parses a selection string (e.g., "1-3,5") into a list of 0-indexed line indices.
pub fn parse_selection(selection: &str) -> Vec<usize> {
    let mut lines = Vec::new();
    for part in selection.split(',') {
        let part = part.trim();
        if part.contains('-') {
            let mut bounds = part.split('-');
            if let (Some(start_str), Some(end_str)) = (bounds.next(), bounds.next()) {
                if let (Ok(start), Ok(end)) = (start_str.parse::<usize>(), end_str.parse::<usize>())
                {
                    for i in start..=end {
                        if i > 0 {
                            lines.push(i - 1);
                        }
                    }
                }
            }
        } else if let Ok(line) = part.parse::<usize>() {
            if line > 0 {
                lines.push(line - 1);
            }
        }
    }
    lines.sort_unstable();
    lines.dedup();
    lines
}

/// Renders a single code token into a Vello [`Scene`].
pub fn draw_token(scene: &mut Scene, transform: Affine, token: &Token, color: Color, opacity: f32) {
    if opacity <= 0.0 {
        return;
    }
    let mut c = color;
    let alpha = (color.a as f32 * opacity).clamp(0.0, 255.0) as u8;
    c.a = alpha;
    let brush = Brush::Solid(c);
    for (glyph_transform, pb) in &token.glyphs {
        scene.fill(
            vello::peniko::Fill::NonZero,
            transform * *glyph_transform,
            &brush,
            None,
            pb,
        );
    }
}
