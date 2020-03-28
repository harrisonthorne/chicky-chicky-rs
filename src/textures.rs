use web_sys::WebGlRenderingContext;
use std::error::Error;
use fs;

/// LazyTexture will dynamically load a texture the instant
/// its ID is attempted to be accessed.
struct LazyTexture {
    path: String,
    id:   u32,
}

impl LazyTexture {
    fn new(image_path: String) -> Self {
        Self {
            path: image_path,
            id: 0,
        }
    }

    /// Returns the texture ID, creating the texture if
    /// necessary
    fn id(&mut self) -> Result<u32, Box<dyn Error>> {
        if self.id == 0 {
            let image_file = fs::open(self.path)?;
            self.id = new_texture(image_file)?;
        }

        self.id
    }
}



/// Creates a new texture with the image data from
/// the reader.
fn new_texture(gl: WebGlRenderingContext, image_reader: io::Reader) -> Result<u32, Box<dyn Error>> {
    let img = image::decode(image_reader)?;

    let rgba = image::new_rgba(img.bounds());
    if rgba.stride != rgba.rect.size().x*4 {
        return Err("unsupported stride")
    }

    draw::draw(rgba, rgba.bounds(), img, image::Point{0, 0}, draw::Over);

    let mut texture: u32;
    gl.gen_textures(1, &mut texture);
    gl.active_texture(gl.TEXTURE0);
    gl.bind_texture(gl.TEXTURE_2D, texture);
    gl.tex_parameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST);
    gl.tex_parameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.NEAREST);
    gl.tex_parameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
    gl.tex_parameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
    gl.tex_image_2d(
        gl.TEXTURE_2D,
        0,
        gl.RGBA,
        rgba.rect.Size().x,
        rgba.rect.Size().y,
        0,
        gl.RGBA,
        gl.UNSIGNED_BYTE,
        gl.ptr(rgba.pix));

    Ok(texture)
}

// Bind binds the provided texture for use with OpenGL.
pub fn bind(gl: WebGlRenderingContext, texture: u32) {
    gl.bind_texture(gl.TEXTURE_2D, texture);
}
