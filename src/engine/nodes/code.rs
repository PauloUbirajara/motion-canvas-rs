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
}

#[derive(Clone, Debug, PartialEq)]
pub struct CodeTransition {
    pub from_tokens: Vec<Token>,
    pub to_tokens: Vec<Token>,
    pub progress: f32,
    pub matches: Vec<(usize, usize)>, // (from_idx, to_idx)
}

#[derive(Clone, Debug, PartialEq)]
pub struct CodeValue {
    pub text: String,
    pub tokens: Vec<Token>,
    pub transition: Option<CodeTransition>,
}

impl CodeValue {
    pub fn new(text: String, node: &CodeNode) -> Self {
        let tokens = node.tokenize(&text);
        Self {
            text,
            tokens,
            transition: None,
        }
    }
}

impl Default for CodeValue {
    fn default() -> Self {
        Self {
            text: String::new(),
            tokens: Vec::new(),
            transition: None,
        }
    }
}

impl Tweenable for CodeValue {
    fn interpolate(a: &Self, b: &Self, t: f32) -> Self {
        if a.text == b.text {
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
            }),
        }
    }
}

pub struct CodeNode {
    pub position: Signal<Vec2>,
    pub code: Signal<CodeValue>,
    pub font_size: Signal<f32>,
    pub language: String,
    pub theme: String,
    pub font_family: String,
}

impl Clone for CodeNode {
    fn clone(&self) -> Self {
        Self {
            position: self.position.clone(),
            code: self.code.clone(),
            font_size: self.font_size.clone(),
            language: self.language.clone(),
            theme: self.theme.clone(),
            font_family: self.font_family.clone(),
        }
    }
}

impl CodeNode {
    pub fn new(pos: Vec2, code: &str, lang: &str) -> Self {
        let node = Self {
            position: Signal::new(pos),
            code: Signal::new(CodeValue::default()),
            font_size: Signal::new(DEFAULT_FONT_SIZE),
            language: lang.to_string(),
            theme: DEFAULT_THEME.to_string(),
            font_family: DEFAULT_FONT_FAMILY.to_string(),
        };
        
        let initial_value = CodeValue::new(code.to_string(), &node);
        node.code.set(initial_value);
        node
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
        self
    }

    pub fn edit(&self, code: &str, duration: Duration) -> crate::engine::animation::SignalTween<CodeValue> {
        let next_value = CodeValue::new(code.to_string(), self);
        self.code.to(next_value, duration)
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

            for line in code.lines() {
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
    fn render(&self, scene: &mut Scene) {
        let pos = self.position.get();
        let code_val = self.code.get();
        let root_transform = Affine::translate((pos.x as f64, pos.y as f64));

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
                
                draw_token(scene, root_transform * Affine::translate((current_pos.x as f64, current_pos.y as f64)), to, current_color, 1.0);
                
                matched_from[from_idx] = true;
                matched_to[to_idx] = true;
            }
            
            // 2. Fade out deletions
            for (i, from) in trans.from_tokens.iter().enumerate() {
                if !matched_from[i] {
                    draw_token(scene, root_transform * Affine::translate((from.pos.x as f64, from.pos.y as f64)), from, from.color, 1.0 - p);
                }
            }
            
            // 3. Fade in additions
            for (i, to) in trans.to_tokens.iter().enumerate() {
                if !matched_to[i] {
                    draw_token(scene, root_transform * Affine::translate((to.pos.x as f64, to.pos.y as f64)), to, to.color, p);
                }
            }
        } else {
            // Static render
            for token in &code_val.tokens {
                draw_token(scene, root_transform * Affine::translate((token.pos.x as f64, token.pos.y as f64)), token, token.color, 1.0);
            }
        }
    }

    fn update(&mut self, _dt: Duration) {}

    fn state_hash(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        use std::collections::hash_map::DefaultHasher;
        let mut s = DefaultHasher::new();
        self.position.get().x.to_bits().hash(&mut s);
        self.position.get().y.to_bits().hash(&mut s);
        self.font_size.get().to_bits().hash(&mut s);
        let val = self.code.get();
        val.text.hash(&mut s);
        if let Some(trans) = &val.transition {
            trans.progress.to_bits().hash(&mut s);
        }
        self.language.hash(&mut s);
        self.theme.hash(&mut s);
        self.font_family.hash(&mut s);
        s.finish()
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
