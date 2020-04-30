pub fn make_render_pipeline_descriptor<'a>(
    render_pipeline_layout: &'a wgpu::PipelineLayout,
    vs_module: &'a wgpu::ShaderModule,
    fs_module: &'a wgpu::ShaderModule,
    color_states: &'a [wgpu::ColorStateDescriptor],
    vertex_buffers: &'a [wgpu::VertexBufferDescriptor<'a>],
    use_depth: bool,
) -> wgpu::RenderPipelineDescriptor<'a> {
    let rasterization_state = wgpu::RasterizationStateDescriptor {
        front_face: wgpu::FrontFace::Ccw,
        cull_mode: wgpu::CullMode::Back,
        depth_bias: 0,
        depth_bias_slope_scale: 0.0,
        depth_bias_clamp: 0.0,
    };

    let depth_stencil_state = if use_depth {
        Some(wgpu::DepthStencilStateDescriptor {
            format: crate::engine::Texture::DEPTH_FORMAT,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            stencil_front: wgpu::StencilStateFaceDescriptor::IGNORE,
            stencil_back: wgpu::StencilStateFaceDescriptor::IGNORE,
            stencil_read_mask: 0,
            stencil_write_mask: 0,
        })
    } else {
        None
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

        // depth
        depth_stencil_state,

        // for indices
        index_format: wgpu::IndexFormat::Uint32,

        vertex_buffers,
        sample_count: 1,

        // specifies which samples should be active; in this case, all
        // (!0 is a bitwise NOT of 0, so all bits are 1)
        sample_mask: !0,

        // covers antialiasing; setting to false for now
        alpha_to_coverage_enabled: false,
    }
}

