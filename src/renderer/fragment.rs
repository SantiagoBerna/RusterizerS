use crate::texture::Sampler;
use crate::texture::Texture;
use crate::texture::DepthTexture;
use crate::math;
use super::data::VertexOutput;


#[derive(Default)]
pub struct FragmentShader {
    pub mesh_texture: Texture,
    pub mesh_sampler: Sampler
}

impl FragmentShader {
    pub fn dispatch(&self, out: &mut Texture, depth_buffer: &mut DepthTexture, vs_output: &VertexOutput, indices: &[usize]) {

        debug_assert!(out.width() == depth_buffer.width());
        debug_assert!(out.height() == depth_buffer.height());

        let half_screen_width = (out.width() as f32) * 0.5;
        let half_screen_height = (out.height() as f32) * 0.5;

        let screen_space_matrix = glam::Mat3::from_scale_angle_translation(
            glam::Vec2::new(half_screen_width, -half_screen_height),
            0.0,
            glam::Vec2::new(half_screen_width, half_screen_height)
        );

        let triangle_count = indices.len() / 3;

        for i in 0..triangle_count {
            let triangle_indices = [indices[i * 3], indices[i * 3 + 1], indices[i * 3 + 2]];
            self.rasterize_triangle(out, depth_buffer, vs_output, triangle_indices, &screen_space_matrix);
        }
    }

    fn rasterize_triangle(&self, out: &mut Texture, depth_buffer: &mut DepthTexture, vs_output: &VertexOutput, indices: [usize; 3], screen_matrix: &glam::Mat3) -> Option<()> {
        
        let v1 = vs_output.ndc_positions[indices[0]];
        let v2 = vs_output.ndc_positions[indices[1]];
        let v3 = vs_output.ndc_positions[indices[2]];

        let colour1 = vs_output.colours[indices[0]];
        let colour2 = vs_output.colours[indices[1]];
        let colour3 = vs_output.colours[indices[2]];

        let uv1 = vs_output.uvs[indices[0]];
        let uv2 = vs_output.uvs[indices[1]];
        let uv3 = vs_output.uvs[indices[2]];

        let screen_1 = screen_matrix.mul_vec3(v1.truncate().truncate().extend(1.0));
        let screen_2 = screen_matrix.mul_vec3(v2.truncate().truncate().extend(1.0));
        let screen_3 = screen_matrix.mul_vec3(v3.truncate().truncate().extend(1.0));

        let triangle_bounds = math::generate_triangle_bounding_box(screen_1.truncate(), screen_2.truncate(), screen_3.truncate());

        let screen_bounds = math::bounding_box::BoundingBox::new(
            glam::UVec2::new(0, 0), 
            glam::UVec2::new(out.width() as u32, out.height() as u32)
        );

        let triangle_bounds = triangle_bounds.intersect(&screen_bounds)?;

        let x_range = (triangle_bounds.start.x as usize)..(triangle_bounds.end.x as usize);
        let y_range = (triangle_bounds.start.y as usize)..(triangle_bounds.end.y as usize);

        for j in y_range {
            for i in x_range.clone() {
                
                let pixel_point = glam::Vec2::new(i as f32 + 0.5, j as f32 + 0.5);

                //means the point is inside the triangle
                if let Some(weights) = math::barycentric_weights(pixel_point, screen_1.truncate(), screen_2.truncate(), screen_3.truncate()) {
                    let depth = weights.dot(glam::Vec3::new(v1.z, v2.z, v3.z));

                    if depth_buffer.depth_test(i, j, depth) {
                        
                        let depth_correction = 1.0 / (weights.x * v1.w + weights.y * v2.w + weights.z * v3.w);
                        let colour = math::barycentric_lerp(weights, colour1, colour2, colour3) * depth_correction;
                        let uv = math::barycentric_lerp(weights, uv1, uv2, uv3) * depth_correction;

                        let out_frag = colour * self.mesh_sampler.sample(&self.mesh_texture, uv).truncate();

                        out.write(i, j, math::colour::vec4_to_hex(out_frag.extend(1.0)));
                    }
                }
            }
        }

        Some(())
    }
}