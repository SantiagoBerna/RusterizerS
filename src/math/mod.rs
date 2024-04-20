use glam::Mat4;
use glam::Quat;
use glam::Vec2;
use glam::Vec3;
use glam::Vec4;

pub mod bounding_box;
use bounding_box::*;

pub mod plane;


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

pub fn cull_back_face(v1: Vec3, v2: Vec3, v3: Vec3) -> bool {

    let edge_1 = v2 - v1;
    let edge_2 = v3 - v1;
    
    //We check if its facing +z
    let normal = edge_1.cross(edge_2);
    normal.dot(-Vec3::Z) >= 0.0
}

pub fn homogenous_clip(a: Vec4, b: Vec4, plane: Vec4) -> Option<f32> {
    let line_vector = b - a;

    let div = plane.dot(line_vector);
    if div.abs() < std::f32::EPSILON { return None; }

    let t = -plane.dot(a) / div;
    if t > 0.0 && t < 1.0 { Some(t) }
    else { None }
}

pub fn clip_homogenous_triangle(vertices: &[Vec4; 3]) -> Vec<(Vec4, f32)> {

    let mut output_list: Vec<(Vec4, f32)> = vertices.iter()
        .enumerate()
        .map(|(i, v)| { (v.clone(), i as f32)})
        .collect();

    let clip_planes = [
        Vec4::new(1.0, 0.0, 0.0, 1.0),
        Vec4::new(-1.0, 0.0, 0.0, 1.0),
        Vec4::new(0.0, 1.0, 0.0, 1.0),
        Vec4::new(0.0, -1.0, 0.0, 1.0),
        Vec4::new(0.0, 0.0, 1.0, 0.0), //Near
        Vec4::new(0.0, 0.0, -1.0, 1.0), //Far
    ];

    for plane in clip_planes {

        let input_list = output_list.clone();
        output_list.clear();

        for i in 0..input_list.len() {

            let current_point = input_list[i];
            let next_point = input_list[(i+1) % input_list.len()];

            let current_inside = plane.dot(current_point.0) >= 0.0;
           
            if current_point.0.z < 0.0 {
                dbg!(current_point.0);
            };

            if current_inside {
                output_list.push(current_point);
            }

            if let Some(t) = homogenous_clip(current_point.0, next_point.0, plane) {

                let interpolated = lerp(current_point.0, next_point.0, t);
                let original_edge = current_point.1.floor();
                let hint = original_edge + t;

                output_list.push((interpolated, hint));
            }
        }
    }

    output_list
}

pub fn should_cull_triangle(v1: Vec4, v2: Vec4, v3: Vec4) -> bool {

    // cull tests against the 6 planes
    if v1.x > v1.w && v2.x > v2.w && v3.x > v3.w { return true; }
    if v1.x < -v1.w && v2.x < -v2.w && v3.x < -v3.w { return true; }

    if v1.y > v1.w && v2.y > v2.w && v3.y > v3.w { return true; }
    if v1.y < -v1.w && v2.y < -v2.w && v3.y < -v3.w { return true; }
    
    if v1.z > v1.w && v2.z > v2.w && v3.z > v3.w { return true; }
    if v1.z < 0.0 && v2.z < 0.0 && v3.z < 0.0 { return true; }

    false
}

pub fn barycentric_weights(point: Vec2, edge_1: Vec2, edge_2: Vec2, edge_3: Vec2) -> Option<Vec3> {

    let bary = Vec3::new(
        edge_function(point, edge_2, edge_3),
        edge_function(point, edge_3, edge_1),
        edge_function(point, edge_1, edge_2))
        / edge_function(edge_1, edge_2, edge_3
    );
    
    if bary.x >= 0.0 && bary.y >= 0.0 && bary.z >= 0.0 { Some(bary) }
    else { None }
}

pub fn edge_function(p: Vec2, a: Vec2, b: Vec2) -> f32 {
    let edge = b - a;
    let to_p = p - a;
    
    edge.x * to_p.y - edge.y * to_p.x
}

pub fn generate_triangle_bounding_box(v1: Vec2, v2: Vec2, v3: Vec2) -> BoundingBox {
    let v_max = v1.max(v2).max(v3).round();
    let v_min = v1.min(v2).min(v3).round();
    
    BoundingBox { start: v_min.as_uvec2(), end: v_max.as_uvec2() }
}

pub fn triangle_in_bounds(v1: Vec4, v2: Vec4, v3: Vec4) -> bool {

    let in_range = |v: Vec4| -> bool {
        v.x > -v.w && v.x < v.w &&
        v.y > -v.w && v.y < v.w &&
        v.z >  0.0 && v.z < v.w
    };

    in_range(v1) || in_range(v2) || in_range(v3)
}

pub fn lerp<T>(start: T, end: T, alpha: f32) -> T
where T: std::ops::Sub<Output = T> + std::ops::Mul<f32, Output = T> + std::ops::Add<Output = T> + Copy
{
    start + (end - start) * alpha
}