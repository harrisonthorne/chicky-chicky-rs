use items;
use world;
use maths;
use sprite;
use render;

let chicken_sprites = make(map[CharacterAction]&sprite::Sprite);

    // InitGL initializes chicken sprites
    fn init_gl() {
        let chicken_height = 0.5f32		// in meters
            let chicken_width = 0.5f32*13/12 as f32;	// in meters

        chicken_sprites[CharacterAction::Nothing] = sprite::must_new("assets/photos/chicken/stand.png", 0, 0)
            chicken_sprites[CharacterAction::Run] = sprite::must_new("assets/photos/chicken/sprint.png", 4, 0.15)
            chicken_sprites[CharacterAction::Walk] = sprite::must_new("assets/photos/chicken/walk.png", 4, 0.2)
            chicken_sprites[CharacterAction::Squat] = sprite::must_new("assets/photos/chicken/squat.png", 0, 0)
            chicken_sprites[CharacterAction::Push] = sprite::must_new("assets/photos/chicken/push.png", 4, 0.75)
            chicken_sprites[CharacterAction::Fall] = sprite::must_new("assets/photos/chicken/fall.png", 2, 0.1)

            for k = range chicken_sprites {
                chicken_sprites[k].SetSize(chickenHeight, chickenWidth)
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

impl Chicken {
    /// Creates and initializes a new Chicken
    fn new() -> Self {
        Default::default()
    }
}

impl Controllable for Chicken {
    /// Moves the chicken!
    fn move(&self, direction Direction, super bool) ->  {
        if super {
            self.action = ActionRun
        } else {
            self.action = ActionWalk
        }
        self.direction = direction
    }

    /// Jumps the chicken
    fn jump(&self, super bool) {
        if self.hitbox != nil {
            self.ApplyForce(maths.Vec3{X: 0, Y: 6, Z: 0})
        }
    }

    /// Squats the chicken
    fn down(&self, super bool) {
        self.Stop()
            self.action = ActionSquat
    }

    /// Stops the chicken's movement
    fn stop(&self) {
        self.action = ActionNothing
    }
}


// hit hits the chicken with the object and power specified.
fn hit(&self, with interface{}, power f32) -> []items.Item {
    return nil
}

// kill kills the chicken, dropping its inventory
fn kill(&self) -> []items.Item {
    tmp = self.backpack
        self.backpack = make([]items.Item, 1)
        return []items.Item(tmp)
}

// is_alive returns true if the chicken is alive
fn is_alive(&self) -> bool {
    return true
}

// Logic performs logic for the Chicken.
fn Logic(&self, delta f32) {
    // self.PhysicalObject.Physics(delta)
    self.Animate(delta)
}

// Animate moves and calculates sprite frames for the
// Chicken
fn Animate(&self, delta f32) {
    chicken_sprites[self.action].Animate(delta)
}

// render renders the chicken onto the screen
fn render(&self, cam &render.Camera) {
    chicken_sprites[self.action].render(cam)
}
