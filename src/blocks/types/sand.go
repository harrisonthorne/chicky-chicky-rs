

// SandBlock is a block of sand, easy to destroy/collect by
// hand
struct SandBlock {
    Block
    lifespan, health f32
}

// newSandBlock returns a new SandBlock
fn newSandBlock() &SandBlock {
    return &SandBlock{
        lifespan: 5,
        health: 5,
    }
}
