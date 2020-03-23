use blocks;
use maths;
use render;
use rand;

const MAX_WORLD_SIZE: i32 = 1024; // in chunks. 1024 yields thousands upon thousands of blocks in each direction

// World contains a slice of Plots
struct World {
    terrain: VecDeque<VecDeque<VecDeque<Chunk>>>,
    seed:    i64,

    height_gradient_vectors:      [[Vec2; MAX_WORLD_SIZE]; MAX_WORLD_SIZE],
    humidity_gradient_vectors:    [[Vec2; MAX_WORLD_SIZE]; MAX_WORLD_SIZE],
    temperature_gradient_vectors: [[Vec2; MAX_WORLD_SIZE]; MAX_WORLD_SIZE],

    caveGradientVectors: [[[Vec3; MAX_WORLD_SIZE]; MAX_WORLD_SIZE]; MAX_WORLD_SIZE],
    oreGradientVectors:  [[[Vec3; MAX_WORLD_SIZE]; MAX_WORLD_SIZE]; MAX_WORLD_SIZE],
    render_distance: i32,
}

impl World {
    fn new(seed: i64) -> Self {
        let height_gradient_vectors = generate_2d_gradient_map(seed);
        let humidity_gradient_vectors = generate_2d_gradient_map(seed + 1);
        let temperature_gradient_vectors = generate_2d_gradient_map(seed + 2);

        let cave_gradient_vectors = generate_3d_gradient_map(seed + 3);
        let ore_gradient_vectors = generate_3d_gradient_map(seed + 4);

        Self {
            terrain: Vec::new(),
            seed,
            height_gradient_vectors,
            humidity_gradient_vectors,
            temperature_gradient_vectors,
            cave_gradient_vectors,
            ore_gradient_vectors,
        }
    }

    /// Serializes the world into a big String that can be parsed to
    /// recreate the world.
    pub fn stringify(&self) -> String {
        return ""
    }

    pub fn generate_chunk(&self, x, y, z i32) {

    }

    fn render(&self, c *render.Camera) {
        eyeChunkX = c.Position().X / (blocks.BLOCK_WIDTH * blocks.CHUNK_SIZE) as i32;
        eyeChunkY = c.Position().Y / (blocks.BLOCK_WIDTH * blocks.CHUNK_SIZE) as i32;
        eyeChunkZ = c.Position().Z / (blocks.BLOCK_WIDTH * blocks.CHUNK_SIZE) as i32;

        for x in (eyeChunkX - w.renderDistance)..(eyeChunkX+w.renderDistance) {
            if x < 0 || x >= w.terrain.len() {
                continue;
            }
            for y in (eyeChunkY - w.renderDistance)..(eyeChunkY+w.renderDistance) {
                if y < 0 || y >= w.terrain[x].len() {
                    continue;
                }
                for z in (eyeChunkZ - w.renderDistance)..(eyeChunkZ+w.renderDistance) {
                    if z < 0 || z >= w.terrain[x][y].len() {
                        continue;
                    }
                    w.terrain[x][y][z].render(c);
                }
            }
        }
    }
}


fn generate_2d_gradient_map(seed: i64) -> [[Vec2; MAX_WORLD_SIZE]; MAX_WORLD_SIZE] {
    r = rand.new(rand.new_source(seed)) as i64;
    m = [MAX_WORLD_SIZE][MAX_WORLD_SIZE]maths.Vec2{};

    for x in 0..m.len(); x++ {
        for y in 0..m[x].len(); y++ {
            m[x][y] = maths.Vec2{r.f32(), r.f32()}
        }
    }

    return m
}

fn generate_3d_gradient_map(seed: i64) -> [MAX_WORLD_SIZE][MAX_WORLD_SIZE][MAX_WORLD_SIZE]maths.Vec3 {
    let r = rand.new(rand.newSource(seed)) as i64;
    let m = [MAX_WORLD_SIZE][MAX_WORLD_SIZE][MAX_WORLD_SIZE]maths.Vec3{};

    for x in 0..m.len() {
        for y in 0..m[x].len() {
            for z 0..m[x][y].len() {
                m[x][y][z] = maths.Vec3{r.f32(), r.f32(), r.f32()}
            }
        }
    }
}
