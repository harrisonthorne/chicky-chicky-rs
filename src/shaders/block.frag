#version 450

layout(location=0) in vec2 v_tex_coords;
layout(location=1) flat in float v_layer;

layout(location=0) out vec4 f_color;

// uniform uses `set` and `binding`
// set=0 corresponds to the first parameter in set_bind_group.
// binding=0 relates the binding specified when creating the BindGroupLayout
// and BindGroup.
layout(set=0, binding=0) uniform texture3D t_diffuse;
layout(set=0, binding=1) uniform sampler s_diffuse;

void main() {
    f_color = texture(sampler3D(t_diffuse, s_diffuse), vec3(v_tex_coords, v_layer));
}
