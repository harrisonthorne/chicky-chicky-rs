use crate::blocks::Chunk;

pub(crate) struct Game {
    // world: Option<World>,
    tmp_chunk: Chunk,
}

impl Game {
    /// Decided to pass in bind groups and pipelines so that this file doesn't become too crowded.
    pub async fn new(device: &wgpu::Device) -> Self {
        Self {
            // world: None,
            tmp_chunk: Chunk::generate(0, 0, 0, &device).await,
        }
    }

    fn start_render_pass<'a>(
        phase: RenderPhase,
        payload: &'a mut crate::RenderPayload,
    ) -> wgpu::RenderPass<'a> {
        let color_attachments = &[wgpu::RenderPassColorAttachmentDescriptor {
            attachment: &payload.frame,
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
            _ => Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
                attachment: payload.depth_texture,
                depth_load_op: wgpu::LoadOp::Clear,
                depth_store_op: wgpu::StoreOp::Store,
                clear_depth: 1.0,
                stencil_load_op: wgpu::LoadOp::Clear,
                stencil_store_op: wgpu::StoreOp::Store,
                clear_stencil: 0,
            })
        };

        let mut pass = payload.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments,
            depth_stencil_attachment,
        });

        #[allow(clippy::single_match)]
        match phase {
            RenderPhase::World => {
                pass.set_pipeline(payload.block_render_pipeline);
                pass.set_bind_group(0, payload.block_texture_bind_group, &[]);
            }
            _ => {}
        }
        pass.set_bind_group(1, payload.uniform_bind_group, &[]);

        pass
    }

    pub fn logic(&mut self, device: &wgpu::Device, queue: &mut wgpu::Queue) {
        self.tmp_chunk.logic(device, queue);
    }

    pub fn render(&self, payload: &mut crate::RenderPayload) {
        let mut world_render_pass = Self::start_render_pass(
            RenderPhase::World,
            payload,
        );

        self.tmp_chunk.render(&mut world_render_pass);
    }
}

#[allow(dead_code)]
enum RenderPhase {
    /// Draw the world: blocks, weather, particles, and more.
    World,

    /// Draw characters.
    Characters,

    /// Draw the user interface: health bars, backpack view, buttons, et cetera.
    Interface,
}
