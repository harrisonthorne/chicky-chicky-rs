use crate::characters;
use crate::render;

const FPS: f32 = 60.0;

fn start() {
    let chicken = characters::new_chicken();
    let character_in_control = chicken;
    let cam = render::Camera::new(Vec3{X:0, Y:0, Z:2}, 70, 800.0/600);
    let last = time::now();

    /// Initializes plane vao and vbo
    let plane_vao: u32;
    let plane_vbo: u32;

    let model_attr_location: u32;
    let texture_uniform: i32;
    let program: u32;

        (plane_vao, plane_vbo) = utils::new_texture_vao(render.texture_program(), &plane_vertices);

        model_attr_location = render::texture_program().locations.model_matrix_location();
        texture_uniform = gl.get_uniform_location(program, gl.str("tex\x00"));
        program = render::texture_program().id();


    last_update = time.Now();

    loop {
        now = time.Now();

        // run no faster than specified fps
        let delta_seconds = (now - last_update).num_seconds();
        if delta_seconds < 1.0 / FPS {
            time.sleep(time.Second * time.duration((1.0/FPS) - delta_seconds));
            continue;
        }

        logic(delta_seconds);
        render();

        // update time
        last_update = now;
    }
}

/// Performs logic for the game. This includes movement, physics,
/// clocks, animation, etc
fn logic(delta: f32, character_in_control: Character) {
    character_in_control.logic(delta);
}

// render renders the game.
fn render() {
    gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);
    character_in_control.render(cam);
    window.swap_buffers();
}
