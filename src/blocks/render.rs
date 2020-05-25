/// NOTE: Why are we passing in block_texture_bind_group_layout when we could just make it here? I
/// think making it more than once causes inconsistencies between bind groups.
pub fn make_chunk_render_pipeline(
    engine: &mut crate::engine::Engine,
    block_texture_bind_group_layout: &wgpu::BindGroupLayout,
    uniform_bind_group_layout: &wgpu::BindGroupLayout,
) -> Result<wgpu::RenderPipeline, Box<dyn std::error::Error>> {
    // describes how colors are stored and processed throughout the pipeline
    let color_states = [wgpu::ColorStateDescriptor {
        format: engine.get_swap_chain_descriptor().format,
        color_blend: wgpu::BlendDescriptor::REPLACE,
        alpha_blend: wgpu::BlendDescriptor::REPLACE,

        // write r, g, b, and a
        write_mask: wgpu::ColorWrite::ALL,
    }];

    // compile texture shaders
    let (vs_module, fs_module) = engine.compile_shader_modules(
        include_str!("../shaders/block.vert"),
        include_str!("../shaders/block.frag"),
    )?;

    let render_pipeline_layout =
        engine
            .get_device()
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                bind_group_layouts: &[&block_texture_bind_group_layout, &uniform_bind_group_layout],
            });

    let chunk_vertex_buffer_descriptors = super::Block::vertex_buffer_descriptors();

    let chunk_render_pipeline_descriptor = crate::utils::make_render_pipeline_descriptor(
        &render_pipeline_layout,
        &vs_module,
        &fs_module,
        &color_states,
        chunk_vertex_buffer_descriptors,
        true,
    );

    Ok(engine
        .get_device()
        .create_render_pipeline(&chunk_render_pipeline_descriptor))
}
