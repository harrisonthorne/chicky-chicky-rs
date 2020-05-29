use super::errors::TextureError;

pub struct Texture3d {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,

    layers: u32,
    label: Option<String>,
    size: wgpu::Extent3d,
}

impl Texture3d {
    pub fn new(
        device: &wgpu::Device,
        px_dimensions: (u32, u32),
        layers: u32,
        label: Option<&str>,
    ) -> Self {
        let size = wgpu::Extent3d {
            width: px_dimensions.0,
            height: px_dimensions.1,
            depth: layers,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label,
            size,
            array_layer_count: 1,
            mip_level_count: 1,
            sample_count: 1,

            dimension: wgpu::TextureDimension::D3,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
        });

        // TextureView: offers us a *view* into our texture
        // NOTE: If this 3D texture isn't updating after loading 2d images into it, consider
        // recreating this view after every update to this texture
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

        Self {
            texture,
            view,
            sampler,
            size,
            layers,
            label: label.map(String::from),
        }
    }

    pub fn set_layer_from_bytes(
        &self,
        device: &wgpu::Device,
        index: u32,
        bytes: &[u8],
    ) -> Result<wgpu::CommandBuffer, TextureError> {
        self.check_index(index)?;

        let img = image::load_from_memory(bytes).map_err(|e| {
            TextureError::from_error_with_detail(e, "setting layer from bytes (loading image)")
        })?;

        self.set_layer_from_image(device, index, img)
    }

    pub fn set_layer_from_image(
        &self,
        device: &wgpu::Device,
        index: u32,
        img: image::DynamicImage,
    ) -> Result<wgpu::CommandBuffer, TextureError> {
        self.check_index(index)?;

        let rgba = img.into_rgba();
        let dimensions = rgba.dimensions();

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
                texture: &self.texture,
                mip_level: 0,
                array_layer: index,
                origin: wgpu::Origin3d::ZERO,
            },
            self.size,
        );

        let cmd_buffer = encoder.finish();

        // NOTE: 3d texture not working as hoped? recreate `view` here

        Ok(cmd_buffer)
    }

    fn check_index(&self, index: u32) -> Result<(), TextureError> {
        if index >= self.layers {
            let label = if let Some(l) = &self.label {
                &l
            } else {
                "(no label)"
            };
            Err(TextureError::from_message(format!("tried to set a texture out of range: index {} on 3d texture `{}`, which has {} layers", index, label, self.layers)))
        } else {
            Ok(())
        }
    }
}
