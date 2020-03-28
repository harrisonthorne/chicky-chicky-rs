use render;
use items;

/// GrassBlock is a block of dirt with a green topping of
/// grass
struct GrassBlock {
    lifespan: f32, health: f32,
    matrix: Mat4,

    gridX: i32, gridY: i32, gridZ: i32,
}

impl GrassBlock {
    /// Creates a new GrassBlock and returns it
    fn new() -> Self {
        Self {
            lifespan: 15,
            health:   15,
        }
    }

}

impl Block for GrassBlock {
    // render renders the GrassBlock
    fn render(&self, c: &render.Camera) {
        renderBlock(c, self.Matrix(), grassTexture.ID())
    }

    // hit is called when the killable is hit. Returns any items that
    // the killable might drop when hit.
    fn hit(&self, with: impl Any, power: f32) -> &[items::Item] {
        &[]
    }

    // kill is called when the killable should be killed.
    // Returns grass and dirt.
    fn kill(&self) -> [items::Item] {
        &[]
    }

    // is_alive returns true if the GrassBlock is still intact
    fn is_alive(&self) -> bool {
        self.health > 0
    }

    // health_left returns the number of health points left on the
    // block
    fn health_left(&self) -> f32 {
        self.health
    }

    // lifespan returns the lifespan of health points on the killable
    fn lifespan(&self) -> f32 {
        self.lifespan
    }

    // Matrix returns a pointer to the orientation matrix of
    // this block
    fn matrix(&self) -> &Mat4 {
        self.matrix
    }

    // SetMatrix sets the matrix of the block
    fn set_matrix(&self, mat Mat4) {
        self.matrix = &mat
    }

    fn get_grid_pos(&self) -> (i32, i32, i32) {
        (self.gridX, self.gridY, self.gridZ)
    }

    fn set_grid_pos(&self, x: i32, y: i32, z: i32) {
        self.gridX = x;
        self.gridY = y;
        self.gridZ = z;
    }

}
