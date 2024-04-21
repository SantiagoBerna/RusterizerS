pub fn u8_to_f32(v: u8) -> f32 {
    (v as f32) / 255.0
}

pub fn f32_to_u8(v: f32) -> u8 {
    let clamped = v.clamp(0.0, 1.0);
    (clamped * 255.0) as u8
}

pub fn u8_to_hex(a: u8, r: u8, g: u8, b: u8) -> u32 {
    let (a, r, g, b) = (a as u32, r as u32, g as u32, b as u32);
    (a << 24) | (r << 16) | (g << 8) | b
}

pub fn f32_to_hex(a: f32, r: f32, g: f32, b: f32) -> u32 {
    u8_to_hex(
        f32_to_u8(a),
        f32_to_u8(r),
        f32_to_u8(g),
        f32_to_u8(b)
    )
}

pub fn vec4_to_hex(v: glam::Vec4) -> u32 {
    f32_to_hex(v.w, v.x, v.y, v.z)
}

//ARGB -> RGBA
pub fn hex_to_f32(hex: u32) -> glam::Vec4 {
    glam::Vec4::new(
        u8_to_f32((hex) as u8),
        u8_to_f32((hex >> 8) as u8),
        u8_to_f32((hex >> 16) as u8),
        u8_to_f32((hex >> 24) as u8)
    )
}