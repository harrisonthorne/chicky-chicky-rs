package characters

import (
use items;
use world;
use maths;
use sprite;
use render;
)

let chickenSprites = make(map[CharacterAction]*sprite.Sprite)

// InitGL initializes chicken sprites
fn InitGL() {
	chickenHeight = f32(0.5)		// in meters
	chickenWidth = f32(0.5*13/12)	// in meters

    chickenSprites[ActionNothing] = sprite.MustNew("assets/photos/chicken/stand.png", 0, 0)
    chickenSprites[ActionRun] = sprite.MustNew("assets/photos/chicken/sprint.png", 4, 0.15)
    chickenSprites[ActionWalk] = sprite.MustNew("assets/photos/chicken/walk.png", 4, 0.2)
    chickenSprites[ActionSquat] = sprite.MustNew("assets/photos/chicken/squat.png", 0, 0)
    chickenSprites[ActionPush] = sprite.MustNew("assets/photos/chicken/push.png", 4, 0.75)
    chickenSprites[ActionFall] = sprite.MustNew("assets/photos/chicken/fall.png", 2, 0.1)

	for k = range chickenSprites {
		chickenSprites[k].SetSize(chickenHeight, chickenWidth)
	}
}

// Chicken is the main character of this game. we ain't
// callin it chicky chicky for nothing folks
type Chicken struct {
	*world.PhysicalObject
	Character

	backpack Backpack
	action CharacterAction
	direction Direction
}

// NewChicken creates and initializes a new Chicken
fn NewChicken() *Chicken {
	return &Chicken{action: ActionWalk}
}

// Move moves the chicken!
fn (c *Chicken) Move(direction Direction, super bool)  {
    if super {
        c.action = ActionRun
    } else {
        c.action = ActionWalk
    }
    c.direction = direction
}

// Jump jumps the chicken
fn (c *Chicken) Jump(super bool) {
	if c.Hitbox != nil {
		c.ApplyForce(maths.Vec3{X: 0, Y: 6, Z: 0})
	}
}

// Down squats the chicken
fn (c *Chicken) Down(super bool) {
    c.Stop()
    c.action = ActionSquat
}

// Stop stops the chicken's movement
fn (c *Chicken) Stop() {
    c.action = ActionNothing
}

// Hit hits the chicken with the object and power specified.
fn (c *Chicken) Hit(with interface{}, power f32) []items.Item {
    return nil
}

// Kill kills the chicken, dropping its inventory
fn (c *Chicken) Kill() []items.Item {
    tmp = c.backpack
	c.backpack = make([]items.Item, 1)
    return []items.Item(tmp)
}

// IsAlive returns true if the chicken is alive
fn (c *Chicken) IsAlive() bool {
	return true
}

// Logic performs logic for the Chicken.
fn (c *Chicken) Logic(delta f32) {
	// c.PhysicalObject.Physics(delta)
	c.Animate(delta)
}

// Animate moves and calculates sprite frames for the
// Chicken
fn (c *Chicken) Animate(delta f32) {
	chickenSprites[c.action].Animate(delta)
}

// render renders the chicken onto the screen
fn (c *Chicken) render(cam *render.Camera) {
	chickenSprites[c.action].render(cam)
}
