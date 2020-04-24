use crate::engine;

pub mod chunk;
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
}

impl From<BlockType> for Block {
    fn from(_: BlockType) -> Self {
        // TODO
        Block {
            block_type: BlockType::Air,
            health: 0.0,
            lifespan: 0.0,
        }
    }
}

#[derive(Clone, Copy, Debug)]
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

impl engine::Renderable for Block {
    fn render(&self) -> bool {
        false
    }
}
