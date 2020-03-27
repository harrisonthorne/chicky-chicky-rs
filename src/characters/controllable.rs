use items;

// Controllable is a type that can be controlled using the
// mouse or keyboard. It implements both KeyboardListener
// and MouseListener.
trait Controllable {
    fn move(direction: Direction, super: bool);
    fn down(super: bool);       // Do something downward (fall or squat maybe?)
    fn jump(super: bool);       // Do something when the space bar is pressed
    fn stop();      // Nothing is happening anymore

    /// Initiates an attack by the Controllable. Returns
    /// whatever the Controllable might be holding, the
    /// attack power, and where the Controllable was aiming
    fn attack(with: &items::Item, power: f32, who: impl Character);
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
