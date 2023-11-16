
const RESOLUTION_LIMIT: (usize, usize) = (65536, 65536);

//A pixel buffer that uses normalized coordinates for access
pub struct Surface {

    width: usize,
    height: usize,

    colour_buffer: Vec<u32>,
    depth_buffer: Vec<f32>
}

impl Surface {
    pub fn new(width: usize, height: usize) -> Self {

        debug_assert!(width < RESOLUTION_LIMIT.0);
        debug_assert!(height < RESOLUTION_LIMIT.1);

        Self {
            width, height, 
            colour_buffer: vec![0; width * height],
            depth_buffer: vec![f32::INFINITY; width * height]
        }
    }

    pub fn get_width(&self) -> usize { self.width }
    pub fn get_height(&self) -> usize { self.height }

    pub fn clear(&mut self, colour: u32, depth_value: f32) {
        self.colour_buffer.fill(colour);
        self.depth_buffer.fill(depth_value);
    }

    pub fn get_pixel(&self, i: usize, j: usize) -> u32 {
        self.colour_buffer[i + j * self.width]
    }

    pub fn set_pixel_coords(&mut self, colour: u32, i: usize, j: usize) {
        self.colour_buffer[i + j * self.width] = colour;
    }

    pub fn set_pixel_index(&mut self, colour: u32, index: usize) {
        self.colour_buffer[index] = colour;
    }

    pub fn get_depth(&self, index: usize) -> f32 {
        self.depth_buffer[index]
    }

    pub fn set_depth(&mut self, index: usize, value: f32) {
        self.depth_buffer[index] = value;
    }

    pub fn data(&self) -> &[u32] {
        &self.colour_buffer
    }

    pub fn len(&self) -> usize {
        self.height * self.width
    }

    pub fn stride(&self) -> usize { self.width }
}