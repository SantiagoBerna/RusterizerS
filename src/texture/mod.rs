use std::path::Path;
use stb_image::image;
use crate::math;

#[derive(Default)]
pub struct Texture {
    data: Vec<u32>,
    width: usize,
    height: usize
}

pub fn load_image_file(path: &Path) -> Result<Texture, String> {
    let decoded_image = image::load(path);

    match decoded_image {
        image::LoadResult::ImageU8(image) => {
            Ok(load_image_memory(&image))
        }
        image::LoadResult::ImageF32(image) => {
            Err("Float images not supported".to_string())
        }
        image::LoadResult::Error(msg) => {
            Err(msg)
        }
    }
}

fn load_image_memory(image: &image::Image<u8>) -> Texture {

    let channels = image.depth;
    let size = image.width * image.height;
    let mut out_data: Vec<u32> = Vec::with_capacity(size);

    if channels == 3 {
        for i in 0..size {
            let index = i * 3;
            out_data.push(
                math::colour::u8_to_hex(255,
                    image.data[index],
                    image.data[index + 1],
                    image.data[index + 2]
                )
            );
        }
    }
    else if channels == 4 {
        for i in 0..size {
            let index = i * 4;
            out_data.push(
                math::colour::u8_to_hex(image.data[index + 3],
                    image.data[index],
                    image.data[index + 1],
                    image.data[index + 2]
                )
            );
        }
    }

    Texture::from_data(out_data, image.width, image.height)
}

//ARGB texture
impl Texture {
    pub fn new(width: usize, height: usize) -> Self {
        Self { data: vec![0; width * height], width, height}
    }

    pub fn from_data(data: Vec<u32>, width: usize, height: usize) -> Self {
        debug_assert!(width * height == data.len());
        Self { data, width, height}
    }

    pub fn width(&self) -> usize { self.width }
    pub fn height(&self) -> usize { self.height }

    //RGB
    pub fn read(&self, i: usize, j: usize) -> u32 {
        let index = self.width * j + i;
        self.data[index]
    }

    pub fn write(&mut self, i: usize, j: usize, colour: u32) {
        let index = self.width * j + i;
        self.data[index] = colour;
    }

    pub fn clear(&mut self, val: u32) {
        self.data.fill(val)
    }

    pub fn as_slice(&self) -> &[u32] { &self.data }
}

pub struct DepthTexture {
    data: Vec<f32>,
    width: usize,
    height: usize
}

impl DepthTexture {
    pub fn new(width: usize, height: usize) -> Self {
        Self { data: vec![1.0; width * height], width, height}
    }

    pub fn width(&self) -> usize { self.width }
    pub fn height(&self) -> usize { self.height }

    //RGB
    pub fn read(&self, i: usize, j: usize) -> f32 {
        let index = self.width * j + i;
        self.data[index]
    }

    pub fn write(&mut self, i: usize, j: usize, val: f32) {
        let index = self.width * j + i;
        self.data[index] = val;
    }

    pub fn clear(&mut self, val: f32) {
        self.data.fill(val)
    }

    //replaces and returns true if input < previous value
    pub fn depth_test(&mut self, i: usize, j: usize, depth_val: f32) -> bool {
        if depth_val < self.read(i, j) { self.write(i, j, depth_val); true}
        else { false }
    }
}

#[derive(Debug, Default)]
pub struct Sampler {

}

impl Sampler {
    pub fn sample(&self, texture: &Texture, uv: glam::Vec2) -> glam::Vec4 {
        let dimensions = ((texture.width() - 1) as f32, (texture.height() - 1) as f32);
        let (i, j) = (uv.x * dimensions.0, (1.0 - uv.y) * dimensions.1);
        math::colour::hex_to_f32(texture.read(i.round() as usize, j.round() as usize))
    }
}