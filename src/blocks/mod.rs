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
    position_offset: Option<cgmath::Vector3<f64>>,
}

impl From<BlockType> for Block {
    fn from(ty: BlockType) -> Self {
        let lifespan = match ty {
            BlockType::Dirt => 10.0,
            BlockType::Grass => 15.0,
            BlockType::Stone => 50.0,
            BlockType::Sand => 5.0,
            BlockType::Air => 0.0,
            _ => {
                eprintln!(
                    "BlockType `{:?}` not implemented. No texture will be rendered!",
                    ty
                );
                0.0
            }
        };

        Self {
            block_type: ty,
            health: lifespan,
            lifespan,
            position_offset: None,
        }
    }
}

impl Block {
    pub const WIDTH: f32 = 0.5;

    pub fn vertex_buffer_descriptors<'a>() -> &'a [wgpu::VertexBufferDescriptor<'a>] {
        use std::mem::size_of;

        &[wgpu::VertexBufferDescriptor {
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
        }]
    }

    pub fn render<'a>(
        &self,
        block_position_uniform_bind_group: &'a wgpu::BindGroup,
        cube_vertex_buffer: &'a wgpu::Buffer,
        render_pass: &mut wgpu::RenderPass<'a>,
        textures: &'a BlockTextures,
        uniform_bind_group: &'a wgpu::BindGroup,
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
        render_pass.set_bind_group(2, block_position_uniform_bind_group, &[]);
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
