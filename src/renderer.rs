
use crate::data::bounding_box::BoundingBox;
use crate::data::vertex_set::VertexSet;
use crate::data::vertex_set::VertexAttributes;
use crate::data::sampler::Sampler;

use crate::display::Surface;
use crate::utility::*;

use glam::BVec3;
use glam::BVec4;
use glam::Mat2;
use glam::Mat3;
use glam::Mat4;
use glam::Quat;
use glam::UVec2;
use glam::Vec4Swizzles;
use std::ops::Range;
use glam::Vec3Swizzles;
use glam::{Vec2, Vec3, Vec4};

pub enum TextureSlot {
    Diffuse,
    MaxTextureSlots
}
pub struct Renderer {
    pub view_matrix: Mat4,
    pub projection_matrix: Mat4,
    bound_vertex_data: Option<VertexSet>,
    texture_slots: Vec<Option<Sampler>>
}

impl Renderer {

    pub fn new() -> Self {
        Self { 
            view_matrix: Mat4::default(),
            projection_matrix: Mat4::default(),
            bound_vertex_data: None,
            texture_slots: vec![None; TextureSlot::MaxTextureSlots as usize] 
        }
    }

    pub fn bind_sampler(&mut self, index: TextureSlot, sampler: Option<Sampler>) {
        self.texture_slots[index as usize] = sampler;
    }

    pub fn bind_vertex_set(&mut self, vertices: Option<VertexSet>) {
        self.bound_vertex_data = vertices;
    }

    pub fn draw_buffer(&self, surface: &mut Surface, model_matrix: &Mat4, triangle_count: usize) -> Option<()> {

        let vertices = self.bound_vertex_data.as_ref()?;
        let index_data = vertices.get_indices();
        let mvp = self.projection_matrix * self.view_matrix * (*model_matrix);

        if let Some(indices) = index_data {
            for i in 0..triangle_count {
                self.draw_triangle(
                    surface, mvp,
                    [indices[i * 3], indices[i * 3 + 1], indices[i * 3 + 2]]
                );
            }
        } else {
            for i in 0..(triangle_count as u32) {
                self.draw_triangle(
                    surface, mvp,
                    [i * 3, (i * 3 + 1), (i * 3 + 2)]
                );
            }
        }

        Some(())
    }

    fn draw_triangle(&self, surface: &mut Surface, mvp: Mat4, indices: [u32; 3]) -> Option<()>
    {
        let vertices = self.bound_vertex_data.as_ref()?;
        let vertex_position = vertices.get_attribute(VertexAttributes::Position).unwrap();

        let v1 = Vec3::from_slice(&vertex_position[component_range(indices[0] as usize, 3)]);
        let v2 = Vec3::from_slice(&vertex_position[component_range(indices[1] as usize, 3)]);
        let v3 = Vec3::from_slice(&vertex_position[component_range(indices[2] as usize, 3)]);

        let proj_1 = mvp * v1.extend(1.0);
        let proj_2 = mvp * v2.extend(1.0);
        let proj_3 = mvp * v3.extend(1.0);

        let inv_w1 = 1.0 / proj_1.w;
        let inv_w2 = 1.0 / proj_2.w;
        let inv_w3 = 1.0 / proj_3.w;

        //Homogenous divide
        let ndc_1 = proj_1.xyz() * inv_w1;
        let ndc_2 = proj_2.xyz() * inv_w2;
        let ndc_3 = proj_3.xyz() * inv_w3;

        //Clipping occurs here
        let (clip_1, clip_2, clip_3) = clip_triangle(ndc_1, ndc_2, ndc_3)?;

        self.raster_triangle(surface, indices, clip_1.extend(inv_w1), clip_2.extend(inv_w2), clip_3.extend(inv_w3));

        Some(())
    }

