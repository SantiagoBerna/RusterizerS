
const RESOLUTION_LIMIT: (usize, usize) = (65536, 65536);

//A pixel buffer that uses normalized coordinates for access
pub struct Surface {
    width: usize,
    height: usize,
    buffer: Vec<u32>
}

impl Surface {
    pub fn new(width: usize, height: usize) -> Self {

        debug_assert!(width < RESOLUTION_LIMIT.0);
        debug_assert!(height < RESOLUTION_LIMIT.1);

        Self { width, height, buffer: vec![0; width * height]}
    }

    pub fn clear(&mut self, colour: u32) {
        self.buffer.fill(colour);
    }

    pub fn get_pixel(&self, i: usize, j: usize) -> u32 {
        self.buffer[i + j * self.width]
    }

    pub fn set_pixel_coords(&mut self, colour: u32, i: usize, j: usize) {
        self.buffer[i + j * self.width] = colour;
    }

    pub fn set_pixel_index(&mut self, colour: u32, index: usize) {
        self.buffer[index] = colour;
    }

    pub fn data(&self) -> &[u32] {
        &self.buffer
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn stride(&self) -> usize { self.width }
}