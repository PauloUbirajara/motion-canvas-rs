use crate::core::animation::Tweenable;

/// The composition mode for masking, mapping standard graphics stenciling
/// and Boolean operations to Porter-Duff blend layers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MaskMode {
    /// Mask using the alpha channel of the mask node. Only renders the source where the mask is visible.
    Alpha,
    /// Mask using the inverse alpha channel. Renders the source where the mask is NOT visible.
    Subtract,
    /// Renders both the source and the mask, but only their non-overlapping parts.
    Xor,

    /// Keep only the regions where both shapes overlap. Identical to Alpha masking.
    Intersect,
    /// Combines/merges both layers, making both shapes fully visible.
    Union,
    /// Keeps everything except the overlapping regions. Identical to Xor masking.
    Exclude,
    /// Synonym for Exclude/Xor masking.
    Difference,
}

impl Tweenable for MaskMode {
    fn interpolate(a: &Self, b: &Self, t: f32) -> Self {
        if t >= 1.0 {
            *b
        } else {
            *a
        }
    }
    fn state_hash(&self) -> u64 {
        *self as u64
    }
}

/// Applies a composition mask to the source node using the mask node.
#[cfg(feature = "runtime")]
pub fn apply_mask(
    scene: &mut vello::Scene,
    mode: MaskMode,
    combined_opacity: f32,
    combined_transform: kurbo::Affine,
    mask_render: impl FnOnce(&mut vello::Scene, kurbo::Affine),
    source_render: impl FnOnce(&mut vello::Scene, kurbo::Affine),
) {
    if combined_opacity <= 0.0 {
        return;
    }

    // 1. Isolated layer to prevent compositing blending from leaking to background
    scene.push_layer(
        peniko::Mix::Normal,
        combined_opacity,
        kurbo::Affine::IDENTITY,
        &kurbo::Rect::new(-10000.0, -10000.0, 10000.0, 10000.0),
    );

    // 2. Render mask node into isolated layer
    mask_render(scene, combined_transform);

    // 3. Map MaskMode to Porter-Duff Compose mode
    let compose_mode = match mode {
        MaskMode::Alpha | MaskMode::Intersect => peniko::Compose::SrcIn,
        MaskMode::Subtract => peniko::Compose::SrcOut,
        MaskMode::Xor | MaskMode::Exclude | MaskMode::Difference => peniko::Compose::Xor,
        MaskMode::Union => peniko::Compose::SrcOver,
    };

    // 4. Push layer with the blend/composite mode for source compositing
    scene.push_layer(
        peniko::BlendMode {
            mix: peniko::Mix::Normal,
            compose: compose_mode,
        },
        1.0,
        kurbo::Affine::IDENTITY,
        &kurbo::Rect::new(-10000.0, -10000.0, 10000.0, 10000.0),
    );

    // 5. Render source node into composition layer
    source_render(scene, combined_transform);

    // 6. Pop composite layer
    scene.pop_layer();

    // 7. Pop isolated layer
    scene.pop_layer();
}
