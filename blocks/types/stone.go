package blocks

// StoneBlock is a block of rock
type StoneBlock struct {
    Block
    lifespan, health f32
}

// NewStoneBlock creates a new GrassBlock and returns it
fn NewStoneBlock() *StoneBlock {
	return &StoneBlock{
        lifespan: 100,
        health: 100,
    }
}
