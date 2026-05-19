use crate::core::animation::tween::Tweenable;
use kurbo::Point;
use peniko::{Brush, Color, ColorStop, ColorStops, Gradient, GradientKind};

/// A representable and animatable paint property that can be either a solid color or a gradient.
///
/// Wraps `peniko::Color` and `peniko::Gradient` and implements `Tweenable` to enable
/// smooth color-to-gradient and gradient-to-gradient transitions.
#[derive(Clone, Debug, PartialEq)]
pub enum Paint {
    /// A solid single-color paint.
    Solid(Color),
    /// A gradient paint (linear, radial, sweep, etc.).
    Gradient(Gradient),
}

impl Paint {
    /// Converts this `Paint` into a standard Peniko `Brush`.
    pub fn to_brush(&self) -> Brush {
        match self {
            Paint::Solid(color) => Brush::Solid(*color),
            Paint::Gradient(grad) => Brush::Gradient(grad.clone()),
        }
    }

    /// Resolves this `Paint` to a `Brush` and scales its transparency by the given opacity factor.
    pub fn to_brush_with_opacity(&self, opacity: f32) -> Brush {
        match self {
            Paint::Solid(color) => {
                let mut c = *color;
                c.a = (color.a as f32 * opacity).clamp(0.0, 255.0) as u8;
                Brush::Solid(c)
            }
            Paint::Gradient(grad) => {
                let mut stops = Vec::new();
                for stop in grad.stops.iter() {
                    let mut c = stop.color;
                    c.a = (stop.color.a as f32 * opacity).clamp(0.0, 255.0) as u8;
                    stops.push(ColorStop {
                        offset: stop.offset,
                        color: c,
                    });
                }
                Brush::Gradient(Gradient {
                    kind: grad.kind.clone(),
                    extend: grad.extend,
                    stops: ColorStops::from(stops),
                })
            }
        }
    }
}

impl From<Color> for Paint {
    fn from(c: Color) -> Self {
        Paint::Solid(c)
    }
}

