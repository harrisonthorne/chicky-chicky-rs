use maths;
use items;

// Controllable is a type that can be controlled using the
// mouse or keyboard. It implements both KeyboardListener
// and MouseListener.
trait Controllable {
    Move(direction Direction, super bool)
	Down(super bool)       // Do something downward (fall or squat maybe?)
	Jump(super bool)       // Do something when the space bar is pressed
	Stop()      // Nothing is happening anymore

    // Initiates an attack by the Controllable. Returns
    // whatever the Controllable might be holding, the
    // attack power, and where the Controllable was aiming
	Attack() (with &items.Item, power f32, at maths.Vec2)
}

// TODO
// // KeyDown handles a Controllable action when a key
// // is pressed. Implemented from KeyboardListener.
// fn KeyDown(&self, key glfw.Key, scancode i32, mods glfw.ModifierKey) {
// 	fmt.Printf("Key down: %v\n", key)
//     if !c.inControl { return }

// 	match key {
// 	glfw.KeyA =>
// 		c.Move(DirectionLeft, false)
// 	glfw.KeyS =>
// 		c.Down(false)
// 	glfw.KeyD =>
// 		c.Move(DirectionRight, false)
// 	glfw.KeySpace =>
//         c.Jump(false)
// 	}
// }

// // KeyUp handles a Controllable action when a key
// // is released. Implemented from KeyboardListener.
// fn KeyUp(&self, key glfw.Key, scancode i32, mods glfw.ModifierKey) {
// 	fmt.Printf("Key up: %v\n", key)
//     if !c.inControl { return }

// }
