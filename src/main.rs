//! chicky-chicky-rs

#![allow(dead_code)]
#![warn(missing_docs)]
#![deny(unused_variables)]
#![deny(clippy::shadow_unrelated)]

mod blocks;
mod camera;
mod characters;
mod engine;
mod game;
mod items;
mod maths;
mod physics;
mod sprite;
mod textures;
mod traits;
mod uniforms;
mod utils;
mod world;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    println!("PRINTING ON MAIN");
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    window.set_title("Chicky Chicky");
    window.set_cursor_grab(true).expect("couldn't grab cursor");
    window.set_cursor_visible(false);

    let mut engine = async_std::task::block_on(engine::Engine::new(60.0, window));

    // textures

    let block_texture_bind_group_layout =
        engine
            .get_device()
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                bindings: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::SampledTexture {
                            multisampled: false,
                            dimension: wgpu::TextureViewDimension::D3,
                            component_type: wgpu::TextureComponentType::Uint,
                        },
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::Sampler { comparison: false },
                    },
                ],
                label: None,
            });

    let texture_dimensions = (16, 16);

    let default_textures = {
        use textures::BlockTextures;

        let (textures, cmds) = match BlockTextures::default_textures(
            engine.get_device(),
            texture_dimensions,
            &block_texture_bind_group_layout,
        ) {
            Ok(tc) => tc,
            Err(e) => {
                eprintln!("couldn't make default textures: {}", e);
                std::process::exit(1);
            }
        };

        engine.get_queue().submit(&cmds);

        textures
    };

    // uniforms and buffer

    let uniforms = uniforms::Uniforms::new();

    let uniform_buffer = engine.get_device().create_buffer_with_data(
        bytemuck::cast_slice(&[uniforms]),
        wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
    );

    let uniform_bind_group_layout =
        engine
            .get_device()
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                bindings: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,

                    // camera manipulates vertices, hence visible to vertex shader stages
                    visibility: wgpu::ShaderStage::VERTEX,

                    ty: wgpu::BindingType::UniformBuffer {
                        // buffer will not change size
                        dynamic: false,
                    },
                }],
                label: Some("uniform bind group layout"),
            });

    let uniform_bind_group = engine
        .get_device()
        .create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            bindings: &[wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::Buffer {
                    buffer: &uniform_buffer,
                    range: 0..std::mem::size_of_val(&uniforms) as wgpu::BufferAddress,
                },
            }],
            label: Some("uniform bind group"),
        });

    // chunk render pipeline
    let block_render_pipeline = match blocks::render::make_chunk_render_pipeline(
        &mut engine,
        &block_texture_bind_group_layout,
        &uniform_bind_group_layout,
    ) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    let camera = camera::Camera::default();
    let camera_controller = camera::CameraController::new(5.0, 1.0);

    let game = game::Game::new(engine.get_device());

    let runner = MainRunner {
        state: GameState::Game(Box::new(game)),

        uniforms,
        uniform_buffer,
        uniform_bind_group,
        // uniform_bind_group_layout,
        block_render_pipeline,
        camera,
        camera_controller,
        block_textures: default_textures,
    };

    engine.set_runner(runner);
    engine.start(event_loop);
}

struct MainRunner {
    state: GameState,

    uniforms: uniforms::Uniforms,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
    // uniform_bind_group_layout: wgpu::BindGroupLayout,
    camera: camera::Camera,
    camera_controller: camera::CameraController,

    block_textures: textures::BlockTextures,
    block_render_pipeline: wgpu::RenderPipeline,
}

impl engine::Runner for MainRunner {
    fn window_event(&mut self, event: &WindowEvent, control_flow: &mut ControlFlow) {
        if let WindowEvent::KeyboardInput {
            input:
                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Q),
                    ..
                },
                ..
        } = event
        {
            *control_flow = ControlFlow::Exit;
        } else {
            self.camera_controller.input(event);
        }
    }

    fn device_event(&mut self, event: &DeviceEvent) {
        if let DeviceEvent::MouseMotion { delta: (x, y) } = event {
            self.camera_controller.mouse_moved(*x, *y);
        }
    }

    fn update(&mut self, delta_sec: f32, device: &wgpu::Device, queue: &mut wgpu::Queue) -> bool {
        self.camera_controller
            .update_camera(delta_sec, &mut self.camera);
        self.uniforms
            .update(device, &self.camera, &mut self.uniform_buffer, queue);

        match &mut self.state {
            GameState::Game(g) => g.logic(device, queue),
        }

        true
    }

    fn render(
        &self,
        _device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        frame: &wgpu::TextureView,
        depth_texture: &wgpu::TextureView,
    ) {
        let mut payload = RenderPayload {
            // device,
            // queue,
            encoder,
            frame,
            depth_texture,
            block_render_pipeline: &self.block_render_pipeline,
            uniform_bind_group: &self.uniform_bind_group,
            block_texture_bind_group: &self.block_textures.get_bind_group(),
        };

        #[allow(clippy::single_match)]
        match &self.state {
            GameState::Game(g) => g.render(&mut payload),
        }
    }
}

enum GameState {
    // MainMenu,
    Game(Box<game::Game>),
}

struct RenderPayload<'a> {
    // device: &'a wgpu::Device,
    // queue: &'a mut wgpu::Queue,
    encoder: &'a mut wgpu::CommandEncoder,
    frame: &'a wgpu::TextureView,
    depth_texture: &'a wgpu::TextureView,
    block_render_pipeline: &'a wgpu::RenderPipeline,
    block_texture_bind_group: &'a wgpu::BindGroup,
    uniform_bind_group: &'a wgpu::BindGroup,
}
