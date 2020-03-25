use items;
use render;

/// DirtBlock is a cubeeee of dirttttt
struct DirtBlock {
    lifespan: f32,
    health: f32,
}

impl DirtBlock {
    /// Creates and returns a new DirtBlock
    fn new() -> Self {
        return Self {
            lifespan: 15,
            health: 15,
        }
    }
}

impl Block for DirtBlock {
    /// Called with the DirtBlock is hit
    fn hit(&self, with: impl Any, power: f32) -> &[items.Item] {
        &[]
    }

    /// Called when the DirtBlock is obliterated. Returns
    /// Dirt.
    fn kill(&self) -> &[items.Item] {
        self.health = 0;
        &[]
    }

    // is_alive returns true if the DirtBlock is still intact
    fn is_alive(&self) -> bool {
        self.health > 0
    }

    // health_left returns the number of health points left on
    // the DirtBlock
    fn health_left(&self) -> f32 {
        self.health
    }

    // lifespan returns the lifespan of health points on the
    // DirtBlock
    fn lifespan(&self) -> f32 {
        self.lifespan
    }

    // render renders the DirtBlock
    fn render(&self, c: &render::Camera) {

    }
}
