use super::errors::TextureError;
use std::path::Path;

pub struct Texture2d {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl Texture2d {
    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

    pub fn load<P: AsRef<Path>>(
        device: &wgpu::Device,
        path: P,
        label: Option<&str>,
    ) -> Result<(Self, wgpu::CommandBuffer), TextureError> {
        let img = image::open(path).map_err(TextureError::from_error)?;
        Self::from_image(device, img, label)
    }

    pub fn make_depth_texture(device: &wgpu::Device, sc_desc: &wgpu::SwapChainDescriptor) -> Self {
        let size = wgpu::Extent3d {
            width: sc_desc.width,
            height: sc_desc.height,
            depth: 1,
        };

        let desc = wgpu::TextureDescriptor {
            label: Some("depth texture"),
            size,
            array_layer_count: 1,
            mip_level_count: 1,
            sample_count: 1,

            dimension: wgpu::TextureDimension::D2,
            format: Self::DEPTH_FORMAT,

            // rendering to this texture
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT
                | wgpu::TextureUsage::SAMPLED
                | wgpu::TextureUsage::COPY_SRC,
        };
        let texture = device.create_texture(&desc);

        let view = texture.create_default_view();
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            compare: wgpu::CompareFunction::LessEqual,
        });

        Self {
            texture,
            view,
            sampler,
        }
    }

    pub fn from_bytes(
        device: &wgpu::Device,
        bytes: &[u8],
        label: Option<&str>,
    ) -> Result<(Self, wgpu::CommandBuffer), TextureError> {
        let img = image::load_from_memory(bytes)
            .map_err(|e| TextureError::from_error_with_detail(e, "loading image from bytes"))?;
        Self::from_image(device, img, label)
    }

    pub fn from_image(
        device: &wgpu::Device,
        img: image::DynamicImage,
        label: Option<&str>,
    ) -> Result<(Self, wgpu::CommandBuffer), TextureError> {
        let rgba = img.into_rgba();
        let dimensions = rgba.dimensions();

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,

            // only one texture here; depth is one
            depth: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            size,

            // multiple textures of the same size can be stored in one texture
            array_layer_count: 1,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,

            // SAMPLED: tells wgpu that we want to use this texture in shaders;
            // COPY_DST: this is our copy destination
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
            label,
        });

        // COPY_SRC: copy from this buffer
        let buffer = device
            .create_buffer_with_data(bytemuck::cast_slice(&rgba), wgpu::BufferUsage::COPY_SRC);

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("texture command encoder"),
        });

        encoder.copy_buffer_to_texture(
            wgpu::BufferCopyView {
                buffer: &buffer,
                offset: 0,
                bytes_per_row: 4 * dimensions.0,
                rows_per_image: dimensions.1,
            },
            wgpu::TextureCopyView {
                texture: &texture,
                mip_level: 0,
                array_layer: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            size,
        );

        let cmd_buffer = encoder.finish();

        // TextureView: offers us a *view* into our texture
        let view = texture.create_default_view();

        // Sampler: controls how the Texture is *sampled*.
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            compare: wgpu::CompareFunction::Always,
        });

        Ok((
            Texture2d {
                texture,
                view,
                sampler,
            },
            cmd_buffer,
        ))
    }
}
