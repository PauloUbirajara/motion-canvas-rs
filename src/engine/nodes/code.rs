//! Code highlighting and animation module.
//! 
//! Credits: The token-based animation logic and diffing approach is inspired by
//! [shiki-magic-move](https://github.com/shikijs/shiki-magic-move).

use crate::engine::animation::{Signal, Node, Tweenable};
use crate::engine::font::FontManager;
use vello::peniko::{Brush, Color, Fill};
use vello::Scene;
use glam::Vec2;
use vello::kurbo::{Affine, BezPath};
use std::time::Duration;
use skrifa::MetadataProvider;
use skrifa::instance::{Size, LocationRef};
use syntect::parsing::SyntaxSet;
use syntect::highlighting::ThemeSet;
use syntect::easy::HighlightLines;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use lazy_static::lazy_static;
use similar::TextDiff;

lazy_static! {
    static ref SYNTAX_SET: SyntaxSet = SyntaxSet::load_defaults_newlines();
    static ref THEME_SET: ThemeSet = ThemeSet::load_defaults();
    static ref GLOBAL_CODE_CACHE: Mutex<HashMap<CodeCacheKey, Arc<Vec<Token>>>> = Mutex::new(HashMap::new());
}

const DEFAULT_FONT_SIZE: f32 = 24.0;
const DEFAULT_THEME: &str = "base16-ocean.dark";
const DEFAULT_FONT_FAMILY: &str = "Fira Code";
const LINE_HEIGHT_MULTIPLIER: f32 = 1.5;
const ADVANCE_FALLBACK_FACTOR: f32 = 0.6;

#[derive(Hash, Eq, PartialEq, Clone)]
struct CodeCacheKey {
    code: String,
    font_size_bits: u32,
    language: String,
    theme: String,
    font_family: String,
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

#[derive(Clone, Debug, PartialEq)]
pub struct CodeTransition {
    pub from_tokens: Vec<Token>,
    pub to_tokens: Vec<Token>,
    pub progress: f32,
    pub matches: Vec<(usize, usize)>, // (from_idx, to_idx)
    pub from_selection: Vec<usize>,
    pub to_selection: Vec<usize>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CodeValue {
    pub text: String,
    pub tokens: Vec<Token>,
    pub transition: Option<CodeTransition>,
    pub selection: Vec<usize>,
}

impl CodeValue {
    pub fn new(text: String, node: &CodeNode) -> Self {
        let text = strip_common_indent(&text);
        let tokens = node.tokenize(&text);
        CodeValue {
            text,
            tokens,
            transition: None,
            selection: Vec::new(),
        }
    }
}

fn strip_common_indent(text: &str) -> String {
    let lines: Vec<&str> = text.lines().collect();
    if lines.is_empty() { return text.to_string(); }

    let min_indent = lines.iter()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.chars().take_while(|c| c.is_whitespace()).count())
        .min()
        .unwrap_or(0);

    if min_indent == 0 { return text.to_string(); }

