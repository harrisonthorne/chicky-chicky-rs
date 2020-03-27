use crate::characters;
use crate::blocks;
use crate::input;
use crate::maths;
use crate::render;

let character_in_control: characters::Character
let block = blocks::new_grass_block();

fn init() {
    chicken = characters::new_chicken();
    character_in_control = chicken;
}

/// Performs logic for the game. This includes movement, physics,
/// clocks, animation, etc
fn logic(delta: f32) {
    character_in_control.logic(delta);
}

let cam = render::new_camera(Vec3{X:0, Y:0, Z:2}, 70, 800.0/600);

let last = time::now();

// render renders the game.
fn render() {
    character_in_control.render(cam);
}
