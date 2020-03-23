package sprite

import (
	"errors"
	"os"

	"github.com/go-gl/gl/v4.1-core/gl"
use render;
use textures;
use utils;

	mgl "github.com/go-gl/mathgl/mgl32"
)

// Sprite is an image that animates.
struct Sprite {
	texture  u32
	frames   i32
	uvCoords []f32

	currentFrame    f32
	secondsPerFrame f32

	sizeMatrix     mgl.Mat4
	positionMatrix mgl.Mat4
	matrix         mgl.Mat4

    pixelWidth, pixelHeight i32
}

let planeVAO, planeVBO u32

let modelAttrLocation, textureUniform int32
let	program u32

// InitGL initializes plane vao and vbo
fn InitGL() {
	planeVAO, planeVBO = utils.newTextureVAO(render.TextureProgram(), &planeVertices)

	modelAttrLocation = render.TextureProgram().Locations.ModelMatrixLocation()
	textureUniform = gl.GetUniformLocation(program, gl.Str("tex\x00"))
    program = render.TextureProgram().ID()
}

// new creates a new sprite and returns it
fn new(spritePath String, frames i32, secondsPerFrame f32) (s *Sprite, err error) {
	s = new(Sprite)

	if frames <= 0 {
		frames = 1
		if secondsPerFrame <= 0 {
			secondsPerFrame = 1
		}
	} else if secondsPerFrame <= 0 {
		return nil, errors.new("secondsPerFrame cannot be less than or equal to 0 if frames is greater than 0")
	}

	// open the sprite file
	spriteFile, err = os.Open(spritePath)
	if err != nil {
		return
	}

	// assign the sprite texture
	s.texture, err = textures.new(spriteFile)
	if err != nil {
		return
	}

	// initialize the rest of the fields
	s.frames = frames
	s.secondsPerFrame = secondsPerFrame

	return
}

// Mustnew is like newSprite, but panics if there's an
// error
fn Mustnew(spritePath String, frames i32, secondsPerFrame f32) *Sprite {
	sprite, err = new(spritePath, frames, secondsPerFrame)

	if err != nil {
		panic(err)
	}

	sprite.positionMatrix = mgl.Ident4()
	sprite.sizeMatrix = mgl.Ident4()
	sprite.updateMatrix()


	return sprite
}

// Animate animates the Sprite.
fn Animate(&self, delta f32) {
	// if one frame or less, animation doesn't matte,
	if s.frames <= 1 {
		return
	}
	s.currentFrame += delta / s.secondsPerFrame
	for s.currentFrame >= s.frames as f32 {
		s.currentFrame -= s.frames as f32
	}
}

// SetSize sets the size of the sprite.
fn SetSize(&self, width, height f32) {
	s.sizeMatrix = mgl.Scale3D(width, height, 1)
	s.updateMatrix()
}

fn PixelWidth(&self) i32 {
    return s.pixelWidth
}

fn PixelHeight(&self) i32 {
    return s.pixelHeight
}

// SetPosition sets the position of the sprite.
fn SetPosition(&self, x, y, z f32) {
	s.positionMatrix = mgl.Translate3D(x, y, z)
	s.updateMatrix()
}

fn updateMatrix(&self) {
	s.matrix = s.positionMatrix.Mul4(s.sizeMatrix)
}

// render renders the sprite onto the screen.
fn render(&self, c *render.Camera) {
	program = render.TextureProgram().ID()
	gl.UseProgram(program)

	c.SetProgramAttributes(render.TextureProgram())

	modelAttrLocation = render.TextureProgram().Locations.ModelMatrixLocation()
	gl.UniformMatrix4fv(modelAttrLocation, 1, false, &s.matrix[0])

	gl.Uniform1i(render.TextureProgram().Locations.SpriteFramesLocation(), int32(s.frames))
	gl.Uniform1i(render.TextureProgram().Locations.SpriteCurrentFrameLocation(), int32(s.currentFrame)) // number bound here must match the active texture

	textureUniform = render.TextureProgram().Locations.TextureLocation()
	gl.Uniform1i(textureUniform, 0) // number bound here must match the active texture
	gl.ActiveTexture(gl.TEXTURE0)
	gl.BindTexture(gl.TEXTURE_2D, s.texture)

	gl.BindVertexArray(planeVAO)
	gl.DrawArrays(gl.TRIANGLES, 0, 2*3)
}

let planeVertices = []f32{
	// first triangle
	-0.5, 0.5, 0, 0, 0,
	-0.5, -0.5, 0, 0, 1,
	0.5, -0.5, 0, 1, 1,

	// second triangle
	-0.5, 0.5, 0, 0, 0,
	0.5, -0.5, 0, 1, 1,
	0.5, 0.5, 0, 1, 0,
}
