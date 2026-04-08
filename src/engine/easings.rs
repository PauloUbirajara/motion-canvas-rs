use std::f32::consts::PI;

pub fn linear(t: f32) -> f32 {
    t
}

pub fn quad_in(t: f32) -> f32 {
    t * t
}

pub fn quad_out(t: f32) -> f32 {
    t * (2.0 - t)
}

pub fn quad_in_out(t: f32) -> f32 {
    if t < 0.5 {
        2.0 * t * t
    } else {
        -1.0 + (4.0 - 2.0 * t) * t
    }
}

pub fn cubic_in(t: f32) -> f32 {
    t * t * t
}

pub fn cubic_out(t: f32) -> f32 {
    let t = t - 1.0;
    t * t * t + 1.0
}

pub fn cubic_in_out(t: f32) -> f32 {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        let t = 2.0 * t - 2.0;
        0.5 * t * t * t + 1.0
    }
}

pub fn sine_in(t: f32) -> f32 {
    1.0 - ((t * PI) / 2.0).cos()
}

pub fn sine_out(t: f32) -> f32 {
    ((t * PI) / 2.0).sin()
}

pub fn sine_in_out(t: f32) -> f32 {
    -0.5 * ((PI * t).cos() - 1.0)
}

pub fn elastic_in(t: f32) -> f32 {
    if t == 0.0 {
        0.0
    } else if t == 1.0 {
        1.0
    } else {
        -(2.0f32.powf(10.0 * (t - 1.0)) * ((t - 1.1) * 5.0 * PI).sin())
    }
}

pub fn elastic_out(t: f32) -> f32 {
    if t == 0.0 {
        0.0
    } else if t == 1.0 {
        1.0
    } else {
        2.0f32.powf(-10.0 * t) * ((t - 0.1) * 5.0 * PI).sin() + 1.0
    }
}


