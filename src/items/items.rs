
pub enum Item {
    Tool(Tool),
    Weapon(Weapon),
    Block(Block),
    Food(Food),
    Other,
}

/// An enum for determining types of tools
/// (shovel, axe, other, etc)
pub enum ToolType {
    Shovel,
    Axe,
    Pick, 
    Other,
}
