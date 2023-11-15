
use std::path::Path;
use glam::{Vec2, Vec4, Vec3};

use crate::utility::{*, self};

pub struct Buffer {
    data: Vec<f32>
}

impl Buffer {
    pub fn new(data: Vec<f32>) -> Self {
        Self { data }
    }

    pub fn data(&self) -> &[f32] {
        &self.data
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

pub struct Texture {
    data: Vec<Vec3>,
    width: usize,
    height: usize,
    colour_channels: usize
}

impl Texture {
    pub fn from_image(path: &Path) -> Self {
        
        let decoded_image = stb_image::image::load(path);
        if let stb_image::image::LoadResult::ImageU8(image) = decoded_image {

            let data = (0..image.data.len() / 3)
                .map(|id| {
                    Vec3::new(
                        u8_to_f32(image.data[id * 3]),
                        u8_to_f32(image.data[id * 3 + 1]),
                        u8_to_f32(image.data[id * 3 + 2])
                    )
                })
                .collect();

            Self {
                data,
                width: image.width,
                height: image.height,
                colour_channels: image.depth,
            }

        } else {
            panic!("Unsupported texture type");
        }
    }

    pub fn sample(&self, uv: Vec2) -> Vec4 {

        let u = (uv.x * self.width as f32).clamp(0.0, self.width as f32 - 1.0);
        let v = (uv.y * self.height as f32).clamp(0.0, self.height as f32 - 1.0);

        let index = v as usize * self.width + u as usize;
        let colour = self.data[index];
        Vec4::new(colour.x, colour.y, colour.z, 1.0)

    }
}
