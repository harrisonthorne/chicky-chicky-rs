pub mod chunk;
pub mod render;
pub mod textures;

pub use self::chunk::*;
pub use self::textures::*;

#[derive(Clone, Copy, Debug)]
pub struct Block {
    block_type: BlockType,
    health: f32,

    /// If Some, offset the Block.
    position_offset: Option<cgmath::Vector3<f32>>,
}

impl From<BlockType> for Block {
    fn from(ty: BlockType) -> Self {
        let lifespan = Self::lifespan_of(ty);

        Self {
            block_type: ty,
            health: lifespan,
            position_offset: None,
        }
    }
}

impl Block {
    pub const WIDTH: f32 = 0.5;

    pub fn vertex_buffer_descriptors<'a>() -> &'a [wgpu::VertexBufferDescriptor<'a>] {
        use std::mem::size_of;

        &[wgpu::VertexBufferDescriptor {
            stride: chunk::ChunkMeshVertex::SIZE,
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
        }]
    }

    pub fn lifespan_of(ty: BlockType) -> f32 {
        match ty {
            BlockType::Dirt => 10.0,
            BlockType::Grass => 15.0,
            BlockType::Stone => 50.0,
            BlockType::Sand => 5.0,
            BlockType::Air => 0.0,
            _ => {
                eprintln!("BlockType `{:?}` not implemented. Lifespan unknown!", ty);
                0.0
            }
        }
    }

    pub fn should_skip_mesh(&self) -> bool {
        self.block_type == BlockType::Air
    }

    pub fn is_see_through(&self) -> bool {
        match self.block_type {
            BlockType::Air | BlockType::Glass => true,
            _ => false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
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
    Glass,
}
