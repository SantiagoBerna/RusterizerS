

use minifb::ScaleMode;
use minifb::Window;
use minifb::WindowOptions;

use std::time::Instant;
use std::path::Path;

mod display;
use display::*;

mod data;
use data::*;

mod renderer;
use renderer::*;

mod utility;
use utility::*;

const RESOLUTION_WIDTH: usize = 640; 
const RESOLUTION_HEIGHT: usize = 480; 
const UPSCALE: usize = 1;

fn main() {
    
    let mut window_options = WindowOptions::default();
    window_options.scale_mode = ScaleMode::Stretch;
    window_options.resize = false;
    
    
    let mut window = match Window::new("Test", RESOLUTION_WIDTH * UPSCALE, RESOLUTION_HEIGHT * UPSCALE, window_options) {
        Ok(win) => win,
        Err(err) => {
            println!("Unable to create window {}", err);
            return;
        }
    };
    
    let mut surface = Surface::new(RESOLUTION_WIDTH, RESOLUTION_HEIGHT);
    let triangle_vertices = Buffer::new( vec![

        300.0, 100.0, 0.0,
        300.0, 300.0, 0.0,
        100.0, 100.0, 0.0,

        100.0, 100.0, 0.0,
        300.0, 300.0, 0.0,
        100.0, 300.0, 0.0,

    ]);

    let triange_uvs = Buffer::new( vec![
        1.0, 0.0,
        1.0, 1.0,
        0.0, 0.0,

        0.0, 0.0,
        1.0, 1.0,
        0.0, 1.0,
    ]);

    let image = Texture::from_image(Path::new("assets/rock.jpg"));

    while window.is_open() {

        //let now = Instant::now();

        surface.clear(0);
        Renderer::draw_buffer(&mut surface, &triangle_vertices, &triange_uvs, &image);

        window.update_with_buffer(surface.data(), RESOLUTION_WIDTH, RESOLUTION_HEIGHT).unwrap();

        //println!("{}", now.elapsed().as_millis());
    }
}
