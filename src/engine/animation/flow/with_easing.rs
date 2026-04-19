use super::all::All;
use crate::engine::animation::base::Animation;

/// Runs multiple animations in parallel with a shared easing override.
///
/// Generally used via the `with_easing!` macro.
///
/// ### Example
/// ```rust
/// # use motion_canvas_rs::prelude::*;
/// # use std::time::Duration;
/// # let node = Rect::default().with_size(Vec2::new(100.0, 100.0)).with_fill(Color::RED);
/// # let target = Vec2::new(100.0, 100.0);
/// # let dur = Duration::from_secs(1);
/// with_easing!(
///     easings::back_out,
///     [
///         node.position.to(target, dur),
///         node.size.to(Vec2::new(200.0, 200.0), dur),
///     ]
/// );
/// ```
pub fn with_easing(
    easing: fn(f32) -> f32,
    animations: Vec<Box<dyn Animation>>,
) -> Box<dyn Animation> {
    let mut all = All::new(animations);
    all.set_easing(easing);
    Box::new(all)
}
