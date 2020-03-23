package blocks

use textures;

const baseDir = "./assets/photos/blocks/"

let (
    dirtTexture = textures.newLazyTexture(baseDir+"dirt.png")
    grassTexture = textures.newLazyTexture(baseDir+"grass.png")
    stoneTexture = textures.newLazyTexture(baseDir+"stone.png")
    sandTexture = textures.newLazyTexture(baseDir+"sand.png")
)
