use crate::engine;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

pub struct BlockTexture {
    texture: engine::Texture,
    bind_group: wgpu::BindGroup,
}

impl BlockTexture {
    fn new(
        device: &wgpu::Device,
        bind_group_layout: &wgpu::BindGroupLayout,
        bytes: &[u8],
    ) -> Result<(Self, wgpu::CommandBuffer), Box<dyn Error>> {
        let (texture, cmd) = engine::Texture::from_bytes(device, bytes)
            .map_err(TextureFromBytesError::from_error)?;

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
            label: Some("block texture bind group"),
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
    pub fn default_textures(
        device: &wgpu::Device,
        block_texture_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Result<(Self, Vec<wgpu::CommandBuffer>), Box<dyn Error>> {
        let (dirt, dirt_cmd) = BlockTexture::new(
            device,
            block_texture_bind_group_layout,
            include_bytes!("../../assets/images/blocks/dirt.png"),
        )
        .map_err(|e| MakeTextureError::new("dirt", e))?;

        let (stone, stone_cmd) = BlockTexture::new(
            device,
            block_texture_bind_group_layout,
            include_bytes!("../../assets/images/blocks/stone.png"),
        )
        .map_err(|e| MakeTextureError::new("stone", e))?;

        let (grass, grass_cmd) = BlockTexture::new(
            device,
            block_texture_bind_group_layout,
            include_bytes!("../../assets/images/blocks/grass.png"),
        )
        .map_err(|e| MakeTextureError::new("grass", e))?;

        let (sand, sand_cmd) = BlockTexture::new(
            device,
            block_texture_bind_group_layout,
            include_bytes!("../../assets/images/blocks/sand.png"),
        )
        .map_err(|e| MakeTextureError::new("sand", e))?;

        let commands = vec![dirt_cmd, stone_cmd, grass_cmd, sand_cmd];

        Ok((
            Self {
                dirt,
                stone,
                grass,
                sand,
            },
            commands,
        ))
    }
}

#[derive(Debug)]
struct MakeTextureError {
    block_name: String,
    error: Box<dyn Error>,
}

impl MakeTextureError {
    fn new(block_name: &str, error: Box<dyn Error>) -> Self {
        Self {
            block_name: String::from(block_name),
            error,
        }
    }
}

impl Display for MakeTextureError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "error making texture for `{}`: {}",
            self.block_name, self.error
        )
    }
}

impl Error for MakeTextureError {}

#[derive(Debug)]
struct TextureFromBytesError {
    error: Box<dyn Error>,
}

impl TextureFromBytesError {
    fn from_error(error: impl Error + 'static) -> Self {
        TextureFromBytesError {
            error: Box::new(error),
        }
    }
}

impl Display for TextureFromBytesError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "couldn't make texture from bytes: {}", self.error)
    }
}

impl Error for TextureFromBytesError {}
