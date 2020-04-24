mod climate;

use crate::blocks::chunk::Chunk;
use std::collections::VecDeque;

// potentially allowing infinite worlds
// const MAX_WORLD_SIZE: i32 = 1024; // in chunks. 1024 yields thousands upon thousands of blocks in each direction

// World contains a slice of Plots
struct World {
    chunks: VecDeque<VecDeque<VecDeque<Chunk>>>,
    seed: i64,
    render_distance: i32,
}

impl World {
    fn new(seed: i64) -> Self {
        Self {
            chunks: Default::default(),
            seed,
            render_distance: 6,
        }
    }

    pub fn generate_chunk(&self, _x: i32, _y: i32, _z: i32) {
        todo!()
    }

    fn render(&self) {
        todo!()
        // let eye_chunk_x = c.position().x / (blocks::BLOCK_WIDTH * blocks::CHUNK_SIZE);
        // let eye_chunk_y = c.position().y / (blocks::BLOCK_WIDTH * blocks::CHUNK_SIZE);
        // let eye_chunk_z = c.position().z / (blocks::BLOCK_WIDTH * blocks::CHUNK_SIZE);

        // for x in (eye_chunk_x - self.render_distance)..(eye_chunk_x + self.render_distance) {
        //     if x < 0 || x >= self.terrain.len() {
        //         continue;
        //     }
        //     for y in (eye_chunk_y - self.render_distance)..(eye_chunk_y + self.render_distance) {
        //         if y < 0 || y >= self.terrain[x].len() {
        //             continue;
        //         }
        //         for z in (eye_chunk_z - self.render_distance)..(eye_chunk_z + self.render_distance) {
        //             if z < 0 || z >= self.terrain[x][y].len() {
        //                 continue;
        //             }
        //             self.terrain[x][y][z].render();
        //         }
        //     }
        // }
    }
}
