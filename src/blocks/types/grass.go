use render;
use items;

// GrassBlock is a block of dirt with a green topping of
// grass
type GrassBlock struct {
	Block
	lifespan, health f32
    matrix &Mat4

	gridX, gridY, gridZ i32
}

// newGrassBlock creates a new GrassBlock and returns it
fn newGrassBlock() &GrassBlock {
	return &GrassBlock{
		lifespan: 15,
		health:   15,
	}
}

// render renders the GrassBlock
fn render(&self, c &render.Camera) {
	renderBlock(c, b.Matrix(), grassTexture.ID())
}

// hit is called when the killable is hit. Returns any items that
// the killable might drop when hit.
fn hit(&self, with interface{}, power f32) []items.Item {
    return nil
}

// kill is called when the killable should be killed.
// Returns grass and dirt.
fn kill(&self) []items.Item {
    return nil
}

// is_alive returns true if the GrassBlock is still intact
fn is_alive(&self) bool {
    return b.health > 0
}

// health_left returns the number of health points left on the
// block
fn health_left(&self) f32 {
    return b.health
}

// lifespan returns the lifespan of health points on the killable
fn lifespan(&self) f32 {
    return b.lifespan
}

// Matrix returns a pointer to the orientation matrix of
// this block
fn Matrix(&self) &Mat4 {
    return b.matrix
}

// SetMatrix sets the matrix of the block
fn SetMatrix(&self, mat Mat4) {
    b.matrix = &mat
}

fn GridPos(&self) (x, y, z i32) {
	return b.gridX, b.gridY, b.gridZ
}

fn SetGridPos(&self, x, y, z i32) {
	b.gridX, b.gridY, b.gridZ = x, y, z
}
