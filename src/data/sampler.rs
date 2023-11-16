
use glam::Vec4;
use super::buffer::FBuffer;
use std::sync::Arc;

#[derive(Clone)]
pub struct Sampler {
    width: usize,
    height: usize,
    colour_channels: usize,
    bound_texture: Option<Arc<FBuffer>>
}

impl Sampler {

    pub fn new(image_size: (usize, usize), channels: usize) -> Self {
        Self { width: image_size.0, height: image_size.1, colour_channels: channels, bound_texture: None }
    }

    pub fn bind_buffer(&mut self, buffer_handle: Arc<FBuffer>) {
        self.bound_texture = Some(buffer_handle);
        debug_assert!(self.bound_texture.is_some());
    }


    pub fn sample(&self, u: f32, v: f32) -> Vec4 {

        //Nearest
        let i = (u * self.width as f32).clamp(0.0, self.width as f32 - 1.0) as usize;
        let j = (v * self.height as f32).clamp(0.0, self.height as f32 - 1.0) as usize;
        
        let index = (j * self.width + i) * self.colour_channels;

        let buffer = self.bound_texture.as_ref().unwrap().data();

        Vec4::new(buffer[index], buffer[index + 1], buffer[index + 2], 1.0)
    }
}