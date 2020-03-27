mod blocks;
mod characters;
mod game;
mod items;
mod maths;
mod sprite;
mod textures;
mod utils;
mod world;
mod traits;

const FPS: f32 = 60.0;

fn main() {
    init_packages_gl();

    gl.clear_color(0, 0.5, 0.7, 1);
    gl.enable(gl.DEPTH_TEST);
    // gl.Enable(gl.TEXTURE_2D);
    gl.depth_func(gl.LESS);
    gl.enable(gl.CULL_FACE);
    gl.cull_face(gl.BACK);

    last_update = time.Now();

    loop {
        now = time.Now();

        // run no faster than specified fps
        delta_seconds = (now - last_update).num_seconds();
        if delta_seconds < 1.0 / FPS {
            time.sleep(time.Second * time.duration((1.0/FPS) - delta_seconds));
            continue;
        }

        // logic
        game.logic(delta_seconds);

        // render
        gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);
        game.render();

        // input
        window.swap_buffers();
        glfw.poll_events();

        // update time
        last_update = now;
    }
}

fn init_packages_gl() {
    render.init_gl();
    blocks.init_gl();
    characters.init_gl();
    sprite.init_gl();
}
