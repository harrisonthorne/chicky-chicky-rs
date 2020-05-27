use winit::event::DeviceEvent;

pub trait Runner {
    /// Do logic on and update the game. This function should handle the logic of other objects in
    /// the game as well. This might include physics, animation, what have you.
    fn update(&mut self, _delta_sec: f32, device: &wgpu::Device, queue: &mut wgpu::Queue) -> bool;

    /// Renders the contents of the game.
    fn render(
        &self,
        device: &wgpu::Device,
        queue: &mut wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        frame: &wgpu::TextureView,
        depth_texture: &wgpu::TextureView,
    );
}

/// A simple abstraction for user input.
#[allow(unused_variables)]
pub trait InputListener {
    /// Handles an input event. Returns true if input was processed. Otherwise, return false to pass
    /// input onto the next listener.
    fn input(&mut self, event: &DeviceEvent) -> bool;
}
