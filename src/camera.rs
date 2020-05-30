use winit::dpi::PhysicalPosition;
use winit::event::*;

#[derive(Debug)]
pub struct Camera {
    eye: cgmath::Point3<f32>,
    target: cgmath::Point3<f32>,
    up: cgmath::Vector3<f32>,
    aspect: f32,
    fov_y: f32,
    z_near: f32,
    z_far: f32,

    matrix: cgmath::Matrix4<f32>,
}

impl Camera {
    fn update_view_projection_matrix(&mut self) {
        // moves the world to be at the position and rotation of the camera.
        let view = cgmath::Matrix4::look_at(self.eye, self.target, self.up);

        // warps the scene for perspective and depth
        let proj = cgmath::perspective(
            cgmath::Deg(self.fov_y),
            self.aspect,
            self.z_near,
            self.z_far,
        );

        self.matrix = proj * view;
    }

    pub fn get_view_projection_matrix(&self) -> &cgmath::Matrix4<f32> {
        &self.matrix
    }

    pub fn jump_position_to(&mut self, position: (f32, f32, f32)) {
        self.eye = position.into();
        self.update_view_projection_matrix()
    }

    pub fn jump_target_to(&mut self, position: (f32, f32, f32)) {
        self.target = position.into();
        self.update_view_projection_matrix()
    }
}

impl Default for Camera {
    fn default() -> Self {
        CameraBuilder::default().build()
    }
}

pub struct CameraBuilder {
    eye: cgmath::Point3<f32>,
    target: cgmath::Point3<f32>,
    up: cgmath::Vector3<f32>,
    aspect: f32,
    fov_y: f32,
    z_near: f32,
    z_far: f32,
}

impl Default for CameraBuilder {
    fn default() -> Self {
        CameraBuilder {
            eye: (0.0, 0.0, -5.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: cgmath::Vector3::unit_y(),
            aspect: 16.0 / 9.0,
            fov_y: 70.0,
            z_near: 0.01,
            z_far: 10000.0,
        }
    }
}

impl CameraBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn position(mut self, pos: (f32, f32, f32)) -> Self {
        self.eye = pos.into();
        self
    }

    /// Sets which position to look at
    pub fn target(mut self, pos: (f32, f32, f32)) -> Self {
        self.target = pos.into();
        self
    }

    pub fn aspect_ratio(mut self, ratio: f32) -> Self {
        self.aspect = ratio;
        self
    }

    /// Convenience method for `aspect_ratio`.
    pub fn viewport_width_height(self, width: f32, height: f32) -> Self {
        self.aspect_ratio(width / height)
    }

    /// Sets the field of view (based on height)
    pub fn fov(mut self, fov: f32) -> Self {
        self.fov_y = fov;
        self
    }

    pub fn clip_near(mut self, distance: f32) -> Self {
        self.z_near = distance;
        self
    }

    pub fn clip_far(mut self, distance: f32) -> Self {
        self.z_far = distance;
        self
    }

    pub fn up_axis(mut self, axis: crate::world::Axis) -> Self {
        use crate::world::Axis;

        self.up = match axis {
            Axis::X => cgmath::Vector3::unit_x(),
            Axis::Y => cgmath::Vector3::unit_y(),
            Axis::Z => cgmath::Vector3::unit_z(),
        };

        self
    }

    pub fn build(self) -> Camera {
        use cgmath::SquareMatrix;

        let mut c = Camera {
            eye: self.eye,
            target: self.target,
            up: self.up,
            aspect: self.aspect,
            fov_y: self.fov_y,
            z_near: self.z_near,
            z_far: self.z_far,
            matrix: cgmath::Matrix4::identity(),
        };

        c.update_view_projection_matrix();

        c
    }
}

pub struct CameraController {
    speed: f32,
    mouse_sensitivity: f32,
    is_up_pressed: bool,
    is_down_pressed: bool,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,

    position: cgmath::Point3<f32>,
    rotation: cgmath::Vector3<f32>,

    last_mouse_position: Option<PhysicalPosition<f64>>,
}

impl CameraController {
    pub fn new(speed: f32, mouse_sensitivity: f32) -> Self {
        Self {
            speed,
            mouse_sensitivity,
            is_up_pressed: false,
            is_down_pressed: false,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,

            position: (0.0, 0.0, 0.0).into(),
            rotation: (0.0, 0.0, 0.0).into(),
            last_mouse_position: None,
        }
    }

    pub fn mouse_moved(&mut self, delta_x: f64, delta_y: f64) {
        self.rotation.x += delta_y as f32 * self.mouse_sensitivity;
        self.rotation.y += delta_x as f32 * self.mouse_sensitivity;

        self.rotation.x = self.rotation.x.min(90.0).max(-90.0);
    }

    pub fn input(&mut self, event: &WindowEvent) {
        if let WindowEvent::KeyboardInput {
            input:
                KeyboardInput {
                    state,
                    virtual_keycode: Some(keycode),
                    ..
                },
                ..
        } = event {
            let is_pressed = *state == ElementState::Pressed;
            match keycode {
                VirtualKeyCode::Space => self.is_up_pressed = is_pressed,
                VirtualKeyCode::LShift => self.is_down_pressed = is_pressed,
                VirtualKeyCode::W | VirtualKeyCode::Up => self.is_forward_pressed = is_pressed,
                VirtualKeyCode::A | VirtualKeyCode::Left => self.is_left_pressed = is_pressed,
                VirtualKeyCode::S | VirtualKeyCode::Down => self.is_backward_pressed = is_pressed,
                VirtualKeyCode::D | VirtualKeyCode::Right => self.is_right_pressed = is_pressed,
                _ => (),
            }
        }
    }

    pub fn update_camera(&mut self, delta_sec: f32, camera: &mut crate::camera::Camera) {
        let sin_of_yaw = self.rotation.y.to_radians().sin();
        let cos_of_yaw = self.rotation.y.to_radians().cos();

        if self.is_left_pressed {
            self.position.x += self.speed * delta_sec * cos_of_yaw;
            self.position.z -= self.speed * delta_sec * sin_of_yaw;
        }
        if self.is_right_pressed {
            self.position.x -= self.speed * delta_sec * cos_of_yaw;
            self.position.z += self.speed * delta_sec * sin_of_yaw;
        }

        if self.is_up_pressed {
            self.position.y += self.speed * delta_sec;
        }
        if self.is_down_pressed {
            self.position.y -= self.speed * delta_sec;
        }

        if self.is_forward_pressed {
            self.position.x += self.speed * delta_sec * sin_of_yaw;
            self.position.z += self.speed * delta_sec * cos_of_yaw;
        }
        if self.is_backward_pressed {
            self.position.x -= self.speed * delta_sec * sin_of_yaw;
            self.position.z -= self.speed * delta_sec * cos_of_yaw;
        }

        camera.eye = self.position;
        camera.target = {
            let sin_of_pitch = self.rotation.x.to_radians().sin();
            let cos_of_pitch = self.rotation.x.to_radians().cos();

            let x = self.position.x + sin_of_yaw * cos_of_pitch;
            let y = self.position.y + sin_of_pitch;
            let z = self.position.z + cos_of_yaw * cos_of_pitch;

            (x, y, z).into()
        };
        camera.update_view_projection_matrix();
    }
}
