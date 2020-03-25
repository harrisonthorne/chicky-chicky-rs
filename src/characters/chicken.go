use items;
use world;
use maths;
use sprite;
use render;

let chickenSprites = make(map[CharacterAction]&sprite.Sprite)

// InitGL initializes chicken sprites
fn InitGL() {
	chickenHeight = 0.5 as f32		// in meters
	chickenWidth = 0.5*13/12 as f32	// in meters

    chickenSprites[ActionNothing] = sprite.Mustnew("assets/photos/chicken/stand.png", 0, 0)
    chickenSprites[ActionRun] = sprite.Mustnew("assets/photos/chicken/sprint.png", 4, 0.15)
    chickenSprites[ActionWalk] = sprite.Mustnew("assets/photos/chicken/walk.png", 4, 0.2)
    chickenSprites[ActionSquat] = sprite.Mustnew("assets/photos/chicken/squat.png", 0, 0)
    chickenSprites[ActionPush] = sprite.Mustnew("assets/photos/chicken/push.png", 4, 0.75)
    chickenSprites[ActionFall] = sprite.Mustnew("assets/photos/chicken/fall.png", 2, 0.1)

	for k = range chickenSprites {
		chickenSprites[k].SetSize(chickenHeight, chickenWidth)
	}
}

// Chicken is the main character of this game. we ain't
// callin it chicky chicky for nothing folks
struct Chicken {
	&world.PhysicalObject
	Character

	backpack Backpack
	action CharacterAction
	direction Direction
}

// newChicken creates and initializes a new Chicken
fn newChicken() -> &Chicken {
	return &Chicken{action: ActionWalk}
}

// Move moves the chicken!
fn Move(&self, direction Direction, super bool) ->  {
    if super {
        c.action = ActionRun
    } else {
        c.action = ActionWalk
    }
    c.direction = direction
}

// Jump jumps the chicken
fn Jump(&self, super bool) {
	if c.hitbox != nil {
		c.ApplyForce(maths.Vec3{X: 0, Y: 6, Z: 0})
	}
}

// Down squats the chicken
fn Down(&self, super bool) {
    c.Stop()
    c.action = ActionSquat
}

// Stop stops the chicken's movement
fn Stop(&self) {
    c.action = ActionNothing
}

// hit hits the chicken with the object and power specified.
fn hit(&self, with interface{}, power f32) -> []items.Item {
    return nil
}

// kill kills the chicken, dropping its inventory
fn kill(&self) -> []items.Item {
    tmp = c.backpack
	c.backpack = make([]items.Item, 1)
    return []items.Item(tmp)
}

// is_alive returns true if the chicken is alive
fn is_alive(&self) -> bool {
	return true
}

// Logic performs logic for the Chicken.
fn Logic(&self, delta f32) {
	// c.PhysicalObject.Physics(delta)
	c.Animate(delta)
}

// Animate moves and calculates sprite frames for the
// Chicken
fn Animate(&self, delta f32) {
	chickenSprites[c.action].Animate(delta)
}

// render renders the chicken onto the screen
fn render(&self, cam &render.Camera) {
	chickenSprites[c.action].render(cam)
}
