let texture_shader: Program;

/// Returns the texture program
fn texture_program() -> &Program {
    &texture_shader
}

fn init_texture_shader() -> Result<(), Box<dyn Error>> {
    textureShader, err = Program::new(vertex_texture_shader_source, fragment_texture_shader_source, texture_shader_names)

    if err != nil {
        println!("vertexTextureShaderSource:");
        println!(vertex_texture_shader_source);
        println!("fragmentTextureShaderSource:");
        println!(fragment_texture_shader_source);
        panic!(err);
    }
}

let texture_shader_names = ProgramAttrNames{
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
}

// VertexTextureShaderSource is the source for the vertex shader of
// all 3D programs
// {{{
let vertexTextureShaderSource = `
#version 330

uniform mat4 ` + textureShaderNames.PerspectiveMatrix + `;
uniform mat4 ` + textureShaderNames.CameraMatrix + `;
uniform mat4 ` + textureShaderNames.ModelMatrix + `;

in vec3 ` + textureShaderNames.InVertex + `;
in vec2 ` + textureShaderNames.VertTexCoord + `;

out vec2 ` + textureShaderNames.FragTexCoord + `;

void main() {
    ` + textureShaderNames.FragTexCoord + ` = ` + textureShaderNames.VertTexCoord + `;
    gl_Position = ` + textureShaderNames.PerspectiveMatrix + ` * ` + textureShaderNames.CameraMatrix + ` * ` + textureShaderNames.ModelMatrix + ` * vec4(` + textureShaderNames.InVertex + `, 1);
}
` + "\x00" // any String being passed to OpenGL needs to terminate with the null character
// }}}

// FragmentTextureShaderSource is the source for the texture
// shader program
// {{{
let fragmentTextureShaderSource = `
#version 330

uniform sampler2D ` + textureShaderNames.TexSampler + `;

uniform i32 ` + textureShaderNames.SpriteFrames + `;
uniform i32 ` + textureShaderNames.SpriteCurrentFrame + `;

in vec2 ` + textureShaderNames.FragTexCoord + `;

out vec4 ` + textureShaderNames.OutColor + `;

void main() {
	vec4 texColor;
	if (` + textureShaderNames.SpriteFrames + ` == 0) {
		texColor = texture(` + textureShaderNames.TexSampler + `, ` + textureShaderNames.FragTexCoord + `);
	} else {
		float width = 1.0 / ` + textureShaderNames.SpriteFrames + `;
		float spriteStartX = width * ` + textureShaderNames.SpriteCurrentFrame + `;
		float texX = spriteStartX + float(` + textureShaderNames.FragTexCoord + `.x) / ` + textureShaderNames.SpriteFrames + `;
		texColor = texture(` + textureShaderNames.TexSampler + `, vec2(texX, ` + textureShaderNames.FragTexCoord + `.y));
	}
	if (texColor.a < 0.05)
		discard;

	` + textureShaderNames.OutColor + ` = texColor;
}
` + "\x00"

// }}}

// vim: foldmethod=marker
