use crate::characters::{Character, CharacterAction, Direction, FacingDirection};
use crate::physics::PhysicalObject;
use crate::items;
use crate::items::{Backpack, Item, ItemStack};
use crate::maths::AABB;
use crate::traits::{Killable, Logicable, Renderable};
use cgmath::Vector3;

/// The main character of this game. we ain't callin it chicky chicky for nothing folks
#[derive(Debug)]
pub struct Chicken {
    physical: PhysicalObject,
    backpack: Backpack,
    action: CharacterAction,
    facing: FacingDirection,
    health: f32,
    lifespan: f32,
    // chicken_sprites: HashMap<CharacterAction, Sprite>,
}

impl Chicken {
    /// Creates and initializes a new Chicken
    pub fn new() -> Self {
        // let c: Self = Default::default();

        // c.chicken_sprites[CharacterAction::Nothing] =
        //     Sprite::must_new("assets/photos/chicken/stand.png", 0, 0);
        // c.chicken_sprites[CharacterAction::Run] =
        //     Sprite::must_new("assets/photos/chicken/sprint.png", 4, 0.15);
        // c.chicken_sprites[CharacterAction::Walk] =
        //     Sprite::must_new("assets/photos/chicken/walk.png", 4, 0.2);
        // c.chicken_sprites[CharacterAction::Squat] =
        //     Sprite::must_new("assets/photos/chicken/squat.png", 0, 0);
        // c.chicken_sprites[CharacterAction::Push] =
        //     Sprite::must_new("assets/photos/chicken/push.png", 4, 0.75);
        // c.chicken_sprites[CharacterAction::Fall] =
        //     Sprite::must_new("assets/photos/chicken/fall.png", 2, 0.1);

        Default::default()
    }
}

impl Default for Chicken {
    fn default() -> Self {
        Self {
            physical: PhysicalObject::new(
                2.0,
                AABB {
                    center_pos: Vector3::from((0.0, 0.0, 0.0)),
                    half_size: Vector3::from((0.0, 0.0, 0.0)),
                },
            ),
            backpack: Default::default(),
            action: Default::default(),
            facing: Default::default(),
            health: Default::default(),
            lifespan: Default::default(),
        }
    }
}

impl Logicable for Chicken {
    fn logic(&mut self, _delta_sec: f32) {
        todo!()
    }
}

impl Renderable for Chicken {
    fn render(&self, _: &wgpu::RenderPass) {
        todo!()
    }
}

impl Character for Chicken {
    /// Walks the chicken
    fn walk(&mut self, direction: Direction, sup: bool) {
        if sup {
            self.action = CharacterAction::Run;
        } else {
            self.action = CharacterAction::Walk;
        }

        match direction {
            Direction::Left => self.facing = FacingDirection::Left,
            Direction::Right => self.facing = FacingDirection::Right,
            _ => (),
        }
    }

    /// Jumps the chicken
    fn jump(&mut self, _sup: bool) {
        self.physical.apply_force(Vector3::from((0.0, 6.0, 0.0)));
    }

    /// Squats the chicken
    fn down(&mut self, _sup: bool) {
        self.stop();
        self.action = CharacterAction::Squat;
    }

    /// Stops the chicken's movement
    fn stop(&mut self) {
        self.action = CharacterAction::Nothing;
    }

    fn attack<K: Killable>(&self, _with: Option<&items::Item>, _power: f32, _who: K) {}

    fn render(&self) {}
}

impl Killable for Chicken {
    /// Hits the chicken with the object and power specified.
    fn hit(&mut self, _with: Option<Item>, _power: f32) -> &[items::ItemStack] {
        &[]
    }

    /// Kills the chicken, dropping its inventory
    fn kill(&mut self) -> &[ItemStack] {
        &self.backpack
    }

    fn health_left(&self) -> f32 {
        self.health
    }

    fn lifespan(&self) -> f32 {
        self.lifespan
    }

    /// Returns true if the chicken is alive
    fn is_alive(&self) -> bool {
        true
    }
}
