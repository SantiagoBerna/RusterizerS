
use std::path::Path;
use std::sync::Arc;
use stb_image::image::LoadResult::*;

pub mod vertex_set;

pub mod sampler;
use sampler::*;

pub fn load_texture_from_file(path: &Path) -> Option<Sampler> {
    let decoded_image = stb_image::image::load(path);

    if let ImageU8(image) = decoded_image {

        let size = (image.width, image.height);

        let buffer = Arc::new(image.data);
        let mut sampler = Sampler::new(size, image.depth);

        sampler.bind_texture(buffer.clone());

        Some(sampler)

    } else {
        None
    }
}
