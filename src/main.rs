//! chicky-chicky-rs

#![warn(missing_docs)]
#![deny(unused_variables)]
#![deny(clippy::shadow_unrelated)]

mod blocks;
mod characters;
mod engine;
mod game;
mod items;
mod maths;
mod sprite;
mod textures;
mod traits;
mod uniforms;
mod utils;
mod world;

use winit::{event_loop::EventLoop, window::WindowBuilder};

#[async_std::main]
async fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut engine = engine::Engine::new(60.0, window).await;

    // make camera
    // let camera = camera::Camera {
    //     // position the camera one unit up and 2 units back
    //     eye: (0.0, 1.0, -2.0).into(),
    //     // have it look at the origin
    //     target: (0.0, 0.0, 0.0).into(),
    //     // which way is "up"
    //     up: cgmath::Vector3::unit_y(),
    //     aspect: swap_chain_descriptor.width as f32 / swap_chain_descriptor.height as f32,
    //     fovy: 45.0,
    //     znear: 0.1,
    //     zfar: 100.0,
    // };

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
                            dimension: wgpu::TextureViewDimension::D2,
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

    let sc_desc = engine.get_swap_chain_descriptor();
    let camera = engine::camera::Camera {
        eye: (0.0, 2.0, -2.0).into(),
        target: (0.0, 0.0, 0.0).into(),
        up: cgmath::Vector3::unit_y(),
        aspect: sc_desc.width as f32 / sc_desc.height as f32,
        fovy: 70.0,
        znear: 0.001,
        zfar: 10000.0,
    };

    let game = game::Game::new().await;

    let runner = MainRunner {
        state: GameState::Game(Box::new(game)),

        uniforms,
        uniform_buffer,
        uniform_bind_group,
        uniform_bind_group_layout,
        block_render_pipeline,
        camera,
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
    uniform_bind_group_layout: wgpu::BindGroupLayout,

    camera: engine::camera::Camera,

    block_textures: textures::BlockTextures,
    block_render_pipeline: wgpu::RenderPipeline,
}

impl engine::Runner for MainRunner {
    fn update(&mut self, _delta_sec: f32, device: &wgpu::Device, queue: &mut wgpu::Queue) -> bool {
        self.uniforms
            .update(device, &self.camera, &mut self.uniform_buffer, queue);
        
        match &mut self.state {
            GameState::Game(g) => g.logic(device),
        }

        true
    }

    fn render(
        &self,
        _device: &wgpu::Device,
        _queue: &mut wgpu::Queue,
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
