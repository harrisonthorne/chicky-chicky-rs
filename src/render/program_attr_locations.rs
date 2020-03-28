/// Holds the locations of attributes in a
/// shader program
pub struct ProgramAttrLocations {
    perspective_matrix: i32,
    camera_matrix: i32,
    model_matrix: i32,
    texture: i32,
    sprite_frames: i32,
    sprite_current_frame: i32,
}

impl ProgramAttrLocations {
    /// Returns the perspective matrix
    /// attribute location
    pub fn perspective_matrix_location(&self) -> i32 {
        self.perspective_matrix
    }

    /// Returns the camera matrix attribute
    /// location
    pub fn camera_matrix_location(&self) -> i32 {
        self.camera_matrix
    }

    /// Returns the model matrix attribute
    /// location
    pub fn model_matrix_location(&self) -> i32 {
        self.model_matrix
    }

    /// Returns the texture attribute location
    pub fn texture_location(&self) -> i32 {
        self.texture
    }

    pub fn sprite_frames_location(&self) -> i32 {
        self.sprite_frames
    }

    pub fn sprite_current_frame_location(&self) -> i32 {
        self.sprite_current_frame
    }
}
