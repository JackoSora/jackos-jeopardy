// Animation system with easing functions and state management
// Most animations removed to reduce warnings - only keep what's needed

// Keep minimal easing functions since they're used
pub fn ease_in_out(t: f32) -> f32 {
    if t < 0.5 {
        2.0 * t * t
    } else {
        -1.0 + (4.0 - 2.0 * t) * t
    }
}

pub fn ease_out_bounce(t: f32) -> f32 {
    if t < 1.0 / 2.75 {
        7.5625 * t * t
    } else if t < 2.0 / 2.75 {
        7.5625 * (t - 1.5 / 2.75) * (t - 1.5 / 2.75) + 0.75
    } else if t < 2.5 / 2.75 {
        7.5625 * (t - 2.25 / 2.75) * (t - 2.25 / 2.75) + 0.9375
    } else {
        7.5625 * (t - 2.625 / 2.75) * (t - 2.625 / 2.75) + 0.984375
    }
}
