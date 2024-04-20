

use camera::Camera;
use data::vertex_set::VertexSet;
use glam::Quat;
use minifb::MouseMode;
use minifb::ScaleMode;
use minifb::Window;
use minifb::WindowOptions;
use math::f32_to_hex;
use math::plane::clip_polygon;
use glam::Vec4;
use std::f32::consts::PI;
use std::sync::Arc;
use glam::Mat4;
use glam::Vec3;
use glam::Vec2;
use std::time::Instant;
use std::path::Path;
use minifb::Key;


mod data;
use data::*;

mod renderer;
use renderer::*;

mod math;

mod texture;
use texture::*;

use crate::camera::first_person_controls;

mod camera;

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
    let mut window_options = WindowOptions::default();
    window_options.scale_mode = ScaleMode::Stretch;
    window_options.resize = false;

    Window::new("Rasterizing with Rust", RESOLUTION_WIDTH * UPSCALE, RESOLUTION_HEIGHT * UPSCALE, window_options)
}

fn main() {
    
    let mut window = create_window().unwrap();
    
    let texture = load_image_file(Path::new("assets/icon.png")).unwrap();

    let mut output_surface = Texture::new(RESOLUTION_WIDTH, RESOLUTION_HEIGHT);
    let mut depth_attachment = DepthTexture::new(RESOLUTION_WIDTH, RESOLUTION_HEIGHT);

    let mut timer = Instant::now();

    //Camera
    let mut camera = Camera::default();
    camera.position = Vec3::new(0.0, 0.0, 1.0);
    camera.aspect_ratio = (RESOLUTION_WIDTH as f32) / (RESOLUTION_HEIGHT as f32);
    camera.fov = std::f32::consts::PI * 0.25;
    camera.near = 0.1;
    camera.far = 10.0;

    //Shader abstractions
    let mut vs = VertexShader::default();
    let mut fs = FragmentShader::default();
    let mut ls = DebugLineShader::default();

    //Setting up vertices
    let mut vertices = VertexInput::default();
    vertices.positions = QUAD_VERTEX_POSITIONS.to_vec();
    vertices.colours = QUAD_VERTEX_UVS.iter().map(|vec2|{ Vec3::new(vec2.x, vec2.y, 1.0) }).collect();

    let indices = QUAD_INDICES.to_owned();
    let mut prev_mouse = Vec2::default();

    while window.is_open() {

        //Delta Time
        let dt = timer.elapsed().as_secs_f32();
        timer = Instant::now(); //reset timer

        //Mouse delta
        let mut mouse_delta = Vec2::default();
        if let Some((x, y)) = window.get_mouse_pos(minifb::MouseMode::Discard) {
                
            let mouse_pos = Vec2::new(x, y);
            mouse_delta = mouse_pos - prev_mouse;
            prev_mouse = mouse_pos;
        }

        //camera controls
        first_person_controls(&mut camera, &window, mouse_delta, dt);
        vs.camera = camera.clone();

        //clear
        output_surface.clear(math::f32_to_hex(1.0, 0.0, 0.0, 0.0));
        depth_attachment.clear(1.0);

        //draw
        let (t, i) = vs.dispatch(&vertices, &indices[0..3]);
        fs.dispatch(&mut output_surface, &mut depth_attachment, &t, &i);

        //Frustum testing
        // let frustum = camera.generate_frustum_perspective();
                
        // let mut triangle_list = Vec::new();
        // for i in 0..3 {
        //     triangle_list.push(vertices.positions[i].extend(1.0));
        // }

        // let clipped = clip_polygon(&triangle_list, &frustum);
        // dbg!(clipped.len());

        // //debug lines
        // ls.camera = camera.clone();
        
        // if !clipped.is_empty() {

        //     let mut triangulation: Vec<(Vec3, Vec3)> = Vec::new();
        //     for i in 2..clipped.len()-1 {
        //         triangulation.push((clipped[0].truncate(), clipped[i].truncate()));
        //     }

        //     for i in 0..clipped.len() {

        //         let current_point = clipped[i];
        //         let next_point = clipped[(i+1) % clipped.len()];
        //         triangulation.push((current_point.truncate(), next_point.truncate()));
        //     }
            
        //     ls.dispatch(&mut output_surface, &triangulation);
        // }

        window.update_with_buffer(output_surface.as_slice(), RESOLUTION_WIDTH, RESOLUTION_HEIGHT).unwrap();
        //dbg!(dt);
    }

//     let mut renderer = Renderer::new();
//     let mut surface = Surface::new(RESOLUTION_WIDTH, RESOLUTION_HEIGHT);

//     let triangle_vertices = Arc::new(vec![

//         -0.5,  0.5, 0.5,
//         -0.5, -0.5, 0.5,
//         0.5, -0.5, 0.5,
//         0.5,  0.5, 0.5,
        
//     ]);

//     let triange_uvs = Arc::new(vec![
//         0.0, 1.0,
//         0.0, 0.0,
//         1.0, 0.0,
//         1.0, 1.0
//     ]);

//     let triangle_indices = Arc::new(vec![
//         0, 1, 2,
//         2, 3, 0
//     ]);

//     let mut vertex_data = VertexSet::new();
//     vertex_data.set_attribute(vertex_set::VertexAttributes::Position, triangle_vertices);
//     vertex_data.set_attribute(vertex_set::VertexAttributes::TextureUV, triange_uvs);
//     vertex_data.set_indices(triangle_indices);

//     let image_result = load_texture_from_file(Path::new("assets/rock.jpg"));
//     let sampler = image_result.unwrap();

//     renderer.projection_matrix = glam::Mat4::perspective_rh(
//         PI / 4.0,
//         RESOLUTION_WIDTH as f32 / RESOLUTION_HEIGHT as f32,
//         0.01, 100.0
//     );

//     let mut angle = 0.0;
//     let mut camera_pos = Vec3::new(0.0, 0.0, 10.0);
//     let mut camera_rot = Vec2::default();
//     let mut prev_mouse_pos = Vec2::default();
//     let mut mouse_pos = prev_mouse_pos;
//     let mut dt = 0.0;


//     while window.is_open() {

//         //Rendering
//         surface.clear(0, 1.0);

//         renderer.view_matrix = glam::Mat4::look_at_rh(
//             camera_pos,
//             camera_pos + camera_transform.transform_vector3(Vec3::new(0.0, 0.0, -1.0)),
//             Vec3::new(0.0, 1.0, 0.0)
//         );

//         renderer.bind_vertex_set(Some(vertex_data.clone()));
//         renderer.bind_sampler(TextureSlot::Diffuse, Some(sampler.clone()));

//         for i in 0..4 {

//             let model_matrix = Mat4::from_rotation_x(i as f32 * PI * 0.5);
//             renderer.draw_buffer(&mut surface, &model_matrix, 2);
//         }

//         let model_matrix = Mat4::from_rotation_y(PI * 0.5);
//         renderer.draw_buffer(&mut surface, &model_matrix, 2);

//         let model_matrix = Mat4::from_rotation_y(-PI * 0.5);
//         renderer.draw_buffer(&mut surface, &model_matrix, 2);

//         window.update_with_buffer(surface.data(), RESOLUTION_WIDTH, RESOLUTION_HEIGHT).unwrap();

//         println!("{}", now.elapsed().as_millis());
//         dt = now.elapsed().as_secs_f32();

//     }
}
