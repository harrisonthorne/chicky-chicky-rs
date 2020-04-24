use crate::items;

/// Flammable can be added to objects that can be ignited. Flammables will ignite any neighboring
/// Flammables as well.
pub trait Flammable {
    fn ignite(&mut self);
    fn ignited(&self) -> bool;
}

/// Killable is here to serve as an embed in blocks or
/// characters. in other words, anything that can be killed.
pub trait Killable {
    /// Called when the Killable is hit. Returns any items that
    /// the killable might drop when hit.
    fn hit(&mut self, with: Option<items::Item>, power: f32) -> &[items::ItemStack];

    /// Called when the killable should be killed. Returns any
    /// items that might be dropped with the Killable dies.
    fn kill(&mut self) -> &[items::ItemStack];

    /// Returns true if the killable is still alive. A
    /// Killable can still be alive even if it has no health
    /// left. Any Killables determined to be dead are removed
    /// from the world
    fn is_alive(&self) -> bool;

    /// Returns the number of health points left on the
    /// Killable
    fn health_left(&self) -> f32;

    /// Returns the lifespan, or max health points on the Killable
    fn lifespan(&self) -> f32;
}