    pub fn raster_triangle(&self, surface: &mut Surface, indices: [u32; 3], v1: Vec4, v2: Vec4, v3: Vec4) -> Option<()> {

        //Map to screen space
        let screen_half = Vec2::new(surface.get_width() as f32, surface.get_height() as f32) * 0.5;

        let screen_1 = to_screen_space(screen_half, v1);
        let screen_2 = to_screen_space(screen_half, v2);
        let screen_3 = to_screen_space(screen_half, v3);

        let triangle_bounds = generate_triangle_bounding_box(screen_1.xy(), screen_2.xy(), screen_3.xy());

        let screen_bounds = BoundingBox::new(
            UVec2::new(0, 0), 
            UVec2::new(surface.get_width() as u32, surface.get_height() as u32)
        );

        let bounds = triangle_bounds.intersect(&screen_bounds)?;

        let vertices = self.bound_vertex_data.as_ref()?;

        for j in bounds.start.y..bounds.end.y {
            for i in bounds.start.x..bounds.end.x {

                let pixel_index = j as usize * surface.get_width() + i as usize;
                let point = Vec2::new(i as f32 + 0.5, j as f32 + 0.5);

                let in_triangle = barycentric_weights(point, screen_1.xy(), screen_2.xy(), screen_3.xy());
                if let Some(weights) = in_triangle {

                    let depth = weights.dot(Vec3::new(v1.z, v2.z, v3.z));
                    let prev_depth = surface.get_depth(pixel_index);

                    if depth <= prev_depth {
                        surface.set_depth(pixel_index, depth);

                        let uv_data = vertices.get_attribute(VertexAttributes::TextureUV).unwrap();

                        let uv_1 = Vec2::from_slice(&uv_data[component_range(indices[0] as usize, 2)]) * v1.w;
                        let uv_2 = Vec2::from_slice(&uv_data[component_range(indices[1] as usize, 2)]) * v2.w;
                        let uv_3 = Vec2::from_slice(&uv_data[component_range(indices[2] as usize, 2)]) * v3.w;

                        let depth_correction = 1.0 / (v1.w * weights.x + v2.w * weights.y + v3.w * weights.z);
                        let uv_coordinate = (uv_1 * weights.x + uv_2 * weights.y + uv_3 * weights.z) * depth_correction;

                        let colour = self.texture_slots[0].as_ref().unwrap().sample(uv_coordinate.x, 1.0 - uv_coordinate.y);
                        surface.set_pixel_index(from_f32_rgb(colour.x, colour.y, colour.z), pixel_index);   
                    }
                }
            }
        }
        Some(())
    }
}


//Helpers
fn component_range(start: usize, stride: usize) -> Range<usize> {
    (start * stride)..(start * stride + stride)
}

fn generate_triangle_bounding_box(v1: Vec2, v2: Vec2, v3: Vec2) -> BoundingBox {
    let v_max = v1.max(v2).max(v3).round();
    let v_min = v1.min(v2).min(v3).round();

    BoundingBox { start: v_min.as_uvec2(), end: v_max.as_uvec2() }
}

fn edge_function(p: Vec2, a: Vec2, b: Vec2) -> f32 {
    let edge = b - a;
    let to_p = p - a;

    edge.x * to_p.y - edge.y * to_p.x
}

fn to_screen_space(half_extent: Vec2, normalized_with_inv_depth: Vec4) -> Vec4 {
    Vec4::new(
        normalized_with_inv_depth.x * half_extent.x + half_extent.x,
        normalized_with_inv_depth.y * -half_extent.y + half_extent.x,
        normalized_with_inv_depth.z,
        normalized_with_inv_depth.w
    )
}

fn barycentric_weights(point: Vec2, edge_1: Vec2, edge_2: Vec2, edge_3: Vec2) -> Option<Vec3> {
    let bary = Vec3::new(
        edge_function(point, edge_2, edge_3),
        edge_function(point, edge_3, edge_1),
        edge_function(point, edge_1, edge_2))
        / edge_function(edge_1, edge_2, edge_3);

    if bary.x >= 0.0 && bary.y >= 0.0 && bary.z >= 0.0 { Some(bary) }
    else { None }
}

fn clip_triangle(v1: Vec3, v2: Vec3, v3: Vec3) -> Option<(Vec3, Vec3, Vec3)> {
    
    if should_cull_face(v1, v2, v3) {
        return None
    }
    
    if let Some(_bool) = triangle_in_bounds(v1, v2, v3) {
        Some((v1, v2, v3))
    } else {
        None
    }
}

fn in_range(val: f32, min: f32, max: f32) -> bool {
    val > min && val < max
}

fn triangle_in_bounds(v1: Vec3, v2: Vec3, v3: Vec3) -> Option<BVec3> {

    let v1_in_range = 
        in_range(v1.x, -1.0, 1.0) &&
        in_range(v1.y, -1.0, 1.0) &&
        in_range(v1.z, 0.0, 1.0);

    let v2_in_range = 
        in_range(v2.x, -1.0, 1.0) &&
        in_range(v2.y, -1.0, 1.0) &&
        in_range(v2.z, 0.0, 1.0);

    let v3_in_range = 
        in_range(v3.x, -1.0, 1.0) &&
        in_range(v3.y, -1.0, 1.0) &&
        in_range(v3.z, 0.0, 1.0);

    if v1_in_range || v2_in_range || v3_in_range {
        Some(BVec3::new(v1_in_range, v2_in_range, v3_in_range))
    } else {
        None
    }
}

fn should_cull_face(v1: Vec3, v2: Vec3, v3: Vec3) -> bool {

    let edge_1 = v2 - v1;
    let edge_2 = v3 - v1;

    //We check if its facing +z
    let normal = edge_1.cross(edge_2);
    normal.dot(-Vec3::Z) >= 0.0
}

enum TriangleClipResult {
    One(Vec4, Vec4, Vec4),
    Two((Vec4, Vec4, Vec4), (Vec4, Vec4, Vec4))
}