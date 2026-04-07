use vello::kurbo::{BezPath, Affine, Stroke};
use vello::peniko::{Brush, Color};
use vello::Scene;
use glam::Vec2;
use std::sync::Arc;
use std::time::Duration;
use crate::engine::animation::{Node};

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
        vello::kurbo::flatten(&path, 0.1, |el| {
            match el {
                vello::kurbo::PathEl::MoveTo(p) => {
                    let pt = Vec2::new(p.x as f32, p.y as f32);
                    segments.push((pt, 0.0));
                    last_point = Some(pt);
                }
                vello::kurbo::PathEl::LineTo(p) => {
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
        Self { path, segments, total_length }
    }
    
    pub fn sample(&self, t: f32) -> Vec2 {
        if self.segments.is_empty() { return Vec2::ZERO; }
        let target_len = t.clamp(0.0, 1.0) * self.total_length;
        let idx = match self.segments.binary_search_by(|&(_, len)| len.partial_cmp(&target_len).unwrap()) {
            Ok(i) => i,
            Err(i) => i,
        };
        if idx == 0 { return self.segments[0].0; }
        if idx >= self.segments.len() { return self.segments.last().unwrap().0; }
        let (p1, l1) = self.segments[idx - 1];
        let (p2, l2) = self.segments[idx];
        let segment_len = l2 - l1;
        if segment_len < 0.0001 { return p2; }
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
        scene.stroke(&Stroke::new(self.thickness as f64), Affine::IDENTITY, &brush, None, &self.data.path);
    }
    fn update(&mut self, _dt: Duration) {}
    fn state_hash(&self) -> u64 {
        let mut hash = 0u64;
        hash ^= self.thickness.to_bits() as u64;
        hash
    }
}
