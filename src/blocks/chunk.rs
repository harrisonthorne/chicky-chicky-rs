use super::*;
use crate::blocks::Block;

pub const CHUNK_SIZE: usize = 64;

/// Chunk contains a three-dimensional array of blocks
pub struct Chunk {
    blocks: [[[Block; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
    // chunk_i: i64,
    // chunk_j: i64,
    // chunk_k: i64,

    /// The vertex buffer for the chunk mesh. Because it can't be initialized at first, we'll make
    /// it an Option so it can be set to Some when it's ready.
    block_mesh_buffer: Option<wgpu::Buffer>,
}

impl Chunk {
    pub fn generate(_chunk_i: i64, _chunk_j: i64, _chunk_k: i64) -> Self {
        let mut c = Self {
            blocks: [[[Block::from(BlockType::Air); CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
            // chunk_i,
            // chunk_j,
            // chunk_k,

            block_mesh_buffer: None,
        };

        for i in 0..CHUNK_SIZE {
            for j in 0..CHUNK_SIZE / 2 {
                for k in 0..CHUNK_SIZE {
                    c.set(i, j, k, Block::from(BlockType::Grass))
                }
            }
        }

        c
    }

    // /// Returns the block at the array position.
    // pub fn at(&self, i: usize, j: usize, k: usize) -> &Block {
    //     &self.blocks[i][j][k]
    // }

    /// Sets the block at the array index.
    pub fn set(&mut self, i: usize, j: usize, k: usize, b: Block) {
        self.blocks[i][j][k] = b;
    }

    /// Renders the Chunk.
    pub fn render(
        &self,
    ) {
        if let Some(_vertex_buffer) = &self.block_mesh_buffer {
            // TODO: render the mesh
        }
    }
}
