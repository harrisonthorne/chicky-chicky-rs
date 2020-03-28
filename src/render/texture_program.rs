const texture_shader_names: ProgramAttrNames = ProgramAttrNames {
    perspective_matrix: "perspective",
    camera_matrix: "camera",
    model_matrix: "model",
    in_vertex: "vert",
    vert_tex_coord: "vertTexCoord",
    frag_tex_coord: "fragTexCoord",
    tex_sampler: "tex",
    out_color: "outputColor",
    sprite_frames: "spriteFrames",
    sprite_current_frame: "spriteCurrentFrame",
};

/// The source for the vertex shader of all 3D programs
pub const vertex_texture_shader_source: String = format!(
    // {{{
    r#"
#version 330

uniform mat4 {perspective_matrix};
uniform mat4 {camera_matrix};
uniform mat4 {model_matrix};

in vec3 {in_vertex};
in vec2 {vert_tex_coord};

out vec2 {frag_tex_coord};

void main() {
    {frag_tex_coord} = {vert_tex_coord};
    gl_Position = {perspective_matrix} * {camera_matrix} * {model_matrix} * vec4({in_vertex}, 1);
}
"#,
    perspective_matrix = texture_shader_names.perspective_matrix,
    camera_matrix = texture_shader_names.camera_matrix,
    model_matrix = texture_shader_names.model_matrix,
    in_vertex = texture_shader_names.in_vertex,
    vert_tex_coord = texture_shader_names.vert_tex_coord,
    frag_tex_coord = texture_shader_names.frag_tex_coord,
    // }}}
);

/// The source for the texture shader program
pub const fragment_texture_shader_source: String = format!(
    // {{{
    r#"
#version 330

uniform sampler2D {tex_sampler};

uniform int {sprite_frames};
uniform int {sprite_current_frame};

in vec2 {frag_tex_coord};

out vec4 {out_color};

void main() {
        vec4 texColor;
        if ({sprite_frames} == 0) {
                texColor = texture({tex_sampler}, {frag_tex_coord});
        } else {
                float width = 1.0 / {sprite_frames};
                float spriteStartX = width * {sprite_current_frame};
                float texX = spriteStartX + float({frag_tex_coord}.x) / {sprite_frames};
                texColor = texture({tex_sampler}, vec2(texX, {frag_tex_coord}.y));
        }
        if (texColor.a < 0.05) discard;

        {out_color} = texColor;
}
"#,
    sprite_frames = texture_shader_names.sprite_frames;
    sprite_current_frame = texture_shader_names.sprite_current_frame;
    out_color = texture_shader_names.out_color,
    tex_sampler = texture_shader_names.tex_sampler,
    frag_tex_coord = texture_shader_names.frag_tex_coord,
    // }}}
);

// vim: foldmethod=marker
