use crate::camera::Camera;

/// Uniforms are used in the shader for attributes that are essentially global.
#[repr(C)] // we need this for Rust to store our data correctly for the shaders
#[derive(Copy, Clone)] // this is so we can store this in a buffer
pub struct Uniforms {
    /// The view-projection matrix.
    pub view_proj: cgmath::Matrix4<f32>,
}

impl Uniforms {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity(),
        }
    }

    fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = *camera.get_view_projection_matrix();
    }

    pub fn update(
        &mut self,
        device: &wgpu::Device,
        camera: &Camera,
        uniform_buffer: &mut wgpu::Buffer,
        queue: &mut wgpu::Queue,
    ) {
        self.update_view_proj(&camera);

        // Copy operations are performed on the gpu, so we'll need
        // a CommandEncoder for that
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("uniforms update command encoder"),
        });

        let staging_buffer = device
            .create_buffer_with_data(bytemuck::cast_slice(&[*self]), wgpu::BufferUsage::COPY_SRC);

        encoder.copy_buffer_to_buffer(
            &staging_buffer,
            0,
            &uniform_buffer,
            0,
            std::mem::size_of::<Uniforms>() as wgpu::BufferAddress,
        );

        // We need to remember to submit our CommandEncoder's output
        // otherwise we won't see any change.
        queue.submit(&[encoder.finish()]);
    }
}

unsafe impl bytemuck::Pod for Uniforms {}
unsafe impl bytemuck::Zeroable for Uniforms {}
