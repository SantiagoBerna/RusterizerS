use glam::Vec2;
use glam::Vec3;
use glam::Vec4;

//Input for vertex shader
#[derive(Default)]
pub struct VertexInput {
    pub positions: Vec<Vec3>,
    pub colours: Vec<Vec3>,
    pub uvs: Vec<Vec2>,
}

impl VertexInput {
    pub fn retrieve<T>(source: &[T], indices: [usize; 3], ) -> [T; 3] 
    where T: Clone
    {
        [source[indices[0]].clone(), source[indices[1]].clone(), source[indices[2]].clone()]
    }
}

//Output for vertex shader
#[derive(Default)]
pub struct VertexOutput {
    pub ndc_positions: Vec<Vec4>,
    pub colours: Vec<Vec3>,
    pub uvs: Vec<Vec2>
}
