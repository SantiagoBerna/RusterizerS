use std::sync::Arc;

pub enum VertexAttributes {
    Position,
    Normal,
    Colour,
    TextureUV,
    
    MaxAttributes
}

#[derive(Clone)]
pub struct VertexSet {
    indices: Option<Arc<Vec<u32>>>,
    attributes: Vec<Option<Arc<Vec<f32>>>>
}

impl VertexSet {

    pub fn new() -> Self {
        Self { indices: None, attributes: vec![None; VertexAttributes::MaxAttributes as usize] }
    }

    pub fn set_indices(&mut self, data: Arc<Vec<u32>>) {
        self.indices = Some(data);
    }

    pub fn get_indices(&self) -> Option<&[u32]> {
        let entry = self.indices.as_ref()?;
        Some(&entry)
    }

    pub fn set_attribute(&mut self, attribute: VertexAttributes, data: Arc<Vec<f32>>) {
        self.attributes[attribute as usize] = Some(data);
    }

    pub fn get_attribute(&self, attribute: VertexAttributes) -> Option<&[f32]> {
        let entry = self.attributes[attribute as usize].as_ref()?;
        Some(&entry)
    }

}