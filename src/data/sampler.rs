
use glam::{Vec4, BVec4, UVec4};
use std::sync::Arc;

#[derive(Clone)]
pub struct Sampler {
    width: usize,
    height: usize,
    colour_channels: usize,
    bound_texture: Option<Arc<Vec<u8>>>
}

impl Sampler {

    pub fn new(image_size: (usize, usize), channels: usize) -> Self {
        Self { width: image_size.0, height: image_size.1, colour_channels: channels, bound_texture: None }
    }

    pub fn bind_texture(&mut self, buffer_handle: Arc<Vec<u8>>) {
        self.bound_texture = Some(buffer_handle);
        debug_assert!(self.bound_texture.is_some());
    }

    fn byte_to_f32(byte: u8) -> f32 {
        (byte as f32) / 255.0
    }


    pub fn sample(&self, u: f32, v: f32) -> Vec4 {

        //Nearest
        let i = (u * self.width as f32).clamp(0.0, self.width as f32 - 1.0) as usize;
        let j = (v * self.height as f32).clamp(0.0, self.height as f32 - 1.0) as usize;
        
        let index = (j * self.width + i) * self.colour_channels;

        let buffer = self.bound_texture.as_ref().unwrap().as_ref().as_slice();

        Vec4::new(
            Self::byte_to_f32(buffer[index]),
            Self::byte_to_f32(buffer[index + 1]),
            Self::byte_to_f32(buffer[index + 2]),
            1.0
        )
    }
}