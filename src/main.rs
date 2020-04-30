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
mod traits;
mod world;
mod utils;

fn main() {
    let mut engine = engine::Engine::new(60.0);

    let device = engine.get_device();

    let uniform_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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

    // uniforms
    let uniforms = Uniforms::new();

    let uniform_buffer = device
        .create_buffer_mapped(1, wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST)
        .fill_from_slice(&[uniforms]);

    let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &uniform_bind_group_layout,
        bindings: &[wgpu::Binding {
            binding: 0,
            resource: wgpu::BindingResource::Buffer {
                buffer: &uniform_buffer,
                range: 0..std::mem::size_of_val(&uniforms) as wgpu::BufferAddress,
            },
        }],
    });

    let block_render_pipeline = match blocks::render::make_block_render_pipeline(&engine, &uniform_bind_group_layout) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    let game = game::Game::new(uniform_bind_group_layout, uniform_bind_group, block_render_pipeline);

    engine.set_runner(game);
    engine.start();
}

/// Uniforms are used in the shader for attributes that are essentially global.
#[repr(C)] // we need this for Rust to store our data correctly for the shaders
#[derive(Copy, Clone)] // this is so we can store this in a buffer
pub struct Uniforms {
    /// The view-projection matrix.
    pub view_proj: cgmath::Matrix4<f32>,
}

impl Uniforms {
    fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity(),
        }
    }

    fn update_view_proj(&mut self, camera: &engine::Camera) {
        self.view_proj = camera.build_view_projection_matrix();
    }

    fn update(&'static mut self, device: &wgpu::Device, camera: &engine::Camera, uniform_buffer: &mut wgpu::Buffer, queue: &mut wgpu::Queue) {
        self.update_view_proj(&camera);

        // Copy operations are performed on the gpu, so we'll need
        // a CommandEncoder for that
        let mut encoder = device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });

        let staging_buffer = device
            .create_buffer_mapped(1, wgpu::BufferUsage::COPY_SRC)
            .fill_from_slice(&[*self]);

        encoder.copy_buffer_to_buffer(
            &staging_buffer,
            0,
            &uniform_buffer,
            0,
            std::mem::size_of::<Uniforms>() as wgpu::BufferAddress,
        );

        // We need to remember to submit our CommandEncoder's output
        // otherwise we won't see any change.
        queue.submit(&[encoder.finish()]);
    }
}

// vim: foldmethod=syntax
