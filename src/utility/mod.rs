
use glam::Vec2;

pub fn u8_to_f32(v: u8) -> f32 {
    (v as f32) / 255.0
}

pub fn edge_function(p: Vec2, a: Vec2, b: Vec2) -> f32 {
    let edge = b - a;
    let to_p = p - a;

    edge.x * to_p.y - edge.y * to_p.x
}

pub fn from_f32_rgb(r: f32, g: f32, b: f32) -> u32 {
    from_u8_rgb(
        (r * 255.0) as u8,
        (g * 255.0) as u8,
        (b * 255.0) as u8
    )
}

pub fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}