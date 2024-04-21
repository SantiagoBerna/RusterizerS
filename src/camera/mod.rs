use crate::math::plane::Plane;

#[derive(Default, Clone)]
pub struct Camera {
    pub position: glam::Vec3,
    pub euler_rotation: glam::Vec3,
    pub fov: f32,
    pub aspect_ratio: f32,
    pub near: f32,
    pub far: f32
}

impl Camera {
    
    pub fn get_rotation(&self) -> glam::Quat {
        glam::Quat::from_euler(glam::EulerRot::YXZ, self.euler_rotation.x, self.euler_rotation.y, self.euler_rotation.z)
    }

    pub fn get_front(&self) -> glam::Vec3 {
        self.get_rotation().mul_vec3(glam::Vec3::NEG_Z)
    }

    pub fn get_right(&self) -> glam::Vec3 {
        self.get_rotation().mul_vec3(glam::Vec3::X)
    }

    pub fn get_up(&self) -> glam::Vec3 {
        self.get_rotation().mul_vec3(glam::Vec3::Y)
    }

    pub fn generate_frustum_perspective(&self) -> Vec<Plane> {

        let half_v = self.far * (self.fov * 0.5).tan();
        let half_h = half_v * self.aspect_ratio;

        let camera_front = self.get_front();
        let camera_right = self.get_right();
        let camera_up = self.get_up();

        let near_vector = camera_front * self.near;
        let near = Plane::from_normal_point(camera_front, self.position + near_vector);

        let far_vector = camera_front * self.far;
        let far = Plane::from_normal_point(-camera_front, self.position + far_vector);

        let right_normal = (far_vector - camera_right * half_h).cross(camera_up).normalize(); 
        let right = Plane::from_normal_point(right_normal, self.position);

        let left_normal = -(far_vector + camera_right * half_h).cross(camera_up).normalize();
        let left = Plane::from_normal_point(left_normal, self.position);

        let top_normal = (far_vector + camera_up * half_v).cross(camera_right).normalize();
        let top = Plane::from_normal_point(top_normal, self.position);

        let bottom_normal = -(far_vector - camera_up * half_v).cross(camera_right).normalize();
        let bottom = Plane::from_normal_point(bottom_normal, self.position);

        vec![near, far, right, left, top, bottom]
    }

    pub fn generate_view_projection(&self) -> (glam::Mat4, glam::Mat4) {
        (
            glam::Mat4::from_rotation_translation(self.get_rotation(), self.position).inverse(),
            glam::Mat4::perspective_rh(self.fov, self.aspect_ratio, self.near, self.far)
        )
    }
}

pub fn first_person_controls(camera : &mut Camera, input: &minifb::Window, mouse_delta: glam::Vec2, dt: f32) {

    //Rotation
    if input.get_mouse_down(minifb::MouseButton::Right) {

        camera.euler_rotation.x -= mouse_delta.x * 0.01;
        camera.euler_rotation.y -= mouse_delta.y * 0.01;

        camera.euler_rotation.y = camera.euler_rotation.y.clamp(
            -std::f32::consts::PI * 0.49, std::f32::consts::PI * 0.49
        );
    }

    let camera_front = camera.get_front();
    let camera_right = camera.get_right();
    let camera_up = camera.get_up();
    
    //Movement
    let mut movement_delta = glam::Vec3::default();

    input.get_keys().iter().for_each(|key|
        match key {
            minifb::Key::S => movement_delta -= camera_front,
            minifb::Key::W => movement_delta += camera_front,
            minifb::Key::A => movement_delta -= camera_right,
            minifb::Key::D => movement_delta += camera_right,
            minifb::Key::E => movement_delta += glam::Vec3::Y,
            minifb::Key::Q => movement_delta += glam::Vec3::NEG_Y,
            _ => (),
        }
    );

    //Normalize and apply movement
    if movement_delta.dot(movement_delta).abs() > std::f32::EPSILON {
        camera.position += movement_delta.normalize() * dt;
    }
}