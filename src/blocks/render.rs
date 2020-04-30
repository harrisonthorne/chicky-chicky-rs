#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub uv_coords: [f32; 2],
}

impl Vertex {
    pub const SIZE: u64 = std::mem::size_of::<Self>() as u64;
}

pub fn make_block_render_pipeline(
    engine: &crate::engine::Engine,
    uniform_bind_group_layout: &wgpu::BindGroupLayout,
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
        include_str!("../shaders/block.vert"),
        include_str!("../shaders/block.frag"),
    )?;

    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        bind_group_layouts: &[&texture_bind_group_layout, &uniform_bind_group_layout],
    });

    let block_vertex_buffer_descriptors = super::Block::vertex_buffer_descriptors();

    let block_render_pipeline_descriptor = crate::utils::make_render_pipeline_descriptor(
        &render_pipeline_layout,
        &vs_module,
        &fs_module,
        &color_states,
        block_vertex_buffer_descriptors,
        true,
    );

    Ok(device.create_render_pipeline(&block_render_pipeline_descriptor))
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
