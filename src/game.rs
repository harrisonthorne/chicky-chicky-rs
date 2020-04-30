use crate::engine;
use crate::world::World;

pub struct Game {
    world: Option<World>,
    uniform_bind_group: wgpu::BindGroup,
    uniform_bind_group_layout: wgpu::BindGroupLayout,
    block_render_pipeline: wgpu::RenderPipeline,
}

impl Game {
    /// Decided to pass in bind groups and pipelines so that this file doesn't become too crowded.
    pub fn new(
        uniform_bind_group_layout: wgpu::BindGroupLayout,
        uniform_bind_group: wgpu::BindGroup,
        block_render_pipeline: wgpu::RenderPipeline,
    ) -> Self {
        Self {
            uniform_bind_group,
            uniform_bind_group_layout,
            block_render_pipeline,
            world: None,
        }
    }

    fn start_render_pass(
        &self,
        phase: RenderPhase,
        encoder: &mut wgpu::CommandEncoder,
        frame: &wgpu::TextureView,
        depth_texture: &wgpu::TextureView,
    ) {
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
        });
    }
}

impl engine::Runner for Game {
    fn render(
        &self,
        _device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        frame: &wgpu::TextureView,
        depth_texture: &wgpu::TextureView,
    ) {
        self.start_render_pass(RenderPhase::World, encoder, frame, depth_texture);
    }

    fn update(&mut self, _delta_sec: f32) -> bool {
        false
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
