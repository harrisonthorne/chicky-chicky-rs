#![allow(dead_code)]

pub mod texture;
pub mod traits;

use std::time::Instant;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

pub use texture::*;
pub use traits::*;

pub struct Engine {
    window: Window,
    window_size: winit::dpi::PhysicalSize<u32>,

    device: wgpu::Device,
    queue: wgpu::Queue,
    swap_chain_descriptor: wgpu::SwapChainDescriptor,

    fps: f32,
    last_update_time: Instant,
    runner: Option<Box<dyn Runner>>,
    modifiers: ModifiersState,

    surface: wgpu::Surface,
    swap_chain: wgpu::SwapChain,

    depth_texture: texture::Texture2d,
}

impl Engine {
    pub async fn new(fps: f32, window: Window) -> Engine {
        // The surface is used to create the swap_chain
        let surface = wgpu::Surface::create(&window);

        let window_size = window.inner_size();

        let (device, queue) = {
            // the adapter is used to create the device and the queue
            let adapter = wgpu::Adapter::request(
                &wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::Default,
                    compatible_surface: Some(&surface),
                },
                wgpu::BackendBit::PRIMARY,
            )
            .await
            .unwrap();
            adapter.request_device(&Default::default()).await
        };

        // Here we are defining and creating the swap_chain.
        //
        // The usage field describes how the swap_chain's underlying textures will be used.
        // OUTPUT_ATTACHMENT specifies that the textures will be used to write to the screen.
        //
        // The format defines how the swap_chains textures will be stored on the gpu. Usually you
        // want to specify the format of the display you're using.

