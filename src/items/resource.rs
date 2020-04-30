/// A Resource is something that can be crafted into something else. A Resource can't be placed
/// down like a Block can be, nor is it edible like Food.
#[derive(Debug)]
#[allow(dead_code)]
pub enum Resource {
    WoodPlanks,
    Rocks,
    Sticks,
    Aluminium,
    IronIngot,
    IronNugget,
    GoldIngot,
    GoldNugget,
    Diamond,
    Coal,
}
