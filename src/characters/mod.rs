pub mod chicken;

pub use chicken::*;

use crate::items;
use crate::traits::*;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// Character is a living, breathing object in the game of Chicky Chicky. They can eat, sleep, run,
/// jump, live and die. hopefully they don't die unless they're bad. A Character is required to
/// also be Killable, Logicable, and Renderable.
pub trait Character: Killable + Logicable + Renderable {
    fn walk(&mut self, direction: Direction, sup: bool);
    fn down(&mut self, sup: bool); // Do something downward (fall or squat maybe?)
    fn jump(&mut self, sup: bool); // Do something when the space bar is pressed
    fn stop(&mut self); // Nothing is happening anymore; stop movement

    /// Initiates an attack by the Controllable. Returns
    /// whatever the Controllable might be holding, the
    /// attack power, and where the Controllable was aiming
    fn attack<K: Killable>(&self, with: Option<&items::Item>, power: f32, who: K);

    /// REnders the character.
    fn render(&self);
}

/// Specifies what a certain character is doing.
#[derive(Debug)]
pub enum CharacterAction {
    Nothing,
    Walk,
    Run,
    Squat,
    Climb,
    Fall,
    Attack,
    Hurt,
    Dying,
    Push,
    Sleep,
    Eat,
}

impl Default for CharacterAction {
    fn default() -> Self {
        Self::Nothing
    }
}

/// Right or Left, telling which direction a character (or whatever) is facing
#[derive(Debug)]
pub enum FacingDirection {
    Right,
    Left,
}

impl Default for FacingDirection {
    fn default() -> Self {
        Self::Right
    }
}
