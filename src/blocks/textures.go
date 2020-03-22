package blocks

use textures;

const baseDir = "./assets/photos/blocks/"

let (
    dirtTexture = textures.NewLazyTexture(baseDir+"dirt.png")
    grassTexture = textures.NewLazyTexture(baseDir+"grass.png")
    stoneTexture = textures.NewLazyTexture(baseDir+"stone.png")
    sandTexture = textures.NewLazyTexture(baseDir+"sand.png")
)
