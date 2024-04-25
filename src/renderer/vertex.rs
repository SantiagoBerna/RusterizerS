use crate::math;
use super::data::VertexInput;
use super::data::VertexOutput;

#[derive(Default)]
pub struct VertexShader {
    pub view: glam::Mat4,
    pub projection: glam::Mat4,
    pub model: glam::Mat4
}

impl VertexShader {

    fn triangle_indices(input_indices: &[usize], triangle_id: usize) -> [usize; 3] {
        [
            input_indices[triangle_id * 3], input_indices[triangle_id * 3 + 1], input_indices[triangle_id * 3 + 2]
        ]
    }

    fn world_to_clip_space(mvp: &glam::Mat4, positions: &[glam::Vec3; 3]) -> [glam::Vec4; 3] {
        [
            mvp.mul_vec4(positions[0].extend(1.0)),
            mvp.mul_vec4(positions[1].extend(1.0)),
            mvp.mul_vec4(positions[2].extend(1.0))
        ]
    }

    pub fn dispatch(&self, vertex_in: &VertexInput, indices: &[usize]) -> (VertexOutput, Vec<usize>) {

        let input_triangle_count = indices.len() / 3;

        //Outputs
        let mut out_indices = Vec::new();
        let mut out_vertex = VertexOutput::default();

        //VP and Frustrum
        let mvp = self.projection * self.view * self.model;

        //Main body
        let mut triangle_start = 0;
        for i in 0..input_triangle_count {

            let triangle_indices = VertexShader::triangle_indices(indices, i);
            let vertices = VertexInput::retrieve(&vertex_in.positions, triangle_indices);
            let clip_coordinates = VertexShader::world_to_clip_space(&mvp, &vertices);

            //Frustum clipping
            if math::should_cull_triangle(clip_coordinates[0], clip_coordinates[1], clip_coordinates[2]) { continue; }         

            let clipped_vertices = math::clip_homogenous_triangle(&clip_coordinates);
            if clipped_vertices.is_empty() { continue; }

            for (vert, bary) in &clipped_vertices {
                
                let inv_depth = 1.0 / vert.w;
                out_vertex.ndc_positions.push((*vert * inv_depth).truncate().extend(inv_depth));

                let colours = VertexInput::retrieve(&vertex_in.colours, triangle_indices);
                let uvs = VertexInput::retrieve(&vertex_in.uvs, triangle_indices);

                let result_colour = math::barycentric_lerp(*bary, colours[0], colours[1],colours[2]);
                let result_uv = math::barycentric_lerp(*bary, uvs[0], uvs[1],uvs[2]);
                out_vertex.colours.push(result_colour * inv_depth);
                out_vertex.uvs.push(result_uv * inv_depth);
            }

            let triangulation_indices: Vec<(usize, usize, usize)> = (1..clipped_vertices.len() - 1)
                .map(|v| { (0, v, v + 1) }).collect();

            for elem in &triangulation_indices {
                out_indices.push(triangle_start + elem.0);
                out_indices.push(triangle_start + elem.1);
                out_indices.push(triangle_start + elem.2);
            }

            triangle_start += clipped_vertices.len();
        }

        (out_vertex, out_indices)
    }
}