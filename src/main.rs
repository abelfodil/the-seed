mod array;
mod dungeon;
mod element;
mod geom;
mod num;
mod rand;
mod room;
mod world;

use crate::dungeon::Dungeon;
use crate::element::BasicElement;
use crate::world::World2D;

use ::image::RgbaImage;
use ::rand::{rngs::StdRng, SeedableRng};

fn main() {
    let mut rng: StdRng = SeedableRng::seed_from_u64(0x0DDB1A5E5BAD5EEDu64);
    let dungeon = Dungeon::new(&mut rng, 1000);
    let world: World2D<BasicElement> = dungeon.into();
    let image: RgbaImage = world.into();
    let _ = image.save("image.png");
}
