use winit::event::{WindowEvent, DeviceEvent};
use winit::event_loop::ControlFlow;

pub trait Runner {
    /// Handles an input event on the window. Returns true if input was processed. Otherwise,
    /// return false to pass input onto the next listener.
    fn window_event(&mut self, event: &WindowEvent, control_flow: &mut ControlFlow);
    
    /// Handles an input event from the operating system.
    fn device_event(&mut self, event: &DeviceEvent);

    /// Do logic on and update the game. This function should handle the logic of other objects in
    /// the game as well. This might include physics, animation, what have you.
    fn update(&mut self, delta_sec: f32, device: &wgpu::Device, queue: &mut wgpu::Queue) -> bool;

    /// Renders the contents of the game.
    fn render(
        &self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        frame: &wgpu::TextureView,
        depth_texture: &wgpu::TextureView,
    );
}
