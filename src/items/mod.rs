mod food;
mod material;
mod resource;

use food::Food;
use material::Material;
use resource::Resource;
use crate::blocks::Block;

#[derive(Debug)]
pub enum Item {
    Resource(Resource),
    Tool(Tool),
    Weapon(Weapon),
    Block(Block),
    Food(Food),
    Other,
}

#[derive(Debug)]
pub struct Tool {
    tool_type: ToolType,
    material: Material,
    health: u32,
    lifespan: u32,
}

#[derive(Debug)]
pub enum ToolType {
    Shovel,
    Axe,
    Pick,
    Hoe,
}

#[derive(Debug)]
pub struct Weapon {
    tool_type: WeaponType,
    material: Material,
    health: f32,
    lifespan: f32,
}

#[derive(Debug)]
pub enum WeaponType {
    Sword,
    Bow,
    Arrow,
}

/// A tuple; the first element is the Item and the second is the size of the stack.
pub type ItemStack = (Item, u8);

/// Backpack is really just a fun alternative name for "item vector" and "inventory" :) It is a
/// vector of ItemStacks.
pub type Backpack = Vec<ItemStack>;
