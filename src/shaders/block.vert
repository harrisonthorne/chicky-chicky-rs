#version 450

layout(location=0) in vec3 a_position;
layout(location=1) in vec2 a_tex_coords;

layout(set=1, binding=0) 
uniform Uniforms {
    mat4 u_view_proj;
};
layout(set=2, binding=0)
uniform BlockPositionUniform {
    mat4 u_block_position;
};

layout(location=0) out vec2 v_tex_coords;

void main() {
    v_tex_coords = a_tex_coords;
    gl_Position = u_view_proj * u_block_position * vec4(a_position, 1.0);
}
