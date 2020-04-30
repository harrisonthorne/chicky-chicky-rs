pub mod chunk;
pub mod render;
pub mod vertices;
// mod textures;

pub use self::chunk::*;
pub use self::vertices::*;
// pub use self::textures::*;

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
            block_type: BlockType::Air,
            health: 0.0,
            lifespan: 0.0,
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
            wgpu::VertexBufferDescriptor {
                stride: FLOAT4_SIZE * 4,
                step_mode: wgpu::InputStepMode::Instance,
                attributes: &[
                    wgpu::VertexAttributeDescriptor {
                        offset: 0,
                        shader_location: 2,
                        format: wgpu::VertexFormat::Float4,
                    },
                    wgpu::VertexAttributeDescriptor {
                        offset: FLOAT4_SIZE,
                        shader_location: 3,
                        format: wgpu::VertexFormat::Float4,
                    },
                    wgpu::VertexAttributeDescriptor {
                        offset: FLOAT4_SIZE * 2,
                        shader_location: 4,
                        format: wgpu::VertexFormat::Float4,
                    },
                    wgpu::VertexAttributeDescriptor {
                        offset: FLOAT4_SIZE * 3,
                        shader_location: 5,
                        format: wgpu::VertexFormat::Float4,
                    },
                ],
            },
        ]
    }
}

impl crate::traits::Renderable for Block {
    fn render(&self) {}
}

#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
pub enum BlockType {
    Air,
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
