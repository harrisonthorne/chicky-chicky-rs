use crate::render;
use crate::types;
use crate::utils;

pub const BLOCK_WIDTH: f32 = 0.5; // in meters

/// Block block block block block block block block
pub trait Block: Killable + Renderable {
    fn get_matrix() -> Mat4;
    fn set_matrix(mat: Mat4);

    fn set_grid_pos(pos: [i64; 3]);
    fn get_grid_pos() -> [i64; 3];
}

/// Initializes OpenGL-specific functionality for the
/// blocks package.
/// TODO This should be moved to the topmost function to be passed to other functions. It should
/// not be a public global. Refusing to make this function public.
pub fn init_gl() {
    let (cube_vao, cube_vbo) = utils.new_texture_vao(texture_program, &cube_vertices);
}

pub fn render_block(c: &render::Camera, mat: &Mat4, texture: u32) {
    gl.use_program(render.texture_program().id());
    gl.bind_vertex_array(cube_vao);

    c.set_program_attributes(render.texture_program());

    let model_attr_location = render.texture_program().locations.model_matrix_location();
    gl.uniform_matrix_4fv(model_attr_location, 1, false, &(*mat)[0]);

    let texture_uniform = render.texture_program().locations.texture_location();
    gl.uniform_1i(texture_uniform, 0); // number bound here must match the active texture
    gl.active_texture(gl.TEXTURE0);
    gl.bind_texture(gl.TEXTURE_2D, texture);

    // six faces times two triangles per face times three
    // vertices per triangle
    gl.bind_vertex_array(cube_vao);
    gl.draw_arrays(gl.TRIANGLES, 0, 6*2*3);
}
