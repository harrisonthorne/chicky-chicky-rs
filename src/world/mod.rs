use std::collections::VecDeque;
use crate::blocks;
use crate::render;
use webgl_matrix::{Vec3};
use rand;
use crate::blocks::chunk::Chunk;

const MAX_WORLD_SIZE: i32 = 1024; // in chunks. 1024 yields thousands upon thousands of blocks in each direction

// World contains a slice of Plots
struct World {
    terrain: VecDeque<VecDeque<VecDeque<Chunk>>>,
    seed:    i64,

    height_gradient_vectors:      [[Vec3; MAX_WORLD_SIZE]; MAX_WORLD_SIZE],
    humidity_gradient_vectors:    [[Vec3; MAX_WORLD_SIZE]; MAX_WORLD_SIZE],
    temperature_gradient_vectors: [[Vec3; MAX_WORLD_SIZE]; MAX_WORLD_SIZE],

    cave_gradient_vectors: [[[Vec3; MAX_WORLD_SIZE]; MAX_WORLD_SIZE]; MAX_WORLD_SIZE],
    ore_gradient_vectors:  [[[Vec3; MAX_WORLD_SIZE]; MAX_WORLD_SIZE]; MAX_WORLD_SIZE],
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
        ""
    }

    pub fn generate_chunk(&self, x: i32, y: i32, z: i32) {

    }

    fn render(&self, c: &render::Camera) {
        let eye_chunk_x = c.position().x / (blocks::BLOCK_WIDTH * blocks::CHUNK_SIZE);
        let eye_chunk_y = c.position().y / (blocks::BLOCK_WIDTH * blocks::CHUNK_SIZE);
        let eye_chunk_z = c.position().z / (blocks::BLOCK_WIDTH * blocks::CHUNK_SIZE);

        for x in (eye_chunk_x - self.renderDistance)..(eye_chunk_x+self.renderDistance) {
            if x < 0 || x >= self.terrain.len() {
                continue;
            }
            for y in (eye_chunk_y - self.renderDistance)..(eye_chunk_y+self.renderDistance) {
                if y < 0 || y >= self.terrain[x].len() {
                    continue;
                }
                for z in (eye_chunk_z - self.renderDistance)..(eye_chunk_z+self.renderDistance) {
                    if z < 0 || z >= self.terrain[x][y].len() {
                        continue;
                    }
                    self.terrain[x][y][z].render(c);
                }
            }
        }
    }
}


fn generate_2d_gradient_map(seed: i64) -> [[Vec3; MAX_WORLD_SIZE]; MAX_WORLD_SIZE] {
    let r = rand::StdRng::from_seed(seed).unwrap();
    let m = [[Vec3::zeros(); MAX_WORLD_SIZE]; MAX_WORLD_SIZE];

    for x in 0..m.len() {
        for y in 0..m[x].len() {
            m[x][y] = maths.Vec3{r.next_f32(), r.next_f32()}
        }
    }

    m
}

fn generate_3d_gradient_map(seed: i64) -> [[[Vec3; MAX_WORLD_SIZE]; MAX_WORLD_SIZE]; MAX_WORLD_SIZE] {
    let r = rand::StdRng::from_seed(seed).unwrap();
    let m = [[[Vec3::zeros(); MAX_WORLD_SIZE]; MAX_WORLD_SIZE]; MAX_WORLD_SIZE];

    for x in 0..m.len() {
        for y in 0..m[x].len() {
            for z 0..m[x][y].len() {
                m[x][y][z] = maths.Vec3{r.next_f32(), r.next_f32(), r.next_f32()}
            }
        }
    }
}
