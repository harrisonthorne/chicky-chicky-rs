mod food;
mod material;
mod resource;

use crate::blocks::Block;
use food::Food;
use material::Material;
use resource::Resource;

#[derive(Debug)]
#[allow(dead_code)]
pub enum Item {
    Resource(Resource),
    Tool(Tool),
    Weapon(Weapon),
    Block(Block),
    Food(Food),
    Other,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Tool {
    tool_type: ToolType,
    material: Material,
    health: u32,
    lifespan: u32,
}

#[derive(Debug)]
#[allow(dead_code)]
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
#[allow(dead_code)]
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
