use items;
use render;

// DirtBlock is a cubeeee of dirttttt
struct DirtBlock {
    Block
    lifespan, health f32 
}

// newDirtBlock creates and returns a new DirtBlock
fn newDirtBlock() &DirtBlock {
	return &DirtBlock {
        lifespan: 15,
        health: 15,
    }
}

// hit is called with the DirtBlock is hit
fn hit(fn hit(&self, with interface{}, power f32)self, with interface{}, power f32) []items.Item {
    return nil
}

// kill is called when the DirtBlock is obliterated. Returns
// Dirt.
// items that might be dropped with the killable dies.
fn kill(&self) []items.Item {
    b.health = 0
    return nil //[]items.Item{items.Dirt}
}

// IsAlive returns true if the DirtBlock is still intact
fn IsAlive(&self) bool {
    return b.health > 0
}

// HealthLeft returns the number of health points left on
// the DirtBlock
fn HealthLeft(&self) f32 {
    return b.health
}

// Lifespan returns the lifespan of health points on the
// DirtBlock
fn Lifespan(&self) f32 {
    return b.lifespan
}

// render renders the DirtBlock
fn render(&self, c &render.Camera) {

}
