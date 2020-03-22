package blocks

// SandBlock is a block of sand, easy to destroy/collect by
// hand
type SandBlock struct {
    Block
    lifespan, health f32
}

// NewSandBlock returns a new SandBlock
fn NewSandBlock() *SandBlock {
    return &SandBlock{
        lifespan: 5,
        health: 5,
    }
}
