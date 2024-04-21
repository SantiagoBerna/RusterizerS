use std::mem::transmute;

use crate::camera::Camera;
use crate::math::clip_homogenous_triangle;
use crate::math::homogenous_clip;
use crate::math::plane::clip_polygon;
use crate::texture::Texture;
use crate::texture::DepthTexture;

use crate::math::bounding_box::BoundingBox;
use crate::math;
use crate::math::bounding_box::Line;
use glam::Mat3;
use glam::Mat4;
use glam::Vec3;
use glam::Vec2;
use glam::Vec3Swizzles;
use glam::Vec4;
use glam::Vec4Swizzles;
use glam::UVec2;

#[derive(Default)]
pub struct VertexInput {
    pub positions: Vec<Vec3>,
    pub colours: Vec<Vec3>,
    pub uvs: Vec<Vec2>,
}

impl VertexInput {
    fn get_positions(&self, indices: [usize; 3]) -> [glam::Vec3; 3] {
        [self.positions[indices[0]], self.positions[indices[1]], self.positions[indices[2]]]
    }
}

#[derive(Default)]
pub struct VertexOutput {
    pub ndc_positions: Vec<Vec4>,
    pub colours: Vec<Vec3>,
    pub uvs: Vec<Vec2>
}

#[derive(Default)]
pub struct VertexShader {
    pub camera: Camera
}

impl VertexShader {

    fn triangle_indices(input_indices: &[usize], triangle_id: usize) -> [usize; 3] {
        [
            input_indices[triangle_id * 3], input_indices[triangle_id * 3 + 1], input_indices[triangle_id * 3 + 2]
        ]
    }

    fn world_to_clip_space(vp: &Mat4, positions: &[Vec3; 3]) -> [Vec4; 3] {
        [
            vp.mul_vec4(positions[0].extend(1.0)),
            vp.mul_vec4(positions[1].extend(1.0)),
            vp.mul_vec4(positions[2].extend(1.0))
        ]
    }

    pub fn dispatch(&self, vertex_in: &VertexInput, indices: &[usize]) -> (VertexOutput, Vec<usize>) {

        let input_triangle_count = indices.len() / 3;

        //Outputs
        let mut out_indices = Vec::new();
        let mut out_vertex = VertexOutput::default();

        //VP and Frustrum
        let (view, projection) = self.camera.generate_view_projection();
        let vp = projection * view;

        //Main body
        let mut triangle_count = 0;
        for i in 0..input_triangle_count {

            let triangle_indices = VertexShader::triangle_indices(indices, i);
            let vertices = vertex_in.get_positions(triangle_indices);
            let clip_coordinates = VertexShader::world_to_clip_space(&vp, &vertices);

            //Frustum clipping
            if math::should_cull_triangle(clip_coordinates[0], clip_coordinates[1], clip_coordinates[2]) { continue; }         

            let clipped_vertices = clip_homogenous_triangle(&clip_coordinates);
            if clipped_vertices.is_empty() { continue; }

            let inverse_w: Vec<f32> = (&clipped_vertices).iter().map(|(i, _)|{ 1.0 / i.w }).collect();

            for i in 0..clipped_vertices.len() {

                out_vertex.ndc_positions.push((clipped_vertices[i].0 * inverse_w[i]).truncate().extend(inverse_w[i]));
                let origin_edge = clipped_vertices[i].1.floor() as usize;

                //forward all attributes
                if clipped_vertices[i].1.fract().abs() < std::f32::EPSILON {
                    out_vertex.colours.push(vertex_in.colours[triangle_indices[origin_edge]]);
                    
                } else {

                    let alpha = clipped_vertices[i].1.fract();
                    let colour_1 = vertex_in.colours[triangle_indices[origin_edge]];
                    let colour_2 = vertex_in.colours[triangle_indices[(origin_edge + 1) % 3]];
    
                    let result = math::lerp(colour_1, colour_2, alpha);
                    out_vertex.colours.push(result);
                }
            }

            let triangulation_indices: Vec<(usize, usize, usize)> = (1..clipped_vertices.len() - 1)
                .map(|i| { (0, i , i + 1) }).collect();

            for indices in triangulation_indices {
                out_indices.push(triangle_count * 3 + indices.0);
                out_indices.push(triangle_count * 3 + indices.1);
                out_indices.push(triangle_count * 3 + indices.2);
            }

            triangle_count += 1;
        }

        (out_vertex, out_indices)
    }
}

#[derive(Default)]
pub struct FragmentShader {

}

impl FragmentShader {
    pub fn dispatch(&self, out: &mut Texture, depth_buffer: &mut DepthTexture, vs_output: &VertexOutput, indices: &[usize]) {

        debug_assert!(out.width() == depth_buffer.width());
        debug_assert!(out.height() == depth_buffer.height());

        let half_screen_width = (out.width() as f32) * 0.5;
        let half_screen_height = (out.height() as f32) * 0.5;

        let screen_space_matrix = Mat3::from_scale_angle_translation(
            Vec2::new(half_screen_width, -half_screen_height),
            0.0,
            Vec2::new(half_screen_width, half_screen_height)
        );

        let triangle_count = indices.len() / 3;

        for i in 0..triangle_count {
            let triangle_indices = [indices[i * 3], indices[i * 3 + 1], indices[i * 3 + 2]];
            self.rasterize_triangle(out, depth_buffer, vs_output, triangle_indices, &screen_space_matrix);
        }
    }

