use glam::Mat4;
use glam::Quat;
use glam::Vec2;
use glam::Vec3;
use glam::Vec4;

pub mod bounding_box;
pub mod colour;
pub mod plane;

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

pub fn clip_homogenous_triangle(vertices: &[Vec4; 3]) -> Vec<(Vec4, Vec3)> {

    let mut output_list: Vec<(Vec4, Vec3)> = vertices.iter()
        .enumerate()
        .map(|(i, v)| {
            if i == 0 {
                (v.clone(), Vec3::X)
            } else if i == 1 {
                (v.clone(), Vec3::Y)
            } else {
                (v.clone(), Vec3::Z)
            }
        })
        .collect();

    let clip_planes = [
        Vec4::new(1.0, 0.0, 0.0, 1.0), //Left
        Vec4::new(-1.0, 0.0, 0.0, 1.0), //Right
        Vec4::new(0.0, 1.0, 0.0, 1.0), //Bottom
        Vec4::new(0.0, -1.0, 0.0, 1.0), //Top
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

            if current_inside {
                output_list.push(current_point);
            }

            if let Some(t) = homogenous_clip(current_point.0, next_point.0, plane) {

                let interpolated = lerp(current_point.0, next_point.0, t);
                let barycentric_coords =  lerp(current_point.1, next_point.1, t);
                output_list.push((interpolated, barycentric_coords));
            }
        }
    }

    output_list
}

pub fn barycentric_lerp<T>(weights: Vec3, v1: T, v2: T, v3: T) -> T 
where T: std::ops::Mul<f32, Output = T> + std::ops::Add<Output = T>
{
    (v1 * weights.x) + (v2 * weights.y) + (v3 * weights.z)
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

pub fn generate_triangle_bounding_box(v1: Vec2, v2: Vec2, v3: Vec2) -> bounding_box::BoundingBox {
    let v_max = v1.max(v2).max(v3).round();
    let v_min = v1.min(v2).min(v3).round();
    
    bounding_box::BoundingBox { start: v_min.as_uvec2(), end: v_max.as_uvec2() }
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