#![allow(dead_code)]

use crate::blocks::BlockType;
use crate::engine;
use crate::world::Direction;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

const BLOCK_TEXTURE_COUNT: u32 = 4;
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BlockTextureIndex {
    Dirt,
    Grass,
    Stone,
    Sand,
}

impl BlockTextureIndex {
    pub fn from_type_and_direction(
        ty: BlockType,
        direction: Direction,
    ) -> Result<Self, NoSuchBlockTextureError> {
        match direction {
            Direction::Up => Self::get_top(ty),
            Direction::Down => Self::get_bottom(ty),
            _ => Self::get_side(ty),
        }
    }

    fn get_top(ty: BlockType) -> Result<Self, NoSuchBlockTextureError> {
        Ok(match ty {
            BlockType::Sand => Self::Sand,
            BlockType::Dirt => Self::Dirt,
            BlockType::Grass => Self::Grass,
            BlockType::Stone => Self::Stone,
            _ => return Err(NoSuchBlockTextureError { for_type: ty }),
        })
    }

    fn get_bottom(ty: BlockType) -> Result<Self, NoSuchBlockTextureError> {
        Ok(match ty {
            BlockType::Sand => Self::Sand,
            BlockType::Dirt => Self::Dirt,
            BlockType::Grass => Self::Dirt,
            BlockType::Stone => Self::Stone,
            _ => return Err(NoSuchBlockTextureError { for_type: ty }),
        })
    }

    fn get_side(ty: BlockType) -> Result<Self, NoSuchBlockTextureError> {
        Ok(match ty {
            BlockType::Sand => Self::Sand,
            BlockType::Dirt => Self::Dirt,
            BlockType::Grass => Self::Grass,
            BlockType::Stone => Self::Stone,
            _ => return Err(NoSuchBlockTextureError { for_type: ty }),
        })
    }
}

impl TryFrom<BlockType> for BlockTextureIndex {
    type Error = NoSuchBlockTextureError;

    fn try_from(t: BlockType) -> Result<Self, Self::Error> {
        Ok(match t {
            BlockType::Dirt => Self::Dirt,
            BlockType::Grass => Self::Grass,
            BlockType::Stone => Self::Stone,
            BlockType::Sand => Self::Sand,
            _ => return Err(Self::Error { for_type: t }),
        })
    }
}

#[derive(Debug)]
pub struct NoSuchBlockTextureError {
    for_type: BlockType,
}

impl Display for NoSuchBlockTextureError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "no texture available for block type `{:?}`",
            self.for_type
        )
    }
}

impl Error for NoSuchBlockTextureError {}

pub struct BlockTextures {
    pub textures: engine::Texture3d,
    bind_group: wgpu::BindGroup,
}

impl BlockTextures {
    pub fn default_textures(
        device: &wgpu::Device,
        texture_dimensions: (u32, u32),
        block_texture_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Result<(Self, Vec<wgpu::CommandBuffer>), Box<dyn Error>> {
        let mut textures = engine::Texture3d::new(
            device,
            texture_dimensions,
            BLOCK_TEXTURE_COUNT,
            Some("block textures"),
        );

        let dirt_cmd = textures
            .set_layer_from_bytes(
                device,
                BlockTextureIndex::Dirt as u32,
                include_bytes!("../../assets/images/blocks/dirt.png"),
            )
            .map_err(|e| MakeTextureError::new("dirt", e))?;

        let grass_cmd = textures
            .set_layer_from_bytes(
                device,
                BlockTextureIndex::Grass as u32,
                include_bytes!("../../assets/images/blocks/grass.png"),
            )
            .map_err(|e| MakeTextureError::new("grass", e))?;

        let stone_cmd = textures
            .set_layer_from_bytes(
                device,
                BlockTextureIndex::Stone as u32,
                include_bytes!("../../assets/images/blocks/stone.png"),
            )
            .map_err(|e| MakeTextureError::new("stone", e))?;

        let sand_cmd = textures
            .set_layer_from_bytes(
                device,
                BlockTextureIndex::Sand as u32,
                include_bytes!("../../assets/images/blocks/sand.png"),
            )
            .map_err(|e| MakeTextureError::new("sand", e))?;

        let commands = vec![dirt_cmd, stone_cmd, grass_cmd, sand_cmd];

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: block_texture_bind_group_layout,
            bindings: &[
                wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&textures.view),
                },
                wgpu::Binding {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&textures.sampler),
                },
            ],
            label: Some("block texture bind group"),
        });

        Ok((
            Self {
                textures,
                bind_group,
            },
            commands,
        ))
    }

    pub fn get_bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }
}

#[derive(Debug)]
struct MakeTextureError {
    block_name: String,
    error: Box<dyn Error>,
}

impl MakeTextureError {
    fn new<E: Error + 'static>(block_name: &str, error: E) -> Self {
        Self {
            block_name: String::from(block_name),
            error: Box::new(error),
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
