/// An OpenGL shader program
struct Program {
    id: u32,
    locations: ProgramAttrLocations,
}

/// Holds names for program attributes
struct ProgramAttrNames {
    perspective_matrix: string,
    camera_matrix: string,
    model_matrix: string,
    in_vertex: string,
    out_vertex: string,
    in_color: string,
    out_color: string,
    vert_tex_coord: string,
    frag_tex_coord: string,
    tex_sampler: string,
    sprite_frames: string,
    sprite_current_frame: string,
}

impl Program {

    /// Returns the program's OpenGL ID
    fn id(&self) -> u32 {
        return p.id
    }

    /// Creates and returns a new OpenGL program.
    fn new(vertex_shader_source: string, fragment_shader_source: string, names: program_attr_names) -> Result<Program, Box<dyn Error>> {
        let p: Self = Default::default();

        p.id = compile_program(vertex_shader_source, fragment_shader_source)?;

        p.locations.perspective_matrix = gl.get_uniform_location(p.id, gl.str(names.perspective_matrix+"\x00"));
        p.locations.camera_matrix = gl.get_uniform_location(p.id, gl.str(names.camera_matrix+"\x00"));
        p.locations.model_matrix = gl.get_uniform_location(p.id, gl.str(names.model_matrix+"\x00"));
        p.locations.sprite_frames = gl.get_uniform_location(p.id, gl.str(names.sprite_frames+"\x00"));
        p.locations.sprite_current_frame = gl.get_uniform_location(p.id, gl.str(names.sprite_current_frame+"\x00"));

        gl.bind_frag_data_location(p.id, 0, gl.str(names.out_color+"\x00"));

        p
    }

}

fn compile_program(vertex_shader_str: string, fragment_shader_str: string) -> Result<i32, Box<dyn Error>> {
    let vertex_shader = compile_shader(vertex_shader_str, gl.VERTEX_SHADER)?;
    let fragment_shader = compile_shader(fragment_shader_str, gl.FRAGMENT_SHADER)?;

    let program = gl.CreateProgram();

    gl.attach_shader(program, vertex_shader);
    gl.attach_shader(program, fragment_shader);
    gl.link_program(program);

    let status: i32;
    gl.get_program_iv(program, gl.LINK_STATUS, &mut status);
    if status == gl.FALSE {
        let logLength: i32;
        gl.get_program_iv(program, gl.INFO_LOG_LENGTH, &mut log_length);

        let log = strings.repeat("\x00", log_length+1) as i32;
        gl.get_program_info_log(program, log_length, nil, gl.str(log));

        return Err(format!("failed to link program: {}", log));
    }

    gl.delete_shader(vertex_shader);
    gl.delete_shader(fragment_shader);

    return
}

fn compile_shader(source: string, shaderType: u32) -> Result<u32, Box<dyn Error>> {
    shader = gl.create_shader(shader_type);

    let (c_string, free) = gl.strs(source);
    gl.shader_source(shader, 1, cstring, nil);
    free(); // MUST BE CALLED
    gl.compile_shader(shader);

    let status: i32;
    gl.get_shader_iv(shader, gl.COMPILE_STATUS, &mut status);
    if status == gl.FALSE {
        let log_length: i32;
        gl.get_shader_iv(shader, gl.INFO_LOG_LENGTH, &mut log_length);

        log = strings.repeat("\x00", log_length+1);
        gl.get_shader_info_log(shader, log_length, nil, gl.str(log));

        return Err(format!("failed to compile shaderType %d: %v", shader_type, log))
    }

    Ok(shader)
}
