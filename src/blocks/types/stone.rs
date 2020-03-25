/// StoneBlock is a block of rock
struct StoneBlock {
    lifespan: f32,
    health: f32,
}

impl StoneBlock {
    /// Creates a new GrassBlock and returns it
    fn new() -> Self {
        Self {
            lifespan: 100,
            health: 100,
        }
    }
}

impl Block for StoneBlock {

}

