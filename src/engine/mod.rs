pub mod camera;
pub mod physics;
pub mod texture;
pub mod traits;

use std::collections::HashMap;
use std::hash::Hash;
use std::time::Instant;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

pub use texture::*;
pub use traits::*;

pub struct Engine<K: Hash + Eq + 'static> {
    state: Option<EngineState>,
    device: wgpu::Device,
    queue: wgpu::Queue,

    fps: f32,
    last_update_time: Instant,
    logicables: Vec<Box<dyn Logicable>>,
    renderables: Vec<Box<dyn Renderable>>,
    input_listeners: Vec<Box<dyn InputListener>>,

    render_pipelines: HashMap<K, wgpu::RenderPipeline>,
}

struct EngineState {
    window: Window,
    window_size: winit::dpi::PhysicalSize<u32>,
    event_loop: EventLoop<()>,

    surface: wgpu::Surface,
    swap_chain_descriptor: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,

    depth_texture: texture::Texture,

    camera: camera::Camera,
    uniforms: Uniforms,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
    uniform_bind_group_layout: wgpu::BindGroupLayout,
}

impl<K: Hash + Eq + 'static> Engine<K> {
    pub fn new(fps: f32) -> Self {
        let (device, queue) = {
            // the adapter is used to create the device and the queue
            let adapter = wgpu::Adapter::request(&Default::default()).unwrap();
            adapter.request_device(&Default::default())
        };
        Self {
            device,
            queue,
            state: None,
            fps,
            last_update_time: Instant::now(),
            logicables: Vec::new(),
            renderables: Vec::new(),
            input_listeners: Vec::new(),
            render_pipelines: HashMap::new(),
        }
    }

    /// Consumes the Engine and starts it.
    pub fn start(mut self) {
        let event_loop = EventLoop::new();

        self.make_state(&event_loop);

        match self.state {
            Some(state) => 
                event_loop.run(move |event, _, control_flow| {
                    match event {
                        Event::WindowEvent {
                            ref event,
                            window_id,
                        } if window_id == state.window.id() => {
                            if self.input(event) {
                                *control_flow = ControlFlow::Wait;
                            } else {
                                match event {
                                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                                    WindowEvent::KeyboardInput { input, .. } => match input {
                                        // exit on <esc>
                                        KeyboardInput {
                                            state: ElementState::Pressed,
                                            virtual_keycode: Some(VirtualKeyCode::Escape),
                                            ..
                                        } => *control_flow = ControlFlow::Exit,
                                        _ => *control_flow = ControlFlow::Wait,
                                    },
                                    WindowEvent::Resized(physical_size) => {
                                        self.resize(*physical_size);
                                        *control_flow = ControlFlow::Wait;
                                    },
                                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                                        // new_inner_size is &mut, so we have to dereference it twice
                                        self.resize(**new_inner_size);
                                        *control_flow = ControlFlow::Wait;
                                    },
                                    _ => *control_flow = ControlFlow::Wait,
                                }
                            }
                        }
                        Event::MainEventsCleared => if self.logic() {
                            state.window.request_redraw();
                        },
                        Event::RedrawRequested(_) => {
                            self.render();
                            *control_flow = ControlFlow::Wait;
                        }
                        _ => *control_flow = ControlFlow::Wait,
                    }
                }),
            None => {
                panic!("engine state was somehow None");
            }
        }
    }

    fn make_state(&mut self, event_loop: &EventLoop<()>) {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();

        let window_size = window.inner_size();

        // The surface is used to create the swap_chain
        let surface = wgpu::Surface::create(&window);

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
            width: window_size.width,
            height: window_size.height,
            present_mode: wgpu::PresentMode::Vsync,
        };
        let swap_chain = self.device.create_swap_chain(&surface, &swap_chain_descriptor);

        // make camera
        let camera = camera::Camera {
            // position the camera one unit up and 2 units back
            eye: (0.0, 1.0, -2.0).into(),
            // have it look at the origin
            target: (0.0, 0.0, 0.0).into(),
            // which way is "up"
            up: cgmath::Vector3::unit_y(),
            aspect: swap_chain_descriptor.width as f32 / swap_chain_descriptor.height as f32,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        };

        // uniforms
        let mut uniforms = Uniforms::new();
        uniforms.update_view_proj(&camera);

        let uniform_buffer = self.device
            .create_buffer_mapped(1, wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST) // COPY_DST will be important later
            .fill_from_slice(&[uniforms]);

        let uniform_bind_group_layout =
            self.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                bindings: &[wgpu::BindGroupLayoutBinding {
                    binding: 0,

                    // camera manipulates vertices, hence visible to vertex shader stages
                    visibility: wgpu::ShaderStage::VERTEX,

                    ty: wgpu::BindingType::UniformBuffer {
                        // buffer will not change size
                        dynamic: false,
                    },
                }],
            });

        let uniform_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            bindings: &[wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::Buffer {
                    buffer: &uniform_buffer,
                    range: 0..std::mem::size_of_val(&uniforms) as wgpu::BufferAddress,
                },
            }],
        });

        let depth_texture = texture::Texture::make_depth_texture(&self.device, &swap_chain_descriptor);

        self.state = Some(EngineState {
            window,
            window_size,
            event_loop,
            surface,
            swap_chain_descriptor,
            swap_chain,
            depth_texture,
            camera,
            uniforms,
            uniform_buffer,
            uniform_bind_group,
            uniform_bind_group_layout,
        });
    }

    /// "Registers" a Logicable with the Engine so that the Engine will perform logic for it.
    /// Logicables can return false in their logic() function to remove themselves from the
    /// Engine's Logicables.
    pub fn register_logicable<L: Logicable + 'static>(&mut self, logicable: L) {
        self.logicables.push(Box::new(logicable));
    }

    /// "Registers" a Renderable with the Engine so that the Engine will render it. Renderables can
    /// return false in their render() function to remove themselves from the Engine's Renderables.
    pub fn register_renderable<R: Renderable + 'static>(&mut self, renderable: R) {
        self.renderables.push(Box::new(renderable));
    }

    /// "Registers" a render pipeline that the Engine can use to render Renderables.
    pub fn register_render_pipeline(&mut self, key: K, pipeline: wgpu::RenderPipeline) {
        self.render_pipelines.insert(key, pipeline);
    }

    /// If we want to support resizing in our application, we're going to need to recreate the
    /// swap_chain everytime the window's size changes. That's the reason we store the logical
    /// size and the swap_chain_descriptor used to create the swapchain. With all of these, the resize method is
    /// very simple.
    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if let Some(state) = self.state {
            state.window_size = new_size;
            state.swap_chain_descriptor.width = new_size.width;
            state.swap_chain_descriptor.height = new_size.height;
            state.swap_chain = self
                .device
                .create_swap_chain(&state.surface, &state.swap_chain_descriptor);
            state.depth_texture =
                texture::Texture::make_depth_texture(&self.device, &state.swap_chain_descriptor);
        }
    }

    /// input() returns a bool to indicate whether an event has been fully processed. If the method
    /// returns true, the main loop won't process the event any further.
    fn input(&mut self, _event: &WindowEvent) -> bool {
        false
    }

    /// Perform logic for all logicables. Returns true if logic was performed; false otherwise.
    fn logic(&mut self) -> bool {
        // skip logic if we're moving faster than the frame rate
        if self.last_update_time.elapsed().as_secs_f32() < 1.0 / self.fps {
            false
        } else {
            self.update_uniforms();

            // update all logicables
            for l in self.logicables.iter_mut() {
                let delta = self.last_update_time.elapsed().as_secs_f32();
                l.logic(delta);
            }

            // update last update time
            self.last_update_time = Instant::now();

            true
        }
    }

    fn render(&mut self) {
        if let Some(state) = self.state {
            // First we need to get a frame to render to. This will include a wgpu::Texture and
            // wgpu::TextureView that will hold the actual image we're drawing to
            let frame = state.swap_chain.get_next_texture();

            // We also need to create a CommandEncoder to create the actual commands to send to the gpu. Most
            // modern graphics frameworks expect commands to be stored in a command buffer before being sent to
            // the gpu. The encoder builds a command buffer that we can then send to the gpu.
            let mut encoder = self
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });

            {
                // TODO
                let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &frame.view,
                        resolve_target: None,
                        load_op: wgpu::LoadOp::Clear,
                        store_op: wgpu::StoreOp::Store,
                        clear_color: wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        },
                    }],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
                        attachment: &state.depth_texture.view,
                        depth_load_op: wgpu::LoadOp::Clear,
                        depth_store_op: wgpu::StoreOp::Store,
                        clear_depth: 1.0,
                        stencil_load_op: wgpu::LoadOp::Clear,
                        stencil_store_op: wgpu::StoreOp::Store,
                        clear_stencil: 0,
                    }),
                });

                // TODO
                // render_pass.set_pipeline(&self.render_pipeline);
            }

            // tell wgpu to finish the command buffer, and to submit it to the gpu's render queue
            // `encoder` must not be borrowed at this point; are previous borrows scoped?
            self.queue.submit(&[encoder.finish()]);
        }
    }

    fn update_uniforms(&mut self) {
        if let Some(state) = self.state {
            state.uniforms.update_view_proj(&state.camera);

            // Copy operations are performed on the gpu, so we'll need
            // a CommandEncoder for that
            let mut encoder = self
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });

            let staging_buffer = self
                .device
                .create_buffer_mapped(1, wgpu::BufferUsage::COPY_SRC)
                .fill_from_slice(&[state.uniforms]);

            encoder.copy_buffer_to_buffer(
                &staging_buffer,
                0,
                &state.uniform_buffer,
                0,
                std::mem::size_of::<Uniforms>() as wgpu::BufferAddress,
            );

            // We need to remember to submit our CommandEncoder's output
            // otherwise we won't see any change.
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

    pub fn get_device(&self) -> &wgpu::Device {
        &self.device
    }
}

#[repr(C)] // We need this for Rust to store our data correctly for the shaders
#[derive(Copy, Clone)] // This is so we can store this in a buffer
pub struct Uniforms {
    pub view_proj: cgmath::Matrix4<f32>,
}

impl Uniforms {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &camera::Camera) {
        self.view_proj = camera.build_view_projection_matrix();
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

impl std::error::Error for BasicError { }
