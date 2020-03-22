use crate::render;
use webgl_matrix::Mat4;

const CHUNK_SIZE: i32 = 64;

// Chunk contains a three-dimensional array of blocks
struct Chunk {
    blocks: [[[Option<Box<dyn Block>>; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
    grid_x: i64,
    grid_y: i64,
    grid_z: i64,
    matrix: Mat4::identity(),
}

impl Chunk {
    fn new(grid_x: i64, grid_y: i64, grid_z: i64) -> Self {
        let c = Self {
            blocks: [[[None; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
            grid_x,
            grid_y,
            grid_z,
            matrix: Matrix::new();
        };

        for i in 0.. p.blocks.len() {
            for j in 0..p.blocks[1].len() {
                for k in 0..p.blocks[i][j].len(); k++ {
                    let block = GrassBlock::new();
                    let (x, y, z) = (i*BLOCK_WIDTH, j*BLOCK_WIDTH, -k*BLOCK_WIDTH);
                    block.set_matrix(mgl.ident4().mul4(mgl.translate3D(x, y, z)));
                    c.set(i, j, k, block);
                }
            }
        }


    }

    /// At returns the block at the array position
    fn at(&self, i: i32, j: i32, k: i32) *Block {
        self.blocks[i][j][k]
    }

    // Set sets the block at the array index
    fn set(&mut self, i, j, k i32, b: impl Block) {
        self.blocks[i][j][k] = &b;
    }

    // render renders Chunk p
    fn render(&self, c: render::Camera) {
        for i in 0..self.blocks.len(); i++ {
            for j in 0..self.blocks[i].len(); j++ {
                for k 0..self.blocks[i][j].len(); k++ {
                    match self.blocks[i][j][k] {
                        None => continue,
                        Some(block) => block.render(c);
                    }
                }
            }
        }
    }
}

