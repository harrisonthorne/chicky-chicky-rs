use super::*;
use crate::blocks::Block;
use crate::textures::BlockTextureIndex;
use crate::world::{Axis, Direction};

pub const CHUNK_BLOCK_WIDTH: usize = 32;
pub const CHUNK_SIZE: f32 = CHUNK_BLOCK_WIDTH as f32 * Block::WIDTH;

/// Chunk contains a three-dimensional array of blocks
pub struct Chunk {
    blocks: [[[Block; CHUNK_BLOCK_WIDTH]; CHUNK_BLOCK_WIDTH]; CHUNK_BLOCK_WIDTH],
    // chunk_i: i64,
    // chunk_j: i64,
    // chunk_k: i64,
    /// The vertex buffer for the chunk mesh. Because it can't be initialized at first, we'll make
    /// it an Option so it can be set to Some when it's ready.
    block_mesh_buffer: Option<wgpu::Buffer>,
    needs_mesh_update: bool,
    vertex_count: usize,

    /// Constant chunk position based on chunk grid position and chunk size.
    chunk_position: [f32; 3],
}

impl Chunk {
    pub fn generate(chunk_i: i64, chunk_j: i64, chunk_k: i64, _device: &wgpu::Device) -> Self {
        let mut c = Self {
            blocks: [[[Block::from(BlockType::Air); CHUNK_BLOCK_WIDTH]; CHUNK_BLOCK_WIDTH];
                CHUNK_BLOCK_WIDTH],
            // chunk_i,
            // chunk_j,
            // chunk_k,
            block_mesh_buffer: None,
            vertex_count: 0,

            // set to true to make the initial mesh on first logic loop
            needs_mesh_update: true,

            chunk_position: [
                chunk_i as f32 * CHUNK_SIZE,
                chunk_j as f32 * CHUNK_SIZE,
                chunk_k as f32 * CHUNK_SIZE,
            ],
        };

        for i in 0..CHUNK_BLOCK_WIDTH {
            for j in 0..CHUNK_BLOCK_WIDTH / 2 {
                for k in 0..CHUNK_BLOCK_WIDTH {
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
        self.needs_mesh_update = true;
    }

    pub fn logic(&mut self, device: &wgpu::Device) {
        // it'd be cool to put this in the `render` method, but `render` isn't provided a queue
        if self.needs_mesh_update {
            self.make_greedy_mesh(device);
            self.needs_mesh_update = false;
        }
    }

    /// Renders the Chunk. This method assumes that the block texture bind group has already been
    /// bound to the render pass.
    pub fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        if let Some(vertex_buffer) = &self.block_mesh_buffer {
            render_pass.set_vertex_buffer(
                0,
                vertex_buffer,
                0,
                self.vertex_count as u64 * ChunkMeshVertex::SIZE,
            );
            render_pass.draw(0..self.vertex_count as u32, 0..1);
        }
    }

    // TODO: create a queue (world-level, probably) that renders new blocks while a chunk adds them
    // to its own greedy mesh, in case re-creating greedy meshes is too slow
    fn make_greedy_mesh(&mut self, device: &wgpu::Device) {
        let mut vertices = Vec::<ChunkMeshVertex>::new();

        vertices.append(&mut self.get_greedy_mesh_along_axis(Axis::X));
        vertices.append(&mut self.get_greedy_mesh_along_axis(Axis::Y));
        vertices.append(&mut self.get_greedy_mesh_along_axis(Axis::Z));

        // save the number of vertices
        self.vertex_count = vertices.len();

        let vertex_slice = &vertices[..];
        let casted_slice = bytemuck::cast_slice(vertex_slice);

        // copy vertices to the vertex buffer
        self.block_mesh_buffer =
            Some(device.create_buffer_with_data(casted_slice, wgpu::BufferUsage::VERTEX));
    }

    // TODO: potentially separate traversals by Direction instead of Axis to further cull adjacent
    // or hidden faces
    fn get_greedy_mesh_along_axis(&self, along_axis: Axis) -> Vec<ChunkMeshVertex> {
        let mut vertices = Vec::<ChunkMeshVertex>::new();

        // `layer` is basically the layer along whichever axis we're traversing
        for layer in 0..CHUNK_BLOCK_WIDTH {
            let mut visited_mask = [[false; CHUNK_BLOCK_WIDTH]; CHUNK_BLOCK_WIDTH];

            for i in 0..CHUNK_BLOCK_WIDTH {
                for j in 0..CHUNK_BLOCK_WIDTH {
                    // the block at the starting point that we have to match
                    let reference_block = self.get_block_along_layer(along_axis, layer, i, j);

                    // decide if we should just skip over this block for meshing
                    if reference_block.should_skip_mesh() || visited_mask[i][j] {
                        continue;
                    }

                    // just mark this spot as visited while we're on it
                    visited_mask[i][j] = true;

                    // starting_point should not be mut
                    let starting_point = (i, j);

                    // inclusive ending! ending_point is also included in the final mesh
                    let mut ending_point = (i, j);

                    // traverse along the "i" axis first, getting the ending_point's `i` (start
                    // at/skip i + 1 because the block at i is the reference_block, which doesn't
                    // need to be matched to itself, silly :))
                    for (k, flag_row) in visited_mask.iter_mut().enumerate().skip(i + 1) {
                        // if blocks are still the same type, move the ending_point
                        if self
                            .get_block_along_layer(along_axis, layer, k, j)
                            .block_type
                            == reference_block.block_type
                        {
                            // block types match, so we can move the ending_point!
                            ending_point.0 = k;
                            flag_row[j] = true;
                        } else {
                            // block types don't match, so don't advance ending_point, don't mark
                            // this position as visited, do not pass go, do not collect $200, just
                            // break
                            break;
                        }
                    }

                    // now, we have to traverse along both the "i" the "j" axis (up until
                    // ending_point's "i" component), making sure all blocks still match
                    // reference_block's type. if we can move along the "j" axis and all block
                    // types are still the same, we can advance ending_point.
                    //
                    // why "j" axis first? for each iteration, we need to traverse along the i axis
                    // to accurately decide if we can *extend* the mesh in the direction of j. we
                    // start at j + 1 because it's already been decided that all blocks along j
                    // match.
                    for l in (j + 1)..CHUNK_BLOCK_WIDTH {
                        // go as far as we can while block types match. if even one of the blocks
                        // along the i axis here don't match, we have to stop. if all blocks in
                        // this row match, we can advance ending_point's j component.
                        //
                        // remember, ending_point is inclusive, which is why we go up to and equal
                        // to ending_point.0
                        let mut should_advance = true;
                        for (m, flag_row) in visited_mask
                            .iter_mut()
                            .enumerate()
                            .take(ending_point.0 + 1)
                            .skip(i)
                        {
                            if self
                                .get_block_along_layer(along_axis, layer, m, l)
                                .block_type
                                != reference_block.block_type
                            {
                                // came across a mismatching block, so signal no advancing and
                                // leave this loop
                                should_advance = false;
                                break;
                            }
                            flag_row[l] = true;
                        }

                        if should_advance {
                            ending_point.1 = l;
                        // then we'll move onto the next row
                        } else {
                            // if we came across a block that doesn't match in this row, we have to
                            // reset all visited flags in this row
                            for row_flags in
                                visited_mask.iter_mut().take(ending_point.0 + 1).skip(i)
                            {
                                row_flags[l] = false;
                            }

                            // and leave this loop
                            break;
                        }
                    }

                    // now, at this point, we should have a starting_point and ending_point. we can
                    // use this data to make our mesh:

                    let width = ending_point.0 - starting_point.0 + 1;
                    let height = ending_point.1 - starting_point.1 + 1;

                    let (direction, other_direction) = match along_axis {
                        Axis::X => (Direction::East, Direction::West),
                        Axis::Y => (Direction::Up, Direction::Down),
                        Axis::Z => (Direction::North, Direction::South),
                    };
                    let quad_start = match along_axis {
                        Axis::X => (layer, starting_point.0, starting_point.1),
                        Axis::Y => (starting_point.0, layer, starting_point.1),
                        Axis::Z => (starting_point.0, starting_point.1, layer),
                    };
                    if let Ok(texture_layer) = BlockTextureIndex::from_type_and_direction(
                        reference_block.block_type,
                        direction,
                    ) {
                        vertices.append(&mut self.get_quad_face_vertices(
                            quad_start,
                            width,
                            height,
                            texture_layer,
                            direction,
                        ));
                    }
                    if let Ok(other_texture_layer) = BlockTextureIndex::from_type_and_direction(
                        reference_block.block_type,
                        other_direction,
                    ) {
                        vertices.append(&mut self.get_quad_face_vertices(
                            quad_start,
                            width,
                            height,
                            other_texture_layer,
                            other_direction,
                        ));
                    }
                }
            }
        }

        vertices
    }

    fn get_block_along_layer(&self, along_axis: Axis, layer: usize, i: usize, j: usize) -> &Block {
        match along_axis {
            Axis::X => &self.blocks[layer][i][j],
            Axis::Y => &self.blocks[i][layer][j],
            Axis::Z => &self.blocks[i][j][layer],
        }
    }

    fn get_quad_face_vertices(
        &self,
        start_grid_pos: (usize, usize, usize),
        width: usize,
        height: usize,
        texture_layer: BlockTextureIndex,
        face_direction: Direction,
    ) -> Vec<ChunkMeshVertex> {
        // the starting block's exact position within the chunk (in other words, relative to the
        // chunk, not the world)
        let block_grid_pos = [
            start_grid_pos.0 as f32 * Block::WIDTH,
            start_grid_pos.1 as f32 * Block::WIDTH,
            start_grid_pos.2 as f32 * Block::WIDTH,
        ];

        // the bottom, south-east vertex position of the cuboid
        let base_vertex_pos = [
            self.chunk_position[0] + block_grid_pos[0],
            self.chunk_position[1] + block_grid_pos[1],
            self.chunk_position[2] + block_grid_pos[2],
        ];

        let (quad_width, quad_height) = (width as f32 * Block::WIDTH, height as f32 * Block::WIDTH);

        let (lower_left_pos, lower_right_pos, upper_right_pos, upper_left_pos) =
            match face_direction {
                Direction::North => {
                    let z = base_vertex_pos[2] + Block::WIDTH;
                    (
                        [base_vertex_pos[0], base_vertex_pos[1], z],
                        [base_vertex_pos[0] + quad_width, base_vertex_pos[1], z],
                        [
                            base_vertex_pos[0] + quad_width,
                            base_vertex_pos[1] + quad_height,
                            z,
                        ],
                        [base_vertex_pos[0], base_vertex_pos[1] + quad_height, z],
                    )
                }
                Direction::South => {
                    let z = base_vertex_pos[2];
                    (
                        [base_vertex_pos[0] + quad_width, base_vertex_pos[1], z],
                        [base_vertex_pos[0], base_vertex_pos[1], z],
                        [base_vertex_pos[0], base_vertex_pos[1] + quad_height, z],
                        [
                            base_vertex_pos[0] + quad_width,
                            base_vertex_pos[1] + quad_height,
                            z,
                        ],
                    )
                }
                Direction::East => {
                    let x = base_vertex_pos[0];
                    (
                        [x, base_vertex_pos[1], base_vertex_pos[2]],
                        [x, base_vertex_pos[1], base_vertex_pos[2] + quad_width],
                        [
                            x,
                            base_vertex_pos[1] + quad_height,
                            base_vertex_pos[2] + quad_width,
                        ],
                        [x, base_vertex_pos[1] + quad_height, base_vertex_pos[2]],
                    )
                }
                Direction::West => {
                    let x = base_vertex_pos[0] + Block::WIDTH;
                    (
                        [x, base_vertex_pos[1], base_vertex_pos[2] + quad_width],
                        [x, base_vertex_pos[1], base_vertex_pos[2]],
                        [x, base_vertex_pos[1] + quad_height, base_vertex_pos[2]],
                        [
                            x,
                            base_vertex_pos[1] + quad_height,
                            base_vertex_pos[2] + quad_width,
                        ],
                    )
                }
                Direction::Up => {
                    let y = base_vertex_pos[1] + Block::WIDTH;
                    (
                        [base_vertex_pos[0] + quad_width, y, base_vertex_pos[2]],
                        [base_vertex_pos[0], y, base_vertex_pos[2]],
                        [base_vertex_pos[0], y, base_vertex_pos[2] + quad_height],
                        [
                            base_vertex_pos[0] + quad_width,
                            y,
                            base_vertex_pos[2] + quad_height,
                        ],
                    )
                }
                Direction::Down => {
                    let y = base_vertex_pos[1];
                    (
                        [base_vertex_pos[0], y, base_vertex_pos[2]],
                        [base_vertex_pos[0] + quad_width, y, base_vertex_pos[2]],
                        [
                            base_vertex_pos[0] + quad_width,
                            y,
                            base_vertex_pos[2] + quad_height,
                        ],
                        [base_vertex_pos[0], y, base_vertex_pos[2] + quad_height],
                    )
                }
            };

        let (lower_left, lower_right, upper_right, upper_left) = (
            ChunkMeshVertex {
                position: lower_left_pos,
                uv_coords: [0.0, quad_height as f32],
                texture_layer: texture_layer as u32,
            },
            ChunkMeshVertex {
                position: lower_right_pos,
                uv_coords: [quad_width as f32, quad_height as f32],
                texture_layer: texture_layer as u32,
            },
            ChunkMeshVertex {
                position: upper_right_pos,
                uv_coords: [quad_width as f32, 0.0],
                texture_layer: texture_layer as u32,
            },
            ChunkMeshVertex {
                position: upper_left_pos,
                uv_coords: [0.0, 0.0],
                texture_layer: texture_layer as u32,
            },
        );

        vec![
            lower_left,
            upper_right,
            upper_left,
            lower_left,
            lower_right,
            upper_right,
        ]
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ChunkMeshVertex {
    pub position: [f32; 3],
    pub uv_coords: [f32; 2],
    pub texture_layer: u32,
}

impl ChunkMeshVertex {
    pub const SIZE: u64 = std::mem::size_of::<Self>() as wgpu::BufferAddress;

    pub fn vertex_buffer_descriptors<'a>() -> &'a [wgpu::VertexBufferDescriptor<'a>] {
        use std::mem::size_of;

        &[wgpu::VertexBufferDescriptor {
            stride: Self::SIZE,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttributeDescriptor {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float3,
                },
                wgpu::VertexAttributeDescriptor {
                    offset: size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float2,
                },
                wgpu::VertexAttributeDescriptor {
                    offset: size_of::<[f32; 5]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Uint,
                },
            ],
        }]
    }
}

unsafe impl bytemuck::Pod for ChunkMeshVertex {}
unsafe impl bytemuck::Zeroable for ChunkMeshVertex {}
