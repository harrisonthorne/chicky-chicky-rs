use super::*;
use crate::traits::Renderable;

pub const CHUNK_SIZE: usize = 64;

// Chunk contains a three-dimensional array of blocks
pub struct Chunk {
    blocks: [[[Block; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
    grid_x: i64,
    grid_y: i64,
    grid_z: i64,
}

impl Chunk {
    pub fn new(grid_x: i64, grid_y: i64, grid_z: i64) -> Self {
        Self {
            blocks: [[[Block::from(BlockType::Air); CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
            grid_x,
            grid_y,
            grid_z,
        }
    }

    /// Returns the block at the array position.
    pub fn at(&self, i: usize, j: usize, k: usize) -> &Block {
        &self.blocks[i][j][k]
    }

    /// Sets the block at the array index.
    pub fn set(&mut self, i: usize, j: usize, k: usize, b: Block) {
        self.blocks[i][j][k] = b;
    }

    /// Renders the Chunk.
    fn render(&self) {
        for i in 0..self.blocks.len() {
            for j in 0..self.blocks[i].len() {
                for k in 0..self.blocks[i][j].len() {
                    self.blocks[i][j][k].render();
                }
            }
        }
    }
}
