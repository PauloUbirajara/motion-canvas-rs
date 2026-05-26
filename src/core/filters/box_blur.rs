/// The default blur radius value (no blur).
pub const DEFAULT_BLUR: f32 = 0.0;

/// A trait that allows elements to implement direct box blurring.
pub trait Blur {
    /// Sets the blur radius of this element.
    fn with_blur(self, radius: f32) -> Self;
}

/// A separable box blur on a raw RGBA8 pixel buffer.
pub fn box_blur_rgba(pixels: &mut [u8], width: u32, height: u32, radius: u32) {
    if radius == 0 || pixels.is_empty() {
        return;
    }
    let w = width as usize;
    let h = height as usize;
    let r = radius as usize;

    let mut temp = vec![0u8; pixels.len()];

    // Pass 1: Horizontal blur (pixels -> temp)
    for y in 0..h {
        let row_offset = y * w * 4;
        let mut r_sum = 0u32;
        let mut g_sum = 0u32;
        let mut b_sum = 0u32;
        let mut a_sum = 0u32;

        let window_size = 2 * r + 1;
        for i in 0..window_size {
            let x = i.saturating_sub(r).min(w - 1);
            let idx = row_offset + x * 4;
            r_sum += pixels[idx] as u32;
            g_sum += pixels[idx + 1] as u32;
            b_sum += pixels[idx + 2] as u32;
            a_sum += pixels[idx + 3] as u32;
        }

        for x in 0..w {
            let out_idx = row_offset + x * 4;
            temp[out_idx] = (r_sum / window_size as u32) as u8;
            temp[out_idx + 1] = (g_sum / window_size as u32) as u8;
            temp[out_idx + 2] = (b_sum / window_size as u32) as u8;
            temp[out_idx + 3] = (a_sum / window_size as u32) as u8;

            let left_x = x.saturating_sub(r).min(w - 1);
            let right_x = (x + r + 1).min(w - 1);

            let left_idx = row_offset + left_x * 4;
            let right_idx = row_offset + right_x * 4;

            r_sum = r_sum + pixels[right_idx] as u32 - pixels[left_idx] as u32;
            g_sum = g_sum + pixels[right_idx + 1] as u32 - pixels[left_idx + 1] as u32;
            b_sum = b_sum + pixels[right_idx + 2] as u32 - pixels[left_idx + 2] as u32;
            a_sum = a_sum + pixels[right_idx + 3] as u32 - pixels[left_idx + 3] as u32;
        }
    }

    // Pass 2: Vertical blur (temp -> pixels)
    for x in 0..w {
        let mut r_sum = 0u32;
        let mut g_sum = 0u32;
        let mut b_sum = 0u32;
        let mut a_sum = 0u32;

        let window_size = 2 * r + 1;
        for i in 0..window_size {
            let y = i.saturating_sub(r).min(h - 1);
            let idx = (y * w + x) * 4;
            r_sum += temp[idx] as u32;
            g_sum += temp[idx + 1] as u32;
            b_sum += temp[idx + 2] as u32;
            a_sum += temp[idx + 3] as u32;
        }

        for y in 0..h {
            let out_idx = (y * w + x) * 4;
            pixels[out_idx] = (r_sum / window_size as u32) as u8;
            pixels[out_idx + 1] = (g_sum / window_size as u32) as u8;
            pixels[out_idx + 2] = (b_sum / window_size as u32) as u8;
            pixels[out_idx + 3] = (a_sum / window_size as u32) as u8;

            let top_y = y.saturating_sub(r).min(h - 1);
            let bottom_y = (y + r + 1).min(h - 1);

            let top_idx = (top_y * w + x) * 4;
            let bottom_idx = (bottom_y * w + x) * 4;

            r_sum = r_sum + temp[bottom_idx] as u32 - temp[top_idx] as u32;
            g_sum = g_sum + temp[bottom_idx + 1] as u32 - temp[top_idx + 1] as u32;
            b_sum = b_sum + temp[bottom_idx + 2] as u32 - temp[top_idx + 2] as u32;
            a_sum = a_sum + temp[bottom_idx + 3] as u32 - temp[top_idx + 3] as u32;
        }
    }
}

/// A wrapper to apply a box blur filter to any element drawing code.
///
/// Encapsulates the offscreen capture, separable CPU-side blur, and main canvas blending back into the scene.
#[cfg(feature = "runtime")]
pub fn apply_blur_filter<F>(
    scene: &mut vello::Scene,
    blur_radius: f32,
    combined_opacity: f32,
    render_func: F,
) where
    F: FnOnce(&mut vello::Scene, f32),
{
    if blur_radius < 0.5 {
        render_func(scene, combined_opacity);
        return;
    }

    let maybe_renderer =
        crate::core::scene::ACTIVE_OFFSCREEN_RENDERER.with(|cell| cell.borrow().clone());

    let Some(offscreen) = maybe_renderer else {
        render_func(scene, combined_opacity);
        return;
    };

    let width = offscreen.width();
    let height = offscreen.height();

    let mut sub_scene = vello::Scene::new();
    // Render contents onto offscreen canvas with full opacity
    render_func(&mut sub_scene, 1.0);

    // Fetch GPU-rendered texture back to raw CPU pixels
    let mut pixels = offscreen.render_to_rgba(&sub_scene);

    // Apply fast box blur
    box_blur_rgba(&mut pixels, width, height, blur_radius.round() as u32);

    let peniko_img = peniko::Image {
        data: peniko::Blob::new(std::sync::Arc::new(pixels)),
        alpha: 255,
        format: peniko::Format::Rgba8,
        width,
        height,
        extend: peniko::Extend::Pad,
    };

    // Draw blurred texture back to main canvas with element's target combined opacity
    if combined_opacity < 1.0 {
        scene.push_layer(
            peniko::Mix::Normal,
            combined_opacity,
            kurbo::Affine::IDENTITY,
            &kurbo::Rect::new(-10000.0, -10000.0, 10000.0, 10000.0),
        );
        scene.draw_image(&peniko_img, kurbo::Affine::IDENTITY);
        scene.pop_layer();
    } else {
        scene.draw_image(&peniko_img, kurbo::Affine::IDENTITY);
    }
}
