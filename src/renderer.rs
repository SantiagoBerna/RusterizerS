
use crate::data::bounding_box::BoundingBox;
use crate::data::vertex_set::VertexSet;
use crate::data::vertex_set::VertexAttributes;
use crate::data::sampler::Sampler;

use crate::display::Surface;
use crate::utility::*;

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
    texture_slots: Vec<Option<Sampler>>
}

impl Renderer {

    pub fn new() -> Self {
        Self { 
            view_matrix: Mat4::default(),
            projection_matrix: Mat4::default(),
            texture_slots: vec![None; TextureSlot::MaxTextureSlots as usize] 
        }
    }

    pub fn set_sampler(&mut self, index: TextureSlot, sampler: Option<Sampler>) {
        self.texture_slots[index as usize] = sampler;
    }

    pub fn draw_buffer(&self, surface: &mut Surface, model_matrix: &Mat4, vertices: &VertexSet, triangle_count: usize) {

        let index_data = vertices.get_indices();
        let mvp = self.projection_matrix * self.view_matrix * (*model_matrix);

        if let Some(indices) = index_data {
            for i in 0..triangle_count {
                self.draw_triangle(
                    surface, vertices, mvp,
                    [indices[i * 3], indices[i * 3 + 1], indices[i * 3 + 2]]
                );
            }
        } else {
            for i in 0..(triangle_count as u32) {
                self.draw_triangle(
                    surface, vertices, mvp,
                    [i * 3, (i * 3 + 1), (i * 3 + 2)]
                );
            }

        }
    }

    fn draw_triangle(&self, surface: &mut Surface, vertices: &VertexSet, mvp: Mat4, indices: [u32; 3])
    {
        let vertex_position = vertices.get_attribute(VertexAttributes::Position).unwrap();

        let v1 = Vec3::from_slice(&vertex_position[component_range(indices[0] as usize, 3)]);
        let v2 = Vec3::from_slice(&vertex_position[component_range(indices[1] as usize, 3)]);
        let v3 = Vec3::from_slice(&vertex_position[component_range(indices[2] as usize, 3)]);

        let clip_1 = mvp * v1.extend(1.0);
        let clip_2 = mvp * v2.extend(1.0);
        let clip_3 = mvp * v3.extend(1.0);

        let w1 = 1.0 / clip_1.w;
        let w2 = 1.0 / clip_2.w;
        let w3 = 1.0 / clip_3.w;

        //Apply depth culling
        //...

        //Homogenous divide
        let ndc_1 = clip_1 * w1;
        let ndc_2 = clip_2 * w2;
        let ndc_3 = clip_3 * w3;

        //Map to screen space
        let to_screen_space = Mat3::from_scale_angle_translation(
            Vec2::new(surface.get_width() as f32 * 0.5, surface.get_height() as f32 * -0.5),
            0.0,
            Vec2::new(surface.get_width() as f32 * 0.5, surface.get_height() as f32 * 0.5)
        );

        let screen_1 = to_screen_space * ndc_1.xy().extend(1.0);
        let screen_2 = to_screen_space * ndc_2.xy().extend(1.0);
        let screen_3 = to_screen_space * ndc_3.xy().extend(1.0);

        let triangle_bounds = generate_triangle_bounding_box(screen_1.xy(), screen_2.xy(), screen_3.xy());
        let screen_bounds = BoundingBox::new(
            UVec2::new(0, 0), 
            UVec2::new(surface.get_width() as u32, surface.get_height() as u32)
        );

        let triangle_screen_bounds = triangle_bounds.intersect(&screen_bounds);

        if let Some(bounds) = triangle_screen_bounds {

            for j in bounds.start.y..bounds.end.y {
                for i in bounds.start.x..bounds.end.x {

                    let pixel_index = j as usize * surface.get_width() + i as usize;
                    let point = Vec2::new(i as f32 + 0.5, j as f32 + 0.5);

                    let in_triangle = barycentric_weights(point, screen_1.xy(), screen_2.xy(), screen_3.xy());
                    if let Some(weights) = in_triangle {

                        let depth = ndc_1.z * weights.x + ndc_2.z * weights.y + ndc_3.z * weights.z;
                        let prev_depth = surface.get_depth(pixel_index);

                        if depth <= prev_depth {

                            surface.set_depth(pixel_index, depth);

                            let uv_data = vertices.get_attribute(VertexAttributes::TextureUV).unwrap();

                            let uv_1 = Vec2::from_slice(&uv_data[component_range(indices[0] as usize, 2)]) * w1;
                            let uv_2 = Vec2::from_slice(&uv_data[component_range(indices[1] as usize, 2)]) * w2;
                            let uv_3 = Vec2::from_slice(&uv_data[component_range(indices[2] as usize, 2)]) * w3;

                            let depth_correction = 1.0 / (w1 * weights.x + w2 * weights.y + w3 * weights.z);
                            let uv_coordinate = (uv_1 * weights.x + uv_2 * weights.y + uv_3 * weights.z) * depth_correction;

                            let colour = self.texture_slots[0].as_ref().unwrap().sample(uv_coordinate.x, 1.0 - uv_coordinate.y);
                            surface.set_pixel_index(from_f32_rgb(colour.x, colour.y, colour.z), pixel_index);   
                        }
                    }
                }
            }
        }
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

pub fn edge_function(p: Vec2, a: Vec2, b: Vec2) -> f32 {
    let edge = b - a;
    let to_p = p - a;

    edge.x * to_p.y - edge.y * to_p.x
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