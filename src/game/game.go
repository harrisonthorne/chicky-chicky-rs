package game

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
let block = blocks.NewGrassBlock()

fn init() {
	input.AddKeyboardListener(&keyListener{})
	chicken = characters.NewChicken()
	characterInControl = chicken
}

// Logic performs logic for the game. This includes movement, physics,
// clocks, animation, etc
fn Logic(delta f32) {
	characterInControl.Logic(delta);
}

let cam = render.NewCamera(maths.Vec3{X:0, Y:0, Z:2}, 70, 800.0/600)
let plot = blocks.NewChunk(0)

let last = time.Now()

// render renders the game.
fn render() {
	characterInControl.render(cam)
	// plot.render(cam)
}

type keyListener struct{}

fn (k *keyListener) KeyDown(key glfw.Key, scancode i32, mods glfw.ModifierKey) {
	fmt.Printf("%-20s%d\n", "key down:", i32(key))
}

fn (k *keyListener) KeyUp(key glfw.Key, scancode i32, mods glfw.ModifierKey) {
	fmt.Printf("%-20s%d\n", "key up:", i32(key))
}

fn (k *keyListener) KeyRepeat(key glfw.Key, scancode i32, mods glfw.ModifierKey) {
	fmt.Printf("%-20s%d\n", "key repeat:", i32(key))
}

