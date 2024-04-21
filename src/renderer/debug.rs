use crate::camera::Camera;
use crate::texture::Texture;
use crate::math::bounding_box::BoundingBox;
use crate::math::bounding_box::Line;
use crate::math::colour;

#[derive(Default)]
pub struct DebugLineShader {
    pub camera: Camera
}

impl DebugLineShader {
    pub fn dispatch(&self, out: &mut Texture, line_list: &[(glam::Vec3, glam::Vec3)]) {

        let (view, projection) = self.camera.generate_view_projection();
        let vp = projection * view;

        let half_screen_width = (out.width() as f32) * 0.5;
        let half_screen_height = (out.height() as f32) * 0.5;

        let screen_space_matrix = glam::Mat3::from_scale_angle_translation(
            glam::Vec2::new(half_screen_width, -half_screen_height),
            0.0,
            glam::Vec2::new(half_screen_width, half_screen_height)
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
            let ndc_1 = proj_1.truncate() * inv_w1;
            let ndc_2 = proj_2.truncate() * inv_w2;

            self.draw_line(out, &screen_space_matrix, ndc_1, ndc_2);
        }
    }

    fn draw_line(&self, out: &mut Texture, screen_space_matrix: &glam::Mat3, start: glam::Vec3, end: glam::Vec3) {

        let screen_bounds = BoundingBox::new(
            glam::UVec2::new(0, 0), 
            glam::UVec2::new(out.width() as u32 - 1, out.height() as u32 - 1)
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
                out.write(x as usize, y as usize, colour::f32_to_hex(1.0, 0.0, 1.0, 0.0));

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