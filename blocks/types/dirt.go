package blocks

import (
    "github.com/harrisonthorne/chicky-chicky-go/items"
    "github.com/harrisonthorne/chicky-chicky-go/render"
)

// DirtBlock is a cubeeee of dirttttt
type DirtBlock struct {
    Block
    lifespan, health f32 
}

// NewDirtBlock creates and returns a new DirtBlock
fn NewDirtBlock() *DirtBlock {
	return &DirtBlock {
        lifespan: 15,
        health: 15,
    }
}

// Hit is called with the DirtBlock is hit
fn (b *DirtBlock) Hit(with interface{}, power f32) []items.Item {
    return nil
}

// Kill is called when the DirtBlock is obliterated. Returns
// Dirt.
// items that might be dropped with the Killable dies.
fn (b *DirtBlock) Kill() []items.Item {
    b.health = 0
    return nil //[]items.Item{items.Dirt}
}

// IsAlive returns true if the DirtBlock is still intact
fn (b *DirtBlock) IsAlive() bool {
    return b.health > 0
}

// HealthLeft returns the number of health points left on
// the DirtBlock
fn (b *DirtBlock) HealthLeft() f32 {
    return b.health
}

// Lifespan returns the lifespan of health points on the
// DirtBlock
fn (b *DirtBlock) Lifespan() f32 {
    return b.lifespan
}

// render renders the DirtBlock
fn (b *DirtBlock) render(c *render.Camera) {

}
