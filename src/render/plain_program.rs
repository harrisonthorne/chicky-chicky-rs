const plain_shader_names: ProgramAttrNames = ProgramAttrNames {
    perspective_matrix: "perspective",
    camera_matrix: "camera",
    model_matrix: "model",
    in_color: "color",
    out_color: "outputColor",
    tex_sampler: "texSampler",
};

/// The source for the vertex shader of plain-color 3D programs
pub const vertex_plain_shader_source: String = format!(
    // {{{
    r#"
#version 330

uniform mat4 {perspective_matrix};
uniform mat4 {camera_matrix};
uniform mat4 {model_matrix};

in vec3 {in_vertex};
in vec4 {in_color};
out vec4 frag_color;

void main() {
    {frag_tex_coord} = {vert_tex_coord};
    frag_color = {in_color};
    gl_Position = {perspective_matrix} * {camera_matrix} * {model_matrix} * vec4({in_vertex}, 1);
}
"#,
    perspective_matrix = plain_shader_names.perspective_matrix,
    camera_matrix = plain_shader_names.camera_matrix,
    model_matrix = plain_shader_names.model_matrix,
    in_vertex = plain_shader_names.in_vertex,
    in_color = plain_shader_names.in_color,
    vert_tex_coord = plain_shader_names.vert_tex_coord,
    frag_tex_coord = plain_shader_names.frag_tex_coord,
    // }}}
);

/// The source for the texture shader program
pub const fragment_plain_shader_source: String = format!(
    // {{{
    r#"
#version 330

in vec4 frag_color;
out vec4 {out_color};

void main() {
    {out_color} = frag_color;
}
"#,
    out_color = plain_shader_names.out_color,
    tex_sampler = plain_shader_names.tex_sampler,
    frag_tex_coord = plain_shader_names.frag_tex_coord,
    // }}}
);

// vim: foldmethod=marker
