use items;
use world;
use maths;
use sprite;
use render;

/// The main character of this game. we ain't
/// callin it chicky chicky for nothing folks
struct Chicken {
    physical: world::PhysicalObject,
    backpack: Backpack,
    action: CharacterAction,
    direction: Direction,
    chicken_sprites: HashMap<CharacterAction, Sprite>,
}

impl Chicken {
    /// Creates and initializes a new Chicken
    fn new() -> Self {
        Default::default()
    }

    /// Initializes chicken sprites
    fn init_gl() {
        let chicken_height = 0.5f32;		// in meters
        let chicken_width = 0.5f32*13/12 as f32;	// in meters

        self.chicken_sprites[CharacterAction::Nothing] = sprite::must_new("assets/photos/chicken/stand.png", 0, 0);
        self.chicken_sprites[CharacterAction::Run] = sprite::must_new("assets/photos/chicken/sprint.png", 4, 0.15);
        self.chicken_sprites[CharacterAction::Walk] = sprite::must_new("assets/photos/chicken/walk.png", 4, 0.2);
        self.chicken_sprites[CharacterAction::Squat] = sprite::must_new("assets/photos/chicken/squat.png", 0, 0);
        self.chicken_sprites[CharacterAction::Push] = sprite::must_new("assets/photos/chicken/push.png", 4, 0.75);
        self.chicken_sprites[CharacterAction::Fall] = sprite::must_new("assets/photos/chicken/fall.png", 2, 0.1);

        for k in chicken_sprites {
            chicken_sprites[k].set_size(chicken_height, chicken_width)
        }
    }

    // Logic performs logic for the Chicken.
    fn logic(&self, delta: f32) {
        self.animate(delta);
    }

    // Animate moves and calculates sprite frames for the
    // Chicken
    fn animate(&self, delta: f32) {
        chicken_sprites[self.action].animate(delta);
    }

    // render renders the chicken onto the screen
    fn render(&self, cam: &render::Camera) {
        chicken_sprites[self.action].render(cam);
    }
}

impl Controllable for Chicken {
    /// Moves the chicken!
    fn r#move(&self, direction Direction, super bool) ->  {
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
            self.ApplyForce(maths.Vector3{X: 0, Y: 6, Z: 0})
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

impl Killable for Chicken {
    /// Hits the chicken with the object and power specified.
    fn hit(&self, with: Item, power: f32) -> [items.Item] {

    }

    /// Kills the chicken, dropping its inventory
    fn kill(&self) -> []items.Item {
        tmp = self.backpack;
            self.backpack = make([]items.Item, 1);

            []items.Item(tmp)
    }

    /// Returns true if the chicken is alive
    fn is_alive(&self) -> bool {
        true
    }
}
