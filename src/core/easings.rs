//! Easing functions library.
//!
//! Easing functions specify the rate of change of a parameter over time.
//! Most animations in `motion-canvas-rs` default to `cubic_in_out`.
//!
//! Credits: The easing functions are based on the equations from [easings.net](https://easings.net/).

use std::f32::consts::PI;

/// Linear interpolation (no easing).
pub fn linear(t: f32) -> f32 {
    t
}

// --- Quad ---

/// Quadratic ease-in.
pub fn quad_in(t: f32) -> f32 {
    t * t
}

/// Quadratic ease-out.
pub fn quad_out(t: f32) -> f32 {
    t * (2.0 - t)
}

/// Quadratic ease-in-out.
pub fn quad_in_out(t: f32) -> f32 {
    if t < 0.5 {
        return 2.0 * t * t;
    }
    -1.0 + (4.0 - 2.0 * t) * t
}

// --- Cubic ---

/// Cubic ease-in.
pub fn cubic_in(t: f32) -> f32 {
    t * t * t
}

/// Cubic ease-out.
pub fn cubic_out(t: f32) -> f32 {
    let t = t - 1.0;
    t * t * t + 1.0
}

/// Cubic ease-in-out (The default easing for most animations).
pub fn cubic_in_out(t: f32) -> f32 {
    if t < 0.5 {
        return 4.0 * t * t * t;
    }
    let t = 2.0 * t - 2.0;
    0.5 * t * t * t + 1.0
}

// --- Sine ---

/// Sinusoidal ease-in.
pub fn sine_in(t: f32) -> f32 {
    1.0 - ((t * PI) / 2.0).cos()
}

/// Sinusoidal ease-out.
pub fn sine_out(t: f32) -> f32 {
    ((t * PI) / 2.0).sin()
}

/// Sinusoidal ease-in-out.
pub fn sine_in_out(t: f32) -> f32 {
    -0.5 * ((PI * t).cos() - 1.0)
}

// --- Quart ---

/// Quartic ease-in.
pub fn quart_in(t: f32) -> f32 {
    t * t * t * t
}

/// Quartic ease-out.
pub fn quart_out(t: f32) -> f32 {
    1.0 - (t - 1.0).powi(4)
}

/// Quartic ease-in-out.
pub fn quart_in_out(t: f32) -> f32 {
    if t < 0.5 {
        return 8.0 * t * t * t * t;
    }
    1.0 - 8.0 * (t - 1.0).powi(4)
}

// --- Quint ---

/// Quintic ease-in.
pub fn quint_in(t: f32) -> f32 {
    t * t * t * t * t
}

/// Quintic ease-out.
pub fn quint_out(t: f32) -> f32 {
    1.0 + (t - 1.0).powi(5)
}

/// Quintic ease-in-out.
pub fn quint_in_out(t: f32) -> f32 {
    if t < 0.5 {
        return 16.0 * t * t * t * t * t;
    }
    1.0 + 16.0 * (t - 1.0).powi(5)
}

// --- Expo ---

/// Exponential ease-in.
pub fn expo_in(t: f32) -> f32 {
    if t == 0.0 {
        return 0.0;
    }
    2.0f32.powf(10.0 * t - 10.0)
}

/// Exponential ease-out.
pub fn expo_out(t: f32) -> f32 {
    if t == 1.0 {
        return 1.0;
    }
    1.0 - 2.0f32.powf(-10.0 * t)
}

/// Exponential ease-in-out.
pub fn expo_in_out(t: f32) -> f32 {
    if t == 0.0 {
        return 0.0;
    }
    if t == 1.0 {
        return 1.0;
    }
    if t < 0.5 {
        return 2.0f32.powf(10.0 * (2.0 * t) - 10.0) / 2.0;
    }
    (2.0 - 2.0f32.powf(-10.0 * (2.0 * t - 1.0))) / 2.0
}

// --- Circ ---

/// Circular ease-in.
pub fn circ_in(t: f32) -> f32 {
    1.0 - (1.0 - t * t).sqrt()
}

/// Circular ease-out.
pub fn circ_out(t: f32) -> f32 {
    (1.0 - (t - 1.0).powi(2)).sqrt()
}

/// Circular ease-in-out.
pub fn circ_in_out(t: f32) -> f32 {
    if t < 0.5 {
        return (1.0 - (1.0 - (2.0 * t).powi(2)).sqrt()) / 2.0;
    }
    ((1.0 - (2.0 * t - 2.0).powi(2)).sqrt() + 1.0) / 2.0
}

// --- Back ---

/// Back ease-in.
pub fn back_in(t: f32) -> f32 {
    let s = 1.70158;
    t * t * ((s + 1.0) * t - s)
}

/// Back ease-out.
pub fn back_out(t: f32) -> f32 {
    let s = 1.70158;
    let t = t - 1.0;
    t * t * ((s + 1.0) * t + s) + 1.0
}

/// Back ease-in-out.
pub fn back_in_out(t: f32) -> f32 {
    let s = 1.70158 * 1.525;
    if t < 0.5 {
        return ((2.0 * t).powi(2) * ((s + 1.0) * 2.0 * t - s)) / 2.0;
    }
    ((2.0 * t - 2.0).powi(2) * ((s + 1.0) * (t * 2.0 - 2.0) + s) + 2.0) / 2.0
}

// --- Elastic ---

/// Elastic ease-in.
pub fn elastic_in(t: f32) -> f32 {
    if t == 0.0 {
        return 0.0;
    }
    if t == 1.0 {
        return 1.0;
    }
    -(2.0f32.powf(10.0 * (t - 1.0)) * ((t - 1.1) * 5.0 * PI).sin())
}

/// Elastic ease-out.
pub fn elastic_out(t: f32) -> f32 {
    if t == 0.0 {
        return 0.0;
    }
    if t == 1.0 {
        return 1.0;
    }
    2.0f32.powf(-10.0 * t) * ((t - 0.1) * 5.0 * PI).sin() + 1.0
}

/// Elastic ease-in-out.
pub fn elastic_in_out(t: f32) -> f32 {
    if t == 0.0 {
        return 0.0;
    }
    if t == 1.0 {
        return 1.0;
    }
    if t < 0.5 {
        return -(2.0f32.powf(10.0 * (2.0 * t - 1.0)) * ((2.0 * t - 1.1) * 5.0 * PI).sin()) / 2.0;
    }
    (2.0f32.powf(-10.0 * (2.0 * t - 1.0)) * ((2.0 * t - 1.1) * 5.0 * PI).sin()) / 2.0 + 1.0
}

// --- Bounce ---

/// Bounce ease-out.
pub fn bounce_out(mut t: f32) -> f32 {
    let n1 = 7.5625;
    let d1 = 2.75;

    if t < 1.0 / d1 {
        return n1 * t * t;
    }

    if t < 2.0 / d1 {
        t -= 1.5 / d1;
        return n1 * t * t + 0.75;
    }

    if t < 2.5 / d1 {
        t -= 2.25 / d1;
        return n1 * t * t + 0.9375;
    }

    t -= 2.625 / d1;
    n1 * t * t + 0.984375
}

/// Bounce ease-in.
pub fn bounce_in(t: f32) -> f32 {
    1.0 - bounce_out(1.0 - t)
}

/// Bounce ease-in-out.
pub fn bounce_in_out(t: f32) -> f32 {
    if t < 0.5 {
        return bounce_in(t * 2.0) * 0.5;
    }
    bounce_out(t * 2.0 - 1.0) * 0.5 + 0.5
}
