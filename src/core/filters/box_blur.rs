/// The default blur radius value (no blur).
pub const DEFAULT_BLUR: f32 = 0.0;

/// A trait that allows elements to implement direct box blurring.
pub trait Blur {
    /// Sets the blur radius of this element.
    fn with_blur(self, radius: f32) -> Self;
}

use rayon::prelude::*;

/// A separable box blur on a raw RGBA8 pixel buffer.
pub fn box_blur_rgba(pixels: &mut [u8], width: u32, height: u32, radius: u32) {
    if radius == 0 || pixels.is_empty() {
        return;
    }
    let w = width as usize;
    let h = height as usize;
    let r = radius as usize;

    // 1. Horizontal blur (pixels -> temp)
    let mut temp = vec![0u8; pixels.len()];
    horizontal_blur_rgba(pixels, &mut temp, w, r);

    // 2. Transpose (temp -> temp_t)
    let mut temp_t = vec![0u8; pixels.len()];
    transpose(&temp, &mut temp_t, w, h);

    // 3. Horizontal blur on transposed image (temp_t -> pixels_t)
    let mut pixels_t = vec![0u8; pixels.len()];
    horizontal_blur_rgba(&temp_t, &mut pixels_t, h, r);

    // 4. Transpose back (pixels_t -> pixels)
    transpose(&pixels_t, pixels, h, w);
}

/// A horizontal box blur on a row-major RGBA8 pixel buffer.
fn horizontal_blur_rgba(src: &[u8], dst: &mut [u8], w: usize, r: usize) {
    dst.par_chunks_exact_mut(w * 4)
        .zip(src.par_chunks_exact(w * 4))
        .for_each(|(dst_row, src_row)| {
            let mut r_sum = 0u32;
            let mut g_sum = 0u32;
            let mut b_sum = 0u32;
            let mut a_sum = 0u32;

            let window_size = 2 * r + 1;
            for i in 0..window_size {
                let x = i.saturating_sub(r).min(w - 1);
                let idx = x * 4;
                r_sum += src_row[idx] as u32;
                g_sum += src_row[idx + 1] as u32;
                b_sum += src_row[idx + 2] as u32;
                a_sum += src_row[idx + 3] as u32;
            }

            for x in 0..w {
                let out_idx = x * 4;
                dst_row[out_idx] = (r_sum / window_size as u32) as u8;
                dst_row[out_idx + 1] = (g_sum / window_size as u32) as u8;
                dst_row[out_idx + 2] = (b_sum / window_size as u32) as u8;
                dst_row[out_idx + 3] = (a_sum / window_size as u32) as u8;

                let left_x = x.saturating_sub(r).min(w - 1);
                let right_x = (x + r + 1).min(w - 1);

                let left_idx = left_x * 4;
                let right_idx = right_x * 4;

                r_sum = r_sum + src_row[right_idx] as u32 - src_row[left_idx] as u32;
                g_sum = g_sum + src_row[right_idx + 1] as u32 - src_row[left_idx + 1] as u32;
                b_sum = b_sum + src_row[right_idx + 2] as u32 - src_row[left_idx + 2] as u32;
                a_sum = a_sum + src_row[right_idx + 3] as u32 - src_row[left_idx + 3] as u32;
            }
        });
}

/// Performs a parallel 2D transpose of a row-major RGBA8 buffer.
fn transpose(src: &[u8], dst: &mut [u8], w: usize, h: usize) {
    dst.par_chunks_exact_mut(h * 4)
        .enumerate()
        .for_each(|(x, col_dst)| {
            for y in 0..h {
                let src_idx = (y * w + x) * 4;
                let dst_idx = y * 4;
                col_dst[dst_idx..dst_idx + 4].copy_from_slice(&src[src_idx..src_idx + 4]);
            }
        });
}

/// Applies a box blur filter to an element's drawing function.
///
/// Captures the drawing on an offscreen texture, applies CPU-side box blur,
/// and draws the result back to the main scene.
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
    // Render contents offscreen.
    render_func(&mut sub_scene, 1.0);

    // Retrieve pixels from GPU.
    let mut pixels = offscreen.render_to_rgba(&sub_scene);

    // Apply box blur.
    box_blur_rgba(&mut pixels, width, height, blur_radius.round() as u32);

    let peniko_img = peniko::Image {
        data: peniko::Blob::new(std::sync::Arc::new(pixels)),
        alpha: 255,
        format: peniko::Format::Rgba8,
        width,
        height,
        extend: peniko::Extend::Pad,
    };

    // Draw blurred texture to main canvas.
    if combined_opacity < 1.0 {
        scene.push_layer(
            peniko::Mix::Normal,
            combined_opacity,
            kurbo::Affine::IDENTITY,
            &kurbo::Rect::new(-10000.0, -10000.0, 10000.0, 10000.0),
        );
        scene.draw_image(&peniko_img, kurbo::Affine::IDENTITY);
        scene.pop_layer();
        return;
    }
    scene.draw_image(&peniko_img, kurbo::Affine::IDENTITY);
}
