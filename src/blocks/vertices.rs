use super::render::Vertex;

const HALF_BLOCK_WIDTH: f32 = super::Block::WIDTH / 2.0;

// TODO fix uv coordinates
// indices are not possible with differing uv coordinates
pub const CUBE_VERTICES: &[Vertex] = &[
    // Bottom
    Vertex {
        position: [-HALF_BLOCK_WIDTH, -HALF_BLOCK_WIDTH, -HALF_BLOCK_WIDTH],
        uv_coords: [0.5, 0.5],
    },
    Vertex {
        position: [HALF_BLOCK_WIDTH, -HALF_BLOCK_WIDTH, -HALF_BLOCK_WIDTH],
        uv_coords: [1.0, 0.5],
    },
    Vertex {
        position: [-HALF_BLOCK_WIDTH, -HALF_BLOCK_WIDTH, HALF_BLOCK_WIDTH],
        uv_coords: [0.5, 0.0],
    },
    Vertex {
        position: [HALF_BLOCK_WIDTH, -HALF_BLOCK_WIDTH, -HALF_BLOCK_WIDTH],
        uv_coords: [1.0, 0.5],
    },
    Vertex {
        position: [HALF_BLOCK_WIDTH, -HALF_BLOCK_WIDTH, HALF_BLOCK_WIDTH],
        uv_coords: [1.0, 0.0],
    },
    Vertex {
        position: [-HALF_BLOCK_WIDTH, -HALF_BLOCK_WIDTH, HALF_BLOCK_WIDTH],
        uv_coords: [0.5, 0.0],
    },
    // Top
    Vertex {
        position: [-HALF_BLOCK_WIDTH, HALF_BLOCK_WIDTH, -HALF_BLOCK_WIDTH],
        uv_coords: [0.0, 0.5],
    },
    Vertex {
        position: [-HALF_BLOCK_WIDTH, HALF_BLOCK_WIDTH, HALF_BLOCK_WIDTH],
        uv_coords: [0.0, 0.0],
    },
    Vertex {
        position: [HALF_BLOCK_WIDTH, HALF_BLOCK_WIDTH, -HALF_BLOCK_WIDTH],
        uv_coords: [0.5, 0.5],
    },
    Vertex {
        position: [HALF_BLOCK_WIDTH, HALF_BLOCK_WIDTH, -HALF_BLOCK_WIDTH],
        uv_coords: [0.5, 0.5],
    },
    Vertex {
        position: [-HALF_BLOCK_WIDTH, HALF_BLOCK_WIDTH, HALF_BLOCK_WIDTH],
        uv_coords: [0.0, 0.0],
    },
    Vertex {
        position: [HALF_BLOCK_WIDTH, HALF_BLOCK_WIDTH, HALF_BLOCK_WIDTH],
        uv_coords: [0.5, 0.0],
    },
    // Front
    Vertex {
        position: [-HALF_BLOCK_WIDTH, -HALF_BLOCK_WIDTH, HALF_BLOCK_WIDTH],
        uv_coords: [0.0, 1.0],
    },
    Vertex {
        position: [HALF_BLOCK_WIDTH, -HALF_BLOCK_WIDTH, HALF_BLOCK_WIDTH],
        uv_coords: [0.5, 1.0],
    },
    Vertex {
        position: [-HALF_BLOCK_WIDTH, HALF_BLOCK_WIDTH, HALF_BLOCK_WIDTH],
        uv_coords: [0.0, 0.5],
    },
    Vertex {
        position: [HALF_BLOCK_WIDTH, -HALF_BLOCK_WIDTH, HALF_BLOCK_WIDTH],
        uv_coords: [0.5, 1.0],
    },
    Vertex {
        position: [HALF_BLOCK_WIDTH, HALF_BLOCK_WIDTH, HALF_BLOCK_WIDTH],
        uv_coords: [0.5, 0.5],
    },
    Vertex {
        position: [-HALF_BLOCK_WIDTH, HALF_BLOCK_WIDTH, HALF_BLOCK_WIDTH],
        uv_coords: [0.0, 0.5],
    },
    // Back
    Vertex {
        position: [-HALF_BLOCK_WIDTH, -HALF_BLOCK_WIDTH, -HALF_BLOCK_WIDTH],
        uv_coords: [0.5, 1.0],
    },
    Vertex {
        position: [-HALF_BLOCK_WIDTH, HALF_BLOCK_WIDTH, -HALF_BLOCK_WIDTH],
        uv_coords: [0.5, 0.5],
    },
    Vertex {
        position: [HALF_BLOCK_WIDTH, -HALF_BLOCK_WIDTH, -HALF_BLOCK_WIDTH],
        uv_coords: [0.0, 1.0],
    },
    Vertex {
        position: [HALF_BLOCK_WIDTH, -HALF_BLOCK_WIDTH, -HALF_BLOCK_WIDTH],
        uv_coords: [0.0, 1.0],
    },
    Vertex {
        position: [-HALF_BLOCK_WIDTH, HALF_BLOCK_WIDTH, -HALF_BLOCK_WIDTH],
        uv_coords: [0.5, 0.5],
    },
    Vertex {
        position: [HALF_BLOCK_WIDTH, HALF_BLOCK_WIDTH, -HALF_BLOCK_WIDTH],
        uv_coords: [0.0, 0.5],
    },
    // Left
    Vertex {
        position: [-HALF_BLOCK_WIDTH, -HALF_BLOCK_WIDTH, HALF_BLOCK_WIDTH],
        uv_coords: [1.0, 1.0],
    },
    Vertex {
        position: [-HALF_BLOCK_WIDTH, HALF_BLOCK_WIDTH, -HALF_BLOCK_WIDTH],
        uv_coords: [0.5, 0.5],
    },
    Vertex {
        position: [-HALF_BLOCK_WIDTH, -HALF_BLOCK_WIDTH, -HALF_BLOCK_WIDTH],
        uv_coords: [0.5, 1.0],
    },
    Vertex {
        position: [-HALF_BLOCK_WIDTH, -HALF_BLOCK_WIDTH, HALF_BLOCK_WIDTH],
        uv_coords: [1.0, 1.0],
    },
    Vertex {
        position: [-HALF_BLOCK_WIDTH, HALF_BLOCK_WIDTH, HALF_BLOCK_WIDTH],
        uv_coords: [1.0, 0.5],
    },
    Vertex {
        position: [-HALF_BLOCK_WIDTH, HALF_BLOCK_WIDTH, -HALF_BLOCK_WIDTH],
        uv_coords: [0.5, 0.5],
    },
    // Right
    Vertex {
        position: [HALF_BLOCK_WIDTH, -HALF_BLOCK_WIDTH, HALF_BLOCK_WIDTH],
        uv_coords: [0.5, 1.0],
    },
    Vertex {
        position: [HALF_BLOCK_WIDTH, -HALF_BLOCK_WIDTH, -HALF_BLOCK_WIDTH],
        uv_coords: [1.0, 1.0],
    },
    Vertex {
        position: [HALF_BLOCK_WIDTH, HALF_BLOCK_WIDTH, -HALF_BLOCK_WIDTH],
        uv_coords: [1.0, 0.5],
    },
    Vertex {
        position: [HALF_BLOCK_WIDTH, -HALF_BLOCK_WIDTH, HALF_BLOCK_WIDTH],
        uv_coords: [0.5, 1.0],
    },
    Vertex {
        position: [HALF_BLOCK_WIDTH, HALF_BLOCK_WIDTH, -HALF_BLOCK_WIDTH],
        uv_coords: [1.0, 0.5],
    },
    Vertex {
        position: [HALF_BLOCK_WIDTH, HALF_BLOCK_WIDTH, HALF_BLOCK_WIDTH],
        uv_coords: [0.5, 0.5],
    },
];

pub fn make_cube_vertex_buffer(device: &wgpu::Device) -> wgpu::Buffer {
    device.create_buffer_with_data(
        bytemuck::cast_slice(CUBE_VERTICES),
        wgpu::BufferUsage::VERTEX,
    )
}
