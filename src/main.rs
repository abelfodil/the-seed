mod dungeon;
mod element;
mod img;
mod random;
mod world;
mod array;

use dungeon::Dungeon;
use img::ToImage;
use rand::{rngs::StdRng, SeedableRng};

fn main() {
    let rng: StdRng = SeedableRng::seed_from_u64(0x0DDB1A5E5BAD5EEDu64);
    let dungeon = Dungeon::new(rng, 1000, 10);
    let world = dungeon.to_world();
    let _ = world.to_image().save("image.png");
}
