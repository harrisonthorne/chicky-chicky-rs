pub mod chunk;
pub mod render;
pub mod textures;
pub mod vertices;

pub use self::chunk::*;
pub use self::textures::*;
pub use self::vertices::*;

#[derive(Clone, Copy, Debug)]
pub struct Block {
    block_type: BlockType,
    health: f32,
    lifespan: f32,

    /// If Some, offset the Block.
    position_offset: Option<cgmath::Vector3<f32>>,
}

impl From<BlockType> for Block {
    fn from(_ty: BlockType) -> Self {
        // TODO
        Block {
            block_type: BlockType::Grass,
            health: 15.0,
            lifespan: 15.0,
            position_offset: None,
        }
    }
}

impl Block {
    pub fn vertex_buffer_descriptors<'a>() -> &'a [wgpu::VertexBufferDescriptor<'a>] {
        use std::mem::size_of;
        const FLOAT4_SIZE: u64 = std::mem::size_of::<[f32; 4]>() as u64;

        &[
            wgpu::VertexBufferDescriptor {
                stride: render::Vertex::SIZE,
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
                ],
            },
        ]
    }

    pub fn render<'a>(
        &self,
        render_pass: &mut wgpu::RenderPass<'a>,
        uniform_bind_group: &'a wgpu::BindGroup,
        cube_vertex_buffer: &'a wgpu::Buffer,
        textures: &'a textures::BlockTextures,
    ) {
        let block_texture = match self.block_type {
            BlockType::Air => return,
            BlockType::Dirt => &textures.dirt,
            BlockType::Grass => &textures.grass,
            BlockType::Sand => &textures.sand,
            _ => {
                println!("[WARNING] block tried to render but texture isn't used or available in Block::render: {:?}", self.block_type);
                return;
            }
        };

        render_pass.set_bind_group(0, block_texture.get_bind_group(), &[]);
        render_pass.set_bind_group(1, uniform_bind_group, &[]);
        render_pass.set_vertex_buffer(0, &cube_vertex_buffer, 0, 0);
        render_pass.draw(0..CUBE_VERTICES.len() as u32, 0..1);
    }
}

#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
pub enum BlockType {
    Air,
    Sand,
    Dirt,
    Grass,
    Stone,
    Cobblestone,
    CoalOre,
    IronOre,
    CopperOre,
    GoldOre,
    DiamondOre,
    Wood,
    WoodPlanks,
    Furnace,
    Leaves,
}
