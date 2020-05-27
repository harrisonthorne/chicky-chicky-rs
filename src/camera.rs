use winit::event::*;

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
        let proj = cgmath::perspective(cgmath::Deg(self.fov_y), self.aspect, self.z_near, self.z_far);

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
            aspect: 16.0/9.0,
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
    pub fn viewport_width_height(mut self, width: f32, height: f32) -> Self {
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
    is_up_pressed: bool,
    is_down_pressed: bool,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            is_up_pressed: false,
            is_down_pressed: false,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
        }
    }

    pub fn process_events(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state,
                        virtual_keycode: Some(keycode),
                        ..
                    },
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;
                match keycode {
                    VirtualKeyCode::Space => {
                        self.is_up_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::LShift => {
                        self.is_down_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::W | VirtualKeyCode::Up => {
                        self.is_forward_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::A | VirtualKeyCode::Left => {
                        self.is_left_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::S | VirtualKeyCode::Down => {
                        self.is_backward_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::D | VirtualKeyCode::Right => {
                        self.is_right_pressed = is_pressed;
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }

    pub fn update_camera(&self, camera: &mut crate::camera::Camera) {
        use cgmath::InnerSpace;
        let forward = (camera.target - camera.eye).normalize();

        if self.is_forward_pressed {
            camera.eye += forward * self.speed;
        }
        if self.is_backward_pressed {
            camera.eye -= forward * self.speed;
        }

        let right = forward.cross(camera.up);

        if self.is_right_pressed {
            camera.eye += right * self.speed;
        }
        if self.is_left_pressed {
            camera.eye -= right * self.speed;
        }
    }
}
