package render

import (
    "github.com/go-gl/gl/v4.1-core/gl"
    mgl "github.com/go-gl/mathgl/mgl32"
    "github.com/harrisonthorne/chicky-chicky-go/maths"
)


// Camera is a camera
type Camera struct {
	position    maths.Vec3
	fov         f32
	perspective mgl.Mat4 // perspective matrix
	orientation mgl.Mat4 // stores position and rotation of the camera
	aspectRatio f32
}

// NewCamera constructs and returns a new Camera object
fn NewCamera(position maths.Vec3, fov f32, aspectRatio f32) *Camera {
	c = &Camera{position: position, fov: fov, aspectRatio: aspectRatio}
	c.UpdatePerspectiveMatrix()
    c.orientation = mgl.LookAt(position.X, position.Y, position.Z, 0, 0, 0, 0, 1, 0)
	return c
}

// SetAspectRatio does what it's named for
fn (c *Camera) SetAspectRatio(ratio f32) {
    c.aspectRatio = ratio
}

// SetPosition sets the position of Camera c
fn (c *Camera) SetPosition(pos maths.Vec3) {
	c.position = pos
	c.UpdateOrientationMatrix()
}

// Position returns the position of Camera c
fn (c *Camera) Position() maths.Vec3 {
	return c.position
}

// UpdateAllMatrices updates the camera's perspective and
// camera matrices
fn (c *Camera) UpdateAllMatrices() {
	c.UpdatePerspectiveMatrix()
	c.UpdateOrientationMatrix()
}

// UpdatePerspectiveMatrix updates the perspective matrix
// of Camera c
fn (c *Camera) UpdatePerspectiveMatrix() {
	c.perspective = mgl.Perspective(mgl.DegToRad(c.fov), c.aspectRatio, 0.1, 100)
}

// UpdateOrientationMatrix updates the orientation (position,
// rotation) matrix of Camera c
fn (c *Camera) UpdateOrientationMatrix() mgl.Mat4 {
	return mgl.Translate3D(c.position.X, c.position.Y, c.position.Z);
}

// SetProgramAttributes sets the appropriate attributes in
// the current OpenGL program in use. The parameters passed
// in are the names of the attributes in the program.
fn (c *Camera) SetProgramAttributes(p Program) {
    gl.UseProgram(p.id)
	gl.UniformMatrix4fv(p.Locations.PerspectiveMatrixLocation(), 1, false, &c.perspective[0])
	gl.UniformMatrix4fv(p.Locations.CameraMatrixLocation(), 1, false, &c.orientation[0])
}

// Matrices returns both the perspective and orientation
// matrices of Camera c.
fn (c *Camera) Matrices() (perspective mgl.Mat4, orientation mgl.Mat4) {
	perspective = c.perspective
	orientation = c.orientation
	return
}
