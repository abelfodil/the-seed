mod dungeon;
mod element;
mod img;
mod rand;
mod world;
mod array;
mod num;

use dungeon::Dungeon;
use img::ToImage;
use ::rand::{rngs::StdRng, SeedableRng};

fn main() {
    let mut rng: StdRng = SeedableRng::seed_from_u64(0x0DDB1A5E5BAD5EEDu64);
    let dungeon = Dungeon::new(&mut rng, 1000);
    let world = dungeon.to_world();
    let _ = world.to_image().save("image.png");
}