impl From<Gradient> for Paint {
    fn from(g: Gradient) -> Self {
        Paint::Gradient(g)
    }
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

fn interpolate_point(p1: Point, p2: Point, t: f32) -> Point {
    let t = t as f64;
    Point::new(p1.x + (p2.x - p1.x) * t, p1.y + (p2.y - p1.y) * t)
}

/// Sample a gradient's color at a given offset by interpolating between adjacent stops.
fn sample_color_at(stops: &[ColorStop], offset: f32) -> Color {
    if stops.is_empty() {
        return Color::TRANSPARENT;
    }
    if stops.len() == 1 || offset <= stops[0].offset {
        return stops[0].color;
    }
    let last = stops.len() - 1;
    if offset >= stops[last].offset {
        return stops[last].color;
    }
    for i in 1..stops.len() {
        if offset <= stops[i].offset {
            let range = stops[i].offset - stops[i - 1].offset;
            if range < 1e-6 {
                return stops[i].color;
            }
            let local_t = (offset - stops[i - 1].offset) / range;
            return Color::interpolate(&stops[i - 1].color, &stops[i].color, local_t);
        }
    }
    stops[last].color
}

fn interpolate_stops(stops1: &[ColorStop], stops2: &[ColorStop], t: f32) -> ColorStops {
    // Collect the union of all offsets from both stop arrays
    let mut offsets: Vec<f32> = stops1
        .iter()
        .map(|s| s.offset)
        .chain(stops2.iter().map(|s| s.offset))
        .collect();
    offsets.sort_by(|a, b| a.partial_cmp(b).unwrap());
    offsets.dedup_by(|a, b| (*a - *b).abs() < 1e-4);

    let mut result = Vec::with_capacity(offsets.len());
    for &offset in &offsets {
        let c1 = sample_color_at(stops1, offset);
        let c2 = sample_color_at(stops2, offset);
        result.push(ColorStop {
            offset,
            color: Color::interpolate(&c1, &c2, t),
        });
    }
    ColorStops::from(result)
}

fn promote_solid_to_gradient(color: Color, target: &Gradient) -> Gradient {
    let mut stops = Vec::new();
    for stop in target.stops.iter() {
        stops.push(ColorStop {
            offset: stop.offset,
            color,
        });
    }
    Gradient {
        kind: target.kind.clone(),
        extend: target.extend,
        stops: ColorStops::from(stops),
    }
}

impl Tweenable for Paint {
    fn interpolate(a: &Self, b: &Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        match (a, b) {
            (Paint::Solid(c1), Paint::Solid(c2)) => Paint::Solid(Color::interpolate(c1, c2, t)),
            (Paint::Gradient(g1), Paint::Gradient(g2)) => {
                let kind = match (&g1.kind, &g2.kind) {
                    (
                        GradientKind::Linear { start: s1, end: e1 },
                        GradientKind::Linear { start: s2, end: e2 },
                    ) => GradientKind::Linear {
                        start: interpolate_point(*s1, *s2, t),
                        end: interpolate_point(*e1, *e2, t),
                    },
                    (
                        GradientKind::Radial {
                            start_center: sc1,
                            start_radius: sr1,
                            end_center: ec1,
                            end_radius: er1,
                        },
                        GradientKind::Radial {
                            start_center: sc2,
                            start_radius: sr2,
                            end_center: ec2,
                            end_radius: er2,
                        },
                    ) => GradientKind::Radial {
                        start_center: interpolate_point(*sc1, *sc2, t),
                        start_radius: lerp(*sr1, *sr2, t),
                        end_center: interpolate_point(*ec1, *ec2, t),
                        end_radius: lerp(*er1, *er2, t),
                    },
                    (k1, k2) => {
                        if t < 0.5 {
                            k1.clone()
                        } else {
                            k2.clone()
                        }
                    }
                };

                let extend = if t < 0.5 { g1.extend } else { g2.extend };
                let stops = interpolate_stops(&g1.stops, &g2.stops, t);

                Paint::Gradient(Gradient {
                    kind,
                    extend,
                    stops,
                })
            }
            (Paint::Solid(c), Paint::Gradient(g)) => {
                let dummy_g = promote_solid_to_gradient(*c, g);
                Paint::interpolate(&Paint::Gradient(dummy_g), b, t)
            }
            (Paint::Gradient(g), Paint::Solid(c)) => {
                let dummy_g = promote_solid_to_gradient(*c, g);
                Paint::interpolate(a, &Paint::Gradient(dummy_g), t)
            }
        }
    }

    fn state_hash(&self) -> u64 {
        let mut h = crate::assets::hash::Hasher::new();
        match self {
            Paint::Solid(c) => {
                h.update_u64(0);
                h.update_u64(Color::state_hash(c));
            }
            Paint::Gradient(g) => {
                h.update_u64(1);
                match &g.kind {
                    GradientKind::Linear { start, end } => {
                        h.update_u64(0);
                        h.update_u64(crate::assets::hash::hash_f32(start.x as f32));
                        h.update_u64(crate::assets::hash::hash_f32(start.y as f32));
                        h.update_u64(crate::assets::hash::hash_f32(end.x as f32));
                        h.update_u64(crate::assets::hash::hash_f32(end.y as f32));
                    }
                    GradientKind::Radial {
                        start_center,
                        start_radius,
                        end_center,
                        end_radius,
                    } => {
                        h.update_u64(1);
                        h.update_u64(crate::assets::hash::hash_f32(start_center.x as f32));
                        h.update_u64(crate::assets::hash::hash_f32(start_center.y as f32));
                        h.update_u64(crate::assets::hash::hash_f32(*start_radius));
                        h.update_u64(crate::assets::hash::hash_f32(end_center.x as f32));
                        h.update_u64(crate::assets::hash::hash_f32(end_center.y as f32));
                        h.update_u64(crate::assets::hash::hash_f32(*end_radius));
                    }
                    _ => {
                        h.update_u64(2);
                    }
                }
                for stop in g.stops.iter() {
                    h.update_u64(crate::assets::hash::hash_f32(stop.offset));
                    h.update_u64(Color::state_hash(&stop.color));
                }
            }
        }
        h.finish()
    }
}
