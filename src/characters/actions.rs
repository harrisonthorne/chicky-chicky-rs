/// Specifies what a certain character is doing.
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


/// Right or Left, telling which direction a character (or whatever) is facing
pub enum Direction {
    Right,
    Left,
}
