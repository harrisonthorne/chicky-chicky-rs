use render;
use items;

/// Animatable is an interface that can be added to objects
/// that animate. It calls its `animate()` method during
/// every logical loop to compute whether or not the
/// animation should advance to the next frame, which frame
/// the animation should be on, or the animation
/// itself.
pub trait Animatable {
    fn animate(delta: f32);
}

/// Flammable can be added to objects that can be ignited.
/// Such objects can burn any objects with the Burnable
/// interface attached.
pub trait Flammable {
    fn ignite();
    fn ignited() -> bool;
}

/// Burnable can be added to objects that can be burned.  Its
/// burn() method will be called when an offending object is
/// placed next to it.
pub trait Burnable {
    fn burn();
}

/// Killable is here to serve as an embed in blocks or
/// characters. in other words, anything that can be killed.
pub trait Killable {
    /// Called when the killable is hit. Returns any items that
    /// the killable might drop when hit.
    fn hit(with: items::Item, power: f32) -> Vec<items::Item>;

    /// Called when the killable should be killed. Returns any
    /// items that might be dropped with the killable dies.
    fn kill() -> Vec<items::Item>;

    /// Returns true if the killable is still alive. A
    /// killable can still be alive even if it has no health
    /// left. Any killables determined to be dead are removed
    /// from the world
    fn is_alive() -> bool;

    /// Returns the number of health points left on the
    /// killable
    fn health_left() -> f32;

    /// Returns the lifespan of health points on the killable
    fn lifespan() -> f32;
}

/// Renderable is implemented by anything that can be
/// rendered.
pub trait Renderable {
    fn render(c: &render::Camera);
}

/// Logicable is on objects that should have logic calculated for them
pub trait Logicable {
    fn logic(delta: f32);
}
