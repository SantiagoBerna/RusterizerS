use glam::Vec3;
use glam::Vec2;

use minifb::Window;

mod math;
use math::*;

mod texture;
use texture::*;

mod camera;
use camera::*;

mod renderer;
use renderer::*;



const RESOLUTION_WIDTH: usize = 640; 
const RESOLUTION_HEIGHT: usize = 480; 
const UPSCALE: usize = 1;

const QUAD_INDICES: [usize; 6] = [
    0, 1, 2,
    2, 3, 0
];

const QUAD_VERTEX_POSITIONS: [Vec3; 4] = [
    Vec3::new(-0.5,  0.5, 0.5),
    Vec3::new(-0.5, -0.5, 0.5),
    Vec3::new(0.5, -0.5, 0.5),
    Vec3::new(0.5,  0.5, 0.5),  
];

const QUAD_VERTEX_UVS: [Vec2; 4] = [
    Vec2::new(0.0, 1.0),
    Vec2::new(0.0, 0.0),
    Vec2::new(1.0, 0.0),
    Vec2::new(1.0, 1.0)
];


fn create_window() -> minifb::Result<Window> {
    let mut window_options = minifb::WindowOptions::default();
    window_options.scale_mode = minifb::ScaleMode::Stretch;
    window_options.resize = false;

    Window::new("Rasterizing with Rust", RESOLUTION_WIDTH * UPSCALE, RESOLUTION_HEIGHT * UPSCALE, window_options)
}

fn main() {
    
    let mut window = create_window().unwrap();
    
    let mut output_surface = Texture::new(RESOLUTION_WIDTH, RESOLUTION_HEIGHT);
    let mut depth_attachment = DepthTexture::new(RESOLUTION_WIDTH, RESOLUTION_HEIGHT);

    let mut timer = std::time::Instant::now();

    //Camera
    let mut camera = Camera::default();
    camera.position = Vec3::new(0.0, 0.0, 1.0);
    camera.aspect_ratio = (RESOLUTION_WIDTH as f32) / (RESOLUTION_HEIGHT as f32);
    camera.fov = std::f32::consts::PI * 0.25;
    camera.near = 0.1;
    camera.far = 10.0;

    //Shader abstractions
    let mut vs = vertex::VertexShader::default();
    let mut fs = fragment::FragmentShader::default();
    let mut ls = debug::DebugLineShader::default();

    let texture = load_image_file(std::path::Path::new("assets/icon.png")).unwrap();
    fs.mesh_texture = texture;

    //Setting up vertices
    let mut vertices = data::VertexInput::default();
    vertices.positions = QUAD_VERTEX_POSITIONS.to_vec();
    vertices.colours = QUAD_VERTEX_UVS.iter().map(|vec2|{ Vec3::new(vec2.x, vec2.y, 1.0) }).collect();
    vertices.uvs = QUAD_VERTEX_UVS.to_vec();

    let indices = QUAD_INDICES.to_owned();
    let mut prev_mouse = Vec2::default();

    while window.is_open() {

        //Delta Time
        let dt = timer.elapsed().as_secs_f32();
        timer = std::time::Instant::now(); //reset timer

        //Mouse delta
        let mut mouse_delta = Vec2::default();
        if let Some((x, y)) = window.get_mouse_pos(minifb::MouseMode::Discard) {
                
            let mouse_pos = Vec2::new(x, y);
            mouse_delta = mouse_pos - prev_mouse;
            prev_mouse = mouse_pos;
        }

        //camera controls
        first_person_controls(&mut camera, &window, mouse_delta, dt);
        let (view, projection) = camera.generate_view_projection();
        vs.view = view;
        vs.projection = projection;

        //clear
        output_surface.clear(colour::f32_to_hex(1.0, 0.0, 0.0, 0.0));
        depth_attachment.clear(1.0);

        //draw

        let model_matrices = [
            glam::Mat4::IDENTITY,
            glam::Mat4::from_rotation_y(std::f32::consts::PI * 0.5),
            glam::Mat4::from_rotation_y(std::f32::consts::PI * 1.0),
            glam::Mat4::from_rotation_y(std::f32::consts::PI * 1.5),
            glam::Mat4::from_rotation_x(std::f32::consts::PI * 0.5),
            glam::Mat4::from_rotation_x(std::f32::consts::PI * -0.5),
        ];

        for model in model_matrices { 
            vs.model = model;  
            let (t, i) = vs.dispatch(&vertices, &indices);
            fs.dispatch(&mut output_surface, &mut depth_attachment, &t, &i);
        }

        window.update_with_buffer(output_surface.as_slice(), RESOLUTION_WIDTH, RESOLUTION_HEIGHT).unwrap();
        //dbg!(dt);
    }

}
