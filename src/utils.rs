use render;
use web_sys::WebGlRenderingContext;

/// Creates and returns a new vao and vbo built with
/// the vertices passed in. Each vertex should consist of
/// five f32 values. The first three values are XYZ
/// coordinates. The last two are UV coordinates for textures
fn new_texture_vao(gl: WebGlRenderingContext, program: render::Program, vertices: &[f32]) -> (u32, u32) {
    let mut vao: u32 = 0;
    let mut vbo: u32 = 0;

    // create a vertex array object
    gl.gen_vertex_arrays(1, &mut vao);
    gl.bind_vertex_array(vao);

    // create a vertex buffer object
    gl.gen_buffers(1, &mut vbo);
    gl.bind_buffer(gl.ARRAY_BUFFER, vbo);
    gl.buffer_data(gl.ARRAY_BUFFER, (*vertices.len())*4, gl.Ptr(*vertices), gl.STATIC_DRAW);

    // attribute pointers for the texture program
    let vert_attrib = gl.get_attrib_location(program.ID(), gl.Str("vert\x00"));
    gl.enable_vertex_attrib_array(vert_attrib);
    gl.vertex_attrib_pointer(vert_attrib, 3, gl.FLOAT, false, 5*4, gl.ptr_offset(0));

    let tex_coord_attrib = gl.get_attrib_location(program.ID(), gl.str("vertTexCoord\x00"));
    gl.enable_vertex_attrib_array(tex_coord_attrib);
    gl.vertex_attrib_pointer(tex_coord_attrib, 2, gl.FLOAT, false, 5*4, gl.ptr_offset(3*4));

    gl.bind_vertex_array(0);
    gl.bind_buffer(gl.ARRAY_BUFFER, 0);

    (vao, vbo)
}
