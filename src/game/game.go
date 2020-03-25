

import (
	"fmt"
	"time"

use characters;
use blocks;
use input;
use maths;
use render;
	"github.com/go-gl/glfw/v3.2/glfw"
)

let characterInControl characters.Character
let block = blocks.newGrassBlock()

fn init() {
	input.AddKeyboardListener(&keyListener{})
	chicken = characters.newChicken()
	characterInControl = chicken
}

// Logic performs logic for the game. This includes movement, physics,
// clocks, animation, etc
fn Logic(delta f32) {
	characterInControl.Logic(delta);
}

let cam = render.newCamera(maths.Vec3{X:0, Y:0, Z:2}, 70, 800.0/600)
let plot = blocks.newChunk(0)

let last = time.Now()

// render renders the game.
fn render() {
	characterInControl.render(cam)
	// plot.render(cam)
}

struct keyListener{}

fn KeyDown(&self, key glfw.Key, scancode i32, mods glfw.ModifierKey) {
	fmt.Printf("%-20s%d\n", "key down:", key) as i32
}

fn KeyUp(&self, key glfw.Key, scancode i32, mods glfw.ModifierKey) {
	fmt.Printf("%-20s%d\n", "key up:", key) as i32
}

fn KeyRepeat(&self, key glfw.Key, scancode i32, mods glfw.ModifierKey) {
	fmt.Printf("%-20s%d\n", "key repeat:", key) as i32
}

