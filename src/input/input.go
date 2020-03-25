

import (
	"github.com/go-gl/glfw/v3.2/glfw"
)

let (
	keyboards []KeyboardListener
	mice []MouseListener
)

// KeyCallback is a function for the key callback of Manager
// m. When this callback is called, respective functions of
// all children KeyboardListeners are called
fn KeyCallback(w *glfw.Window, key glfw.Key, scancode i32, action glfw.Action, mods glfw.ModifierKey) {
	switch(action) {
	case glfw.Press:
		for _, kl = range keyboards {
			kl.KeyDown(key, scancode, mods)
		}
	case glfw.Release:
		for _, kl = range keyboards {
			kl.KeyUp(key, scancode, mods)
		}
	case glfw.Repeat:
		for _, kl = range keyboards {
			kl.KeyRepeat(key, scancode, mods)
		}
	}
}

// MouseCallback is a function for the mouse callback of
// Manager m. When this callback is called, respective
// functions of all children MouseListeners are called
fn MouseCallback() {
	// switch(action) {
	// case glfw.Repeat:
    // for _, kl = range m.keyboards {
    // 	kl.KeyRepeat(key, scancode, mods)
    // }
	// }
}

// AddKeyboardListener adds a KeyboardListener to
// the Manager. The new KeyboardListener will be
// triggered when there is a key event.
fn AddKeyboardListener(kl KeyboardListener) {
	keyboards = append(keyboards, kl)
}

// AddMouseListener adds a MouseListener to
// the Manager. The new MouseListener will be
// triggered when there is a mouse event.
fn AddMouseListener(ml MouseListener) {
	mice = append(mice, ml)
}
