
use std::path::Path;
use std::sync::Arc;
use stb_image::image::LoadResult::*;
use crate::utility;

pub mod buffer;
use buffer::*;

pub mod vertex_set;

pub mod sampler;
use sampler::*;

pub fn load_texture_from_file(path: &Path) -> Option<(Arc<FBuffer>, Sampler)> {
    let decoded_image = stb_image::image::load(path);

    if let ImageU8(image) = decoded_image {

        let size = (image.width, image.height);
        let data: Vec<f32> = 
            (0..image.data.len()).map(|i| { utility::u8_to_f32(image.data[i])}).collect();

        let buffer = Arc::new(FBuffer::new(data));
        let mut sampler = Sampler::new(size, image.depth);

        sampler.bind_buffer(buffer.clone());

        Some((buffer, sampler))

    } else {
        None
    }
}
