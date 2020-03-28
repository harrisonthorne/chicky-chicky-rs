/// A camera.
pub struct Camera {
    position:    Vec3,
    fov:         f32,
    perspective: Mat4, // perspective matrix
    orientation: Mat4, // stores position and rotation of the camera
    aspect_ratio: f32,
}

impl Camera {
    /// Constructs and returns a new Camera object.
    fn new(position: Vec3, fov: f32, aspectRatio: f32) -> Self {
        let c = Self{position, fov, aspect_ratio};
        c.update_perspective_matrix();
        c.orientation = look_at(position.x, position.y, position.z, 0, 0, 0, 0, 1, 0);

        c
    }

    /// Does what it's named for.
    fn set_aspect_ratio(&mut self, ratio: f32) {
        self.aspect_ratio = ratio;
    }

    /// Sets the position of the Camera.
    fn set_position(&mut self, pos: Vec3) {
        self.position = pos;
        self.update_orientation_matrix();
    }

    /// Returns the position of the Camera.
    fn position(&self) -> Vec3 {
        self.position
    }

    /// Updates the Camera's perspective and
    /// orientation matrices.
    fn update_all_matrices(&mut self) {
        self.update_perspective_matrix();
        self.update_orientation_matrix();
    }

    /// Updates the perspective matrix
    /// of the Camera.
    fn update_perspective_matrix(&mut self) {
        self.perspective = perspective(deg_to_rad(self.fov), self.aspect_ratio, 0.1, 100)
    }

    /// Updates the orientation (position,
    /// rotation) matrix of the Camera.
    fn update_orientation_matrix(&mut self) {
        self.orientation = translate_3d(self.position.x, self.position.y, self.position.z)
    }

    /// Sets the appropriate attributes in
    /// the current OpenGL program in use. The parameters passed
    /// in are the names of the attributes in the program.
    fn set_program_attributes(&self, p: Program) {
        gl.use_program(p.id);
        gl.uniform_matrix_4fv(p.locations.perspective_matrix_location(), 1, false, &self.perspective[0]);
        gl.uniform_matrix_4fv(p.locations.camera_matrix_location(), 1, false, &self.orientation[0]);
    }

    /// Returns both the perspective and orientation
    /// matrices of the Camera.
    fn get_matrices(&self) -> (Mat4, Mat4) {
        (self.perspective, self.orientation)
    }
}
