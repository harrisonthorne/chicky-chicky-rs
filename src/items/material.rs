/// Materials apply usually only to tools or weapons. Material describes what something is made out
/// of.
#[derive(Debug)]
pub enum Material {
    Wood,
    Stone,
    Steel, // made from iron and coal (carbon)
    Gold,
    Diamond,
}
