use crate::blocks::Chunk;

pub(crate) struct Game {
    // world: Option<World>,
    tmp_chunk: Chunk,
}

impl Game {
    /// Decided to pass in bind groups and pipelines so that this file doesn't become too crowded.
    pub fn new() -> Self {
        Self {
            // world: None,
            tmp_chunk: Chunk::generate(0, 0, 0),
        }
    }

    fn start_render_pass<'a>(
        phase: RenderPhase,
        encoder: &'a mut wgpu::CommandEncoder,
        frame: &'a wgpu::TextureView,
        depth_texture: &'a wgpu::TextureView,
    ) -> wgpu::RenderPass<'a> {
        let color_attachments = &[wgpu::RenderPassColorAttachmentDescriptor {
            attachment: &frame,
            resolve_target: None,
            load_op: wgpu::LoadOp::Clear,
            store_op: wgpu::StoreOp::Store,
            clear_color: wgpu::Color {
                r: 0.1,
                g: 0.2,
                b: 0.3,
                a: 1.0,
            },
        }];

        // determine depth attachment. only ignore depth if the phase is the Interface phase.
        let depth_stencil_attachment = match phase {
            RenderPhase::Interface => None,
            _ => Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
                attachment: depth_texture,
                depth_load_op: wgpu::LoadOp::Clear,
                depth_store_op: wgpu::StoreOp::Store,
                clear_depth: 1.0,
                stencil_load_op: wgpu::LoadOp::Clear,
                stencil_store_op: wgpu::StoreOp::Store,
                clear_stencil: 0,
            }),
        };

        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments,
            depth_stencil_attachment,
        })
    }

    pub fn render(&self, payload: &mut crate::RenderPayload) {
        let mut world_render_pass: wgpu::RenderPass = Self::start_render_pass(
            RenderPhase::World,
            payload.encoder,
            payload.frame,
            payload.depth_texture,
        );

        world_render_pass.set_pipeline(payload.block_render_pipeline);

        self.tmp_chunk.render(
            payload.block_position_uniform_bind_group,
            payload.block_position_uniform_buffer,
            payload.cube_vertex_buffer,
            payload.device,
            payload.queue,
            &mut world_render_pass,
            payload.textures,
            payload.uniform_bind_group,
        );
    }
}

enum RenderPhase {
    /// Draw the world: blocks, weather, particles, and more.
    World,

    /// Draw characters.
    Characters,

    /// Draw the user interface: health bars, backpack view, buttons, et cetera.
    Interface,
}
