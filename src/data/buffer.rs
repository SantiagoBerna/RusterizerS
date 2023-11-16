
pub struct FBuffer {
    data: Vec<f32>
}

pub struct IBuffer {
    data: Vec<u32>
}

impl FBuffer {
    pub fn new(data: Vec<f32>) -> Self {
        Self { data }
    }

    pub fn fill(&mut self, v: f32) {
        self.data.fill(v);
    }

    pub fn data(&self) -> &[f32] {
        &self.data
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn set(&mut self, index: usize, val: f32) {
        self.data[index] = val;
    }
}

impl IBuffer {
    pub fn new(data: Vec<u32>) -> Self {
        Self { data }
    }

    pub fn fill(&mut self, v: u32) {
        self.data.fill(v);
    }

    pub fn data(&self) -> &[u32] {
        &self.data
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}