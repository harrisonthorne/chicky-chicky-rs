use super::*;
use crate::blocks::{render::BlockPositionUniform, Block};

pub const CHUNK_SIZE: usize = 16;

/// Chunk contains a three-dimensional array of blocks
pub struct Chunk {
    blocks: [[[Block; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
    chunk_i: i64,
    chunk_j: i64,
    chunk_k: i64,

    chunk_position: cgmath::Vector3<f32>,
}

impl Chunk {
    pub fn generate(chunk_i: i64, chunk_j: i64, chunk_k: i64) -> Self {
        let chunk_x = (CHUNK_SIZE as f32 * chunk_i as f32) * Block::WIDTH as f32;
        let chunk_y = (CHUNK_SIZE as f32 * chunk_j as f32) * Block::WIDTH as f32;
        let chunk_z = (CHUNK_SIZE as f32 * chunk_k as f32) * Block::WIDTH as f32;
        let chunk_position = (chunk_x, chunk_y, chunk_z).into();

        let mut c = Self {
            blocks: [[[Block::from(BlockType::Air); CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
            chunk_i,
            chunk_j,
            chunk_k,

            chunk_position,
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

    /// Returns the block at the array position.
    pub fn at(&self, i: usize, j: usize, k: usize) -> &Block {
        &self.blocks[i][j][k]
    }

    /// Sets the block at the array index.
    pub fn set(&mut self, i: usize, j: usize, k: usize, b: Block) {
        self.blocks[i][j][k] = b;
    }

    /// Renders the Chunk.
    pub fn render<'a>(
        &'a self,
        block_position_uniform_bind_group: &'a wgpu::BindGroup,
        block_position_uniform_buffer: &wgpu::Buffer,
        cube_vertex_buffer: &'a wgpu::Buffer,
        device: &wgpu::Device,
        queue: &mut wgpu::Queue,
        render_pass: &mut wgpu::RenderPass<'a>,
        textures: &'a BlockTextures,
        uniform_bind_group: &'a wgpu::BindGroup,
    ) {
        for i in 0..self.blocks.len() {
            for j in 0..self.blocks[i].len() {
                for k in 0..self.blocks[i][j].len() {
                    let block = &self.blocks[i][j][k];

                    // the block's position within the chunk
                    let block_grid_x = (i as f32 * Block::WIDTH) as f32;
                    let block_grid_y = (j as f32 * Block::WIDTH) as f32;
                    let block_grid_z = (k as f32 * Block::WIDTH) as f32;
                    let block_grid_position: cgmath::Vector3<f32> =
                        (block_grid_x, block_grid_y, block_grid_z).into();

                    // the block's position within the *world*
                    let block_base_position = self.chunk_position + block_grid_position;

                    // the block's position within the world, plus its offset
                    let block_position_uniform = if let Some(offset) = block.position_offset {
                        BlockPositionUniform {
                            mat: cgmath::Matrix4::from_translation(block_base_position + offset),
                        }
                    } else {
                        BlockPositionUniform {
                            mat: cgmath::Matrix4::from_translation(block_base_position),
                        }
                    };

                    // load the block position into the buffer
                    {
                        let block_position_staging_buffer = device.create_buffer_with_data(
                            bytemuck::cast_slice(&[block_position_uniform]),
                            wgpu::BufferUsage::COPY_SRC,
                        );

                        let mut buffer_encoder = device.create_command_encoder(
                            &wgpu::CommandEncoderDescriptor {
                                label: Some("block position buffer encoder"),
                            },
                        );
                        buffer_encoder.copy_buffer_to_buffer(
                            &block_position_staging_buffer,
                            0,
                            &block_position_uniform_buffer,
                            0,
                            std::mem::size_of::<BlockPositionUniform>() as wgpu::BufferAddress,
                        );
                        queue.submit(&[buffer_encoder.finish()]);
                    }

                    self.blocks[i][j][k].render(
                        block_position_uniform_bind_group, 
                        cube_vertex_buffer, 
                        render_pass, 
                        textures,
                        uniform_bind_group,
                    );
                }
            }
        }
    }
}
