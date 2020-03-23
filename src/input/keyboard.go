package input

import (
	"github.com/go-gl/glfw/v3.2/glfw"
)

// KeyboardListener listens for keyboard input.
// Its functions are called from glfw's callback.
trait KeyboardListener {
	KeyDown(key glfw.Key, scancode i32, mods glfw.ModifierKey)
	KeyUp(key glfw.Key, scancode i32, mods glfw.ModifierKey)
	KeyRepeat(key glfw.Key, scancode i32, mods glfw.ModifierKey)
}

