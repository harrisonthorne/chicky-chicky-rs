// use std::error::Error;
// use cgmath::Matrix4;

// /// An image that can be animated.
// pub struct Sprite {
//     texture:  u32,
//     frames:   i32,
//     uv_coords: (f32, f32),

//     current_frame:    f32,
//     seconds_per_frame: f32,

//     size_matrix:     Matrix4,
//     position_matrix: Matrix4,
//     matrix:         Matrix4,

//     pixel_width: i32,
//     pixel_height: i32,
// }

// /// Creates a new sprite and returns it
// impl Sprite {
//     pub fn new(sprite_path: String, frames: i32, seconds_per_frame: f32) -> Result<Self, Box<dyn Error>> {
//         let s: Self = Default::default();

//         if frames <= 0 {
//             frames = 1;
//             if seconds_per_frame <= 0 {
//                 seconds_per_frame = 1;
//             }
//         } else if seconds_per_frame <= 0 {
//             return Err("seconds_per_frame cannot be less than or equal to 0 if frames is greater than 0");
//         }

//         // open the sprite file
//         let sprite_file = fs::open(sprite_path)?;

//         // assign the sprite texture
//         s.texture = textures::new(sprite_file)?;

//         // initialize the rest of the fields
//         s.frames = frames;
//         s.seconds_per_frame = seconds_per_frame;

//         Ok(s)
//     }

//     /// Animates the Sprite.
//     fn animate(&self, delta: f32) {
//         // if one frame or less, animation doesn't matter
//         if self.frames <= 1 {
//             return
//         }
//         self.current_frame += delta / self.seconds_per_frame;
//         for self.current_frame >= self.frames {
//             self.current_frame -= self.frames;
//         }
//     }

//     /// Sets the size of the sprite.
//     fn set_size(&mut self, width: f32, height: f32) {
//         self.size_matrix = Matrix4::identity().scale(&[width, height, 1.]);
//         self.update_matrix();
//     }

//     fn get_pixel_width(&self) -> i32 {
//         self.pixel_width
//     }

//     fn get_pixel_height(&self) -> i32 {
//         self.pixel_height
//     }

//     /// Sets the position of the sprite.
//     fn set_position(&mut self, x: f32, y: f32, z: f32) {
//         self.position_matrix = Matrix4::identity().translate(&[x, y, z]);
//         self.update_matrix();
//     }

//     fn update_matrix(&mut self) {
//         self.matrix = self.position_matrix.mul4(self.size_matrix);
//     }

//     /// Renders the sprite onto the screen.
//     fn render(&self, c: &render::Camera, plane_vao: i32) {

//     }

// }

// const plane_vertices: [f32] = [
//     // first triangle
//     -0.5, 0.5, 0, 0, 0,
//     -0.5, -0.5, 0, 0, 1,
//     0.5, -0.5, 0, 1, 1,

//     // second triangle
//     -0.5, 0.5, 0, 0, 0,
//     0.5, -0.5, 0, 1, 1,
//     0.5, 0.5, 0, 1, 0,
// ];
