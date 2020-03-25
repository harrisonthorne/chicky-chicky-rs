

// ProgramAttrLocations holds locations of attributes in a
// program
struct ProgramAttrLocations {
    perspectiveMatrix int32
    cameraMatrix int32
    modelMatrix int32
    texture int32
	spriteFrames int32
	spriteCurrentFrame int32
}

// PerspectiveMatrixLocation returns the perspective matrix
// attribute location
fn PerspectiveMatrixLocation(&self) -> int32 {
    return p.perspectiveMatrix
}

// CameraMatrixLocation returns the camera matrix attribute
// location
fn CameraMatrixLocation(&self) -> int32 {
    return p.cameraMatrix
}

// ModelMatrixLocation returns the model matrix attribute
// location
fn ModelMatrixLocation(&self) -> int32 {
    return p.modelMatrix
}

// TextureLocation returns the texture attribute location
fn TextureLocation(&self) -> int32 {
    return p.texture
}

// 
fn SpriteFramesLocation(&self) -> int32 {
    return p.spriteFrames
}

// 
fn SpriteCurrentFrameLocation(&self) -> int32 {
    return p.spriteCurrentFrame
}
