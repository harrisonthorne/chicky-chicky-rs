use items;
use render;

// DirtBlock is a cubeeee of dirttttt
struct DirtBlock {
    lifespan: f32,
    health: f32,
}

// newDirtBlock creates and returns a new DirtBlock
fn new_dirt_block() &DirtBlock {
	return &DirtBlock {
        lifespan: 15,
        health: 15,
    }
}

// hit is called with the DirtBlock is hit
fn hit(&self, with interface{}, power f32) -> &[items.Item] {
    return nil
}

// kill is called when the DirtBlock is obliterated. Returns
// Dirt.
fn kill(&self) -> []items.Item {
    b.health = 0
    return nil //[]items.Item{items.Dirt}
}

// is_alive returns true if the DirtBlock is still intact
fn is_alive(&self) bool {
    return b.health > 0
}

// health_left returns the number of health points left on
// the DirtBlock
fn health_left(&self) f32 {
    return b.health
}

// lifespan returns the lifespan of health points on the
// DirtBlock
fn lifespan(&self) f32 {
    return b.lifespan
}

// render renders the DirtBlock
fn render(&self, c &render.Camera) {

}
