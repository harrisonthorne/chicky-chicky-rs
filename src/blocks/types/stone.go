package blocks

// StoneBlock is a block of rock
struct StoneBlock {
    Block
    lifespan, health f32
}

// newStoneBlock creates a new GrassBlock and returns it
fn newStoneBlock() *StoneBlock {
	return &StoneBlock{
        lifespan: 100,
        health: 100,
    }
}
