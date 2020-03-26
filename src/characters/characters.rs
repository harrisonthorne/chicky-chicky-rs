/// Character is a living, breathing object in the game of Chicky
/// Chicky. They can eat, sleep, run, jump, live and die. hopefully
/// they don't die unless they're bad. Character requires
/// Killable, Logicable, Renderable, and Controllable
trait Character: Killable + Logicable + Renderable + Controllable {}
