
use crate::data::{Buffer, Texture};
use crate::display::Surface;
use crate::utility::*;

use glam::{Vec2, Vec3};

pub struct Renderer {
}

impl Renderer {

    pub fn draw_buffer(surface: &mut Surface, vertices: &Buffer, texture_uvs: &Buffer, texture: &Texture) {

        debug_assert!(vertices.len() % 9 == 0);
        debug_assert!(texture_uvs.len() % 6 == 0);
        debug_assert!(vertices.len() / 9 == texture_uvs.len() / 6);

        let vertex_data = vertices.data();
        let colour_data = texture_uvs.data();

        for i in 0..(vertices.len() / 9) {

            let vert_base = i * 9;
            let uv_base = i * 6;

            Self::draw_triangle(surface,
                Vec2::new(vertex_data[vert_base + 0], vertex_data[vert_base + 1]),
                Vec2::new(vertex_data[vert_base + 3], vertex_data[vert_base + 4]),
                Vec2::new(vertex_data[vert_base + 6], vertex_data[vert_base + 7]),
                Vec2::new(colour_data[uv_base + 0], colour_data[uv_base + 1]),
                Vec2::new(colour_data[uv_base + 2], colour_data[uv_base + 3]),
                Vec2::new(colour_data[uv_base + 4], colour_data[uv_base + 5]),
                texture
            );
        }

    }

    fn draw_triangle(
        surface: &mut Surface,
        v0: Vec2, v1: Vec2, v2: Vec2,
        uv0: Vec2, uv1: Vec2, uv2: Vec2,
        texture: &Texture
    ) {

        for index in 0..surface.len() {

            let x = (index % surface.stride()) as f32;
            let y = (index / surface.stride()) as f32;
    
            let point = Vec2::new(x, y);
    
            let res_1 = edge_function(point, v0, v1);
            let res_2 = edge_function(point, v1, v2);
            let res_3 = edge_function(point, v2, v0);
    
            if res_1 >= 0.0 && res_2 >= 0.0 && res_3 >= 0.0 {
                
                let area = edge_function(v0, v1, v2);
                let bary_coords = Vec3::new(res_2, res_3, res_1) / area; 

                let uv_coordinate = (uv0 * bary_coords.x) + (uv1 * bary_coords.y) + (uv2 * bary_coords.z);

                let colour = texture.sample(uv_coordinate);
                surface.set_pixel_index(from_f32_rgb(colour.x, colour.y, colour.z), index);
            }
        }
    }
}