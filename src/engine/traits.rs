use winit::dpi::PhysicalPosition;
use winit::event::{ModifiersState, MouseButton, MouseScrollDelta, VirtualKeyCode};

pub trait Logicable {
    /// Do logic on an object. Returns true if the object should still be considered for updates, or
    /// false if it should be removed from the Engine's Logicables.
    fn logic(&mut self, delta_sec: f32) -> bool;
}

pub trait Renderable {
    /// Render an object. Returns true if the object should still be considered for rendering, or
    /// false if it should never be rendered again until it is added back into an Engine's
    /// renderables collection.
    fn render(&self) -> bool;
}

pub trait InputListener {
    /// Handles a key press. Returns true if input was processed. Otherwise, return false to pass
    /// input onto the next listener.
    fn key_down(&mut self, _key: Option<VirtualKeyCode>, _modifiers: ModifiersState) -> bool {
        false
    }

    /// Handles the release of a key. Returns true if input was processed. Otherwise, return false
    /// to pass input onto the next listener.
    fn key_up(&mut self, _key: Option<VirtualKeyCode>, _modifiers: ModifiersState) -> bool {
        false
    }

    /// Handles the repetition of a key. Returns true if input was processed. Otherwise, return
    /// false to pass input onto the next listener.
    fn key_repeat(&mut self, _key: Option<VirtualKeyCode>, _modifiers: ModifiersState) -> bool {
        false
    }

    /// Handles a movement of the mouse cursor. Returns true if input was processed. Otherwise,
    /// return false to pass input onto the next listener.
    fn cursor_move(
        &mut self,
        _position: PhysicalPosition<f64>,
        _modifiers: ModifiersState,
    ) -> bool {
        false
    }

    /// Handles a mouse button press. Returns true if input was processed. Otherwise, return false
    /// to pass input onto the next listener.
    fn mouse_button_down(&mut self, _button: MouseButton, _modifiers: ModifiersState) -> bool {
        false
    }

    /// Handles a mouse button release. Returns true if input was processed. Otherwise, return
    /// false to pass input onto the next listener.
    fn mouse_button_up(&mut self, _button: MouseButton, _modifiers: ModifiersState) -> bool {
        false
    }

    /// Handles a mouse scroll event. Returns true if input was processed. Otherwise, return false
    /// to pass input onto the next listener.
    fn mouse_scroll(
        &mut self,
        _scroll_delta: MouseScrollDelta,
        _modifiers: ModifiersState,
    ) -> bool {
        false
    }
}
