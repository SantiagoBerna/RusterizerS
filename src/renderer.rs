
use crate::data::vertex_set::VertexSet;
use crate::data::vertex_set::VertexAttributes;
use crate::data::sampler::Sampler;

use crate::display::Surface;
use crate::utility::*;

use glam::Mat2;
use glam::Mat3;
use glam::Mat4;
use glam::Quat;
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

    fn component_range(start: usize, stride: usize) -> Range<usize> {
        (start * stride)..(start * stride + stride)
    }

    fn draw_triangle(&self, surface: &mut Surface, vertices: &VertexSet, mvp: Mat4, indices: [u32; 3])
    {
        let vertex_position = vertices.get_attribute(VertexAttributes::Position).unwrap();

        let v1 = Vec3::from_slice(&vertex_position[Self::component_range(indices[0] as usize, 3)]);
        let v2 = Vec3::from_slice(&vertex_position[Self::component_range(indices[1] as usize, 3)]);
        let v3 = Vec3::from_slice(&vertex_position[Self::component_range(indices[2] as usize, 3)]);

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

        for index in 0..surface.len() {

            let x = (index % surface.stride()) as f32;
            let y = (index / surface.stride()) as f32;
    
            let point = Vec2::new(x + 0.5, y + 0.5);

            let res_1 = edge_function(point, screen_1.xy(), screen_2.xy());
            let res_2 = edge_function(point, screen_2.xy(), screen_3.xy());
            let res_3 = edge_function(point, screen_3.xy(), screen_1.xy());
    
            //checking for negatives since we go from rh to lh in screen transformation
            if res_1 <= 0.0 && res_2 <= 0.0 && res_3 <= 0.0 {
                
                let area = edge_function(screen_1.xy(), screen_2.xy(), screen_3.xy());

                let bary_coords = Vec3::new(res_2, res_3, res_1) / area; 

                let depth = ndc_1.z * bary_coords.x + ndc_2.z * bary_coords.y + ndc_3.z * bary_coords.z;
                let prev_depth = surface.get_depth(index);

                if depth <= prev_depth {

                    surface.set_depth(index, depth);

                    let uv_data = vertices.get_attribute(VertexAttributes::TextureUV).unwrap();

                    let uv_1 = Vec2::from_slice(&uv_data[Self::component_range(indices[0] as usize, 2)]) * w1;
                    let uv_2 = Vec2::from_slice(&uv_data[Self::component_range(indices[1] as usize, 2)]) * w2;
                    let uv_3 = Vec2::from_slice(&uv_data[Self::component_range(indices[2] as usize, 2)]) * w3;

                    let depth_correction = 1.0 / (w1 * bary_coords.x + w2 * bary_coords.y + w3 * bary_coords.z);
                    let uv_coordinate = (uv_1 * bary_coords.x + uv_2 * bary_coords.y + uv_3 * bary_coords.z) * depth_correction;

                    let colour = self.texture_slots[0].as_ref().unwrap().sample(uv_coordinate.x, 1.0 - uv_coordinate.y);
                    surface.set_pixel_index(from_f32_rgb(colour.x, colour.y, colour.z), index);   
                }
            }
        }
    }
}