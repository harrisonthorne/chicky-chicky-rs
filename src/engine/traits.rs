use winit::dpi::PhysicalPosition;
use winit::event::{ModifiersState, MouseButton, MouseScrollDelta, VirtualKeyCode};

pub trait Runner {
    /// Do logic on and update the game. This function should handle the logic of other objects in
    /// the game as well. This might include physics, animation, what have you.
    fn update(&mut self, delta_sec: f32) -> bool;

    /// Renders the contents of the game.
    fn render(&self, device: &wgpu::Device, encoder: &mut wgpu::CommandEncoder, frame: &wgpu::TextureView, depth_texture: &wgpu::TextureView);
}

/// A simple abstraction for user input.
#[allow(unused_variables)]
pub trait InputListener {
    /// Handles a key press. Returns true if input was processed. Otherwise, return false to pass
    /// input onto the next listener.
    fn key_down(&mut self, key: Option<VirtualKeyCode>, modifiers: ModifiersState) -> bool {
        false
    }

    /// Handles the release of a key. Returns true if input was processed. Otherwise, return false
    /// to pass input onto the next listener.
    fn key_up(&mut self, key: Option<VirtualKeyCode>, modifiers: ModifiersState) -> bool {
        false
    }

    /// Handles the repetition of a key. Returns true if input was processed. Otherwise, return
    /// false to pass input onto the next listener.
    fn key_repeat(&mut self, key: Option<VirtualKeyCode>, modifiers: ModifiersState) -> bool {
        false
    }

    /// Handles a movement of the mouse cursor. Returns true if input was processed. Otherwise,
    /// return false to pass input onto the next listener.
    fn cursor_move(&mut self, position: PhysicalPosition<f64>, modifiers: ModifiersState) -> bool {
        false
    }

    /// Handles a mouse button press. Returns true if input was processed. Otherwise, return false
    /// to pass input onto the next listener.
    fn mouse_button_down(&mut self, button: MouseButton, modifiers: ModifiersState) -> bool {
        false
    }

    /// Handles a mouse button release. Returns true if input was processed. Otherwise, return
    /// false to pass input onto the next listener.
    fn mouse_button_up(&mut self, button: MouseButton, modifiers: ModifiersState) -> bool {
        false
    }

    /// Handles a mouse scroll event. Returns true if input was processed. Otherwise, return false
    /// to pass input onto the next listener.
    fn mouse_scroll(&mut self, scroll_delta: MouseScrollDelta, modifiers: ModifiersState) -> bool {
        false
    }
}
