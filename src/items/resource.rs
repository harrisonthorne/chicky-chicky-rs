/// A Resource is something that can be crafted into something else. A Resource can't be placed
/// down like a Block can be, nor is it edible like Food.
#[derive(Debug)]
pub enum Resource {
    WoodPlanks,
    Rocks,
    Sticks,
    IronIngot,
    IronNugget,
    GoldIngot,
    GoldNugget,
    Diamond,
    Coal,
}
