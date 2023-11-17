

use data::vertex_set::VertexSet;
use glam::Quat;
use minifb::MouseMode;
use minifb::ScaleMode;
use minifb::Window;
use minifb::WindowOptions;
use std::f32::consts::PI;
use std::sync::Arc;
use glam::Mat4;
use glam::Vec3;
use glam::Vec2;
use std::time::Instant;
use std::path::Path;
use minifb::Key;

mod display;
use display::*;

mod data;
use data::*;

mod renderer;
use renderer::*;

mod utility;

const RESOLUTION_WIDTH: usize = 640; 
const RESOLUTION_HEIGHT: usize = 480; 
const UPSCALE: usize = 1;

fn main() {
    
    let mut window_options = WindowOptions::default();
    window_options.scale_mode = ScaleMode::Stretch;
    window_options.resize = false;
    
    let mut window = match Window::new("Rasterizing with Rust", RESOLUTION_WIDTH * UPSCALE, RESOLUTION_HEIGHT * UPSCALE, window_options) {
        Ok(win) => win,
        Err(err) => {
            println!("Unable to create window {}", err);
            return;
        }
    };
    
    let mut renderer = Renderer::new();
    let mut surface = Surface::new(RESOLUTION_WIDTH, RESOLUTION_HEIGHT);

    let triangle_vertices = Arc::new(vec![

        -0.5,  0.5, 0.5,
        -0.5, -0.5, 0.5,
        0.5, -0.5, 0.5,
        0.5,  0.5, 0.5,
        
    ]);

    let triange_uvs = Arc::new(vec![
        0.0, 1.0,
        0.0, 0.0,
        1.0, 0.0,
        1.0, 1.0
    ]);

    let triangle_indices = Arc::new(vec![
        0, 1, 2,
        2, 3, 0
    ]);

    let mut vertex_data = VertexSet::new();
    vertex_data.set_attribute(vertex_set::VertexAttributes::Position, triangle_vertices);
    vertex_data.set_attribute(vertex_set::VertexAttributes::TextureUV, triange_uvs);
    vertex_data.set_indices(triangle_indices);

    let image_result = load_texture_from_file(Path::new("assets/rock.jpg"));
    let sampler = image_result.unwrap();

    renderer.projection_matrix = glam::Mat4::perspective_rh(
        PI / 4.0,
        RESOLUTION_WIDTH as f32 / RESOLUTION_HEIGHT as f32,
        0.01, 100.0
    );

    let mut angle = 0.0;
    let mut camera_pos = Vec3::new(0.0, 0.0, 10.0);
    let mut camera_rot = Vec2::default();
    let mut prev_mouse_pos = Vec2::default();
    let mut mouse_pos = prev_mouse_pos;
    let mut dt = 0.0;


    while window.is_open() {

        let now = Instant::now();

        //Input
        if let Some((x, y)) = window.get_mouse_pos(MouseMode::Discard) {
            prev_mouse_pos = mouse_pos;
            mouse_pos = Vec2::new(x, y);
            let mouse_delta = mouse_pos - prev_mouse_pos;

            if window.get_mouse_down(minifb::MouseButton::Right) {
                camera_rot.x = camera_rot.x - mouse_delta.x * 0.01;
                camera_rot.y = camera_rot.y - mouse_delta.y * 0.01;
                camera_rot.y = camera_rot.y.clamp(-PI / 2.1, PI / 2.1);
            }
        }

        let camera_quat = Quat::from_euler(glam::EulerRot::YXZ, camera_rot.x, camera_rot.y, 0.0);

        window.get_keys().iter().for_each(|key|
            match key {
                Key::W => camera_pos = camera_pos + camera_quat.mul_vec3(Vec3::new(0.0, 0.0, -1.0)) * dt,
                Key::S => camera_pos = camera_pos + camera_quat.mul_vec3(Vec3::new(0.0, 0.0, 1.0)) * dt,
                Key::A => camera_pos = camera_pos + camera_quat.mul_vec3(Vec3::new(-1.0, 0.0, 0.0)) * dt,
                Key::D => camera_pos = camera_pos + camera_quat.mul_vec3(Vec3::new(1.0, 0.0, 0.0)) * dt,
                Key::E => camera_pos = camera_pos + Vec3::new(0.0, 1.0, 0.0) * dt,
                Key::Q => camera_pos = camera_pos + Vec3::new(0.0, -1.0, 0.0) * dt,
                _ => (),
            }
        );

        let camera_transform = Mat4::from_rotation_translation(
            camera_quat,
             camera_pos
        );

        angle = angle + dt;

        //Rendering
        surface.clear(0, 1.0);

        renderer.view_matrix = glam::Mat4::look_at_rh(
            camera_pos,
            camera_pos + camera_transform.transform_vector3(Vec3::new(0.0, 0.0, -1.0)),
            Vec3::new(0.0, 1.0, 0.0)
        );

        renderer.bind_vertex_set(Some(vertex_data.clone()));
        renderer.bind_sampler(TextureSlot::Diffuse, Some(sampler.clone()));

        for i in 0..4 {

            let model_matrix = Mat4::from_rotation_x(i as f32 * PI * 0.5);
            renderer.draw_buffer(&mut surface, &model_matrix, 2);
        }

        let model_matrix = Mat4::from_rotation_y(PI * 0.5);
        renderer.draw_buffer(&mut surface, &model_matrix, 2);

        let model_matrix = Mat4::from_rotation_y(-PI * 0.5);
        renderer.draw_buffer(&mut surface, &model_matrix, 2);

        window.update_with_buffer(surface.data(), RESOLUTION_WIDTH, RESOLUTION_HEIGHT).unwrap();

        println!("{}", now.elapsed().as_millis());
        dt = now.elapsed().as_secs_f32();

    }
}
