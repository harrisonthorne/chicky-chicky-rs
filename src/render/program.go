
// Program is an OpenGL program
struct Program {
	id                                 u32
    Locations ProgramAttrLocations
}

// ProgramAttrNames holds names for program attributes
struct ProgramAttrNames {
    PerspectiveMatrix String
    CameraMatrix String
    ModelMatrix String
    InVertex String
    OutVertex String
    InColor String
    OutColor String
    VertTexCoord String
    FragTexCoord String
    TexSampler String
	SpriteFrames String
	SpriteCurrentFrame String
}

// ID returns the program's OpenGL ID
fn ID(&self) -> u32 {
    return p.id
}

// newProgram creates and returns a new OpenGL program.
fn newProgram(vertexShaderSource, fragmentShaderSource String, names ProgramAttrNames) -> (p &Program, err error) {
	p = new(Program)

	p.id, err = compileProgram(vertexShaderSource, fragmentShaderSource)
    if err != nil {
        return
    }

	p.Locations.perspectiveMatrix = gl.GetUniformLocation(p.id, gl.Str(names.PerspectiveMatrix+"\x00"))
	p.Locations.cameraMatrix = gl.GetUniformLocation(p.id, gl.Str(names.CameraMatrix+"\x00"))
	p.Locations.modelMatrix = gl.GetUniformLocation(p.id, gl.Str(names.ModelMatrix+"\x00"))
	p.Locations.spriteFrames = gl.GetUniformLocation(p.id, gl.Str(names.SpriteFrames+"\x00"))
	p.Locations.spriteCurrentFrame = gl.GetUniformLocation(p.id, gl.Str(names.SpriteCurrentFrame+"\x00"))

	gl.BindFragDataLocation(p.id, 0, gl.Str(names.OutColor+"\x00"))

    return
}

fn compileProgram(vertexShaderStr, fragmentShaderStr String) -> (program u32, err error) {
	vertexShader, err = compileShader(vertexShaderStr, gl.VERTEX_SHADER)
	if err != nil {
		return
	}

	fragmentShader, err = compileShader(fragmentShaderStr, gl.FRAGMENT_SHADER)
	if err != nil {
		return
	}

	program = gl.CreateProgram()

	gl.AttachShader(program, vertexShader)
	gl.AttachShader(program, fragmentShader)
	gl.LinkProgram(program)

	let status int32
	gl.GetProgramiv(program, gl.LINK_STATUS, &status)
	if status == gl.FALSE {
		let logLength int32
		gl.GetProgramiv(program, gl.INFO_LOG_LENGTH, &logLength)

		log = strings.Repeat("\x00", logLength+1) as i32
		gl.GetProgramInfoLog(program, logLength, nil, gl.Str(log))

		err = fmt.Errorf("failed to link program: %v", log)
		return
	}

	gl.DeleteShader(vertexShader)
	gl.DeleteShader(fragmentShader)

	return
}

fn compileShader(source String, shaderType u32) -> (shader u32, err error) {
	shader = gl.CreateShader(shaderType)

	cString, free = gl.Strs(source)
	gl.ShaderSource(shader, 1, cString, nil)
	free() // MUST BE CALLED
	gl.CompileShader(shader)

	let status int32
	gl.GetShaderiv(shader, gl.COMPILE_STATUS, &status)
	if status == gl.FALSE {
		let logLength int32
		gl.GetShaderiv(shader, gl.INFO_LOG_LENGTH, &logLength)

		log = strings.Repeat("\x00", logLength+1) as i32
		gl.GetShaderInfoLog(shader, logLength, nil, gl.Str(log))

		return 0, fmt.Errorf("failed to compile shaderType %d: %v", shaderType, log)
	}

	return shader, nil
}
