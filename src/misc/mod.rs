pub mod button;
pub mod direction;

pub fn lerp(start: f32, end: f32, t: f32) -> f32 {
    let lerp = start + (end - start) * t;
    lerp.clamp(start.min(end), start.max(end))
}

pub fn exp_decay(start: f32, end: f32, decay: f32, dt: f32) -> f32 {
    let lerp_speed = (-decay * dt).exp();
    lerp(end, start, lerp_speed)
}
