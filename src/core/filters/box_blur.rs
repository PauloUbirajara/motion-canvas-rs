/// The default blur radius value (no blur).
pub const DEFAULT_BLUR: f32 = 0.0;

/// A trait that allows elements to implement direct box blurring.
pub trait Blur {
    /// Sets the blur radius of this element.
    fn with_blur(self, radius: f32) -> Self;
}

use rayon::prelude::*;

/// A separable box blur on a raw RGBA8 pixel buffer.
///
/// Operates in f32 high-precision space internally and supports continuous
/// fractional radius filtering to prevent stepping artifacts during animations.
pub fn box_blur_rgba(pixels: &mut [u8], width: u32, height: u32, radius: f32) {
    if radius <= 0.0 || pixels.is_empty() {
        return;
    }
    let w = width as usize;
    let h = height as usize;
    let r_floor = radius.floor() as usize;
    let fract = radius - r_floor as f32;
    let window_size = 2.0 * radius + 1.0;

    // Convert pixels to f32 to retain high precision and eliminate u8 rounding/truncation banding.
    let mut buffer: Vec<f32> = pixels.iter().map(|&p| p as f32).collect();

    // 1. Horizontal blur (buffer -> temp)
    let mut temp = vec![0.0f32; buffer.len()];
    horizontal_blur_rgba(&buffer, &mut temp, w, r_floor, fract, window_size);

    // 2. Transpose (temp -> temp_t)
    let mut temp_t = vec![0.0f32; buffer.len()];
    transpose(&temp, &mut temp_t, w, h);

    // 3. Horizontal blur on transposed image (temp_t -> pixels_t)
    let mut pixels_t = vec![0.0f32; buffer.len()];
    horizontal_blur_rgba(&temp_t, &mut pixels_t, h, r_floor, fract, window_size);

    // 4. Transpose back (pixels_t -> buffer)
    transpose(&pixels_t, &mut buffer, h, w);

    // 5. Convert back to u8
    for (p, &b) in pixels.iter_mut().zip(buffer.iter()) {
        *p = b.round().clamp(0.0, 255.0) as u8;
    }
}

/// A horizontal box blur on a row-major f32 RGBA pixel buffer.
/// Supports fractional boundary weights for continuous sub-pixel radius.
fn horizontal_blur_rgba(
    src: &[f32],
    dst: &mut [f32],
    w: usize,
    r_floor: usize,
    fract: f32,
    window_size: f32,
) {
    dst.par_chunks_exact_mut(w * 4)
        .zip(src.par_chunks_exact(w * 4))
        .for_each(|(dst_row, src_row)| {
            let mut r_sum = 0.0f32;
            let mut g_sum = 0.0f32;
            let mut b_sum = 0.0f32;
            let mut a_sum = 0.0f32;

            // Sum the core fully-weighted elements in the sliding window.
            for i in -(r_floor as isize)..=(r_floor as isize) {
                let px = i.max(0).min(w as isize - 1) as usize;
                let idx = px * 4;
                r_sum += src_row[idx];
                g_sum += src_row[idx + 1];
                b_sum += src_row[idx + 2];
                a_sum += src_row[idx + 3];
            }

            // Factor in the partially-weighted edge elements.
            if fract > 0.0 {
                let left_px = (-(r_floor as isize) - 1).max(0) as usize;
                let right_px = (r_floor as isize + 1).min(w as isize - 1) as usize;
                let l_idx = left_px * 4;
                let r_idx = right_px * 4;

                r_sum += (src_row[l_idx] + src_row[r_idx]) * fract;
                g_sum += (src_row[l_idx + 1] + src_row[r_idx + 1]) * fract;
                b_sum += (src_row[l_idx + 2] + src_row[r_idx + 2]) * fract;
                a_sum += (src_row[l_idx + 3] + src_row[r_idx + 3]) * fract;
            }

            for x in 0..w {
                let out_idx = x * 4;
                dst_row[out_idx] = r_sum / window_size;
                dst_row[out_idx + 1] = g_sum / window_size;
                dst_row[out_idx + 2] = b_sum / window_size;
                dst_row[out_idx + 3] = a_sum / window_size;

                let left_outer = (x as isize - r_floor as isize - 1)
                    .max(0)
                    .min(w as isize - 1) as usize
                    * 4;
                let left_inner =
                    (x as isize - r_floor as isize).max(0).min(w as isize - 1) as usize * 4;
                let right_inner = (x as isize + r_floor as isize + 1)
                    .max(0)
                    .min(w as isize - 1) as usize
                    * 4;
                let right_outer = (x as isize + r_floor as isize + 2)
                    .max(0)
                    .min(w as isize - 1) as usize
                    * 4;

                let one_minus_fract = 1.0 - fract;

                r_sum += fract * (src_row[right_outer] - src_row[left_outer])
                    + one_minus_fract * (src_row[right_inner] - src_row[left_inner]);
                g_sum += fract * (src_row[right_outer + 1] - src_row[left_outer + 1])
                    + one_minus_fract * (src_row[right_inner + 1] - src_row[left_inner + 1]);
                b_sum += fract * (src_row[right_outer + 2] - src_row[left_outer + 2])
                    + one_minus_fract * (src_row[right_inner + 2] - src_row[left_inner + 2]);
                a_sum += fract * (src_row[right_outer + 3] - src_row[left_outer + 3])
                    + one_minus_fract * (src_row[right_inner + 3] - src_row[left_inner + 3]);
            }
        });
}

/// Performs a parallel 2D transpose of a row-major f32 RGBA buffer.
fn transpose(src: &[f32], dst: &mut [f32], w: usize, h: usize) {
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
    if blur_radius < 0.01 {
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
    box_blur_rgba(&mut pixels, width, height, blur_radius);

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
