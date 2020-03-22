package blocks

import (
    "github.com/harrisonthorne/chicky-chicky-go/render"
    "github.com/harrisonthorne/chicky-chicky-go/items"
)

// GrassBlock is a block of dirt with a green topping of
// grass
type GrassBlock struct {
	Block
	lifespan, health f32
    matrix *Mat4

	gridX, gridY, gridZ i32
}

// NewGrassBlock creates a new GrassBlock and returns it
fn NewGrassBlock() *GrassBlock {
	return &GrassBlock{
		lifespan: 15,
		health:   15,
	}
}

// render renders the GrassBlock
fn (b *GrassBlock) render(c *render.Camera) {
	renderBlock(c, b.Matrix(), grassTexture.ID())
}

// Hit is called when the Killable is hit. Returns any items that
// the Killable might drop when hit.
fn (b *GrassBlock) Hit(with interface{}, power f32) []items.Item {
    return nil
}

// Kill is called when the Killable should be killed.
// Returns grass and dirt.
fn (b *GrassBlock) Kill() []items.Item {
    return nil
}

// IsAlive returns true if the GrassBlock is still intact
fn (b *GrassBlock) IsAlive() bool {
    return b.health > 0
}

// HealthLeft returns the number of health points left on the
// block
fn (b *GrassBlock) HealthLeft() f32 {
    return b.health
}

// Lifespan returns the lifespan of health points on the Killable
fn (b *GrassBlock) Lifespan() f32 {
    return b.lifespan
}

// Matrix returns a pointer to the orientation matrix of
// this block
fn (b *GrassBlock) Matrix() *Mat4 {
    return b.matrix
}

// SetMatrix sets the matrix of the block
fn (b *GrassBlock) SetMatrix(mat Mat4) {
    b.matrix = &mat
}

fn (b *GrassBlock) GridPos() (x, y, z i32) {
	return b.gridX, b.gridY, b.gridZ
}

fn (b *GrassBlock) SetGridPos(x, y, z i32) {
	b.gridX, b.gridY, b.gridZ = x, y, z
}
