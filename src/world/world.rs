use blocks;
use maths;
use render;
use rand;

const MAX_WORLD_SIZE: i32 = 1024; // in chunks. 256 yields thousands and thousands of blocks in each direction

// World contains a slice of Plots
struct World {
    terrain: Vec<Vec<Vec<Chunk>>>,
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

    /// Serializes the world into a big string that can be parsed to
    /// recreate the world.
    pub fn stringify(&self) -> String {
        return ""
    }

    pub fn generate_chunk(&self, x, y, z i32) {

    }

    fn render(&self, c *render.Camera) {
        eyeChunkX = i32(c.Position().X / (blocks.BlockWidth * blocks.ChunkSize))
            eyeChunkY = i32(c.Position().Y / (blocks.BlockWidth * blocks.ChunkSize))
            eyeChunkZ = i32(c.Position().Z / (blocks.BlockWidth * blocks.ChunkSize))

            for x = eyeChunkX - w.renderDistance; x < eyeChunkX+w.renderDistance; x++ {
                if x < 0 || x >= len(w.terrain) {
                    continue
                }
                for y = eyeChunkY - w.renderDistance; y < eyeChunkY+w.renderDistance; y++ {
                    if y < 0 || y >= len(w.terrain[x]) {
                        continue
                    }
                    for z = eyeChunkZ - w.renderDistance; z < eyeChunkZ+w.renderDistance; z++ {
                        if z < 0 || z >= len(w.terrain[x][y]) {
                            continue
                        }
                        w.terrain[x][y][z].render(c)
                    }
                }
            }
    }
}


fn generate_2d_gradient_map(seed: i64) -> [[Vec2; MAX_WORLD_SIZE]; MAX_WORLD_SIZE] {
    r = rand.New(rand.new_source(i64(seed)));
    m = [MaxWorldSize][MaxWorldSize]maths.Vec2{};

    for x = 0; x < len(m); x++ {
        for y = 0; y < len(m[x]); y++ {
            m[x][y] = maths.Vec2{r.f32(), r.f32()}
        }
    }

    return m
}

fn generate_3d_gradient_map(seed: i64) -> [MaxWorldSize][MaxWorldSize][MaxWorldSize]maths.Vec3 {
    r = rand.New(rand.NewSource(i64(seed)))
        m = [MaxWorldSize][MaxWorldSize][MaxWorldSize]maths.Vec3{}

    for x = 0; x < len(m); x++ {
        for y = 0; y < len(m[x]); y++ {
            for z = 0; z < len(m[x][y]); z++ {
                m[x][y][z] = maths.Vec3{r.f32(), r.f32(), r.f32()}
            }
        }
    }


    return m
}