    fn rasterize_triangle(&self, out: &mut Texture, depth_buffer: &mut DepthTexture, vs_output: &VertexOutput, indices: [usize; 3], screen_matrix: &Mat3) -> Option<()> {
        
        let v1 = vs_output.ndc_positions[indices[0]];
        let v2 = vs_output.ndc_positions[indices[1]];
        let v3 = vs_output.ndc_positions[indices[2]];

        let colour1 = vs_output.colours[indices[0]];
        let colour2 = vs_output.colours[indices[1]];
        let colour3 = vs_output.colours[indices[2]];

        let screen_1 = screen_matrix.mul_vec3(v1.xy().extend(1.0));
        let screen_2 = screen_matrix.mul_vec3(v2.xy().extend(1.0));
        let screen_3 = screen_matrix.mul_vec3(v3.xy().extend(1.0));

        let triangle_bounds = math::generate_triangle_bounding_box(screen_1.xy(), screen_2.xy(), screen_3.xy());

        let screen_bounds = BoundingBox::new(
            UVec2::new(0, 0), 
            UVec2::new(out.width() as u32, out.height() as u32)
        );

        let triangle_bounds = triangle_bounds.intersect(&screen_bounds)?;

        let x_range = (triangle_bounds.start.x as usize)..(triangle_bounds.end.x as usize);
        let y_range = (triangle_bounds.start.y as usize)..(triangle_bounds.end.y as usize);

        for j in y_range {
            for i in x_range.clone() {
                
                let pixel_point = Vec2::new(i as f32 + 0.5, j as f32 + 0.5);

                //means the point is inside the triangle
                if let Some(weights) = math::barycentric_weights(pixel_point, screen_1.xy(), screen_2.xy(), screen_3.xy()) {
                    let depth = weights.dot(Vec3::new(v1.z, v2.z, v3.z));

                    if depth_buffer.depth_test(i, j, depth) {
                        
                        let depth_correction = 1.0 / (v1.w * weights.x + v2.w * weights.y + v3.w * weights.z);
                        let colour = ((colour1 * v1.w * weights.x) + (colour2 * v2.w * weights.y) + (colour3 * v3.w * weights.z)) * depth_correction;

                        out.write(i, j, math::f32_to_hex(1.0, colour.x, colour.y, colour.z));
                    }
                }
            }
        }

        Some(())
    }
}

#[derive(Default)]
pub struct DebugLineShader {
    pub camera: Camera
}

impl DebugLineShader {
    pub fn dispatch(&self, out: &mut Texture, line_list: &[(Vec3, Vec3)]) {

        let (view, projection) = self.camera.generate_view_projection();
        let vp = projection * view;

        let half_screen_width = (out.width() as f32) * 0.5;
        let half_screen_height = (out.height() as f32) * 0.5;

        let screen_space_matrix = Mat3::from_scale_angle_translation(
            Vec2::new(half_screen_width, -half_screen_height),
            0.0,
            Vec2::new(half_screen_width, half_screen_height)
        );

        for (start, end) in line_list {
           
            //calculate ndc coordinates of triangle
            let proj_1 = vp * start.extend(1.0);
            let proj_2 = vp * end.extend(1.0);

            if proj_1.z > proj_1.w || proj_1.z < 0.0 || proj_2.z > proj_2.w || proj_2.z < 0.0 {
                continue;
            }

            let inv_w1 = 1.0 / proj_1.w;
            let inv_w2 = 1.0 / proj_2.w;

            //Homogenous divide
            let ndc_1 = proj_1.xyz() * inv_w1;
            let ndc_2 = proj_2.xyz() * inv_w2;

            self.draw_line(out, &screen_space_matrix, ndc_1, ndc_2);
        }
    }

    fn draw_line(&self, out: &mut Texture, screen_space_matrix: &Mat3, start: Vec3, end: Vec3) {

        let screen_bounds = BoundingBox::new(
            UVec2::new(0, 0), 
            UVec2::new(out.width() as u32 - 1, out.height() as u32 - 1)
        );

        let screen_start = screen_space_matrix.mul_vec3(start).truncate();
        let screen_end = screen_space_matrix.mul_vec3(end).truncate();

        if let Some(clipped_line) = screen_bounds.clip_line(&Line::new(screen_start, screen_end)) {
          
            let clipped_start = clipped_line.start.as_uvec2();
            let clipped_end = clipped_line.end.as_uvec2();

            let mut x = clipped_start.x as i32;
            let mut y = clipped_start.y as i32;

            let final_x = clipped_end.x as i32;
            let final_y = clipped_end.y as i32;

            let dx = (final_x - x).abs();
            let dy = -(final_y - y).abs();

            let sx = if x < final_x { 1 } else { -1 }; 
            let sy = if y < final_y { 1 } else { -1 }; 

            let mut error = dy + dx;

            loop {
                out.write(x as usize, y as usize, math::f32_to_hex(1.0, 0.0, 1.0, 0.0));

                if x == final_x && y == final_y { break };
                let e2 = 2 * error;

                if e2 >= dy {
                    error = error + dy;
                    x = x + sx;
                }

                if e2 <= dx {
                    error = error + dx;
                    y = y + sy;
                }
            }
        }
    }
}

