use std::sync::Arc;

use super::{FBuffer, buffer::IBuffer};

pub enum VertexAttributes {
    Position,
    Normal,
    Colour,
    TextureUV,
    
    MaxAttributes
}

pub struct VertexSet {
    indices: Option<Arc<IBuffer>>,
    attributes: Vec<Option<Arc<FBuffer>>>
}

impl VertexSet {

    pub fn new() -> Self {
        Self { indices: None, attributes: vec![None; VertexAttributes::MaxAttributes as usize] }
    }

    pub fn set_indices(&mut self, data: Arc<IBuffer>) {
        self.indices = Some(data);
    }

    pub fn get_indices(&self) -> Option<&[u32]> {
        let entry = self.indices.as_ref()?;
        Some(entry.data())
    }

    pub fn set_attribute(&mut self, attribute: VertexAttributes, data: Arc<FBuffer>) {
        self.attributes[attribute as usize] = Some(data);
    }

    pub fn get_attribute(&self, attribute: VertexAttributes) -> Option<&[f32]> {
        let entry = self.attributes[attribute as usize].as_ref()?;
        Some(entry.data())
    }

}