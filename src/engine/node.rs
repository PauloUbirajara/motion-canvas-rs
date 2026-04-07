use glam::Vec2;
use vello::peniko::{Brush, Color, Fill};
use vello::kurbo::{BezPath, PathEl, Shape, Stroke, Affine, Line as KurboLine, Circle as KurboCircle, RoundedRect as KurboRoundedRect};
use std::sync::Arc;
use vello::Scene;
use std::time::Duration;
use crate::engine::animation::{Signal, Node};

pub struct Circle {
    pub position: Signal<Vec2>,
    pub radius: Signal<f32>,
    pub fill: Color,
}

impl Node for Circle {
    fn render(&self, scene: &mut Scene) {
        let brush = Brush::Solid(self.fill);
        let pos = self.position.data.lock().unwrap().value;
        let radius = self.radius.data.lock().unwrap().value;
        
        scene.fill(
            Fill::NonZero,
            Affine::translate((pos.x as f64, pos.y as f64)),
            &brush,
            None,
            &KurboCircle::new((0.0, 0.0), radius as f64),
        );
    }

    fn update(&mut self, _dt: Duration) {}

    fn state_hash(&self) -> u64 {
        let pos = self.position.data.lock().unwrap().value;
        let radius = self.radius.data.lock().unwrap().value;
        let mut hash = 0u64;
        hash ^= pos.x.to_bits() as u64;
        hash ^= pos.y.to_bits() as u64;
        hash ^= radius.to_bits() as u64;
        hash ^= self.fill.r as u64;
        hash ^= (self.fill.g as u64) << 8;
        hash ^= (self.fill.b as u64) << 16;
        hash ^= (self.fill.a as u64) << 24;
        hash
    }
}

pub struct Rect {
    pub position: Signal<Vec2>,
    pub size: Signal<Vec2>,
    pub fill: Color,
    pub radius: f32, // border radius
}

impl Node for Rect {
    fn render(&self, scene: &mut Scene) {
        let brush = Brush::Solid(self.fill);
        let pos = self.position.data.lock().unwrap().value;
        let size = self.size.data.lock().unwrap().value;
        
        scene.fill(
            Fill::NonZero,
            Affine::translate((pos.x as f64, pos.y as f64)),
            &brush,
            None,
            &KurboRoundedRect::new(0.0, 0.0, size.x as f64, size.y as f64, self.radius as f64),
        );
    }

    fn update(&mut self, _dt: Duration) {}

    fn state_hash(&self) -> u64 {
        let pos = self.position.data.lock().unwrap().value;
        let size = self.size.data.lock().unwrap().value;
        let mut hash = 0u64;
        hash ^= pos.x.to_bits() as u64;
        hash ^= pos.y.to_bits() as u64;
        hash ^= size.x.to_bits() as u64;
        hash ^= size.y.to_bits() as u64;
        hash ^= self.radius.to_bits() as u64;
        hash ^= self.fill.r as u64;
        hash ^= (self.fill.g as u64) << 8;
        hash ^= (self.fill.b as u64) << 16;
        hash ^= (self.fill.a as u64) << 24;
        hash
    }
}

pub struct Line {
    pub start: Signal<Vec2>,
    pub end: Signal<Vec2>,
    pub stroke: Color,
    pub thickness: f32,
}

impl Node for Line {
    fn render(&self, scene: &mut Scene) {
        let brush = Brush::Solid(self.stroke);
        let start = self.start.data.lock().unwrap().value;
        let end = self.end.data.lock().unwrap().value;
        
        scene.stroke(
            &Stroke::new(self.thickness as f64),
            Affine::IDENTITY,
            &brush,
            None,
            &KurboLine::new((start.x as f64, start.y as f64), (end.x as f64, end.y as f64)),
        );
    }

    fn update(&mut self, _dt: Duration) {}

    fn state_hash(&self) -> u64 {
        let start = self.start.data.lock().unwrap().value;
        let end = self.end.data.lock().unwrap().value;
        let mut hash = 0u64;
        hash ^= start.x.to_bits() as u64;
        hash ^= start.y.to_bits() as u64;
        hash ^= end.x.to_bits() as u64;
        hash ^= end.y.to_bits() as u64;
        hash ^= self.thickness.to_bits() as u64;
        hash ^= self.stroke.r as u64;
        hash ^= (self.stroke.g as u64) << 8;
        hash ^= (self.stroke.b as u64) << 16;
        hash ^= (self.stroke.a as u64) << 24;
        hash
    }
}

pub struct PathData {
    pub path: BezPath,
    pub segments: Vec<(Vec2, f32)>,
    pub total_length: f32,
}

impl PathData {
    pub fn new(path: BezPath) -> Self {
        let mut segments = Vec::new();
        let mut total_length = 0.0;
        let mut last_point: Option<Vec2> = None;

        // In kurbo 0.11+, use free flatten function
        vello::kurbo::flatten(&path, 0.1, |el| {
            match el {
                PathEl::MoveTo(p) => {
                    let pt = Vec2::new(p.x as f32, p.y as f32);
                    segments.push((pt, 0.0));
                    last_point = Some(pt);
                }
                PathEl::LineTo(p) => {
                    if let Some(last) = last_point {
                        let pt = Vec2::new(p.x as f32, p.y as f32);
                        total_length += last.distance(pt);
                        segments.push((pt, total_length));
                        last_point = Some(pt);
                    }
                }
                _ => {} 
            }
        });

        Self {
            path,
            segments,
            total_length,
        }
    }

    pub fn sample(&self, t: f32) -> Vec2 {
        if self.segments.is_empty() {
            return Vec2::ZERO;
        }

        let target_len = t.clamp(0.0, 1.0) * self.total_length;
        
        let idx = match self.segments.binary_search_by(|&(_, len)| len.partial_cmp(&target_len).unwrap()) {
            Ok(i) => i,
            Err(i) => i,
        };

        if idx == 0 {
            return self.segments[0].0;
        }
        if idx >= self.segments.len() {
            return self.segments.last().unwrap().0;
        }

        let (p1, l1) = self.segments[idx - 1];
        let (p2, l2) = self.segments[idx];

        let segment_len = l2 - l1;
        if segment_len < 0.0001 {
            return p2;
        }

        let t_segment = (target_len - l1) / segment_len;
        p1.lerp(p2, t_segment)
    }
}

pub struct PathNode {
    pub data: Arc<PathData>,
    pub stroke: Color,
    pub thickness: f32,
}

impl PathNode {
    pub fn new(path: BezPath, stroke: Color, thickness: f32) -> Self {
        Self {
            data: Arc::new(PathData::new(path)),
            stroke,
            thickness,
        }
    }
}


impl Node for PathNode {
    fn render(&self, scene: &mut Scene) {
        let brush = Brush::Solid(self.stroke);
        scene.stroke(
            &Stroke::new(self.thickness as f64),
            Affine::IDENTITY,
            &brush,
            None,
            &self.data.path,
        );
    }

    fn update(&mut self, _dt: Duration) {}

    fn state_hash(&self) -> u64 {
        let mut hash = 0u64;
        hash ^= self.thickness.to_bits() as u64;
        hash ^= self.stroke.r as u64;
        hash ^= (self.stroke.g as u64) << 8;
        hash ^= (self.stroke.b as u64) << 16;
        hash ^= (self.stroke.a as u64) << 24;
        if let Some((p, _)) = self.data.segments.first() {
            hash ^= p.x.to_bits() as u64;
        }
        hash
    }
}

