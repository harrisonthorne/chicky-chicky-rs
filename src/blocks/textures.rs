use crate::engine;

pub struct BlockTexture {
    texture: engine::Texture,
    bind_group: wgpu::BindGroup,
}

impl BlockTexture {
    fn new(
        device: &wgpu::Device,
        bind_group_layout: &wgpu::BindGroupLayout,
        bytes: &[u8],
    ) -> Result<(Self, wgpu::CommandBuffer), Box<dyn std::error::Error>> {
        let (texture, cmd) = engine::Texture::from_bytes(device, bytes)?;
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: bind_group_layout,
            bindings: &[
                wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                wgpu::Binding {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                },
            ],
        });

        Ok((
            Self {
                texture,
                bind_group,
            },
            cmd,
        ))
    }

    pub fn get_bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }
}

pub struct BlockTextures {
    pub dirt: BlockTexture,
    pub stone: BlockTexture,
    pub grass: BlockTexture,
    pub sand: BlockTexture,
}

impl BlockTextures {
    pub fn default(
        device: &wgpu::Device,
        block_texture_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> (Self, Vec<wgpu::CommandBuffer>) {
        let (dirt, dirt_cmd) = BlockTexture::new(
            device,
            block_texture_bind_group_layout,
            include_bytes!("../../assets/images/blocks/dirt.png"),
        )
        .unwrap();

        let (stone, stone_cmd) = BlockTexture::new(
            device,
            block_texture_bind_group_layout,
            include_bytes!("../../assets/images/blocks/stone.png"),
        )
        .unwrap();

        let (grass, grass_cmd) = BlockTexture::new(
            device,
            block_texture_bind_group_layout,
            include_bytes!("../../assets/images/blocks/grass.png"),
        )
        .unwrap();

        let (sand, sand_cmd) = BlockTexture::new(
            device,
            block_texture_bind_group_layout,
            include_bytes!("../../assets/images/blocks/sand.png"),
        )
        .unwrap();

        let commands = vec![dirt_cmd, stone_cmd, grass_cmd, sand_cmd];

        (
            Self {
                dirt,
                stone,
                grass,
                sand,
            },
            commands,
        )
    }
}
