mod element;
mod img;
mod random;
mod world;

use element::BasicElement;
use img::ToImage;
use world::World2D;

fn main() {
    let mut world: World2D<BasicElement> = World2D::new_random((200, 300), 0x0DDB1A5E5BAD5EEDu64);
    world.randomize();
    let _ = world.to_image().save("image.png");
}
