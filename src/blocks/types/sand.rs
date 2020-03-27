/// SandBlock is a block of sand, easy to destroy/collect by
/// hand
struct SandBlock {
    lifespan: f32, 
    health: f32,
}

impl SandBlock {
    /// Returns a new SandBlock
    fn new() -> Self {
        Self {
            lifespan: 5,
            health: 5,
        }
    }
}

impl Block for SandBlock {

}
