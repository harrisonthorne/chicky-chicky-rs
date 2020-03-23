package types

import (
use render;
use items;
)

// Animatable is an interface that can be added to objects
// that animate. It calls its Animate(delta) method during
// every logical loop to compute whether or not the
// animation should advance to the next frame, which frame
// the animation should be on, or the animation
// itself.
// trait Animatable {
// 	Animate(delta f32)
// }

// Flammable can be added to objects that can be ignited.
// Such objects can burn any objects with the Burnable
// interface attached.
trait Flammable {
    Ignite()
	Ignited() bool
}

// Burnable can be added to objects that can be burned.  Its
// Burn() method will be called when an offending object is
// placed next to it.
trait Burnable {
	Burn()
}

// killable is here to serve as an embed in blocks or
// characters. in other words, anything that can be killed.
trait killable {
	// Called when the killable is hit. Returns any items that
	// the killable might drop when hit.
	hit(with interface{}, power f32) []items.Item

	// Called when the killable should be killed. Returns any
	// items that might be dropped with the killable dies.
	kill() []items.Item

	// Returns true if the killable is still alive. A
	// killable can still be alive even if it has no health
	// left. Any killables determined to be dead are removed
	// from the world
	IsAlive() bool
// Returns the number of health points left on the
	// killable
	HealthLeft() f32

	// Returns the lifespan of health points on the killable
	Lifespan() f32
}

// Renderable is implemented by anything that can be
// rendered.
trait Renderable {
	render(c *render.Camera)
}

// Logicable is on objects that should have logic calculated for them
trait Logicable {
	Logic(delta f32)
}