        let swap_chain_descriptor = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: 100,
            height: 100,
            present_mode: wgpu::PresentMode::Fifo,
        };

        let swap_chain = device.create_swap_chain(&surface, &swap_chain_descriptor);

        let depth_texture = texture::Texture2d::make_depth_texture(&device, &swap_chain_descriptor);

        Self {
            window,
            window_size,
            fps,
            device,
            queue,
            swap_chain_descriptor,
            last_update_time: Instant::now(),
            modifiers: Default::default(),
            runner: None,

            surface,
            depth_texture,
            swap_chain,
        }
    }

    /// Sets the runner that will update and render the scene for the Engine.
    pub fn set_runner<R: Runner + 'static>(&mut self, r: R) {
        self.runner = Some(Box::new(r));
    }

    /// If we want to support resizing in our application, we're going to need to recreate the
    /// swap_chain everytime the window's size changes. That's the reason we store the logical
    /// size and the swap_chain_descriptor used to create the swapchain. With all of these, the resize method is
    /// very simple.
    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.window_size = new_size;

        self.swap_chain_descriptor.width = new_size.width;
        self.swap_chain_descriptor.height = new_size.height;

        self.swap_chain = self
            .device
            .create_swap_chain(&self.surface, &self.swap_chain_descriptor);
        self.depth_texture =
            texture::Texture2d::make_depth_texture(&self.device, &self.swap_chain_descriptor);
    }

    /// Handles window events.
    fn window_event(&mut self, event: &WindowEvent, control_flow: &mut ControlFlow) {
        if let Some(runner) = &mut self.runner {
            runner.window_event(event, control_flow);
        }    
    }

    /// Handles device events sent by the operating system.
    fn device_event(&mut self, event: DeviceEvent) {
        if let Some(runner) = &mut self.runner {
            runner.device_event(&event)
        }
    }


    /// Perform logic for all logicables. Returns true if logic was performed; false otherwise.
    fn logic(&mut self, delta_secs: f32) -> bool {
        if let Some(updater) = &mut self.runner {
            // update via the updater
            updater.update(delta_secs, &self.device, &mut self.queue)
        } else {
            false
        }
    }

    fn render(&mut self) {
        if let Some(renderer) = &self.runner {
            // First we need to get a frame to render to. This will include a wgpu::Texture and
            // wgpu::TextureView that will hold the actual image we're drawing to
            let frame = self.swap_chain.get_next_texture().unwrap();

            // We also need to create a CommandEncoder to create the actual commands to send to the gpu. Most
            // modern graphics frameworks expect commands to be stored in a command buffer before being sent to
            // the gpu. The encoder builds a command buffer that we can then send to the gpu.
            let mut encoder = self
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("render encoder"),
                });

            renderer.render(
                &self.device,
                &mut encoder,
                &frame.view,
                &self.depth_texture.view,
            );

            // tell wgpu to finish the command buffer, and to submit it to the gpu's render queue.
            // `encoder` must not be borrowed at this point; are previous borrows scoped?
            self.queue.submit(&[encoder.finish()]);
        }
    }

    pub fn compile_shader_modules(
        &self,
        vs_src: &str,
        fs_src: &str,
    ) -> Result<(wgpu::ShaderModule, wgpu::ShaderModule), BasicError> {
        let vs_spirv = match glsl_to_spirv::compile(vs_src, glsl_to_spirv::ShaderType::Vertex) {
            Ok(v) => v,
            Err(e) => return Err(BasicError::from(("couldn't compile vertex shader", e))),
        };
        let fs_spirv = match glsl_to_spirv::compile(fs_src, glsl_to_spirv::ShaderType::Fragment) {
            Ok(f) => f,
            Err(e) => return Err(BasicError::from(("couldn't compile fragment shader", e))),
        };

        let vs_data = match wgpu::read_spirv(vs_spirv) {
            Ok(v) => v,
            Err(e) => return Err(BasicError::from(("couldn't read vertex spirv", e))),
        };
        let fs_data = match wgpu::read_spirv(fs_spirv) {
            Ok(f) => f,
            Err(e) => return Err(BasicError::from(("couldn't read fragment spirv", e))),
        };

        let vs_module = self.device.create_shader_module(&vs_data);
        let fs_module = self.device.create_shader_module(&fs_data);

        Ok((vs_module, fs_module))
    }

    pub fn get_window(&self) -> &Window {
        &self.window
    }

    pub fn get_device(&self) -> &wgpu::Device {
        &self.device
    }

    pub fn get_queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    pub fn get_swap_chain_descriptor(&self) -> &wgpu::SwapChainDescriptor {
        &self.swap_chain_descriptor
    }

    /// Consumes the Engine and starts it.
    pub fn start<T>(mut self, event_loop: EventLoop<T>) -> ! {
        let mut frame_count = 0;
        let mut last_fps_report = Instant::now();
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == self.window.id() => {
                    self.window_event(event, control_flow);

                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            self.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            // new_inner_size is &mut, so we have to dereference it twice
                            self.resize(**new_inner_size);
                        }
                        _ => (),
                    }
                }
                Event::DeviceEvent { event, ..} => {
                    self.device_event(event);
                }
                Event::MainEventsCleared => {
                    let elapsed = self.last_update_time.elapsed().as_secs_f32();
                    if elapsed >= 1.0 / self.fps {
                        // only request rendering if something was updated
                        if self.logic(elapsed) {
                            self.window.request_redraw();
                        }

                        self.last_update_time = Instant::now();
                    } else {
                        // sleep until the next update. NOTE: this might be bad, so remove if there are
                        // problems.
                        std::thread::sleep(std::time::Duration::from_secs_f32(
                            1.0 / self.fps - elapsed,
                        ));
                    }
                }
                Event::RedrawRequested(_) => {
                    self.render();
                    frame_count += 1;

                    // report fps
                    let elapsed = last_fps_report.elapsed();
                    if elapsed >= std::time::Duration::from_secs(1) {
                        println!("{} fps", frame_count as f32 / elapsed.as_secs_f32());
                        frame_count = 0;
                        last_fps_report = Instant::now();
                    }
                }
                _ => (),
            }
        })
    }
}

#[derive(Debug)]
pub struct BasicError {
    message: String,
}

impl std::fmt::Display for BasicError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl<E: std::fmt::Display> From<(&str, E)> for BasicError {
    fn from(tuple: (&str, E)) -> Self {
        Self {
            message: format!("{}: {}", tuple.0, tuple.1),
        }
    }
}

impl std::error::Error for BasicError {}

// vim: foldmethod=syntax