    lines.iter()
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
    fn interpolate(a: &Self, b: &Self, t: f32) -> Self {
        if t <= 0.0 { return a.clone(); }
        if t >= 1.0 { return b.clone(); }

        // If both text and highlights are identical, no need to transition
        if a.text == b.text && a.selection == b.selection {
            return b.clone();
        }

        // Find matches between a and b tokens using similar crate
        // We include color in the key to distinguish tokens better
        let a_toks: Vec<String> = a.tokens.iter().map(|t| format!("{}{:?}", t.text, t.color)).collect();
        let b_toks: Vec<String> = b.tokens.iter().map(|t| format!("{}{:?}", t.text, t.color)).collect();
        
        // similar::diff_slices works best with &[&str]
        let a_tok_refs: Vec<&str> = a_toks.iter().map(|s| s.as_str()).collect();
        let b_tok_refs: Vec<&str> = b_toks.iter().map(|s| s.as_str()).collect();

        let diff = TextDiff::configure()
            .algorithm(similar::Algorithm::Patience)
            .diff_slices(&a_tok_refs, &b_tok_refs);
        let mut matches = Vec::new();
        
        for op in diff.ops() {
            match *op {
                similar::DiffOp::Equal { old_index, new_index, len } => {
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
}

pub struct CodeNode {
    pub transform: Signal<Affine>,
    pub code: Signal<CodeValue>,
    pub font_size: Signal<f32>,
    pub opacity: Signal<f32>,
    pub dim_opacity: Signal<f32>,
    pub language: String,
    pub theme: String,
    pub font_family: String,
}

impl Clone for CodeNode {
    fn clone(&self) -> Self {
        Self {
            transform: self.transform.clone(),
            code: self.code.clone(),
            font_size: self.font_size.clone(),
            opacity: self.opacity.clone(),
            dim_opacity: self.dim_opacity.clone(),
            language: self.language.clone(),
            theme: self.theme.clone(),
            font_family: self.font_family.clone(),
        }
    }
}

impl CodeNode {
    pub fn new(pos: Vec2, code: &str, lang: &str) -> Self {
        let node = Self {
            transform: Signal::new(Affine::translate((pos.x as f64, pos.y as f64))),
            code: Signal::new(CodeValue::default()),
            font_size: Signal::new(DEFAULT_FONT_SIZE),
            opacity: Signal::new(1.0),
            dim_opacity: Signal::new(0.2), // Default dimming factor
            language: lang.to_string(),
            theme: DEFAULT_THEME.to_string(),
            font_family: DEFAULT_FONT_FAMILY.to_string(),
        };
        
        let initial_value = CodeValue::new(code.to_string(), &node);
        node.code.set(initial_value);
        node
    }

    pub fn with_transform(mut self, transform: Affine) -> Self {
        self.transform = Signal::new(transform);
        self
    }

    pub fn with_position(mut self, position: Vec2) -> Self {
        self.transform = Signal::new(Affine::translate((position.x as f64, position.y as f64)));
        self
    }

    pub fn with_rotation(mut self, angle: f32) -> Self {
        let current = self.transform.get();
        let coeffs = current.as_coeffs();
        let tx = coeffs[4];
        let ty = coeffs[5];
        self.transform = Signal::new(Affine::translate((tx, ty)) * Affine::rotate(angle as f64));
        self
    }

    pub fn with_scale(mut self, scale: f32) -> Self {
        let current = self.transform.get();
        let coeffs = current.as_coeffs();
        let tx = coeffs[4];
        let ty = coeffs[5];
        self.transform = Signal::new(Affine::translate((tx, ty)) * Affine::scale(scale as f64));
        self
    }

    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = Signal::new(opacity);
        self
    }

    pub fn with_theme(mut self, theme: &str) -> Self {
        self.theme = theme.to_string();
        self
    }

    pub fn with_font(mut self, font: &str) -> Self {
        self.font_family = font.to_string();
        self
    }

    pub fn with_font_size(mut self, size: f32) -> Self {
        self.font_size = Signal::new(size);
        // Re-tokenize current code with new size to avoid "spazzing"
        let current_text = self.code.get().text;
        let mut val = CodeValue::new(current_text, &self);
        val.selection = self.code.get().selection;
        self.code.set(val);
        self
    }

    pub fn with_dim_opacity(mut self, dim: f32) -> Self {
        self.dim_opacity = Signal::new(dim);
        self
    }

    pub fn edit(&self, code: &str, duration: Duration) -> crate::engine::animation::SignalTween<CodeValue> {
        let code = code.to_string();
        let node = self.clone();
        self.code.to_lazy(move |current| {
            let mut next_value = CodeValue::new(code, &node);
            next_value.selection = current.selection.clone();
            next_value
        }, duration)
    }

    pub fn append(&self, text: &str, duration: Duration) -> crate::engine::animation::SignalTween<CodeValue> {
        let text = text.to_string();
        let node = self.clone();
        self.code.to_lazy(move |current| {
            let next_text = format!("{}{}", current.text, text);
            let mut next_val = CodeValue::new(next_text, &node);
            next_val.selection = current.selection.clone();
            next_val
        }, duration)
    }

    pub fn prepend(&self, text: &str, duration: Duration) -> crate::engine::animation::SignalTween<CodeValue> {
        let text = text.to_string();
        let node = self.clone();
        self.code.to_lazy(move |current| {
            let next_text = format!("{}{}", text, current.text);
            let mut next_val = CodeValue::new(next_text, &node);
            next_val.selection = current.selection.clone();
            next_val
        }, duration)
    }

    pub fn select_lines(&self, lines: Vec<usize>, duration: Duration) -> crate::engine::animation::SignalTween<CodeValue> {
        self.code.to_lazy(move |current| {
            let mut next_value = current.clone();
            next_value.transition = None; // Reset transition for the target
            next_value.selection = lines;
            next_value
        }, duration)
    }

    /// Select lines using a printer-style selection string (e.g., "1-3, 5").
    /// Uses 1-based indexing for user convenience.
    pub fn select_string(&self, selection: &str, duration: Duration) -> crate::engine::animation::SignalTween<CodeValue> {
        let lines = self.parse_selection(selection);
        self.select_lines(lines, duration)
    }

    fn parse_selection(&self, selection: &str) -> Vec<usize> {
        let mut lines = Vec::new();
        for part in selection.split(',') {
            let part = part.trim();
            if part.contains('-') {
                let mut bounds = part.split('-');
                if let (Some(start_str), Some(end_str)) = (bounds.next(), bounds.next()) {
                    if let (Ok(start), Ok(end)) = (start_str.parse::<usize>(), end_str.parse::<usize>()) {
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

    fn tokenize(&self, code: &str) -> Vec<Token> {
        let size = self.font_size.get();
        let key = CodeCacheKey {
            code: code.to_string(),
            font_size_bits: size.to_bits(),
            language: self.language.clone(),
            theme: self.theme.clone(),
            font_family: self.font_family.clone(),
        };

        if let Some(cached) = GLOBAL_CODE_CACHE.lock().unwrap().get(&key) {
            return (**cached).clone();
        }

        let mut tokens = Vec::new();
        let syntax = SYNTAX_SET.find_syntax_by_extension(&self.language)
            .or_else(|| SYNTAX_SET.find_syntax_by_name(&self.language))
            .or_else(|| SYNTAX_SET.find_syntax_by_name(&self.language.to_lowercase()))
            .or_else(|| SYNTAX_SET.find_syntax_by_name(&format!("{}{}", (&self.language[..1]).to_uppercase(), &self.language[1..])))
            .unwrap_or_else(|| SYNTAX_SET.find_syntax_plain_text());
        
        let theme_name = if THEME_SET.themes.contains_key(&self.theme) {
            &self.theme
        } else {
            DEFAULT_THEME
        };
        let theme = &THEME_SET.themes[theme_name];
        let mut h = HighlightLines::new(syntax, theme);
        let mut y_offset = 0.0;

        if let Some(font_data) = FontManager::get_font_with_fallback(&[&self.font_family, "Fira Code", "Courier New", "monospace"]) {
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
                        let mut advance = (size * ADVANCE_FALLBACK_FACTOR) as f64;
                        
                        if let Some(glyph) = outlines.get(glyph_id) {
                            let mut sink = PathSink(&mut pb);
                            let font_size = Size::new(size);
                            let _ = glyph.draw(font_size, &mut sink);
                            
                            if let Some(metrics) = font_ref.glyph_metrics(font_size, LocationRef::default()).advance_width(glyph_id) {
                                advance = metrics as f64;
                            }
                        }
                        
                        let base_transform = Affine::translate((token_width, size as f64)) * Affine::scale_non_uniform(1.0, -1.0);
                        glyphs.push((base_transform, pb));
                        token_width += advance;
                        token_text.push(c);
                    }
                    
                    tokens.push(Token {
                        text: token_text,
                        color,
                        pos: Vec2::new(x_offset as f32, y_offset as f32),
                        size,
                        glyphs,
                        width: token_width as f32,
                        line_index: line_idx,
                    });
                    
                    x_offset += token_width;
                }
                y_offset += (size * LINE_HEIGHT_MULTIPLIER) as f64;
            }
        }

        let arc_tokens = Arc::new(tokens.clone());
        GLOBAL_CODE_CACHE.lock().unwrap().insert(key, arc_tokens);
        tokens
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

impl Node for CodeNode {
    fn render(&self, scene: &mut Scene, parent_transform: Affine, parent_opacity: f32) {
        let code_val = self.code.get();
        let local_transform = self.transform.get();
        let opacity = self.opacity.get();
        let root_transform = parent_transform * local_transform;
        let combined_opacity = parent_opacity * opacity;

        let dim_factor = self.dim_opacity.get();

        if let Some(trans) = &code_val.transition {
            let p = trans.progress;
            
            let mut matched_from = vec![false; trans.from_tokens.len()];
            let mut matched_to = vec![false; trans.to_tokens.len()];
            
            // 1. Draw moving matches
            for &(from_idx, to_idx) in &trans.matches {
                let from = &trans.from_tokens[from_idx];
                let to = &trans.to_tokens[to_idx];
                
                let current_pos = from.pos.lerp(to.pos, p);
                let current_color = Color::interpolate(&from.color, &to.color, p);
                
                let scale = if from.size != to.size {
                    (from.size + (to.size - from.size) * p) / to.size
                } else {
                    1.0
                };

                let from_is_dimmed = !trans.from_selection.is_empty() && !trans.from_selection.contains(&from.line_index);
                let to_is_dimmed = !trans.to_selection.is_empty() && !trans.to_selection.contains(&to.line_index);
                
                let from_dim = if from_is_dimmed { dim_factor } else { 1.0 };
                let to_dim = if to_is_dimmed { dim_factor } else { 1.0 };
                let current_dim = from_dim + (to_dim - from_dim) * p;

                draw_token(scene, root_transform * Affine::translate((current_pos.x as f64, current_pos.y as f64)) * Affine::scale(scale as f64), to, current_color, combined_opacity * current_dim);
                
                matched_from[from_idx] = true;
                matched_to[to_idx] = true;
            }
            
            // Draw unmatched from-tokens (vanishing)
            for (i, matched) in matched_from.iter().enumerate() {
                if !*matched {
                    let from = &trans.from_tokens[i];
                    let is_dimmed = !trans.from_selection.is_empty() && !trans.from_selection.contains(&from.line_index);
                    let dim = if is_dimmed { dim_factor } else { 1.0 };
                    draw_token(scene, root_transform * Affine::translate((from.pos.x as f64, from.pos.y as f64)), from, from.color, combined_opacity * dim * (1.0 - p));
                }
            }
            
            // Draw unmatched to-tokens (appearing)
            for (i, matched) in matched_to.iter().enumerate() {
                if !*matched {
                    let to = &trans.to_tokens[i];
                    let is_dimmed = !trans.to_selection.is_empty() && !trans.to_selection.contains(&to.line_index);
                    let dim = if is_dimmed { dim_factor } else { 1.0 };
                    draw_token(scene, root_transform * Affine::translate((to.pos.x as f64, to.pos.y as f64)), to, to.color, combined_opacity * dim * p);
                }
            }
        } else {
            // Static render
            let has_selection = !code_val.selection.is_empty();
            let dim_factor = self.dim_opacity.get();
            for token in &code_val.tokens {
                let is_selected = !has_selection || code_val.selection.contains(&token.line_index);
                let dim = if is_selected { 1.0 } else { dim_factor };
                draw_token(scene, root_transform * Affine::translate((token.pos.x as f64, token.pos.y as f64)), token, token.color, combined_opacity * dim);
            }
        }
    }

    fn update(&mut self, _dt: Duration) {}

    fn state_hash(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        use std::collections::hash_map::DefaultHasher;
        let mut s = DefaultHasher::new();
        
        let coeffs = self.transform.get().as_coeffs();
        for c in coeffs {
            c.to_bits().hash(&mut s);
        }
        
        self.font_size.get().to_bits().hash(&mut s);
        let val = self.code.get();
        val.text.hash(&mut s);
        if let Some(trans) = &val.transition {
            trans.progress.to_bits().hash(&mut s);
        }
        self.language.hash(&mut s);
        self.theme.hash(&mut s);
        self.font_family.hash(&mut s);
        self.opacity.get().to_bits().hash(&mut s);
        s.finish()
    }

    fn clone_node(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}

fn draw_token(scene: &mut Scene, transform: Affine, token: &Token, color: Color, opacity: f32) {
    if opacity <= 0.0 { return; }
    let mut c = color;
    // We need to be careful with alpha. 
    // Multiply the token's original alpha by the transition opacity.
    let alpha = (color.a as f32 * opacity).clamp(0.0, 255.0) as u8;
    c.a = alpha;
    let brush = Brush::Solid(c);
    for (glyph_transform, pb) in &token.glyphs {
        scene.fill(Fill::NonZero, transform * *glyph_transform, &brush, None, pb);
    }
}
