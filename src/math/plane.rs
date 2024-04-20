use glam::{Vec3, Vec4};

#[derive(Debug, Default, Clone, Copy)]
pub struct Plane {
    normal: Vec3,
    d: f32,
}

impl Plane {

    pub fn new(normal: Vec3, d: f32) -> Self {
        Plane { normal, d }
    }

    pub fn from_normal_point(normal: Vec3, point: Vec3) -> Self {
        Plane::new(normal, point.dot(normal))
    }

    pub fn signed_distance(&self, point: Vec3) -> f32 {
        let res = self.normal.dot(point) - self.d;
        res
    }

    pub fn intersect(&self, start: Vec3, end: Vec3) -> Option<f32> {

        let line_dir = end - start;

        let normal_dir_dot = self.normal.dot(line_dir);
        if normal_dir_dot == 0.0 { return None; }

        let t = (self.d - self.normal.dot(start)) / normal_dir_dot;

        if t >= 0.0 && t <= 1.0 { Some(t) }
        else { None }
    }
}

pub fn clip_polygon(polygon: &[Vec3], frustum: &[Plane]) -> Vec<Vec3> {

    let mut output_list = polygon.to_owned();

    for plane in frustum {
        let input_list = output_list.clone();
    
        output_list.clear();

        for i in 0..input_list.len() {

            let current_point = input_list[i];
            let next_point = input_list[(i+1) % input_list.len()];

            let current_inside = plane.signed_distance(current_point) >= 0.0;
           
            if current_inside {
                output_list.push(current_point);
            }

            if let Some(t) = plane.intersect(current_point, next_point) {
                output_list.push(super::lerp(current_point, next_point, t));
            }
        }
    }
    output_list
}