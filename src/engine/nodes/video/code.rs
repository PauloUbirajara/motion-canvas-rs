//! Code highlighting and animation module.
//!
//! Credits: The token-based animation logic and diffing approach is inspired by
//! [shiki-magic-move](https://github.com/shikijs/shiki-magic-move).

use crate::engine::animation::{Node, Signal, Tweenable};
use crate::engine::util::code_tokenizer::{draw_token, parse_selection, CodeValue};
use glam::Vec2;
use std::time::Duration;
use vello::kurbo::Affine;
use vello::peniko::Color;
use vello::Scene;

pub struct CodeNode {
    pub position: Signal<Vec2>,
    pub rotation: Signal<f32>,
    pub scale: Signal<Vec2>,
    pub code: Signal<CodeValue>,
    pub font_size: Signal<f32>,
    pub opacity: Signal<f32>,
    pub dim_opacity: Signal<f32>,
    pub language: String,
    pub theme: String,
    pub font_family: String,
}

impl Default for CodeNode {
    fn default() -> Self {
        let font_size = crate::engine::util::code_tokenizer::DEFAULT_FONT_SIZE;
        let language = crate::engine::util::code_tokenizer::DEFAULT_LANGUAGE.to_string();
        let theme = crate::engine::util::code_tokenizer::DEFAULT_THEME.to_string();
        let font_family = crate::engine::util::code_tokenizer::DEFAULT_FONT_FAMILY.to_string();

        let val = CodeValue::new("".to_string(), font_size, &language, &theme, &font_family);

        Self {
            position: Signal::new(Vec2::ZERO),
            rotation: Signal::new(0.0),
            scale: Signal::new(Vec2::ONE),
            code: Signal::new(val),
            font_size: Signal::new(font_size),
            opacity: Signal::new(crate::engine::util::code_tokenizer::DEFAULT_OPACITY),
            dim_opacity: Signal::new(crate::engine::util::code_tokenizer::DEFAULT_DIM_OPACITY),
            language,
            theme,
            font_family,
        }
    }
}

impl Clone for CodeNode {
    fn clone(&self) -> Self {
        Self {
            position: self.position.clone(),
            rotation: self.rotation.clone(),
            scale: self.scale.clone(),
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
        Self::default()
            .with_position(pos)
            .with_language(lang)
            .with_code(code)
    }

    pub fn with_position(mut self, position: Vec2) -> Self {
        self.position = Signal::new(position);
        self
    }

    pub fn with_rotation(mut self, angle: f32) -> Self {
        self.rotation = Signal::new(angle);
        self
    }

    pub fn with_scale(mut self, scale: f32) -> Self {
        self.scale = Signal::new(Vec2::splat(scale));
        self
    }

    pub fn with_scale_xy(mut self, scale: Vec2) -> Self {
        self.scale = Signal::new(scale);
        self
    }

    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = Signal::new(opacity);
        self
    }

    pub fn with_code(mut self, code: &str) -> Self {
        let val = CodeValue::new(
            code.to_string(),
            self.font_size.get(),
            &self.language,
            &self.theme,
            &self.font_family,
        );
        self.code = Signal::new(val);
        self
    }

    pub fn with_language(mut self, lang: &str) -> Self {
        self.language = lang.to_string();
        // Re-tokenize if code exists
        let current_text = self.code.get().text;
        let val = CodeValue::new(
            current_text,
            self.font_size.get(),
            &self.language,
            &self.theme,
            &self.font_family,
        );
        self.code = Signal::new(val);
        self
    }

    pub fn with_theme(mut self, theme: &str) -> Self {
        self.theme = theme.to_string();
        let current_text = self.code.get().text;
        let val = CodeValue::new(
            current_text,
            self.font_size.get(),
            &self.language,
            &self.theme,
            &self.font_family,
        );
        self.code = Signal::new(val);
        self
    }

    pub fn with_font(mut self, font: &str) -> Self {
        self.font_family = font.to_string();
        let current_text = self.code.get().text;
        let val = CodeValue::new(
            current_text,
            self.font_size.get(),
            &self.language,
            &self.theme,
            &self.font_family,
        );
        self.code = Signal::new(val);
        self
    }

    pub fn with_font_size(mut self, size: f32) -> Self {
        self.font_size = Signal::new(size);
        // Re-tokenize current code with new size to avoid "spazzing"
        let current_text = self.code.get().text;
        let mut val = CodeValue::new(
            current_text,
            size,
            &self.language,
            &self.theme,
            &self.font_family,
        );
        val.selection = self.code.get().selection;
        self.code = Signal::new(val);
        self
    }

    pub fn with_dim_opacity(mut self, dim: f32) -> Self {
        self.dim_opacity = Signal::new(dim);
        self
    }

    pub fn edit(
        &self,
        code: &str,
        duration: Duration,
    ) -> crate::engine::animation::SignalTween<CodeValue> {
        let code = code.to_string();
        let font_size = self.font_size.get();
        let lang = self.language.clone();
        let theme = self.theme.clone();
        let font = self.font_family.clone();
        self.code.to_lazy(
            move |current| {
                let mut next_value = CodeValue::new(code.clone(), font_size, &lang, &theme, &font);
                next_value.selection = current.selection.clone();
                next_value
            },
            duration,
        )
    }

    pub fn append(
        &self,
        text: &str,
        duration: Duration,
    ) -> crate::engine::animation::SignalTween<CodeValue> {
        let text = text.to_string();
        let font_size = self.font_size.get();
        let lang = self.language.clone();
        let theme = self.theme.clone();
        let font = self.font_family.clone();
        self.code.to_lazy(
            move |current| {
                let next_text = format!("{}{}", current.text, text);
                let mut next_val = CodeValue::new(next_text, font_size, &lang, &theme, &font);
                next_val.selection = current.selection.clone();
                next_val
            },
            duration,
        )
    }

