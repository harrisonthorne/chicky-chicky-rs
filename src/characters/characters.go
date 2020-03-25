

import (
	// "chicky-chicky-go/game/space"
use types;
)

// Character is a living, breathing object in the game of Chicky
// Chicky Go. They can eat, sleep, run, jump, live and die. hopefully
// they don't die unless they're bad. Character embeds
// PhysicalObject, Renderable, and killable
trait Character {
	types.killable
	types.Logicable
	types.Renderable
	Controllable
}
