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

use std::hash::Hash;

fn main() {
    let mut engine = engine::Engine::new(60.0);

    let texture_render_pipeline = match make_texture_render_pipeline(&engine) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };
    engine.register_render_pipeline(RenderPipelineKey::Texture, texture_render_pipeline);

    engine.start();
}

fn make_texture_render_pipeline(
    engine: &engine::Engine<RenderPipelineKey>,
) -> Result<wgpu::RenderPipeline, Box<dyn std::error::Error>> {
    let device = engine.get_device();

    // describes how colors are stored and processed throughout the pipeline
    let color_states = [wgpu::ColorStateDescriptor {
        format: engine.get_swap_chain_descriptor().format,
        color_blend: wgpu::BlendDescriptor::REPLACE,
        alpha_blend: wgpu::BlendDescriptor::REPLACE,

        // write r, g, b, and a
        write_mask: wgpu::ColorWrite::ALL,
    }];

    // BindGroupLayout: describes a set of resources and how they can be accessed by a shader. this
    // creates a BindGroup using a BindGroupLayout:
    let texture_bind_group_layout = make_texture_bind_group_layout(device);

    // compile texture shaders
    let (vs_module, fs_module) = engine.compile_shader_modules(
        include_str!("shaders/threed_tex.vert"),
        include_str!("shaders/tex.frag"),
    )?;

    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        bind_group_layouts: &[
            &texture_bind_group_layout,
            &engine.get_uniform_bind_group_layout(),
        ],
    });

    let render_pipeline_descriptor = get_render_pipeline_descriptor(
        &render_pipeline_layout,
        &vs_module,
        &fs_module,
        &color_states,
    );

    Ok(device.create_render_pipeline(&render_pipeline_descriptor))
}

fn make_texture_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        bindings: &[
            wgpu::BindGroupLayoutBinding {
                binding: 0,
                visibility: wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::SampledTexture {
                    multisampled: false,
                    dimension: wgpu::TextureViewDimension::D2,
                },
            },
            wgpu::BindGroupLayoutBinding {
                binding: 1,
                visibility: wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::Sampler,
            },
        ],
    })
}

fn get_render_pipeline_descriptor<'a>(
    render_pipeline_layout: &'a wgpu::PipelineLayout,
    vs_module: &'a wgpu::ShaderModule,
    fs_module: &'a wgpu::ShaderModule,
    color_states: &'a [wgpu::ColorStateDescriptor],
) -> wgpu::RenderPipelineDescriptor<'a> {
    let rasterization_state = wgpu::RasterizationStateDescriptor {
        front_face: wgpu::FrontFace::Ccw,
        cull_mode: wgpu::CullMode::Back,
        depth_bias: 0,
        depth_bias_slope_scale: 0.0,
        depth_bias_clamp: 0.0,
    };

    wgpu::RenderPipelineDescriptor {
        layout: render_pipeline_layout,
        vertex_stage: wgpu::ProgrammableStageDescriptor {
            module: &vs_module,
            entry_point: "main",
        },
        fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
            module: &fs_module,
            entry_point: "main",
        }),

        rasterization_state: Some(rasterization_state),
        color_states,

        // use triangles to draw
        primitive_topology: wgpu::PrimitiveTopology::TriangleList,

        // no depth or stencil buffer for now
        depth_stencil_state: Some(wgpu::DepthStencilStateDescriptor {
            format: engine::Texture::DEPTH_FORMAT,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            stencil_front: wgpu::StencilStateFaceDescriptor::IGNORE,
            stencil_back: wgpu::StencilStateFaceDescriptor::IGNORE,
            stencil_read_mask: 0,
            stencil_write_mask: 0,
        }),

        // for indices
        index_format: wgpu::IndexFormat::Uint32,

        vertex_buffers: &[],
        sample_count: 1,

        // specifies which samples should be active; in this case, all
        // (!0 is a bitwise NOT of 0, so all bits are 1)
        sample_mask: !0,

        // covers antialiasing; setting to false for now
        alpha_to_coverage_enabled: false,
    }
}

#[derive(Hash, PartialEq, Eq)]
enum RenderPipelineKey {
    Texture,
}