    pub fn prepend(
        &self,
        text: &str,
        duration: Duration,
    ) -> crate::engine::animation::SignalTween<CodeValue> {
        let text = text.to_string();
        let font_size = self.font_size.get();
        let lang = self.language.clone();
        let theme = self.theme.clone();
        let font = self.font_family.clone();
        self.code.to_lazy(
            move |current| {
                let next_text = format!("{}{}", text, current.text);
                let mut next_val = CodeValue::new(next_text, font_size, &lang, &theme, &font);
                next_val.selection = current.selection.clone();
                next_val
            },
            duration,
        )
    }

    pub fn select_lines(
        &self,
        lines: Vec<usize>,
        duration: Duration,
    ) -> crate::engine::animation::SignalTween<CodeValue> {
        self.code.to_lazy(
            move |current| {
                let mut next_value = current.clone();
                next_value.transition = None; // Reset transition for the target
                next_value.selection = lines.clone();
                next_value
            },
            duration,
        )
    }

    /// Select lines using a printer-style selection string (e.g., "1-3, 5").
    /// Uses 1-based indexing for user convenience.
    pub fn select_string(
        &self,
        selection: &str,
        duration: Duration,
    ) -> crate::engine::animation::SignalTween<CodeValue> {
        let lines = parse_selection(selection);
        self.select_lines(lines, duration)
    }
}

impl Node for CodeNode {
    fn render(&self, scene: &mut Scene, parent_transform: Affine, parent_opacity: f32) {
        let code_val = self.code.get();
        let opacity = self.opacity.get();

        let pos = self.position.get();
        let rot = self.rotation.get();
        let sc = self.scale.get();

        let local_transform = Affine::translate((pos.x as f64, pos.y as f64))
            * Affine::rotate(rot as f64)
            * Affine::scale_non_uniform(sc.x as f64, sc.y as f64);

        let root_transform = parent_transform * local_transform;
        let combined_opacity = parent_opacity * opacity;

        let dim_factor = self.dim_opacity.get();

        let trans = match &code_val.transition {
            Some(t) => t,
            None => {
                // Static render
                let has_selection = !code_val.selection.is_empty();
                for token in &code_val.tokens {
                    let is_selected =
                        !has_selection || code_val.selection.contains(&token.line_index);
                    let dim = if is_selected { 1.0 } else { dim_factor };
                    draw_token(
                        scene,
                        root_transform
                            * Affine::translate((token.pos.x as f64, token.pos.y as f64)),
                        token,
                        token.color,
                        combined_opacity * dim,
                    );
                }
                return;
            }
        };

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

            let from_is_dimmed = !trans.from_selection.is_empty()
                && !trans.from_selection.contains(&from.line_index);
            let to_is_dimmed =
                !trans.to_selection.is_empty() && !trans.to_selection.contains(&to.line_index);

            let from_dim = if from_is_dimmed { dim_factor } else { 1.0 };
            let to_dim = if to_is_dimmed { dim_factor } else { 1.0 };
            let current_dim = from_dim + (to_dim - from_dim) * p;

            draw_token(
                scene,
                root_transform
                    * Affine::translate((current_pos.x as f64, current_pos.y as f64))
                    * Affine::scale(scale as f64),
                to,
                current_color,
                combined_opacity * current_dim,
            );

            matched_from[from_idx] = true;
            matched_to[to_idx] = true;
        }

        // Draw unmatched from-tokens (vanishing)
        for (i, matched) in matched_from.iter().enumerate() {
            if !*matched {
                let from = &trans.from_tokens[i];
                let is_dimmed = !trans.from_selection.is_empty()
                    && !trans.from_selection.contains(&from.line_index);
                let dim = if is_dimmed { dim_factor } else { 1.0 };
                draw_token(
                    scene,
                    root_transform * Affine::translate((from.pos.x as f64, from.pos.y as f64)),
                    from,
                    from.color,
                    combined_opacity * dim * (1.0 - p),
                );
            }
        }

        // Draw unmatched to-tokens (appearing)
        for (i, matched) in matched_to.iter().enumerate() {
            if !*matched {
                let to = &trans.to_tokens[i];
                let is_dimmed =
                    !trans.to_selection.is_empty() && !trans.to_selection.contains(&to.line_index);
                let dim = if is_dimmed { dim_factor } else { 1.0 };
                draw_token(
                    scene,
                    root_transform * Affine::translate((to.pos.x as f64, to.pos.y as f64)),
                    to,
                    to.color,
                    combined_opacity * dim * p,
                );
            }
        }
    }

    fn update(&mut self, _dt: Duration) {}

    fn state_hash(&self) -> u64 {
        use crate::engine::util::hash::Hasher;
        let mut h = Hasher::new();

        h.update_u64(self.position.state_hash());
        h.update_u64(self.rotation.state_hash());
        h.update_u64(self.scale.state_hash());
        h.update_u64(self.font_size.state_hash());
        h.update_u64(self.code.state_hash());
        h.update_u64(self.opacity.state_hash());
        h.update_u64(self.dim_opacity.state_hash());

        h.update_bytes(self.language.as_bytes());
        h.update_bytes(self.theme.as_bytes());
        h.update_bytes(self.font_family.as_bytes());

        h.finish()
    }

    fn clone_node(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }

    fn reset(&mut self) {
        self.position.reset();
        self.rotation.reset();
        self.scale.reset();
        self.code.reset();
        self.font_size.reset();
        self.opacity.reset();
        self.dim_opacity.reset();
    }
}
